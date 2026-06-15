// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::config::{ConfigManager, UserSettings};
use crate::utils::logging::{init_logging, LoggingSystem};
use crate::i18n;
use tracing::info;

pub struct AppContext {
    pub config_manager: ConfigManager,
    pub settings: UserSettings,
    pub logging_system: LoggingSystem,
}

pub async fn bootstrap(args: &[String]) -> AppContext {
    // 1. Initialize configuration manager first
    let config_manager = ConfigManager::new().await;
    let settings = config_manager.get_settings().clone();

    // 2. Load i18n based on settings
    let lang = if settings.ui_language == "auto" {
        &config_manager.get_config().system.system_language
    } else {
        &settings.ui_language
    };
    i18n::load_resources(lang);
    
    // 3. Set up tracing logs
    let initial_logs_location = settings.logs_location.clone();
    let log_level = settings.log_level;
    let timezone = config_manager.get_config().system.timezone.clone();
    let logging_system = init_logging(&initial_logs_location, log_level, &timezone);

    // 3.1 Log startup mode
    let modes = [
        ("/scheduler", vec!["/scheduler"]),
        ("/silent", vec!["/silent"]),
        ("/initialize", vec!["/initialize"]),
        ("/clean", vec!["/clean"]),
        ("/version", vec!["/version", "-v", "--version"]),
        ("/help", vec!["/help", "-h", "--help", "/?"]),
    ];

    let startup_mode = modes.iter()
        .find(|(_, flags)| flags.iter().any(|&f| args.iter().any(|a| a == f)))
        .map(|(name, _)| *name)
        .unwrap_or("normal");
    
    info!("[STARTUP] Mode: {}", startup_mode);

    AppContext {
        config_manager,
        settings,
        logging_system,
    }
}
