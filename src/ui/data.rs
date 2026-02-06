use std::sync::Arc;
use std::rc::Rc;
use tokio::sync::Mutex;
use tracing::{warn, trace, debug};
use slint::{ModelRc, VecModel, Model, ComponentHandle};
use std::sync::atomic::{AtomicBool, Ordering};

// Import Slint UI components
use crate::{AppState, AppWindow, Distro, InstallableDistro, SettingsStrings, wsl};
use crate::i18n;

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

// Refresh all core data
pub async fn refresh_data(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    debug!("refresh_data: Starting background data refresh");
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    
    // Only refresh installed list at startup, it's lightweight
    refresh_distros_ui(ah, as_ptr).await;
    
    debug!("refresh_data: Background data refresh complete");
}

// Static lock to ensure only one refresh runs at a time to prevent UI thread flooding
static IS_REFRESHING: AtomicBool = AtomicBool::new(false);

// Refresh UI list of installed distributions
pub async fn refresh_distros_ui(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Basic debounce: if already refreshing, skip this request
    // Since this is called frequently on status changes, skipping is usually safe
    if IS_REFRESHING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        trace!("refresh_distros_ui: Refresh already in progress, skipping");
        return;
    }

    // Ensure we reset the flag when done
    let _refresh_guard = scopeguard::guard((), |_| {
        IS_REFRESHING.store(false, Ordering::SeqCst);
    });

    debug!("refresh_distros_ui: Starting refresh");
    debug!("refresh_distros_ui: Locking app_state");
    
    // Acquire all needed data under a single lock
    let (distros, executor, is_manual_op) = {
        let app_state_lock = app_state.lock().await;
        
        let mut distros = app_state_lock.wsl_dashboard.get_distros().await;
        // Sort by name A-Z
        distros.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        
        debug!("refresh_distros_ui: Found {} distributions", distros.len());
        for distro in &distros {
            debug!("refresh_distros_ui: {} status: {:?}", distro.name, distro.status);
        }
        
        (
            distros,
            app_state_lock.wsl_dashboard.executor().clone(),
            app_state_lock.wsl_dashboard.is_manual_operation()
        )
    };

    debug!("refresh_distros_ui: Starting model conversion");

    let mut intermediate_distros = Vec::new();
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
            // Pre-load icon data in background thread
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
                // Mark as probed immediately to prevent concurrent duplicate requests
                crate::utils::icon_mapper::mark_distro_probed(name.clone());

                let result = exec.execute_command(&["-d", &name, "--exec", "cat", "/etc/os-release"]).await;
                if result.success {
                    for line in result.output.lines() {
                        let line = line.trim();
                        if line.is_empty() { continue; }
                        
                        // Parse key=value pairs from os-release
                        if let Some(eq_pos) = line.find('=') {
                            let key = line[..eq_pos].trim().to_lowercase();
                            let value = line[eq_pos + 1..].trim().trim_matches('"').trim();
                            
                            if !value.is_empty() {
                                // Try to match various fields to an icon key
                                // Fields like ID, ID_LIKE, NAME, PRETTY_NAME often contain distro identifiers
                                match key.as_str() {
                                    "id" | "id_like" | "name" | "pretty_name" => {
                                        if let Some(icon_key) = crate::utils::icon_mapper::map_name_to_icon_key(value) {
                                            debug!("Found icon key '{}' for distro '{}' via os-release field {}='{}'", icon_key, name, key, value);
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
                    warn!("Failed to probe distro '{}' for icon: {}", name, result.error.unwrap_or_default());
                    // Unmark as probed so it can be retried on next refresh
                    crate::utils::icon_mapper::unmark_distro_probed(&name);
                }
            }
            if found_any {
                // Trigger another refresh by notifying state change
                // spawn_state_listener will handle the actual refresh calling
                let state = as_ptr.lock().await;
                state.wsl_dashboard.state_changed().notify_one();
            }
        });
    }

    // Static lock to ensure only one refresh runs at a time to prevent UI thread flooding
    static IS_UI_UPDATING: AtomicBool = AtomicBool::new(false);

    // If a UI update is already pending in the event loop, skip this refresh
    if IS_UI_UPDATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        trace!("refresh_distros_ui: UI update already pending, skipping");
        return;
    }

    debug!("refresh_distros_ui: Preparing UI update, model count: {}", intermediate_distros.len());

thread_local! {
    static LAST_DISTROS: std::cell::RefCell<Option<Vec<Distro>>> = std::cell::RefCell::new(None);
}

    let result = slint::invoke_from_event_loop(move || {
        // Ensure we reset the pending flag and the main refresh flag when the UI task is done
        let _update_guard = scopeguard::guard((), |_| {
            IS_UI_UPDATING.store(false, Ordering::SeqCst);
        });

        trace!("refresh_distros_ui: Executing in UI thread");
        let slint_distros: Vec<Distro> = intermediate_distros.into_iter().map(|(name, status, version, is_default, icon_key, initial, preloaded_icon)| {
            let mut image = slint::Image::default();
            let mut has_icon = false;
            
            if let Some(icon_data) = preloaded_icon {
                // Use icon_key if available, otherwise use name as cache key
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

        // Optimization: Only update UI if the model has actually changed
        let should_update = LAST_DISTROS.with(|last| {
            let mut last = last.borrow_mut();
            if let Some(ref l) = *last {
                if l.len() != slint_distros.len() {
                    *last = Some(slint_distros.clone());
                    return true;
                }
                for i in 0..l.len() {
                    let old = &l[i];
                    let new = &slint_distros[i];
                    // Skip 'icon' comparison as slint::Image doesn't implement PartialEq easily
                    if old.name != new.name || old.status != new.status || old.version != new.version 
                        || old.is_default != new.is_default || old.has_icon != new.has_icon 
                        || old.initial != new.initial || old.distro_display_name != new.distro_display_name {
                         *last = Some(slint_distros.clone());
                         return true;
                    }
                }
                false
            } else {
                *last = Some(slint_distros.clone());
                true
            }
        });

        if !should_update {
            trace!("refresh_distros_ui: Model unchanged, skipping UI update");
            return;
        }

        let model = VecModel::from(slint_distros);
        let model_rc = ModelRc::from(Rc::new(model));
        
        if let Some(app) = app_handle.upgrade() {
            trace!("refresh_distros_ui: App instance acquired, setting model");
            app.set_distros(model_rc);
            
            // Auto-hide task status when no manual operations are pending
            if !is_manual_op && app.get_task_status_visible() {
                debug!("refresh_distros_ui: No manual operations pending, hiding task status");
                app.set_task_status_visible(false);
            }

            trace!("refresh_distros_ui: UI model update complete");
        } else {
            warn!("refresh_distros_ui: Could not acquire app instance");
        }
    });
    
    match result {
        Ok(_) => {
            debug!("refresh_distros_ui: UI update successful");
        }
        Err(e) => {
            warn!("refresh_distros_ui: UI update failed: {}", e);
        }
    }
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
    app.set_tray_autostart(tray.autostart);
    app.set_tray_start_minimized(tray.start_minimized);
    app.set_tray_close_to_tray(tray.close_to_tray);
    app.set_log_level(settings.log_level as i32);
    
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
