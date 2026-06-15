// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use slint::{ComponentHandle, Model};

// Include Slint modules locally in this runner
slint::include_modules!();

use crate::app::{AppState, APP_NAME, APP_ID, PROJECT_REPOSITORY, GITHUB_ISSUES};
use crate::config::ConfigManager;
use crate::utils::logging::LoggingSystem;
use crate::ui;
use crate::i18n;

// Run the main GUI application
pub async fn run_app(config_manager: ConfigManager, logging_system: LoggingSystem, is_silent_mode: bool) {
    let settings = config_manager.get_settings().clone();
    let tray_settings = config_manager.get_tray_settings().clone();
    let system_language = config_manager.get_config().system.system_language.clone();

    // 1. Create app state
    let app_state = Arc::new(Mutex::new(AppState::new(config_manager, logging_system, is_silent_mode)));

    // Async fetch bootstrap data and write to AppState
    {
        let state_clone = app_state.clone();
        tokio::spawn(async move {
            let data = tokio::task::spawn_blocking(|| crate::api::common::wslui_helper_bootstrap()).await.unwrap_or_default();
            let ts = data.unix_time;
            
            state_clone.lock().await.bootstrap_data = Some(data);
            
            if ts > 0 {
                info!("[STARTUP] Bootstrap data fetched, unix_time: {}", ts);
            } else {
                info!("[STARTUP] Bootstrap data fetched (unix_time unavailable)");
            }
        });
    }

    // Pre-warm WSL version cache so WslCompatTask gets an instant cache hit.
    // Without this, in silent mode the background distro monitors may exhaust the
    // executor semaphore, causing check_wsl_version_support to wait up to 20s before
    // returning detection_failed — preventing the compat dialog from ever showing.
    {
        let state_clone = app_state.clone();
        tokio::spawn(async move {
            let executor = {
                let state = state_clone.lock().await;
                state.wsl_dashboard.executor().clone()
            };
            let meta = crate::wsl::ops::config::check_wsl_version_support(&executor).await;
            if meta.detection_failed {
                info!("[STARTUP] WSL version pre-warm: detection failed (will retry in compat_task)");
            } else {
                info!("[STARTUP] WSL version pre-warm complete: {}", meta.version_string);
            }
        });
    }
    
    // 2. Create Slint window
    let app = AppWindow::new().expect("Failed to create app");
    app.set_system_language(system_language.into());
    
    // 3. Register i18n callback
    app.global::<AppI18n>().on_t(|key, args| {
        let args_vec: Vec<String> = args.iter().map(|s: slint::SharedString| s.to_string()).collect();
        i18n::tr(&key, &args_vec).into()
    });

    // Initialize localization language code
    let current_lang = i18n::current_lang();
    app.global::<AppI18n>().set_locale_code(current_lang.into());

    // Trigger initial evaluation of all i18n properties
    app.global::<AppI18n>().set_version(1);
    
    // Build language options array in Rust to avoid Slint compiler stack overflow
    build_language_options(&app);
    
    // Register language index callback via AppI18n (LanguageData delegates to it)
    app.global::<AppI18n>().on_get_language_index(|lang| {
        get_language_index(&lang.to_string())
    });

    // Set version and URL
    app.global::<AppInfo>().set_version(env!("CARGO_PKG_VERSION").into());
    app.global::<AppInfo>().set_project_repository(PROJECT_REPOSITORY.into());
    app.global::<AppInfo>().set_issues_url(format!("{}{}", PROJECT_REPOSITORY, GITHUB_ISSUES).into());
    app.global::<AppInfo>().set_donate_url(format!("{}{}", crate::app::PROJECT_WEBSITE, crate::app::DONATE_URI).into());

    // 4. Initialize system tray
    if let Err(e) = crate::app::tray::SystemTray::initialize(app.as_weak(), !is_silent_mode) {
        error!("Failed to initialize system tray: {}", e);
    }

    app.on_reinit_tray({
        let ah = app.as_weak();
        move || {
            let current_visible = if let Some(app) = ah.upgrade() {
                app.get_is_window_visible()
            } else {
                false
            };
            info!("Re-initializing tray, current visibility: {}", current_visible);
            if let Err(e) = crate::app::tray::SystemTray::initialize(ah.clone(), current_visible) {
                error!("Failed to re-initialize system tray: {}", e);
            }
        }
    });

    // 5. Load settings to UI (critical for i18n, font and theme)
    ui::data::load_settings_to_ui(&app, &app_state, &settings, &tray_settings).await;

    // 6. Initialize theme watcher if system theme sync is enabled
    if settings.system_color {
        match crate::utils::theme::ThemeWatcher::new(app.as_weak()) {
            Ok(watcher) => {
                let theme = crate::utils::theme::ThemeWatcher::get_current_theme();
                app.global::<Theme>().set_dark_mode(theme == crate::utils::theme::Theme::Dark);
                
                let mut state = app_state.lock().await;
                state.theme_watcher = Some(watcher);
            }
            Err(e) => {
                error!("Failed to initialize ThemeWatcher: {}", e);
            }
        }
    }

    // 7. Set up UI handlers
    ui::handlers::setup(&app, app.as_weak(), app_state.clone()).await;
    
    // 8. Refresh initial data (distro list)
    ui::data::refresh_data(app.as_weak(), app_state.clone()).await;

    // 9. Start background tasks (WSL/USB status monitoring)
    crate::app::tasks::spawn_wsl_monitor(app.as_weak(), app_state.clone());
    crate::app::tasks::spawn_usb_monitor(app.as_weak());
    crate::app::tasks::spawn_state_listener(app.as_weak(), app_state.clone());
    crate::app::tasks::spawn_wakeup_listener(app.as_weak());

    // 10. Show window and center it
    crate::app::window::show_and_center(&app, is_silent_mode);

    // 11. Start unified timer scheduler (includes startup checks: compat + expiry + update)
    // Placed after show_and_center, delayed start ensures bootstrap_data is ready
    {
        let ah = app.as_weak();
        let state_clone = app_state.clone();
        tokio::spawn(async move {
            // Wait for window to fully display (show_and_center internal thread needs ~1.2 seconds)
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            let mut scheduler = crate::app::task_scheduler::TaskScheduler::new(ah);

            // Message Sync Task (runs early, ignores DND)
            scheduler.register(crate::app::tasks::MessageSyncTask);

            // Popup Sync Task (Priority 4, respects DND)
            let sync_is_running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            scheduler.register(crate::app::tasks::PopupSyncTask {
                app_state: state_clone.clone(),
                is_running: sync_is_running,
            });

            // WSL version compatibility check (highest priority, immediate, runs in all modes)
            scheduler.register(crate::app::tasks::WslCompatTask { app_state: state_clone.clone() });

            // Version expiry check (3s delay, runs in all modes)
            scheduler.register(crate::app::tasks::VersionExpiryTask { app_state: state_clone.clone() });

            // Update check (5s delay, runs in all modes)
            scheduler.register(crate::app::tasks::UpdateCheckTask { app_state: state_clone });

            scheduler.start();
        });
    }
    
    // 11. Run application event loop with keep-alive timer to prevent exit when hidden
    let keep_alive_timer = slint::Timer::default();
    keep_alive_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), || {
        // Keep-alive heartbeat
    });

    info!("Starting {} (ID: {})...", APP_NAME, APP_ID);
    slint::run_event_loop().expect("Failed to run Slint event loop");

    // 12. Handle cleanup on exit
    crate::app::tasks::handle_app_exit(&app, &app_state).await;
}

