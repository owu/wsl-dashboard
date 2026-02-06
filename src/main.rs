#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, error};
use slint::{ComponentHandle, Model};

// Define application modules
mod wsl;
mod utils;
mod ui;
mod config;
mod app;
mod i18n;

// Import Slint UI components
slint::include_modules!();

use app::{AppState, APP_NAME, APP_ID, COMPANY_NAME, LEGAL_COPYRIGHT, GITHUB_URL, GITHUB_ISSUES};
use ui::data::refresh_data;
use ui::handlers;

#[tokio::main]
async fn main() {
    // Check for /silent command line argument first
    let args: Vec<String> = std::env::args().collect();
    let is_silent_mode = args.iter().any(|arg| arg.eq_ignore_ascii_case("/silent"));
    
    // 1. Single Instance check
    let instance = app::single_instance::SingleInstance::new("wsldashboard-v0.3-lock");
    if !instance.is_single() {
        // Another instance is running
        if !is_silent_mode {
            // Try to activate the existing instance
            if app::single_instance::try_activate_existing_instance() {
                info!("Activated existing instance, exiting...");
            } else {
                eprintln!("Another instance is already running. Exiting.");
            }
        } else {
            // In silent mode, just exit quietly
            eprintln!("Another instance is already running (silent mode). Exiting.");
        }
        return;
    }
    
    // Initialize configuration manager to get log path
    let config_manager = config::ConfigManager::new().await;
    let settings = config_manager.get_settings().clone();
    let tray_settings = config_manager.get_tray_settings().clone();

    // Load i18n based on settings
    let system_language = config_manager.get_config().system.system_language.clone();
    let lang = if settings.ui_language == "auto" {
        &config_manager.get_config().system.system_language
    } else {
        &settings.ui_language
    };
    i18n::load_resources(lang);
    
    let initial_logs_location = settings.logs_location.clone();
    let log_level = settings.log_level;
    let timezone = config_manager.get_config().system.timezone.clone();

    // Set up tracing logs
    let logging_system = utils::logging::init_logging(&initial_logs_location, log_level, &timezone);

    // Cleanup expired logs
    utils::logging::cleanup_expired_logs(&initial_logs_location, settings.log_days);

    info!("Starting {} (ID: {})...", APP_NAME, APP_ID);
    
    if is_silent_mode {
        info!("Silent mode detected via /silent parameter");
    }

    // 8. Path automatic repair: update registry if the exe has been moved
    app::autostart::repair_autostart_path(tray_settings.autostart, tray_settings.start_minimized).await;

    // Create application state
    let app_state = Arc::new(Mutex::new(AppState::new(config_manager, logging_system)));
    
    // Create Slint application
    let app = AppWindow::new().expect("Failed to create app");
    app.set_system_language(system_language.into());
    
    // Register i18n callback
    app.global::<AppI18n>().on_t(|key, args| {
        let args_vec: Vec<String> = args.iter().map(|s: slint::SharedString| s.to_string()).collect();
        i18n::tr(&key, &args_vec).into()
    });

    // Trigger initial evaluation of all i18n properties
    app.global::<AppI18n>().set_version(1);
    
    // Set version number and URLs - homepage and issues always use GITHUB_URL
    app.global::<AppInfo>().set_version(env!("CARGO_PKG_VERSION").into());
    app.global::<AppInfo>().set_homepage(GITHUB_URL.into());
    app.global::<AppInfo>().set_issues_url(format!("{}{}", GITHUB_URL, GITHUB_ISSUES).into());
    
    debug!("App Metadata - Company: {}, Copyright: {}", COMPANY_NAME, LEGAL_COPYRIGHT);
    
    let app_handle = app.as_weak();

    // Initialize system tray (window is initially visible unless in silent mode)
    if let Err(e) = app::tray::SystemTray::initialize(app_handle.clone(), !is_silent_mode) {
        error!("Failed to initialize system tray: {}", e);
    }
    
    // Rename windows for Task Manager identification
    app::window::rename_app_windows();

    // Handle tray re-initialization (e.g. on language change)
    app.on_reinit_tray({
        let ah = app_handle.clone();
        let silent = is_silent_mode;
        move || {
            if let Err(e) = app::tray::SystemTray::initialize(ah.clone(), !silent) {
                error!("Failed to re-initialize system tray: {}", e);
            }
        }
    });

    // Initialize data refresh
    refresh_data(app_handle.clone(), app_state.clone()).await;

    // Load configuration to UI
    {
        let state = app_state.lock().await;
        let settings = state.config_manager.get_settings().clone();
        let tray = state.config_manager.get_tray_settings().clone();
        drop(state);
        ui::data::load_settings_to_ui(&app, &app_state, &settings, &tray).await;
        ui::data::refresh_localized_strings(&app);
    }

    // Set release_url based on timezone
    {
        let state = app_state.lock().await;
        let timezone = &state.config_manager.get_config().system.timezone;
        let release_base_url = if timezone == app::ZH_TIMEZONE {
            app::GITEE_URL
        } else {
            GITHUB_URL
        };
        app.global::<AppInfo>().set_release_url(format!("{}{}", release_base_url, app::GITHUB_RELEASES).into());
    }

    // Set up all UI callbacks
    handlers::common::setup(&app, app_handle.clone(), app_state.clone());
    handlers::window::setup(&app, app_handle.clone());
    handlers::distro::setup(&app, app_handle.clone(), app_state.clone());
    handlers::settings::setup(&app, app_handle.clone(), app_state.clone());
    handlers::update::setup(&app, app_handle.clone(), app_state.clone());
    handlers::instance::setup(&app, app_handle.clone(), app_state.clone());


    // Start WSL status monitoring
    app::tasks::spawn_wsl_monitor(app_state.clone());

    // Listen for distribution state changes and automatically update UI
    app::tasks::spawn_state_listener(app_handle.clone(), app_state.clone());

    // Automatically check for updates and expiration at startup
    app::startup::spawn_check_task(app_handle.clone(), app_state.clone());

    // Show window and center it only if NOT in silent mode
    if !is_silent_mode {
        app::window::show_and_center(&app);
    } else {
        // If /silent parameter is present, ensure window is hidden and taskbar is skipped
        // We still use a short delay to ensure the Slint event loop is fully initialized 
        // and the tray icon is correctly handled.
        info!("Starting minimized to system tray (silent mode).");
        let ah = app_handle.clone();
        slint::Timer::single_shot(std::time::Duration::from_millis(100), move || {
            if let Some(app) = ah.upgrade() {
                app.set_is_window_visible(false);
                app::window::set_skip_taskbar(&app, true);
                debug!("Window hidden to tray after startup delay.");
            }
        });
    }

    // Run application event loop with a keep-alive timer to prevent exit when hidden
    // We Box::leak to ensure the timer stays alive as long as the process runs.
    let keep_alive_timer = Box::leak(Box::new(slint::Timer::default()));
    keep_alive_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), || {
        // Keep-alive heartbeat
    });
    
    info!("Event loop started.");
    slint::run_event_loop().expect("Failed to run event loop");
    info!("Event loop exited.");

    // Processing after application exit
    app::tasks::handle_app_exit(&app, &app_state).await;
}

