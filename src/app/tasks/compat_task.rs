// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// WSL version compatibility check task (oneshot, highest priority)
//
// Starts immediately (0s delay). Polls AppState.bootstrap_data for up to 15 seconds.
// Once bootstrap_data is ready (unix_time > 0), checks wsl_support configuration.
// If enabled and since_version is set, compares user's WSL version against the
// [since_version, until_version] range. If outside range, force-shows the compat dialog
// (not subject to DND suppression) and records DND timestamp.

use tracing::{info, warn};
use crate::AppWindow;
use crate::app::task_scheduler::{ScheduledTask, TaskInterval};

const MAX_POLL_ATTEMPTS: u32 = 75; // 75 × 200ms = 15s

pub struct WslCompatTask {
    pub app_state: std::sync::Arc<tokio::sync::Mutex<crate::AppState>>,
}

#[async_trait::async_trait]
impl ScheduledTask for WslCompatTask {
    fn name(&self) -> &str {
        "wsl_compat"
    }

    fn interval(&self) -> TaskInterval {
        TaskInterval::Custom(1)
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

        // Run the actual check, then ALWAYS signal completion regardless of outcome.
        // This unblocks VersionExpiryTask and UpdateCheckTask from their compat_task_done wait,
        // even when compat_task exits early (disabled, detection_failed, version in range, etc.)
        let result = self.do_check(app_handle).await;

        {
            let state = self.app_state.lock().await;
            state.compat_task_done.store(true, std::sync::atomic::Ordering::Release);
            info!("wsl_compat: compat_task_done signaled");
        }

        result
    }
}

impl WslCompatTask {
    async fn do_check(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        // Poll bootstrap_data until ready (unix_time > 0)
        let mut wsl_support: Option<crate::api::models::WslSupport> = None;
        for i in 0..MAX_POLL_ATTEMPTS {
            let state = self.app_state.lock().await;
            if let Some(ref b) = state.bootstrap_data {
                if b.unix_time > 0 {
                    wsl_support = Some(b.wsl_support.clone());
                    info!("wsl_compat: bootstrap data ready after {}ms", i * 200);
                    break;
                }
            }
            drop(state);
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        let ws = match wsl_support {
            Some(ws) => ws,
            None => {
                info!("wsl_compat: bootstrap_data unavailable after 15s, skipping");
                return Ok(());
            }
        };

        // Check if enabled
        if !ws.enabled {
            info!("wsl_compat: wsl_support.enabled=false, skipping");
            return Ok(());
        }

        // Check if since_version is set
        if ws.since_version.is_empty() {
            info!("wsl_compat: since_version is empty, skipping");
            return Ok(());
        }

        // Get WSL version
        let executor = {
            let state = self.app_state.lock().await;
            state.wsl_dashboard.executor().clone()
        };

        let version_meta = crate::wsl::ops::config::check_wsl_version_support(&executor).await;
        if version_meta.detection_failed {
            warn!("wsl_compat: WSL version detection failed, skipping");
            return Ok(());
        }

        let current = &version_meta.version_string;
        let since = &ws.since_version;
        let until = &ws.until_version;

        info!("wsl_compat: current={}, range=[{}, {}]", current, since, until);

        // Compare: current >= since && current <= until
        let in_range = version_gte(current, since) && version_lte(current, until);

        if !in_range {
            warn!("wsl_compat: version {} not in range [{}, {}]", current, since, until);

            // Highest priority, not subject to DND suppression
            // Only record DND timestamp to suppress subsequent low-priority dialogs
            {
                let mut state = self.app_state.lock().await;
                state.dialog_dnd.record_dialog_shown();
            }

            // Build dialog text
            let msg1 = crate::i18n::tr("dialog.wsl_compat_msg1", &[current.to_string(), since.to_string(), until.to_string()]);
            let msg2 = crate::i18n::t("dialog.wsl_compat_msg2");
            let msg3 = crate::i18n::tr("dialog.wsl_compat_msg3", &[until.to_string()]);
            let hint_text = crate::i18n::t("dialog.wsl_compat_issues_hint");
            let link_text = crate::i18n::t("dialog.wsl_compat_issues_link");
            let issues_url = format!("{}{}", crate::app::PROJECT_REPOSITORY, crate::app::GITHUB_ISSUES);
            let show_confirm = ws.confirm != 0;

            info!("wsl_compat: showing compat dialog (show_confirm={})", show_confirm);

            let ah = app_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah.upgrade() {
                    app.set_wsl_compat_msg1(msg1.into());
                    app.set_wsl_compat_msg2(msg2.into());
                    app.set_wsl_compat_msg3(msg3.into());
                    app.set_wsl_compat_hint(hint_text.into());
                    app.set_wsl_compat_link(link_text.into());
                    app.set_wsl_compat_issues_url(issues_url.into());
                    app.set_wsl_compat_show_confirm(show_confirm);
                    app.set_show_wsl_compat_dialog(true);
                }
            });
        } else {
            info!("wsl_compat: version {} is within compatible range", current);
        }

        Ok(())
    }
}

// Parse version string into Vec<u64>, supports 2-4 segments
fn parse_version(s: &str) -> Vec<u64> {
    s.split('.')
        .filter_map(|p| p.parse().ok())
        .collect()
}

// Check if a >= b
fn version_gte(a: &str, b: &str) -> bool {
    let a_parts = parse_version(a);
    let b_parts = parse_version(b);
    let len = a_parts.len().max(b_parts.len());
    for i in 0..len {
        let av = a_parts.get(i).copied().unwrap_or(0);
        let bv = b_parts.get(i).copied().unwrap_or(0);
        if av > bv { return true; }
        if av < bv { return false; }
    }
    true // Equal
}

// Check if a <= b
fn version_lte(a: &str, b: &str) -> bool {
    let a_parts = parse_version(a);
    let b_parts = parse_version(b);
    let len = a_parts.len().max(b_parts.len());
    for i in 0..len {
        let av = a_parts.get(i).copied().unwrap_or(0);
        let bv = b_parts.get(i).copied().unwrap_or(0);
        if av < bv { return true; }
        if av > bv { return false; }
    }
    true // Equal
}
