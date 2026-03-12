use std::sync::Arc;
use std::rc::Rc;
use tokio::sync::Mutex;
use tracing::{trace,debug, error};
use slint::{ModelRc, VecModel, Model, ComponentHandle};
use std::sync::atomic::{AtomicBool, Ordering};

// Import Slint UI components
use crate::{AppState, AppWindow, Distro, InstallableDistro, SettingsStrings, wsl};
use crate::i18n;
use once_cell::sync::Lazy;
use std::time::{Instant, Duration};

static LAST_WSL_REFRESH: Lazy<std::sync::Mutex<Option<Instant>>> = Lazy::new(|| std::sync::Mutex::new(None));
static LAST_USB_REFRESH: Lazy<std::sync::Mutex<Option<Instant>>> = Lazy::new(|| std::sync::Mutex::new(None));

pub fn should_refresh_wsl(reason: &str, is_visible: bool) -> bool {
    let mut last = LAST_WSL_REFRESH.lock().unwrap();
    if let Some(t) = *last {
        let elapsed = t.elapsed();
        // Allow manual trigger to bypass the 4s debounce
        if reason != "manual trigger" && elapsed < Duration::from_secs(4) {
            trace!("WSL refresh skipped (reason: {}, elapsed: {:?})", reason, elapsed);
            return false;
        }
    }

    // Manual triggers represent explicit user intent and should bypass visibility checks
    if !is_visible && reason != "manual trigger" {
        trace!("WSL refresh skipped (reason: {}, window hidden in tray)", reason);
        return false;
    }

    *last = Some(Instant::now());
    trace!("WSL refresh triggered (reason: {})", reason);
    true
}

pub fn should_refresh_usb(reason: &str, is_visible: bool) -> bool {
    let mut last = LAST_USB_REFRESH.lock().unwrap();
    if let Some(t) = *last {
        let elapsed = t.elapsed();
        // Allow manual trigger to bypass the 4s debounce
        if reason != "manual trigger" && elapsed < Duration::from_secs(4) {
            trace!("USB refresh skipped (reason: {}, elapsed: {:?})", reason, elapsed);
            return false;
        }
    }

    // Manual triggers represent explicit user intent and should bypass visibility checks
    if !is_visible && reason != "manual trigger" {
        trace!("USB refresh skipped (reason: {}, window hidden in tray)", reason);
        return false;
    }

    *last = Some(Instant::now());
    trace!("USB refresh triggered (reason: {})", reason);
    true
}

pub fn refresh_localized_strings(app: &AppWindow) {
    app.set_settings_strings(SettingsStrings {
        language: i18n::tr("settings.language", &[]).into(),
        auto_update: i18n::tr("settings.update_interval", &[]).into(),
        log_level: i18n::tr("settings.log_level", &[]).into(),
        log_retention: i18n::tr("settings.log_retention_days", &[]).into(),
        log_path: i18n::tr("settings.log_path", &[]).into(),
        select_folder: i18n::tr("settings.select_folder", &[]).into(),
        install_dir: i18n::tr("settings.distro_dir", &[]).into(),
        auto_close: i18n::tr("settings.auto_shutdown_msg", &[]).into(),
        auto_start: i18n::tr("settings.tray_autostart", &[]).into(),
        minimize_tray: i18n::tr("settings.tray_start_minimized", &[]).into(),
        close_to_tray: i18n::tr("settings.tray_close_to_tray", &[]).into(),
        save: i18n::tr("settings.save", &[]).into(),
        wsl_settings: i18n::tr("settings.wsl_settings", &[]).into(),
    });

    app.set_about_strings(crate::AboutStrings {
        title: i18n::tr("about.title", &[]).into(),
        description: i18n::tr("about.description", &[]).into(),
        version: i18n::tr("about.version", &[]).into(),
        check_update: i18n::tr("about.check_update", &[]).into(),
        website: i18n::tr("about.homepage", &[]).into(),
        issues: i18n::tr("about.issues", &[]).into(),
        discussions: i18n::tr("about.discussions", &[]).into(),
        github: "".into(), // Not used in UI currently
        license: "".into(),
        copyright: "".into(),
    });
}

/// Generic helper to get localized text for use in Handlers
pub fn get_i18n_text(key: &str) -> String {
    i18n::tr(key, &[])
}

