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
    let mut config_manager = ConfigManager::new().await;
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

    // 3.1 Generate/persist device-id AFTER logging is initialized,
    //     otherwise ConfigManager logs would be silently dropped.
    config_manager.ensure_device_id().await;

    // 3.2 Fill global API request metadata after device-id is ensured,
    //        so all subsequent wslui API requests carry version/language/device-id headers.
    let cfg = config_manager.get_config();
    crate::api::client::set_request_meta(crate::api::client::RequestMeta {
        app_version: cfg.application.app_version.clone(),
        system_language: cfg.system.system_language.clone(),
        device_id: cfg.application.device_id.clone(),
    });

    // 3.3 Log startup mode
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

    // 4. Log system version information asynchronously (non-blocking)
    tokio::spawn(async {
        crate::app::startup_info::log_startup_system_info().await;
    });

    AppContext {
        config_manager,
        settings,
        logging_system,
    }
}
