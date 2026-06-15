// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use slint::ComponentHandle;
use crate::{AppWindow, AppState, Theme, AppI18n, i18n};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_save_general_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let ui_language = app.get_ui_language().to_string();
                let tray_autostart = app.get_tray_autostart();
                let tray_start_minimized = app.get_tray_start_minimized();
                let tray_close_to_tray = app.get_tray_close_to_tray();
                let check_update = app.get_check_update_interval() as u8;

                let mut state = as_ptr.lock().await;

                // Apply Dashboard autostart setting to Windows
                if let Err(e) = crate::app::autostart::set_dashboard_autostart(tray_autostart, tray_start_minimized).await {
                    error!("Failed to apply dashboard autostart: {}", e);
                }

                // Update tray settings in config
                let tray_settings = crate::config::TraySettings {
                    autostart: tray_autostart,
                    start_minimized: tray_start_minimized,
                    close_to_tray: tray_close_to_tray,
                };
                if let Err(e) = state.config_manager.update_tray_settings(tray_settings) {
                    error!("Failed to save tray settings: {}", e);
                }

                // Update i18n
                let system_lang = state.config_manager.get_config().system.system_language.clone();
                let old_lang = state.config_manager.get_settings().ui_language.clone();
                let lang_to_load = if ui_language == "auto" {
                    &system_lang
                } else {
                    &ui_language
                };
                i18n::load_resources(lang_to_load);
                app.global::<AppI18n>().set_is_rtl(i18n::is_rtl(lang_to_load));
                app.global::<AppI18n>().set_locale_code(i18n::current_lang().into());
                app.global::<AppI18n>().set_version(app.global::<AppI18n>().get_version() + 1);
                // Rebuild language options after language change
                crate::app::runner::build_language_options(&app);
                crate::ui::data::refresh_localized_strings(&app);
                
                let font_family = crate::app::constants::get_font_for(lang_to_load, &system_lang);
                app.global::<Theme>().set_default_font(font_family.into());

                if old_lang != ui_language {
                    info!("Language changed from '{}' to '{}', triggering system tray re-initialization...", old_lang, ui_language);
                    let ah_tray = ah.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_tray.upgrade() {
                            app.invoke_reinit_tray();
                        }
                    });
                }

                let mut settings = state.config_manager.get_settings().clone();
                settings.ui_language = ui_language;
                settings.check_update = check_update;

                match state.config_manager.update_settings(settings) {
                    Ok(_) => {
                        drop(state);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah.upgrade() {
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
    let as_ptr = app_state.clone();
    app.on_save_advanced_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let distro_location = app.get_distro_location().to_string();
                let logs_location = app.get_logs_location().to_string();
                let auto_shutdown = app.get_auto_shutdown();
                let log_level = app.get_log_level() as u8;
                let log_days = app.get_log_days() as u8;
                let sparse_vhd = app.get_sparse_vhd();
                
                // Write sparseVhd directly to ~/.wslconfig
                if let Err(e) = crate::utils::wsl_config::set_sparse_vhd(sparse_vhd) {
                    error!("Failed to set sparseVhd in ~/.wslconfig: {}", e);
                } else {
                    info!("Successfully updated sparseVhd to {} in ~/.wslconfig", sparse_vhd);
                }

                let mut state = as_ptr.lock().await;

                // Update logging system if changed
                let current_logs_location = state.config_manager.get_settings().logs_location.clone();
                if let Some(ls) = state.logging_system.as_mut() {
                    if current_logs_location != logs_location {
                        ls.update_path(&logs_location);
                    }
                    ls.update_level(log_level);
                }

                let mut settings = state.config_manager.get_settings().clone();
                settings.distro_location = distro_location;
                settings.logs_location = logs_location;
                settings.auto_shutdown = auto_shutdown;
                settings.log_level = log_level;
                settings.log_days = log_days;

                match state.config_manager.update_settings(settings) {
                    Ok(_) => {
                        drop(state);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah.upgrade() {
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
    let as_ptr = app_state.clone();
    app.on_save_interface_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let sidebar_add = app.get_sidebar_add();
                let sidebar_toggle = app.get_sidebar_toggle();
                let sidebar_usb = app.get_sidebar_usb();
                let sidebar_network = app.get_sidebar_network();
                let sidebar_donate = app.get_sidebar_donate();
                let sidebar_about = app.get_sidebar_about();
                let colorful_icons = app.get_colorful_icons();
                let system_color = app.get_system_color();
                let mail_icon_always = app.get_mail_icon_always();
                let hide_pin_icon = app.get_hide_pin_icon();

                let mut state = as_ptr.lock().await;

                // Sync to global theme for immediate effect
                app.global::<crate::Theme>().set_colorful_icons(colorful_icons);

                // Dynamic ThemeWatcher switching
                let old_system_color = state.config_manager.get_settings().system_color;
                if old_system_color != system_color {
                    if system_color {
                        match crate::utils::theme::ThemeWatcher::new(ah.clone()) {
                            Ok(watcher) => {
                                let theme = crate::utils::theme::ThemeWatcher::get_current_theme();
                                app.global::<crate::Theme>().set_dark_mode(theme == crate::utils::theme::Theme::Dark);
                                state.theme_watcher = Some(watcher);
                                info!("ThemeWatcher enabled via settings.");
                            }
                            Err(e) => {
                                error!("Failed to enable ThemeWatcher: {}", e);
                            }
                        }
                    } else {
                        state.theme_watcher = None;
                        info!("ThemeWatcher disabled via settings.");
                    }
                }

                let mut settings = state.config_manager.get_settings().clone();
                settings.colorful_icons = colorful_icons;
                settings.system_color = system_color;
                settings.mail = mail_icon_always;
                settings.hide_pin = hide_pin_icon;
                let _ = state.config_manager.update_settings(settings);

                // If hiding pin icon while window is pinned, auto-unpin
                if hide_pin_icon && app.get_is_pinned() {
                    app.set_is_pinned(false);
                    crate::app::window::set_always_on_top(false);
                }

                let sidebar_config = crate::config::SidebarConfig {
                    add: sidebar_add,
                    toggle: sidebar_toggle,
                    usb: sidebar_usb,
                    network: sidebar_network,
                    donate: sidebar_donate,
                    about: sidebar_about,
                };

                match state.config_manager.update_sidebar_settings(sidebar_config) {
                    Ok(_) => {
                        drop(state);
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah.upgrade() {
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

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_open_wsl_settings(move || {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            let state = as_ptr.lock().await;
            let executor = state.wsl_dashboard.executor().clone();
            drop(state);

            let show_upgrade_prompt = |app: slint::Weak<AppWindow>| {
                if let Some(app) = app.upgrade() {
                    app.set_current_message(i18n::t("settings.wsl2_required").into());
                    app.set_current_message_link(i18n::t("settings.update_wsl").into());
                    app.set_current_message_url(crate::app::WSL_GITHUB_RELEASES.into());
                    app.set_show_message_dialog(true);
                }
            };

            // 1. Check if it's the Store version (which supports WSL Settings)
            // If wsl --version fails, it's likely the Inbox version or an old version
            let version_check = executor.execute_command(&["--version"]).await;
            if !version_check.success {
                show_upgrade_prompt(ah);
                return;
            }

            // 2. Discover wslsettings.exe path
            let rel_path = "Program Files\\WSL\\wslsettings\\wslsettings.exe";
            let mut exe_path = std::path::PathBuf::from(format!("C:\\{}", rel_path));
            let mut found = exe_path.exists();

            if !found {
                // Try SystemDrive if not C:
                if let Ok(system_drive) = std::env::var("SystemDrive") {
                    if system_drive.to_uppercase() != "C:" {
                        let alt_path = std::path::PathBuf::from(format!("{}\\{}", system_drive, rel_path));
                        if alt_path.exists() {
                            exe_path = alt_path;
                            found = true;
                        }
                    }
                }
            }

            if !found {
                // Exhaustive search on other drive letters
                for drive in b'C'..=b'Z' {
                    let drive_str = format!("{}:", drive as char);
                    let alt_path = std::path::PathBuf::from(format!("{}\\{}", drive_str, rel_path));
                    if alt_path.exists() {
                        exe_path = alt_path;
                        found = true;
                        break;
                    }
                }
            }

            if found {
                let mut cmd = std::process::Command::new(exe_path);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    cmd.creation_flags(CREATE_NO_WINDOW);
                }
                let _ = cmd.spawn().map_err(|e| {
                    error!("Failed to launch WSL settings: {}", e);
                });
            } else {
                // If wslsettings.exe is not found even on multiple drives,
                // it's almost certainly because the WSL version is < 2.3.0
                show_upgrade_prompt(ah);
            }
        });
    });
    

    let as_ptr = app_state.clone();
    app.on_confirm_stop_wsl(move || {
        let as_ptr = as_ptr.clone();
        let ah = app_handle.clone();
        tokio::spawn(async move {
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };
            
            // Execute wsl --shutdown
            let result = executor.execute_command(&["--shutdown"]).await;
            
            if !result.success {
                error!("Failed to shutdown WSL: {}", result.error.unwrap_or_default());
            } else {
                let check_result = executor.list_distros().await;
                let mut all_stopped = true;
                if check_result.success {
                    if let Some(distros) = check_result.data {
                        for d in distros {
                            if d.status != crate::wsl::models::WslStatus::Stopped {
                                all_stopped = false;
                                break;
                            }
                        }
                    }
                }
                if all_stopped {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.mark_all_distros_stopped().await;
                }
                
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        if all_stopped {
                            app.set_current_message(i18n::t("dialog.stop_wsl_success").into());
                        } else {
                            app.set_current_message("wsl --shutdown executed, but not all distributions are stopped.".into());
                        }
                        app.set_show_message_dialog(true);
                    }
                });
            }
        });
    });
}
