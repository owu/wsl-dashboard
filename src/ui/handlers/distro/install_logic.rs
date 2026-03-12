use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;
use tracing::{info, error};
use crate::{AppState, AppWindow, i18n};
use crate::ui::data::refresh_distros_ui;
use super::{sanitize_instance_name, generate_random_suffix};

pub async fn perform_install(
    ah: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    source_idx: i32,
    name: String,
    friendly_name: String,
    internal_id: String,
    install_path: String,
    file_path: String,
) {
    info!("perform_install started: source={}, name={}, friendly={}, internal_id={}, path={}", 
          source_idx, name, friendly_name, internal_id, install_path);

    // Guard against UI thread blocks - yield initially
    tokio::task::yield_now().await;

    // 2. Setup initial state and manual operation guard
    let (dashboard, executor, config_manager, distro_snapshot) = {
        let lock_timeout = std::time::Duration::from_millis(3000);
        match tokio::time::timeout(lock_timeout, as_ptr.lock()).await {
            Ok(state) => {
                // Get a snapshot of distros for conflict check (using async to avoid deadlock)
                let distros = state.wsl_dashboard.get_distros().await;
                (Arc::new(state.wsl_dashboard.clone()), state.wsl_dashboard.executor().clone(), state.config_manager.clone(), distros)
            },
            Err(_) => {
                error!("perform_install: Failed to acquire AppState lock within 3s");
                let ah_err = ah.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_err.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_install_status(i18n::t("install.error").into());
                        app_typed.set_terminal_output("Error: System is busy (AppState lock timeout). Please try again.".into());
                    }
                });
                return;
            }
        }
    };
    
    dashboard.increment_manual_operation();
    let dashboard_cleanup = dashboard.clone();
    let _manual_op_guard = scopeguard::guard(dashboard_cleanup, |db| {
        db.decrement_manual_operation();
    });

    info!("perform_install: Initializing UI state...");
    let ah_init = ah.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah_init.upgrade() {
            let app_typed: AppWindow = app;
            app_typed.set_is_installing(true);
            app_typed.set_install_status(i18n::t("install.checking").into());
            app_typed.set_install_success(false);
            app_typed.set_terminal_output("".into());
            app_typed.set_name_error("".into());
        }
    });

    // 3. Name validation and conflict detection
    let mut final_name = name.clone();
    if final_name.is_empty() {
        if source_idx == 2 {
            final_name = friendly_name.clone();
        } else if !file_path.is_empty() {
            if let Some(stem) = std::path::Path::new(&file_path).file_stem() {
                final_name = stem.to_string_lossy().to_string();
            }
        }
    }

    if final_name.is_empty() {
        let ah_err = ah.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_err.upgrade() {
                let app_typed: AppWindow = app;
                app_typed.set_name_error(i18n::t("dialog.name_required").into());
                app_typed.set_is_installing(false);
                app_typed.set_install_status(i18n::t("install.error").into());
            }
        });
        return;
    }

    let is_valid_chars = final_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if !is_valid_chars || final_name.len() > 25 {
        let ah_err = ah.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_err.upgrade() {
                let app_typed: AppWindow = app;
                app_typed.set_name_error(i18n::t("dialog.install_name_invalid").into());
                app_typed.set_is_installing(false);
                app_typed.set_install_status(i18n::t("install.error").into());
            }
        });
        return;
    }

    let name_exists = distro_snapshot.iter().any(|d| d.name == final_name);

    if name_exists {
        let new_suggested_name = sanitize_instance_name(&generate_random_suffix(&final_name));
        let ah_err = ah.clone();
        let mut distro_location = String::new();
        if let Some(app) = ah_err.upgrade() {
             let app_typed: AppWindow = app;
            distro_location = app_typed.get_distro_location().to_string();
        }
        
        let new_path = std::path::Path::new(&distro_location)
            .join(&new_suggested_name)
            .to_string_lossy()
            .to_string();

        let final_name_clone = final_name.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_err.upgrade() {
                let app_typed: AppWindow = app;
                app_typed.set_new_instance_name(new_suggested_name.into());
                app_typed.set_new_instance_path(new_path.into());
                app_typed.set_name_error(i18n::tr("dialog.install_name_exists", &[final_name_clone]).into());
                app_typed.set_is_installing(false);
                app_typed.set_install_status(i18n::t("install.conflict_error").into());
            }
        });
        return;
    }

    let mut success = false;
    let mut error_msg = String::new();

    // 4. Source-specific installation logic
    match source_idx {
        2 => { // Store Source
            let real_id = if !internal_id.is_empty() {
                internal_id.clone()
            } else {
                // Fallback for custom RootFS/VHDX if applicable, though usually they go through different match arms
                friendly_name.clone()
            };

            if real_id.is_empty() {
                let ah_err = ah.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_err.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_install_status(i18n::t("install.unknown_distro").into());
                        app_typed.set_is_installing(false);
                    }
                });
                return;
            }

            let ah_status = ah.clone();
            let real_id_clone = real_id.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_status.upgrade() {
                    let app_typed: AppWindow = app;
                    app_typed.set_install_status(i18n::t("install.installing").into());
                    app_typed.set_terminal_output(format!("{}\n", i18n::tr("install.step_1", &[real_id_clone])).into());
                }
            });
            let mut terminal_buffer = format!("{}\n", i18n::tr("install.step_1", &[real_id.clone()]));
            info!("Starting store installation for distribution ID: {}", real_id);
            
            // Cleanup existing if any
            let _ = executor.delete_distro(&config_manager, &real_id).await;
            
            terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_2")));
            let ah_cb = ah.clone();
            let tb_clone = terminal_buffer.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_cb.upgrade() {
                     let app_typed: AppWindow = app;
                    app_typed.set_terminal_output(tb_clone.into());
                }
            });

            // Detect fastest source can involve network calls
            let use_web_download = executor.detect_fastest_source().await;
            
            let mut install_args = vec!["--install", "-d", &real_id, "--no-launch"];
            if use_web_download {
                install_args.push("--web-download");
            }
            let cmd_str = format!("wsl {}", install_args.join(" "));
            
            terminal_buffer.push_str(&format!("{}\n", i18n::tr("install.step_3", &[cmd_str.clone()])));
            let source_text = if use_web_download { "GitHub" } else { "Microsoft" };
            terminal_buffer.push_str(&i18n::tr("install.step_4", &[source_text.to_string()])); 
            
            let ah_cb = ah.clone();
            let tb_clone = terminal_buffer.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_cb.upgrade() {
                    let app_typed: AppWindow = app;
                    app_typed.set_terminal_output(tb_clone.into());
                }
            });

            info!("Installing from source: {}", source_text);

            let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
            
            // Channel-based UI update task to throttle updates and prevent freezing
            let ah_ui = ah.clone();
            let initial_tb = terminal_buffer.clone();
            let ui_task = tokio::spawn(async move {
                let mut buffer = initial_tb;
                let mut dot_count = 0;
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(800));
                
                loop {
                    tokio::select! {
                        msg = rx.recv() => {
                            if msg.is_none() {
                                break; // Channel closed
                            }
                            // We consume the messages but don't append to buffer to hide all WSL output
                        }
                        _ = interval.tick() => {
                            // Only add dots if the current line is an "active" one (doesn't end in newline)
                            if !buffer.ends_with('\n') {
                                dot_count = (dot_count % 3) + 1; // Always show 1, 2, or 3 dots
                                let mut dots = String::new();
                                for _ in 0..dot_count { dots.push('.'); }
                                let text_to_set = format!("{}{}", buffer, dots);
                                
                                let ah_cb = ah_ui.clone();
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(app) = ah_cb.upgrade() {
                                        app.set_terminal_output(text_to_set.into());
                                    }
                                });
                            }
                        }
                    }

                    if buffer.len() > 20_000 {
                        let to_drain = buffer.len() - 10_000;
                        if let Some(pos) = buffer[to_drain..].find('\n') {
                            buffer.drain(..to_drain + pos + 1);
                        } else {
                            buffer.drain(..to_drain);
                        }
                    }

                     // Throttled UI update removed to prevent overwriting dots animation 
                     // since all WSL output is hidden in this phase.
                }
                
                // No final flush needed since we are hiding all WSL output
                
                let ah_final = ah_ui.clone();
                let text_to_set = buffer.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_final.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_terminal_output(text_to_set.into());
                    }
                });
                buffer
            });

             info!("Waiting for WSL installation to complete...");
            let tx_callback = tx.clone();
            let result = executor.execute_command_streaming(&install_args, move |text| {
                let _ = tx_callback.try_send(text);
            }).await;
            
            drop(tx);
            terminal_buffer = ui_task.await.unwrap_or(terminal_buffer);
            // Don't add newline yet, verification will keep dots rolling
            if !terminal_buffer.ends_with('.') && !terminal_buffer.ends_with('\n') {
                terminal_buffer.push_str("."); // Start with one dot to bridge the gap
            }
            
            let ah_res = ah.clone();
            let tb_clone = terminal_buffer.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_res.upgrade() {
                    let app_typed: AppWindow = app;
                    app_typed.set_terminal_output(tb_clone.into());
                }
            });

            if result.success {
                let mut distro_registered = false;
                let ah_status = ah.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_status.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_install_status(i18n::t("install.verifying").into());
                    }
                });

                let mut verify_dot_count = 1;
                for _ in 0..15 {
                     dashboard.refresh_distros().await;
                     
                     let distros_final = dashboard.get_distros().await;
                     if distros_final.iter().any(|d| d.name == real_id) {
                         distro_registered = true;
                         break;
                     }
                     
                     // Keep dots rolling even during verification (2s sleep -> small steps)
                     for _ in 0..3 {
                         tokio::time::sleep(std::time::Duration::from_millis(666)).await;
                         verify_dot_count = (verify_dot_count % 3) + 1;
                         let mut dots = String::new();
                         for _ in 0..verify_dot_count { dots.push('.'); }
                         let text_to_set = format!("{}{}", terminal_buffer, dots);
                         
                         let ah_v = ah.clone();
                         let _ = slint::invoke_from_event_loop(move || {
                             if let Some(app) = ah_v.upgrade() {
                                 app.set_terminal_output(text_to_set.into());
                             }
                         });
                     }
                }
                
                // Final dots and newline before next step
                terminal_buffer.push_str("...");
                terminal_buffer.push('\n');

                if !distro_registered {
                     error_msg = i18n::tr("install.verify_failed", &[real_id.clone()]);
                } else {
                    terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_5")));
                    let ah_cb = ah.clone();
                    let tb_clone = terminal_buffer.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_cb.upgrade() {
                            let app_typed: AppWindow = app;
                            app_typed.set_terminal_output(tb_clone.into());
                        }
                    });

                    if final_name != real_id || !install_path.is_empty() {
                         info!("Relocating distribution to {}...", install_path);
                         let ah_cb = ah.clone();
                         let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_cb.upgrade() {
                                let app_typed: AppWindow = app;
                                app_typed.set_install_status(i18n::t("install.customizing").into());
                            }
                         });
                         
                         terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_6")));
                         let ah_cb = ah.clone();
                         let tb_clone = terminal_buffer.clone();
                         let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_cb.upgrade() {
                                let app_typed: AppWindow = app;
                                app_typed.set_terminal_output(tb_clone.into());
                            }
                         });

                        let (temp_dir, temp_file_str) = {
                            let temp_location = config_manager.get_settings().temp_location.clone();
                            let temp_dir = PathBuf::from(temp_location);
                            let temp_file = temp_dir.join(format!("wsl_move_{}.tar", uuid::Uuid::new_v4()));
                            (temp_dir, temp_file.to_string_lossy().to_string())
                        };
                        
                        let _ = tokio::task::spawn_blocking(move || std::fs::create_dir_all(&temp_dir)).await;
                        let target_path = install_path.clone();

                        tokio::task::yield_now().await;
                        executor.execute_command(&["--export", &real_id, &temp_file_str]).await;
                        
                        tokio::task::yield_now().await;
                        terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_7")));
                        let ah_cb = ah.clone();
                        let tb_clone = terminal_buffer.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_cb.upgrade() {
                                let app_typed: AppWindow = app;
                                app_typed.set_terminal_output(tb_clone.into());
                            }
                        });

                        tokio::task::yield_now().await;
                        executor.execute_command(&["--unregister", &real_id]).await;
                        
                        let final_path = if target_path.is_empty() {
                            let distro_location = config_manager.get_settings().distro_location.clone();
                            let base = PathBuf::from(&distro_location);
                            base.join(&final_name).to_string_lossy().to_string()
                        } else {
                            target_path
                        };
                        
                        let fp_clone = final_path.clone();
                        let _ = tokio::task::spawn_blocking(move || std::fs::create_dir_all(&fp_clone)).await;
                        
                        tokio::task::yield_now().await;
                        let import_res = executor.execute_command(&["--import", &final_name, &final_path, &temp_file_str]).await;
                        
                        let tf_clone = temp_file_str.clone();
                        let _ = tokio::task::spawn_blocking(move || std::fs::remove_file(&tf_clone)).await;
                        
                        success = import_res.success;
                        if success {
                            terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_8")));
                            terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_9")));
                        } else {
                            error_msg = import_res.error.unwrap_or_else(|| i18n::t("install.import_failed_custom"));
                        }
                    } else {
                        success = true;
                        terminal_buffer.push_str(&format!("{}\n", i18n::t("install.step_9")));
                    }
                    
                    let ah_cb = ah.clone();
                    let tb_clone = terminal_buffer.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_cb.upgrade() {
                            let app_typed: AppWindow = app;
                            app_typed.set_terminal_output(tb_clone.into());
                        }
                    });
                }
            } else {
                if !result.output.trim().is_empty() {
                     terminal_buffer.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                }
                error_msg = result.error.unwrap_or_else(|| i18n::t("install.install_failed"));
                let ah_cb = ah.clone();
                let tb_clone = terminal_buffer.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_cb.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_terminal_output(tb_clone.into());
                    }
                });
            }
        },
        0 | 1 => { // RootFS or VHDX Import
            if file_path.is_empty() {
                error_msg = i18n::t("install.select_file");
            } else {
                let mut terminal_buffer = format!("{}\n", i18n::t("install.step_1_3"));
                let ah_cb = ah.clone();
                let tb_clone = terminal_buffer.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_cb.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_terminal_output(tb_clone.into());
                    }
                });
                
                let mut target_path = install_path.clone();
                if target_path.is_empty() {
                    let distro_location = config_manager.get_settings().distro_location.clone();
                    let base = PathBuf::from(&distro_location);
                    target_path = base.join(&final_name).to_string_lossy().to_string();
                }
                
                let tp_clone = target_path.clone();
                if let Err(e) = tokio::task::spawn_blocking(move || std::fs::create_dir_all(&tp_clone)).await.unwrap() {
                    let err = format!("Failed to create directory: {}", e);
                    let ah_cb = ah.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_cb.upgrade() {
                            let app_typed: AppWindow = app;
                            app_typed.set_install_success(false);
                            app_typed.set_install_status(format!("{}: {}", i18n::t("install.error"), err).into());
                            app_typed.set_is_installing(false);
                        }
                    });
                    return;
                }

                let ah_cb = ah.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_cb.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_install_status(i18n::t("install.importing").into());
                    }
                });

                let mut import_args = vec!["--import", &final_name, &target_path, &file_path];
                if source_idx == 1 {
                    import_args.push("--vhd");
                }
                
                let cmd_str = format!("wsl {}", import_args.join(" "));
                terminal_buffer.push_str(&i18n::tr("install.step_2_3", &[cmd_str.clone()]));
                let ah_cb = ah.clone();
                let tb_clone = terminal_buffer.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_cb.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_terminal_output(tb_clone.into());
                    }
                });

                let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
                let ah_ui = ah.clone();
                let initial_tb = terminal_buffer.clone();
                let ui_task = tokio::spawn(async move {
                    let mut buffer = initial_tb;
                    let mut dot_count = 0;
                    let mut interval = tokio::time::interval(std::time::Duration::from_millis(800));
                    
                    loop {
                        tokio::select! {
                            msg = rx.recv() => {
                                if msg.is_none() {
                                    break;
                                }
                                // Consume but don't display
                            }
                            _ = interval.tick() => {
                                if !buffer.ends_with('\n') {
                                     dot_count = (dot_count % 3) + 1;
                                     let mut dots = String::new();
                                     for _ in 0..dot_count { dots.push('.'); }
                                     let text_to_set = format!("{}{}", buffer, dots);
                                     
                                     let ah_cb = ah_ui.clone();
                                     let _ = slint::invoke_from_event_loop(move || {
                                         if let Some(app) = ah_cb.upgrade() {
                                             app.set_terminal_output(text_to_set.into());
                                         }
                                     });
                                }
                            }
                        }

                        if buffer.len() > 20_000 {
                            let to_drain = buffer.len() - 10_000;
                            if let Some(pos) = buffer[to_drain..].find('\n') {
                                buffer.drain(..to_drain + pos + 1);
                            } else {
                                buffer.drain(..to_drain);
                            }
                        }
                         // Throttled UI update removed to prevent overwriting dots animation
                    }
                    if !buffer.ends_with('\n') {
                        buffer.push('\n');
                    }
                    let ah_final = ah_ui.clone();
                    let text_to_set = buffer.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_final.upgrade() {
                            let app_typed: AppWindow = app;
                            app_typed.set_terminal_output(text_to_set.into());
                        }
                    });
                    buffer
                });

                let tx_callback = tx.clone();
                let result = executor.execute_command_streaming(&import_args, move |text| {
                    let _ = tx_callback.try_send(text);
                }).await;

                drop(tx);
                terminal_buffer = ui_task.await.unwrap_or(terminal_buffer);

                success = result.success;
                if !success {
                     if !result.output.trim().is_empty() {
                         terminal_buffer.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                    }
                    error_msg = result.error.unwrap_or_else(|| i18n::t("install.import_failed"));
                } else {
                    terminal_buffer.push_str(&format!("{}\n", i18n::tr("install.step_3_3", &[final_name.clone()])));
                }
                
                let ah_cb = ah.clone();
                let tb_clone = terminal_buffer.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_cb.upgrade() {
                        let app_typed: AppWindow = app;
                        app_typed.set_terminal_output(tb_clone.into());
                    }
                });
            }
        },
        _ => {
            error_msg = i18n::t("install.unknown_source");
        }
    }

    let ah_final = ah.clone();
    let final_name_clone = final_name.clone();
    let error_msg_clone = error_msg.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah_final.upgrade() {
            let app_typed: AppWindow = app;
            if success {
                app_typed.set_install_success(true);
                app_typed.set_install_status(i18n::tr("install.created_success", &[final_name_clone]).into());
            } else {
                app_typed.set_install_success(false);
                app_typed.set_install_status(format!("{}: {}", i18n::t("install.error"), error_msg_clone).into());
            }
            app_typed.set_is_installing(false);
        }
    });
    
    if success {
        refresh_distros_ui(ah.clone(), as_ptr.clone()).await;
    }
}
