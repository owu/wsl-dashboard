// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Version expiry check task (oneshot)
//
// Starts after a 3-second delay. Polls AppState.bootstrap_data for up to 20 seconds.
// Once unix_time is available, compares against APP_EXPIRE_TIME.
// If expired, shows the expiration dialog unconditionally (highest priority, not subject to DND suppression).
// Records DND timestamp after showing the dialog to suppress subsequent low-priority dialogs.

use tracing::{info, warn};
use crate::AppWindow;
use crate::app::task_scheduler::{ScheduledTask, TaskInterval};

const MAX_POLL_ATTEMPTS: u32 = 100; // 100 × 200ms = 20s
const INITIAL_DELAY_SECS: u64 = 3;

pub struct VersionExpiryTask {
    pub app_state: std::sync::Arc<tokio::sync::Mutex<crate::AppState>>,
}

#[async_trait::async_trait]
impl ScheduledTask for VersionExpiryTask {
    fn name(&self) -> &str {
        "version_expiry"
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

        // Delay 3 seconds before checking
        tokio::time::sleep(std::time::Duration::from_secs(INITIAL_DELAY_SECS)).await;

        // Parse APP_EXPIRE_TIME (compile-time env var)
        let expire_time_str = env!("APP_EXPIRE_TIME");
        let expire_time: i64 = expire_time_str.parse().unwrap_or(0);

        if expire_time <= 0 {
            info!("expiry: APP_EXPIRE_TIME not set, skipping");
            return Ok(());
        }

        // Poll AppState.bootstrap_data for unix_time (up to 30 seconds)
        let mut now: i64 = 0;
        for _ in 0..MAX_POLL_ATTEMPTS {
            {
                let state = self.app_state.lock().await;
                if let Some(ref b) = state.bootstrap_data {
                    if b.unix_time > 0 {
                        now = b.unix_time;
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        if now <= 0 {
            warn!("expiry: bootstrap_data.unix_time unavailable after 30s, skipping");
            return Ok(());
        }

        info!("expiry: unix_time={}, expire_time={}", now, expire_time);

        if now > expire_time {
            // WSL compat dialog has highest priority: if it already showed (DND recorded), skip the expiry dialog
            {
                let state = self.app_state.lock().await;
                if !state.dialog_dnd.can_show_dialog() {
                    info!("expiry: compat dialog already shown (DND active), skipping expiry dialog");
                    return Ok(());
                }
            }

            // Record DND timestamp (suppresses subsequent low-priority dialogs)
            let mut state = self.app_state.lock().await;
            state.dialog_dnd.record_dialog_shown();
            drop(state);

            info!("expiry: app expired! Showing expiration dialog.");
            let ah = app_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah.upgrade() {
                    app.set_show_expire_dialog(true);
                }
            });
        }

        Ok(())
    }
}
