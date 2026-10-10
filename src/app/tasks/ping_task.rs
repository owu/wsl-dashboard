// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Ping monitoring task
//
// Periodically calls wslui_utils_ping() to notify the server of activity.

use tracing::trace;
use crate::AppWindow;
use crate::app::task_scheduler::{ScheduledTask, TaskInterval};

pub struct PingTask;

#[async_trait::async_trait]
impl ScheduledTask for PingTask {
    fn name(&self) -> &str {
        "ping"
    }

    fn interval(&self) -> TaskInterval {
        // Run every 120 seconds
        TaskInterval::Custom(120)
    }

    fn requires_window_visible(&self) -> bool {
        // Should keep running even when the window is hidden
        false
    }

    async fn execute(&self, _app_handle: &slint::Weak<AppWindow>) -> Result<(), String> {
        let _ = tokio::task::spawn_blocking(|| {
            crate::api::common::wslui_utils_ping()
        })
        .await
        .map_err(|e| format!("spawn_blocking failed: {}", e))?;

        // Runs frequently (every 120s), so use trace level to avoid log spam.
        trace!("ping: ping sent to wslui");

        Ok(())
    }
}