// Build language options array in Rust to avoid Slint compiler stack overflow
pub fn build_language_options(app: &AppWindow) {
    let lang_keys = [
        "settings.languages.auto", "settings.languages.en",
        "settings.languages.zh_cn", "settings.languages.zh_tw",
        "settings.languages.hi", "settings.languages.es",
        "settings.languages.fr", "settings.languages.ar",
        "settings.languages.bn", "settings.languages.pt",
        "settings.languages.ru", "settings.languages.ur",
        "settings.languages.id", "settings.languages.de",
        "settings.languages.ja", "settings.languages.tr",
        "settings.languages.ko", "settings.languages.it",
        "settings.languages.nl", "settings.languages.sv",
        "settings.languages.cs", "settings.languages.el",
        "settings.languages.hu", "settings.languages.he",
        "settings.languages.no", "settings.languages.da",
        "settings.languages.fi", "settings.languages.sk",
        "settings.languages.sl", "settings.languages.is",
        "settings.languages.vi", "settings.languages.te",
        "settings.languages.jv", "settings.languages.th",
        "settings.languages.ta", "settings.languages.fil",
        "settings.languages.pa", "settings.languages.ms",
        "settings.languages.pl", "settings.languages.uk",
        "settings.languages.fa", "settings.languages.kn",
        "settings.languages.mr", "settings.languages.ha",
        "settings.languages.my", "settings.languages.uz",
        "settings.languages.az", "settings.languages.ceb",
        "settings.languages.ml", "settings.languages.sd",
        "settings.languages.am",
    ];

    let options: slint::ModelRc<slint::SharedString> = {
        let vec_model = slint::VecModel::from(
            lang_keys.iter()
                .map(|key| i18n::tr(key, &[]).into())
                .collect::<Vec<slint::SharedString>>()
        );
        std::rc::Rc::new(vec_model).into()
    };

    app.global::<AppI18n>().set_language_options(options);
}

// Get language index from language code (Rust implementation to avoid Slint stack overflow)
fn get_language_index(lang: &str) -> i32 {
    match lang {
        "auto" => 0,
        "en" => 1,
        "zh-CN" => 2,
        "zh-TW" => 3,
        "hi" => 4,
        "es" => 5,
        "fr" => 6,
        "ar" => 7,
        "bn" => 8,
        "pt" => 9,
        "ru" => 10,
        "ur" => 11,
        "id" => 12,
        "de" => 13,
        "ja" => 14,
        "tr" => 15,
        "ko" => 16,
        "it" => 17,
        "nl" => 18,
        "sv" => 19,
        "cs" => 20,
        "el" => 21,
        "hu" => 22,
        "he" => 23,
        "no" => 24,
        "da" => 25,
        "fi" => 26,
        "sk" => 27,
        "sl" => 28,
        "is" => 29,
        "vi" => 30,
        "te" => 31,
        "jv" => 32,
        "th" => 33,
        "ta" => 34,
        "fil" => 35,
        "pa" => 36,
        "ms" => 37,
        "pl" => 38,
        "uk" => 39,
        "fa" => 40,
        "kn" => 41,
        "mr" => 42,
        "ha" => 43,
        "my" => 44,
        "uz" => 45,
        "az" => 46,
        "ceb" => 47,
        "ml" => 48,
        "sd" => 49,
        "am" => 50,
        _ => 0,
    }
}