// Refresh all core data
pub async fn refresh_data(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    debug!("refresh_data: Starting background data refresh");
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    
    // 1. Show cached list immediately (warm start)
    refresh_distros_ui(ah, as_ptr).await;
    
    // 2. Trigger initial background refresh to get live WSL data
    // Mark as refreshed NOW to prevent periodic monitor from triggering immediately
    {
        let mut last_wsl = LAST_WSL_REFRESH.lock().unwrap();
        *last_wsl = Some(Instant::now());
        let mut last_usb = LAST_USB_REFRESH.lock().unwrap();
        *last_usb = Some(Instant::now());
    }

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let dashboard = {
            let state = app_state_clone.lock().await;
            state.wsl_dashboard.clone()
        };
        debug!("refresh_data: Triggering initial live WSL sync...");
        let _ = dashboard.refresh_distros().await;
    });
    
    debug!("refresh_data: Background data refresh scheduled");
}

// Static lock to ensure only one refresh runs at a time to prevent UI thread flooding
static IS_REFRESHING: AtomicBool = AtomicBool::new(false);

// Global static snapshot to prevent redundant refreshes across all threads
static LAST_REFRESH_SNAPSHOT: Lazy<std::sync::Mutex<Option<Vec<(String, String, String, bool, Option<&'static str>)>>>> = Lazy::new(|| std::sync::Mutex::new(None));
static LAST_INSTALLABLE_SNAPSHOT: Lazy<std::sync::Mutex<Option<Vec<String>>>> = Lazy::new(|| std::sync::Mutex::new(None));

// Refresh UI list of installed distributions
pub async fn refresh_distros_ui(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Basic debounce: if already refreshing, skip this request
    if IS_REFRESHING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return;
    }

    // Ensure we reset the flag when done
    let _refresh_guard = scopeguard::guard((), |_| {
        IS_REFRESHING.store(false, Ordering::SeqCst);
    });

    // Acquire all needed data under a single lock
    let (distros, executor, is_manual_op) = {
        let lock_timeout = std::time::Duration::from_millis(1000);
        match tokio::time::timeout(lock_timeout, app_state.lock()).await {
            Ok(app_state_lock) => {
                let mut distros = app_state_lock.wsl_dashboard.get_distros().await;
                // Sort by: 1. Default first, 2. Name A-Z
                distros.sort_by(|a, b| {
                    if a.is_default != b.is_default {
                        b.is_default.cmp(&a.is_default) // true (1) comes before false (0)
                    } else {
                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                    }
                });
                (
                    distros,
                    app_state_lock.wsl_dashboard.executor().clone(),
                    app_state_lock.wsl_dashboard.is_manual_operation(),
                )
            }
            Err(_) => {
                error!("refresh_distros_ui: AppState lock timeout, skipping UI refresh");
                return;
            }
        }
    };

    // Quick check: has the actual data changed before we do heavy icon loading?
    let current_snapshot: Vec<(String, String, String, bool, Option<&'static str>)> = distros.iter().map(|d| {
        (
            d.name.clone(),
            format!("{:?}", d.status),
            format!("{:?}", d.version),
            d.is_default,
            crate::utils::icon_mapper::map_name_to_icon_key(&d.name)
        )
    }).collect();

    let data_changed = {
        let mut last = LAST_REFRESH_SNAPSHOT.lock().unwrap();
        if let Some(ref l) = *last {
            if l.len() != current_snapshot.len() || l.iter().zip(current_snapshot.iter()).any(|(a, b)| a != b) {
                *last = Some(current_snapshot);
                true
            } else {
                false
            }
        } else {
            *last = Some(current_snapshot);
            true
        }
    };

    if !data_changed {
        return;
    }

    let mut intermediate_distros = Vec::new();
    if data_changed {
        debug!("refresh_distros_ui: Data changed, proceeding with icon loading (count: {})", distros.len());
        let mut needs_background_icon_check = Vec::new();

        for d in distros {
            let icon_key: Option<&'static str> = crate::utils::icon_mapper::map_name_to_icon_key(&d.name);
            
            if icon_key.is_none() && d.status == wsl::models::WslStatus::Running {
                 if !crate::utils::icon_mapper::is_distro_probed(&d.name) {
                    needs_background_icon_check.push(d.name.clone());
                 }
            }

            intermediate_distros.push((
                d.name.clone(),
                match d.status {
                    wsl::models::WslStatus::Running => "Running",
                    wsl::models::WslStatus::Stopped => "Stopped",
                },
                match d.version {
                    wsl::models::WslVersion::V1 => "1",
                    wsl::models::WslVersion::V2 => "2",
                },
                d.is_default,
                icon_key,
                crate::utils::icon_mapper::get_initial(&d.name),
                icon_key.and_then(crate::utils::icon_mapper::load_icon_data),
            ));
        }

        // Trigger background icon check if needed
        if !needs_background_icon_check.is_empty() {
            let as_ptr = app_state.clone();
            let exec = executor.clone();
            tokio::spawn(async move {
                let mut found_any = false;
                for name in needs_background_icon_check {
                    if !crate::utils::icon_mapper::start_probing(name.clone()) {
                        continue; // Already probing or probed
                    }
                    
                    let result = exec.execute_command(&["-d", &name, "--exec", "cat", "/etc/os-release"]).await;
                    if result.success {
                        // Mark as probed so we don't retry constantly even if we don't find a match
                        crate::utils::icon_mapper::mark_distro_probed(name.clone());
                        
                        for line in result.output.lines() {
                            let line = line.trim();
                            if line.is_empty() { continue; }
                            if let Some(eq_pos) = line.find('=') {
                                let key = line[..eq_pos].trim().to_lowercase();
                                let value = line[eq_pos + 1..].trim().trim_matches('"').trim();
                                if !value.is_empty() {
                                    match key.as_str() {
                                        "id" | "id_like" | "name" | "pretty_name" => {
                                            if let Some(icon_key) = crate::utils::icon_mapper::map_name_to_icon_key(value) {
                                                crate::utils::icon_mapper::add_dynamic_mapping(name.clone(), icon_key);
                                                found_any = true;
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    } else {
                        // If failed, also mark as probed to avoid infinite retry loop if distro is temporarily broken
                        // or we could retry with backoff, but for now safe approach:
                         crate::utils::icon_mapper::mark_distro_probed(name.clone());
                    }
                }
                if found_any {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.state_changed().notify_one();
                }
            });
        }
    }

    static IS_UI_UPDATING: AtomicBool = AtomicBool::new(false);
    if IS_UI_UPDATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return;
    }

    let _ = slint::invoke_from_event_loop(move || {
        let _update_guard = scopeguard::guard((), |_| {
            IS_UI_UPDATING.store(false, Ordering::SeqCst);
        });

        if let Some(app) = app_handle.upgrade() {
            if data_changed {
                let slint_distros: Vec<Distro> = intermediate_distros.into_iter().map(|(name, status, version, is_default, icon_key, initial, preloaded_icon)| {
                    let mut image = slint::Image::default();
                    let mut has_icon = false;
                    
                    if let Some(icon_data) = preloaded_icon {
                        let cache_key = icon_key.map(|s| s.to_string()).unwrap_or_else(|| name.clone());
                        if let Some(img) = crate::utils::icon_mapper::load_image_from_data(cache_key, icon_data) {
                            image = img;
                            has_icon = true;
                        }
                    }

                    Distro {
                        name: name.into(),
                        status: status.into(),
                        version: version.into(),
                        is_default,
                        icon: image,
                        has_icon,
                        initial: initial.into(),
                        distro_display_name: crate::utils::icon_mapper::get_display_name(icon_key).into(),
                    }
                }).collect();

                // Try to update existing model in-place if possible to minimize Repeater churn
                let existing_model = app.get_distros();
                let needs_full_rebuild = existing_model.row_count() != slint_distros.len();
                
                let mut data_actually_changed = true;
                if !needs_full_rebuild {
                    data_actually_changed = false;
                    for (i, new_distro) in slint_distros.iter().enumerate() {
                        if let Some(old_distro) = existing_model.row_data(i) {
                            if old_distro.name != new_distro.name 
                                || old_distro.status != new_distro.status
                                || old_distro.is_default != new_distro.is_default 
                                || old_distro.has_icon != new_distro.has_icon {
                                data_actually_changed = true;
                                break;
                            }
                        } else {
                            data_actually_changed = true;
                            break;
                        }
                    }
                }
                
                if data_actually_changed {
                    let model = VecModel::from(slint_distros);
                    let model_rc = ModelRc::from(Rc::new(model));
                    app.set_distros(model_rc);
                }
            }
            
            if !is_manual_op && app.get_task_status_visible() {
                app.set_task_status_visible(false);
            }
        }
    });
}


// Refresh UI list of installable distributions
pub async fn refresh_installable_distros(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let result = {
        let app_state = app_state.lock().await;
        app_state.wsl_dashboard.executor().list_available_distros().await
    };

    if result.success {
        let mut available = wsl::parser::parse_available_distros(&result.output);
        // Sort by distribution name Z-A (Reverse order as requested)
        available.sort_by(|a, b| b.1.to_lowercase().cmp(&a.1.to_lowercase()));
        
        let current_names: Vec<String> = available.iter().map(|(n, _)| n.clone()).collect();
        let unchanged = {
            let mut last = LAST_INSTALLABLE_SNAPSHOT.lock().unwrap();
            if let Some(ref l) = *last {
                if *l == current_names {
                    true
                } else {
                    *last = Some(current_names);
                    false
                }
            } else {
                *last = Some(current_names);
                false
            }
        };

        if unchanged {
            return;
        }

        let slint_installables: Vec<InstallableDistro> = available.iter().map(|(name, friendly)| {
            InstallableDistro {
                name: name.clone().into(),
                friendly_name: friendly.clone().into(),
            }
        }).collect();

        let friendly_names: Vec<slint::SharedString> = available.iter().map(|(_, friendly)| {
            friendly.clone().into()
        }).collect();

        let _ = slint::invoke_from_event_loop(move || {
            let model = VecModel::from(slint_installables);
            let model_rc = ModelRc::from(Rc::new(model));
            
            let names_model = VecModel::from(friendly_names);
            let names_rc = ModelRc::from(Rc::new(names_model));
            
            if let Some(app) = app_handle.upgrade() {
                app.set_installable_distros(model_rc);
                app.set_installable_distro_names(names_rc);
                
                // If no selection, default to first item and sync UI fields
                if app.get_selected_install_distro().is_empty() && app.get_installable_distro_names().row_count() > 0 {
                    if let Some(first) = app.get_installable_distro_names().row_data(0) {
                        app.set_selected_install_distro(first.clone());
                        // Trigger the synchronization logic for Instance Name and Path
                        app.invoke_distro_selected(first);
                    }
                }
            }
        });
    }
}

// Load configuration to UI
pub async fn load_settings_to_ui(app: &AppWindow, app_state: &Arc<Mutex<AppState>>, settings: &crate::config::UserSettings, tray: &crate::config::TraySettings) {
    app.set_ui_language(settings.ui_language.clone().into());
    app.set_distro_location(settings.distro_location.clone().into());
    app.set_new_instance_path(settings.distro_location.clone().into());
    app.set_logs_location(settings.logs_location.clone().into());
    app.set_auto_shutdown(settings.auto_shutdown);
    app.set_sidebar_collapsed(settings.sidebar_collapsed);
    app.set_tray_autostart(tray.autostart);
    app.set_tray_start_minimized(tray.start_minimized);
    app.set_tray_close_to_tray(tray.close_to_tray);
    app.set_log_level(settings.log_level as i32);

    // Set RTL mode based on current resolved language
    let current_lang = i18n::current_lang();
    app.global::<crate::AppI18n>().set_is_rtl(i18n::is_rtl(&current_lang));
    // Refresh localized strings to apply translations immediately
    refresh_localized_strings(app);
    
    // Validate and set log retention days
    let mut log_days = settings.log_days;
    if !vec![7, 15, 30].contains(&log_days) {
        debug!("Invalid log-days value ({}), resetting to 7", log_days);
        log_days = 7;
    }
    app.set_log_days(log_days as i32);

    // Validate and set check_update interval
    let mut check_update = settings.check_update;
    if !vec![1, 7, 15, 30].contains(&check_update) {
        debug!("Invalid check-update value ({}), resetting to 7", check_update);
        check_update = 7;
    }
    app.set_check_update_interval(check_update as i32);

    // Update settings if any were invalid
    if log_days != settings.log_days || check_update != settings.check_update {
        let mut state_mut = app_state.lock().await;
        let mut settings_mut = state_mut.config_manager.get_settings().clone();
        settings_mut.log_days = log_days;
        settings_mut.check_update = check_update;
        let _ = state_mut.config_manager.update_settings(settings_mut);
    }
    
    app.global::<crate::Theme>().set_dark_mode(settings.dark_mode);
    
    // Set default font based on language to fix Chinese rendering issues
    let font_family = if crate::app::is_chinese_lang(&settings.ui_language) {
        crate::app::constants::FONT_ZH
    } else if settings.ui_language == "auto" {
        let state = app_state.lock().await;
        let sys_lang = state.config_manager.get_config().system.system_language.clone();
        drop(state);
        if crate::app::is_chinese_lang(&sys_lang) {
            crate::app::constants::FONT_ZH
        } else {
            crate::app::constants::FONT_EN_FALLBACK
        }
    } else {
        crate::app::constants::FONT_EN_FALLBACK
    };
    app.global::<crate::Theme>().set_default_font(font_family.into());

    debug!("Configuration loaded to UI (Language: {}, Mode: {}, LogLevel: {}, LogDays: {})", 
          settings.ui_language, if settings.dark_mode { "Dark" } else { "Light" }, settings.log_level, log_days);
}
