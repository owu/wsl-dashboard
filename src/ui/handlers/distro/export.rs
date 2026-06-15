// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use slint::ComponentHandle;
use crate::{AppWindow, AppState, i18n};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Export process
    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_open_export_dialog(move |name| {
            info!("Operation: Open export dialog - {}", name);
            let ah = ah_outer.clone();
            let as_ptr = as_outer.clone();
            let name_str = name.to_string();
            
            tokio::spawn(async move {
                let manager = {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.clone()
                };

                // Sentinel Check: Distro busy?
                if let Some(op) = manager.get_active_op(&name_str).await {
                    let msg = i18n::tr("toast.distro_busy", &[name_str.clone(), op.to_string()]);
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

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        if app.get_is_exporting() || app.get_is_cloning() || app.get_is_moving() {
                            app.set_current_message(i18n::t("dialog.operation_in_progress").into());
                            app.set_show_message_dialog(true);
                            return;
                        }
                        app.set_export_distro_name(name_str.into());
                        app.set_export_compress(true);
                        let default_path = app.get_distro_location();
                        app.set_export_target_path(default_path);
                        app.set_export_error("".into());
                        app.set_show_export_dialog(true);
                    }
                });
            });
        });
    }

    {
        let ah_select = app_handle.clone();
        app.on_select_export_folder(move || {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t("dialog.select_export_dir"))
                .pick_folder()
            {
                if let Some(app) = ah_select.upgrade() {
                    app.set_export_target_path(path.display().to_string().into());
                }
            }
        });
    }

    {
        let ah_outer = app_handle.clone();
        let as_outer = app_state.clone();
        app.on_confirm_export(move |distro_source, target_path| {
            info!("Operation: Confirm export - Source: {}, Target Dir: {}", distro_source, target_path);
            
            let ah_weak = ah_outer.clone();
            let as_ptr_outer = as_outer.clone();
            let distro_source = distro_source.to_string();
            let target_path = target_path.to_string();

            let _ = slint::spawn_local(async move {
                // Sentinel Check: System heavy op?
                let manager = {
                    let state = as_ptr_outer.lock().await;
                    state.wsl_dashboard.clone()
                };

                if manager.heavy_op_lock().try_lock().is_err() {
                    let msg = i18n::t("toast.system_busy");
                    if let Some(app) = ah_weak.upgrade() {
                        app.set_current_message(msg.into());
                        app.set_show_message_dialog(true);
                    }
                    return;
                }

                if let Some(app) = ah_weak.upgrade() {
                    let app: AppWindow = app;
                    if target_path.is_empty() {
                        app.set_export_error(i18n::t("dialog.select_target_dir").into());
                        return;
                    }

                    if app.get_is_exporting() || app.get_is_cloning() || app.get_is_moving() {
                        return;
                    }

                    app.set_export_error("".into());
                    let use_compress_inner = app.get_export_compress();
                    app.set_show_export_dialog(false);
                    
                    app.set_is_exporting(true);
                    
                    let ah_clone = app.as_weak();
                    let as_ptr = as_ptr_outer.clone();
                    let distro_source_inner = distro_source.to_string();
                    let target_path_inner = target_path.to_string();

                    tokio::spawn(async move {
                        let _guard = crate::ui::data::BusyGuard::new();
                        {
                            let state = as_ptr.lock().await;
                            state.wsl_dashboard.mark_distro_stopped(&distro_source_inner).await;
                        }
                        let stop_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
                        
                        let ah_status = ah_clone.clone();
                        let msg_inner = distro_source_inner.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_status.upgrade() {
                                let initial_msg = i18n::tr("operation.exporting_msg", &[msg_inner, "0 MB".to_string()]);
                                app.set_task_status_text(initial_msg.into());
                                app.set_task_status_visible(true);
                            }
                        });
                        
                        let extension = if use_compress_inner { "tar.gz" } else { "tar" };
                        let mut filename = format!("{}.{}", distro_source_inner, extension);
                        let mut export_file = std::path::Path::new(&target_path_inner).join(&filename);
                        
                        if export_file.exists() {
                            let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
                            filename = format!("{}.{}.{}", distro_source_inner, timestamp, extension);
                            export_file = std::path::Path::new(&target_path_inner).join(&filename);
                        }
                        
                        let export_file_str = export_file.to_string_lossy().to_string();
                        
                        super::spawn_file_size_monitor(
                            ah_clone.clone(),
                            export_file_str.clone(),
                            distro_source_inner.clone(),
                            "operation.exporting_msg".into(),
                            stop_signal.clone()
                        );

                        tokio::task::yield_now().await;
                        let result = {
                            let dashboard = {
                                let state = as_ptr.lock().await;
                                state.wsl_dashboard.clone()
                            };

                            if let Some(op) = dashboard.get_active_op(&distro_source_inner).await {
                                let msg = i18n::tr("toast.distro_busy", &[distro_source_inner.clone(), op.to_string()]);
                                let ah_toast = ah_clone.clone();
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(app) = ah_toast.upgrade() {
                                        app.set_current_message(msg.into());
                                        app.set_show_message_dialog(true);
                                    }
                                });
                                let ah_flags = ah_clone.clone();
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(app) = ah_flags.upgrade() {
                                        app.set_task_status_visible(false);
                                        app.set_is_exporting(false);
                                    }
                                });
                                return;
                            }

                            info!("Exporting distribution '{}' to '{}'...", distro_source_inner, export_file_str);
                            dashboard.export_distro(&distro_source_inner, &export_file_str).await
                        };

                        stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

                        let ah_final = ah_clone.clone();
                        let distro_final = distro_source_inner.clone();
                        let file_final = export_file_str.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_final.upgrade() {
                                app.set_task_status_visible(false);
                                app.set_is_exporting(false);
                                
                                if result.success {
                                    app.set_current_message(i18n::tr("dialog.export_success", &[distro_final, file_final]).into());
                                } else {
                                    let err = result.error.unwrap_or_else(|| i18n::t("dialog.error"));
                                    app.set_current_message(i18n::tr("dialog.export_failed", &[err]).into());
                                }
                                app.set_show_message_dialog(true);
                            }
                        });
                    });
                }
            });
        });
    }
}


