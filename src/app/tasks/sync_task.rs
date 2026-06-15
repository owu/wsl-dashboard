// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Sync monitoring scheduled tasks
//
// wslui_helper_sync() returns both popup and messages.
// 1. PopupSyncTask: Handles popups, part of the DND priority chain (Priority 4).
// 2. MessageSyncTask: Handles messages (red dot on UI), ignores DND, runs early.

use tracing::{debug, info};
use crate::AppWindow;
use crate::app::task_scheduler::{ScheduledTask, TaskInterval};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// ============================================================================
// Popup Sync Task
// ============================================================================

pub struct PopupSyncTask {
    pub app_state: Arc<tokio::sync::Mutex<crate::AppState>>,
    pub is_running: Arc<AtomicBool>,
}

#[async_trait::async_trait]
impl ScheduledTask for PopupSyncTask {
    fn name(&self) -> &str {
        "popup_sync"
    }

    fn interval(&self) -> TaskInterval {
        TaskInterval::Custom(60) // 60s interval, since data is from memory cache
    }

    fn requires_window_visible(&self) -> bool {
        // Return false so the task is scheduled even when window is hidden.
        // Visibility is handled inside execute() with a lock to prevent memory leaks.
        false
    }

    async fn execute(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        // 1. Check lock: prevents memory leak of blocked tasks when window is hidden
        if self.is_running.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            debug!("popup_sync: previous instance still waiting or running, skipping this tick");
            return Ok(());
        }

        // Use a wrapper to ensure lock is released
        let result = self.do_check(app_handle).await;

        // Release lock
        self.is_running.store(false, Ordering::Release);

        result
    }
}

impl PopupSyncTask {
    async fn do_check(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        // 2. Wait for window to become visible (safe now due to is_running lock)
        crate::app::tasks::wait_for_window_visible(app_handle).await;

        // 3. Call API (blocking call via spawn_blocking)
        let sync_data = tokio::task::spawn_blocking(|| {
            crate::api::common::wslui_helper_sync()
        })
        .await
        .map_err(|e| format!("spawn_blocking failed: {}", e))?;

        debug!("popup_sync: received data from wslui_helper_sync");

        // 4. Wait for startup checks (compat, expiry, update) to finish to enforce priority 4
        {
            let flag = {
                let state = self.app_state.lock().await;
                state.startup_checks_done.clone()
            };
            for _ in 0..150u32 { // 150 × 200ms = 30s timeout
                if flag.load(Ordering::Acquire) {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }

        // 5. Check DND
        let should_show = {
            let state = self.app_state.lock().await;
            state.dialog_dnd.can_show_dialog()
        };

        if !should_show {
            info!("popup_sync: sync data available but DND active, suppressing popup");
            return Ok(());
        }

        // Determine if there is actually a popup to show that requires breaking DND
        let has_popup = sync_data.popup.enable == 1;

        if has_popup {
            // Record DND
            {
                let mut state = self.app_state.lock().await;
                state.dialog_dnd.record_dialog_shown();
            }

            // Dispatch popup data to popup handler
            super::popup_handler::handle_popup(&sync_data.popup, app_handle).await;
        }

        Ok(())
    }
}

// ============================================================================
// Message Sync Task
// ============================================================================

pub struct MessageSyncTask;

#[async_trait::async_trait]
impl ScheduledTask for MessageSyncTask {
    fn name(&self) -> &str {
        "message_sync"
    }

    fn interval(&self) -> TaskInterval {
        TaskInterval::Custom(60) // 60s interval
    }

    fn requires_window_visible(&self) -> bool {
        // Red dot UI state can be updated regardless of window visibility
        false
    }

    async fn execute(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        // Delay 3 seconds (specifically requested for startup delay, 
        // will safely shift subsequent ticks by 3s without issues).
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let sync_data = tokio::task::spawn_blocking(|| {
            crate::api::common::wslui_helper_sync()
        })
        .await
        .map_err(|e| format!("spawn_blocking failed: {}", e))?;

        debug!("message_sync: received data from wslui_helper_sync");

        if !sync_data.messages.is_empty() {
            super::message_handler::handle_messages(&sync_data.messages, app_handle).await;
        }

        Ok(())
    }
}
