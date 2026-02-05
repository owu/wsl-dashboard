use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use slint::ComponentHandle;
use crate::{AppWindow, AppState, Theme, AppI18n, config, i18n};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_save_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let ui_language = app.get_ui_language().to_string();
                let distro_location = app.get_distro_location().to_string();
                let logs_location = app.get_logs_location().to_string();
                let auto_shutdown = app.get_auto_shutdown();
                let log_level = app.get_log_level() as u8;
                let log_days = app.get_log_days() as u8;
                let check_update = app.get_check_update_interval() as u8;
                
                let mut state = as_ptr.lock().await;
                let temp_location = state.config_manager.get_settings().temp_location.clone();
                let current_logs_location = state.config_manager.get_settings().logs_location.clone();

                // If log path or level changes, update logging system
                if let Some(ls) = state.logging_system.as_mut() {
                    if current_logs_location != logs_location {
                        ls.update_path(&logs_location);
                    }
                    ls.update_level(log_level);
                }

                // Update i18n
                let system_lang = state.config_manager.get_config().system.system_language.clone();
                let lang_to_load = if ui_language == "auto" {
                    &system_lang
                } else {
                    &ui_language
                };
                i18n::load_resources(lang_to_load);
                app.global::<AppI18n>().set_version(app.global::<AppI18n>().get_version() + 1);

                let user_settings = config::UserSettings {
                    modify_time: chrono::Utc::now().timestamp_millis().to_string(),
                    distro_location,
                    logs_location: logs_location.clone(),
                    temp_location,
                    ui_language,
                    auto_shutdown,
                    dark_mode: app.global::<Theme>().get_dark_mode(),
                    log_level,
                    log_days,
                    check_update,
                    check_time: state.config_manager.get_settings().check_time.clone(),
                };

                match state.config_manager.update_settings(user_settings) {
                    Ok(_) => {
                        drop(state);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah.upgrade() {
                                // Translate message if possible, or just keep english for now as it's dynamic
                                // But better to use a key if we had one "settings.saved_success"
                                app.set_current_message(i18n::t("settings.saved_success").into());
                                app.set_show_message_dialog(true);
                            }
                        });
                    }
                    Err(e) => {
                        let error_msg = i18n::tr("settings.saved_failed", &[e.to_string()]);
                        drop(state);
                        error!("{}", error_msg);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah.upgrade() {
                                app.set_current_message(error_msg.into());
                                app.set_show_message_dialog(true);
                            }
                        });
                    }
                }
            }
        });
    });

    let ah = app_handle.clone();
    app.on_select_distro_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("settings.select_distro_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                app.set_distro_location(path.display().to_string().into());
            }
        }
    });

    let ah = app_handle.clone();
    app.on_select_logs_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("settings.select_log_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                app.set_logs_location(path.display().to_string().into());
            }
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_reset_network(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };
            
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text("Resetting WSL network...".into());
                app.set_task_status_visible(true);
            }

            let result = executor.reset_wsl_network().await;

            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
                if result.success {
                    app.set_current_message("WSL network reset successfully (Subsystem shutdown).".into());
                } else {
                    app.set_current_message(format!("Failed to reset WSL network: {}", result.error.unwrap_or_default()).into());
                }
                app.set_show_message_dialog(true);
            }
        });
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_check_windows_features(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };
            
            let result = executor.check_windows_features().await;
            if let Some(features) = result.data {
                if let Some(app) = ah.upgrade() {
                    for (name, enabled) in features {
                        if name == "Microsoft-Windows-Subsystem-Linux" {
                            app.set_feature_wsl_enabled(enabled);
                        } else if name == "VirtualMachinePlatform" {
                            app.set_feature_vmp_enabled(enabled);
                        }
                    }
                }
            }
        });
    });

    // Trigger initial check
    app.invoke_check_windows_features();

    // Initial load of global wsl config
    let global_conf = crate::wsl::ops::global_config::load_global_config();
    app.set_global_memory(global_conf.memory.into());
    app.set_global_processors(global_conf.processors.into());
    app.set_global_networking_mode(global_conf.networking_mode.into());

    let ah = app_handle.clone();
    app.on_save_global_wsl_config(move |memory, processors, networking_mode| {
        let ah = ah.clone();
        let memory = memory.to_string();
        let processors = processors.to_string();
        let networking_mode = networking_mode.to_string();
        
        let _ = slint::spawn_local(async move {
            let conf = crate::wsl::ops::global_config::GlobalWslConfig {
                memory,
                processors,
                networking_mode,
                swap: "".to_string(), // Keep simple for now
            };
            
            match crate::wsl::ops::global_config::save_global_config(conf) {
                Ok(_) => {
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message("Global WSL configuration (.wslconfig) saved successfully. Please restart WSL to apply.".into());
                        app.set_show_message_dialog(true);
                    }
                },
                Err(e) => {
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(format!("Failed to save .wslconfig: {}", e).into());
                        app.set_show_message_dialog(true);
                    }
                }
            }
        });
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_toggle_theme(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let dark_mode = app.global::<Theme>().get_dark_mode();
                let mut state = as_ptr.lock().await;
                let mut settings = state.config_manager.get_settings().clone();
                settings.dark_mode = dark_mode;
                if let Err(e) = state.config_manager.update_settings(settings) {
                    error!("Failed to save color mode: {}", e);
                } else {
                    info!("Color mode saved: {}", if dark_mode { "Dark" } else { "Light" });
                }
            }
        });
    });
}
