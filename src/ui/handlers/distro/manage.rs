// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use slint::Model;
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;
use crate::ui::handlers::instance;
use crate::utils::system::copy_to_clipboard;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Handle message link click (supports clipboard copy when url is "clipboard")
    {
        let ah = app_handle.clone();
        app.on_message_link_clicked(move || {
            if let Some(app) = ah.upgrade() {
                let url = app.get_current_message_url().to_string();

                if url == "clipboard" {
                    let cmd = app.get_current_message_link().to_string();
                    if !cmd.is_empty() {
                        let _ = copy_to_clipboard(&cmd);
                    }
                    return;
                }

                let mut link = url;
                if link.is_empty() {
                    link = app.get_current_message_link().to_string();
                }
                
                if link.starts_with("http://") || link.starts_with("https://") {
                    let _ = open::that(link);
                } else {
                    let path = std::path::Path::new(&link);
                    if path.exists() {
                        let _ = open::that(link);
                    } else if let Ok(startup_dir) = crate::app::autostart::get_startup_dir() {
                        let _ = open::that(startup_dir.to_string_lossy().to_string());
                    }
                }
            }
        });
    }

    // Terminal
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_terminal_distro(move |name| {
            info!("Operation: Open terminal - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            tokio::spawn(async move {
                let manager = {
                    let app_state = as_ptr.lock().await;
                    app_state.wsl_dashboard.clone()
                };

                if let Some(op) = manager.get_active_op(&name).await {
                    let msg = i18n::tr("toast.distro_busy", &[name.to_string(), op.to_string()]);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.set_current_message(msg.into());
                            app.set_show_message_dialog(true);
                        }
                    });
                    return;
                }



                {
                    let lock_timeout = std::time::Duration::from_millis(500);
                    if let Ok(app_state) = tokio::time::timeout(lock_timeout, as_ptr.lock()).await {
                        let executor = app_state.wsl_dashboard.executor().clone();
                        let instance_config = app_state.config_manager.get_instance_config(&name);
                        let working_dir = instance_config.terminal_dir.clone();
                        let terminal_proxy_enabled = instance_config.terminal_proxy;
                        let proxy_config = app_state.config_manager.get_network_config().proxy.clone();
                        drop(app_state);
                        
                        let mut proxy_exports: Option<Vec<(String, String)>> = None;
                        if terminal_proxy_enabled && proxy_config.is_enabled && !proxy_config.host.is_empty() && !proxy_config.port.is_empty() {
                            let auth = if proxy_config.auth_enabled && !proxy_config.username.is_empty() && !proxy_config.password.is_empty() {
                                format!("{}:{}@", proxy_config.username, proxy_config.password)
                            } else {
                                "".to_string()
                            };
                            let proxy_url = format!("http://{}{}:{}", auth, proxy_config.host, proxy_config.port);
                            
                            let mut exports = Vec::new();
                            exports.push(("HTTP_PROXY".to_string(), proxy_url.clone()));
                            exports.push(("HTTPS_PROXY".to_string(), proxy_url.clone()));
                            
                            if !proxy_config.no_proxy.is_empty() {
                                exports.push(("NO_PROXY".to_string(), proxy_config.no_proxy.clone()));
                            }
                            proxy_exports = Some(exports);
                        }
                        
                        let _ = executor.open_distro_terminal(&name, &working_dir, proxy_exports).await;
                    }
                }
                refresh_distros_ui(ah, as_ptr).await;
            });
        });
    }

    // Folder
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_folder_distro(move |name| {
            info!("Operation: Open folder - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            tokio::spawn(async move {
                let manager = {
                    let app_state = as_ptr.lock().await;
                    app_state.wsl_dashboard.clone()
                };

                if let Some(op) = manager.get_active_op(&name).await {
                    let msg = i18n::tr("toast.distro_busy", &[name.to_string(), op.to_string()]);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.set_current_message(msg.into());
                            app.set_show_message_dialog(true);
                        }
                    });
                    return;
                }



                {
                    let lock_timeout = std::time::Duration::from_millis(500);
                    if let Ok(app_state) = tokio::time::timeout(lock_timeout, as_ptr.lock()).await {
                        let executor = app_state.wsl_dashboard.executor().clone();
                        drop(app_state);
                        let _ = executor.open_distro_folder(&name).await;
                    }
                }
                refresh_distros_ui(ah, as_ptr).await;
            });
        });
    }

    // VS Code
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_vscode_distro(move |name| {
            info!("Operation: Try open VS Code - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let _ = slint::spawn_local(async move {
                let manager = {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.clone()
                };

                if let Some(op) = manager.get_active_op(&name).await {
                    let msg = i18n::tr("toast.distro_busy", &[name.to_string(), op.to_string()]);
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                let ah_timer = ah.clone();
                let executor = manager.executor().clone();
                let check_result = crate::wsl::ops::ui::check_vscode_extension(&executor).await;
                let is_valid_version = check_result.success && check_result.output.contains("ms-vscode-remote.remote-wsl");

                if is_valid_version {
                    if let Some(app) = ah.upgrade() {
                        app.set_show_vscode_startup(true);
                    }

                    let working_dir = {
                        let state = as_ptr.lock().await;
                        state.config_manager.get_instance_config(&name).vscode_dir
                    };
                    
                    let _ = executor.open_distro_vscode(&name, &working_dir).await;
                    refresh_distros_ui(ah, as_ptr).await;

                    slint::Timer::single_shot(std::time::Duration::from_secs(6), move || {
                        if let Some(app) = ah_timer.upgrade() {
                            if app.get_show_vscode_startup() {
                                app.set_show_vscode_startup(false);
                            }
                        }
                    });
                } else {
                    let mut ext_info = {
                        let state = as_ptr.lock().await;
                        state.vscode_extension.clone()
                    };

                    if ext_info.is_none() {
                        instance::refresh_vscode_extension(as_ptr.clone()).await;
                        ext_info = {
                            let state = as_ptr.lock().await;
                            state.vscode_extension.clone()
                        };
                    }

                    let (ext_name, ext_url) = if let Some(info) = ext_info {
                        (info.name, info.url)
                    } else {
                        let default = crate::app::VSCodeExtensionData::default();
                        (default.name, default.url)
                    };

                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(i18n::t("dialog.vscode_extension_required").into());
                        app.set_current_message_link(ext_name.into());
                        app.set_current_message_url(ext_url.into());
                        app.set_show_message_dialog(true);
                    }
                }
            });
        });
    }

    // Edit .bashrc
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_edit_bashrc_distro(move |name| {
            info!("Operation: Edit .bashrc - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let _ = slint::spawn_local(async move {
                let (dashboard, name_str) = {
                    let app_state = as_ptr.lock().await;
                    (app_state.wsl_dashboard.clone(), name.to_string())
                };

                if let Some(op) = dashboard.get_active_op(&name_str).await {
                    let msg = i18n::tr("toast.distro_busy", &[name_str.clone(), op.to_string()]);
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                // Sentinel Check: System heavy op?
                if dashboard.heavy_op_lock().try_lock().is_err() {
                    let msg = i18n::t("toast.system_busy");
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                dashboard.open_distro_bashrc(&name_str).await;
                refresh_distros_ui(ah, as_ptr).await;
            });
        });
    }

    // Information
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_information_clicked(move |name| {
            info!("Operation: Information clicked - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let name = name.to_string();
            let _ = slint::spawn_local(async move {
                let (dashboard, name_str) = {
                    let app_state = as_ptr.lock().await;
                    (app_state.wsl_dashboard.clone(), name.clone())
                };

                if let Some(op) = dashboard.get_active_op(&name_str).await {
                    let msg = i18n::tr("toast.distro_busy", &[name_str.clone(), op.to_string()]);
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                // Sentinel Check: System heavy op?
                if dashboard.heavy_op_lock().try_lock().is_err() {
                    let msg = i18n::t("toast.system_busy");
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }



                if let Some(app) = ah.upgrade() {
                    app.set_task_status_text(i18n::t("operation.fetching_info").into());
                    app.set_task_status_visible(true);
                }
                let result = dashboard.executor().get_distro_information(&name_str).await;
                if let Some(app) = ah.upgrade() {
                    app.set_task_status_visible(false);
                    if result.success {
                        if let Some(data) = result.data {
                            let mut slint_data = app.get_information();
                            slint_data.distro_name = data.distro_name.into();
                            slint_data.wsl_version = data.wsl_version.into();
                            slint_data.status = data.status.into();
                            slint_data.install_location = data.install_location.into();
                            slint_data.vhdx_path = data.vhdx_path.into();
                            slint_data.vhdx_size = data.vhdx_size.into();
                            slint_data.actual_used = data.actual_used.into();
                            slint_data.ip = data.ip.into();
                            slint_data.is_sparse = data.is_sparse;
                            app.set_information(slint_data);
                            app.set_show_information(true);
                        }
                    } else {
                        let err = result.error.unwrap_or_else(|| i18n::t("dialog.error"));
                        app.set_current_message(i18n::tr("dialog.info_failed", &[err]).into());
                        app.set_show_message_dialog(true);
                    }
                }
            });
        });
    }

    // Settings
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_settings_clicked(move |name| {
            info!("Operation: Settings clicked - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let name = name.to_string();
            let _ = slint::spawn_local(async move {
                let manager = {
                    let app_state = as_ptr.lock().await;
                    app_state.wsl_dashboard.clone()
                };

                if let Some(op) = manager.get_active_op(&name).await {
                    let msg = i18n::tr("toast.distro_busy", &[name.clone(), op.to_string()]);
                    if let Some(app) = ah.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                if let Some(app) = ah.upgrade() {
                    let mut is_default = false;
                    {
                        let distros = app.get_distros();
                        for i in 0..distros.row_count() {
                            if let Some(d) = distros.row_data(i) {
                                if d.name == name {
                                    is_default = d.is_default;
                                    break;
                                }
                            }
                        }
                    }

                    let instance_config = {
                        let state = as_ptr.lock().await;
                        state.config_manager.get_instance_config(&name)
                    };
                    
                    app.set_settings_distro_name(name.clone().into());
                    app.set_settings_is_default(is_default);
                    app.set_settings_lock_default(is_default);
                    app.set_settings_terminal_dir(instance_config.terminal_dir.into());
                    app.set_settings_vscode_dir(instance_config.vscode_dir.into());
                    app.set_settings_distro_notes(instance_config.distro_notes.into());
                    app.set_settings_startup_script(instance_config.startup_script.into());
                    app.set_settings_terminal_proxy(instance_config.terminal_proxy);
                    let is_task_exists = crate::network::scheduler::check_task_exists();
                    app.set_settings_autostart(instance_config.auto_startup && is_task_exists);
                    app.set_settings_is_task_exists(is_task_exists);
                    app.set_settings_terminal_dir_error("".into());
                    app.set_settings_vscode_dir_error("".into());
                    app.set_settings_distro_notes_error("".into());
                    app.set_settings_startup_script_error("".into());
                    app.set_settings_default_error("".into());
                    app.set_settings_enable_sparse(false);
                    app.set_settings_sparse_fixed(false);
                    app.set_show_settings(true);

                    let name_for_sparse = name.clone();
                    let ah_sparse = ah.clone();
                    tokio::spawn(async move {
                        let distros_reg = crate::utils::registry::get_wsl_distros_from_reg();
                        if let Some(reg_info) = distros_reg.into_iter().find(|d| d.name == name_for_sparse) {
                            if reg_info.version == 2 {
                                if let Some(p) = crate::wsl::ops::info::get_vhdx_path(&reg_info.base_path) {
                                    let is_sparse = crate::utils::system::is_sparse_file(&p.to_string_lossy());
                                    let _ = slint::invoke_from_event_loop(move || {
                                        if let Some(app) = ah_sparse.upgrade() {
                                            app.set_settings_enable_sparse(is_sparse);
                                            app.set_settings_sparse_fixed(is_sparse);
                                        }
                                    });
                                }
                            }
                        }
                    });

                    let as_fetch = as_ptr.clone();
                    tokio::spawn(async move {
                        instance::refresh_vscode_extension(as_fetch).await;
                    });
                    let ah_fetch2 = ah.clone();
                    tokio::spawn(async move {
                        instance::refresh_startup_script(ah_fetch2).await;
                    });
                }
            });
        });
    }

    // Settings confirm
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_confirm_distro_settings(move |name, terminal_dir, vscode_dir, distro_notes, is_default, autostart, startup_script, terminal_proxy| {
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let name = name.to_string();
            let terminal_dir = terminal_dir.to_string();
            let vscode_dir = vscode_dir.to_string();
            let distro_notes = distro_notes.to_string();
            let startup_script = startup_script.to_string();

            let _ = slint::spawn_local(async move {
                super::settings_logic::perform_save_settings(ah, as_ptr, name, terminal_dir, vscode_dir, distro_notes, is_default, autostart, startup_script, terminal_proxy).await;
            });
        });
    }

    // WSL Config click
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_configs_clicked(move |name| {
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let name = name.to_string();
            tokio::spawn(async move {
                let manager = {
                    let app_state = as_ptr.lock().await;
                    app_state.wsl_dashboard.clone()
                };

                if let Some(op) = manager.get_active_op(&name).await {
                    let msg = i18n::tr("toast.distro_busy", &[name.clone(), op.to_string()]);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.set_current_message(msg.into());
                            app.set_show_message_dialog(true);
                        }
                    });
                    return;
                }

                // Sentinel Check: System heavy op?
                if manager.heavy_op_lock().try_lock().is_err() {
                    let msg = i18n::t("toast.system_busy");
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.set_current_message(msg.into());
                            app.set_show_message_dialog(true);
                        }
                    });
                    return;
                }

                super::config_logic::handle_configs_clicked(ah, as_ptr, name).await;
            });
        });
    }

    // Config Preview
    {
        let ah = app_handle.clone();
        app.on_request_wsl_config_preview(move || {
            let ah = ah.clone();
            let _ = slint::spawn_local(async move {
                super::config_logic::handle_request_preview(ah).await;
            });
        });
    }

    // Config Save
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_save_wsl_config(move || {
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            tokio::spawn(async move {
                super::config_logic::handle_save_wsl_config(ah, as_ptr, false).await;
            });
        });
    }

    // Config Save & Restart
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_save_wsl_config_and_restart(move || {
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            tokio::spawn(async move {
                super::config_logic::handle_save_wsl_config(ah, as_ptr, true).await;
            });
        });
    }

    // Home click
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_home_clicked(move || {
            let as_ptr = as_outer.clone();
            if let Some(app) = ah_outer.upgrade() {
                let is_visible = app.get_is_window_visible();
                tokio::spawn(async move {
                    if crate::ui::data::should_refresh_wsl("manual trigger", is_visible) {
                        let dashboard = {
                            let state = as_ptr.lock().await;
                            state.wsl_dashboard.clone()
                        };
                        let _ = dashboard.refresh_distros().await;
                    }
                });
            }
        });
    }
}
