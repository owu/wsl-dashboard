// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Update check task (oneshot)
//
// Runs after a 5-second delay. Checks settings.check_time interval first.
// If interval has passed and window is visible, checks for updates via API.
// If an update is found, checks DND before showing the dialog.
// Records DND timestamp after showing the dialog.
// Always updates AppInfo properties for the About page.

use tracing::{info, warn};
use slint::ComponentHandle;
use crate::{AppWindow, AppInfo};
use crate::app::task_scheduler::{ScheduledTask, TaskInterval};

pub struct UpdateCheckTask {
    pub app_state: std::sync::Arc<tokio::sync::Mutex<crate::AppState>>,
}

#[async_trait::async_trait]
impl ScheduledTask for UpdateCheckTask {
    fn name(&self) -> &str {
        "update_check"
    }

    fn interval(&self) -> TaskInterval {
        TaskInterval::Custom(1) // minimum interval (oneshot, executes once)
    }

    fn requires_window_visible(&self) -> bool {
        false // visibility is handled inside execute() via wait_for_window_visible
    }

    fn is_oneshot(&self) -> bool {
        true
    }

    async fn execute(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        let result = self.do_check(app_handle).await;

        // Signal downstream tasks (sync_monitor) that the startup checks are done.
        // SyncMonitorTask waits on this flag before checking DND, ensuring it has the lowest priority.
        {
            let state = self.app_state.lock().await;
            state.startup_checks_done.store(true, std::sync::atomic::Ordering::Release);
            info!("update_check: startup_checks_done signaled");
        }

        result
    }
}

impl UpdateCheckTask {
    async fn do_check(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        // Ultimate gate: wait for window visibility at the very start to preserve startup timing
        crate::app::tasks::wait_for_window_visible(app_handle).await;

        // Wait for WslCompatTask to finish (up to 30s) so its DND is recorded before us, preserving the priority chain
        {
            let flag = {
                let state = self.app_state.lock().await;
                state.compat_task_done.clone()
            };
            for _ in 0..150u32 { // 150 × 200ms = 30s timeout
                if flag.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }

        // Delay 5 seconds before checking
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let current_v = env!("CARGO_PKG_VERSION");

        // Read settings
        let (last_check_time, check_update_days) = {
            let state = self.app_state.lock().await;
            let settings = state.config_manager.get_settings();
            (
                settings.check_time.parse::<i64>().unwrap_or(0),
                settings.check_update as i64,
            )
        };

        // Check if interval has been reached
        let now_ms = chrono::Utc::now().timestamp_millis();
        let interval_ms = check_update_days * 24 * 60 * 60 * 1000;
        if (now_ms - last_check_time) < interval_ms {
            info!("update_check: interval not reached, skipping");
            return Ok(());
        }

        info!("update_check: interval reached, checking for updates...");

        // Set checking_update = true
        {
            let ah = app_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah.upgrade() {
                    app.global::<AppInfo>().set_checking_update(true);
                }
            });
        }

        match crate::app::updater::check_update(current_v).await {
            Ok(result) => {
                let has_update = result.has_update;
                let latest_version = result.latest_version.clone();
                let release_date = result.release_date.clone();
                let download_url = format!(
                    "{}{}",
                    crate::app::PROJECT_WEBSITE,
                    crate::app::DOWNLOAD_URI
                );

                // Always update AppInfo properties (used by About page)
                let ah = app_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        app.global::<AppInfo>().set_checking_update(false);
                        if has_update {
                            app.global::<AppInfo>().set_has_update(true);
                            app.global::<AppInfo>().set_latest_version(latest_version.into());
                            app.global::<AppInfo>().set_latest_release_date(release_date.into());
                            app.global::<AppInfo>().set_update_download_url(download_url.into());
                        }
                    }
                });

                if has_update {
                    // DND check
                    let should_show = {
                        let state = self.app_state.lock().await;
                        state.dialog_dnd.can_show_dialog()
                    };

                    if !should_show {
                        info!("update_check: update available but DND active, suppressing dialog");
                        return Ok(());
                    }

                    // Record DND timestamp and show dialog
                    {
                        let mut state = self.app_state.lock().await;
                        state.dialog_dnd.record_dialog_shown();
                    }

                    info!("update_check: update available, showing dialog");
                    let ah = app_handle.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.set_show_update_dialog(true);
                        }
                    });

                    // Update check_time
                    let mut state = self.app_state.lock().await;
                    let _ = state.config_manager.update_check_time();
                }
            }
            Err(e) => {
                if e == "RequestTimeOut" {
                    warn!("update_check: timed out");
                } else {
                    warn!("update_check: failed: {}", e);
                }
                let ah = app_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        app.global::<AppInfo>().set_checking_update(false);
                    }
                });
            }
        }

        Ok(())
    }
}
