// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::wsl::dashboard::WslDashboard;
use crate::wsl::models::{WslDistro, WslStatus, WslVersion};
use crate::config::{ConfigManager, DebugConfig};
use crate::utils::logging::LoggingSystem;
use crate::api::models::HelperBootstrapData;
use std::sync::{Arc, atomic::AtomicBool};

// Define application state
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VSCodeExtensionData {
    pub name: String,
    pub url: String,
}

impl Default for VSCodeExtensionData {
    fn default() -> Self {
        let def = crate::api::models::VSCodeExtension::default();
        Self {
            name: def.name,
            url: def.url,
        }
    }
}

/// Dialog DND (Do Not Disturb) state machine.
/// Prevents multiple dialogs from appearing within a short time window.
pub struct DialogDndState {
    /// Timestamp (millis) of the most recent dialog popup
    pub auto_popup_millis: i64,
}

const DND_COOLDOWN_MS: i64 = 30_000;

impl DialogDndState {
    /// Check if a dialog can be shown (no popup in the last 30 seconds)
    pub fn can_show_dialog(&self) -> bool {
        if self.auto_popup_millis == 0 {
            return true;
        }
        let now = chrono::Utc::now().timestamp_millis();
        (now - self.auto_popup_millis) >= DND_COOLDOWN_MS
    }

    /// Record that a dialog was shown at the current timestamp
    pub fn record_dialog_shown(&mut self) {
        self.auto_popup_millis = chrono::Utc::now().timestamp_millis();
    }
}

impl Default for DialogDndState {
    fn default() -> Self {
        Self { auto_popup_millis: 0 }
    }
}

pub struct AppState {
    pub wsl_dashboard: WslDashboard,
    pub config_manager: ConfigManager,
    pub logging_system: Option<LoggingSystem>,
    pub vscode_extension: Option<VSCodeExtensionData>,
    pub is_silent_mode: bool,
    pub theme_watcher: Option<crate::utils::theme::ThemeWatcher>,
    pub bootstrap_data: Option<HelperBootstrapData>,
    pub debug_config: DebugConfig,
    pub dialog_dnd: DialogDndState,
    /// Set to true when WslCompatTask finishes (whether or not it showed a dialog).
    /// VersionExpiryTask and UpdateCheckTask wait on this before checking DND,
    /// guaranteeing that compat always wins the priority race.
    pub compat_task_done: Arc<AtomicBool>,
    /// Set to true when the startup checks (Expiry, Update) finish.
    /// SyncMonitorTask waits on this before checking DND,
    /// making it the lowest priority task (priority 4).
    pub startup_checks_done: Arc<AtomicBool>,
}

impl AppState {
    pub fn new(config_manager: ConfigManager, logging_system: LoggingSystem, is_silent_mode: bool) -> Self {
        // Load initial distros from cache for fast startup (warm start)
        let cached = config_manager.get_cached_distros();
        let initial_distros: Vec<WslDistro> = cached.into_iter().map(|c| {
            WslDistro {
                name: c.name,
                status: if c.status == "Running" { WslStatus::Running } else { WslStatus::Stopped },
                version: if c.version == "V1" || c.version == "1" { WslVersion::V1 } else { WslVersion::V2 },
                is_default: c.is_default,
                last_start_time: None,
            }
        }).collect();

        // Load debug config once at startup (optional file, never fails)
        let debug_config = ConfigManager::load_debug_config();

        Self {
            wsl_dashboard: WslDashboard::new(initial_distros),
            config_manager,
            logging_system: Some(logging_system),
            vscode_extension: None,
            is_silent_mode,
            theme_watcher: None,
            bootstrap_data: None,
            debug_config,
            dialog_dnd: DialogDndState::default(),
            compat_task_done: Arc::new(AtomicBool::new(false)),
            startup_checks_done: Arc::new(AtomicBool::new(false)),
        }
    }
}
