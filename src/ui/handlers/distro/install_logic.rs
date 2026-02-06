use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;
use tracing::info;
use slint::Model;
use crate::{AppState, AppWindow, i18n};
use crate::ui::data::refresh_distros_ui;
use super::{sanitize_instance_name, generate_random_suffix};

pub async fn perform_install(
    ah: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    source_idx: i32,
    name: String,
    friendly_name: String,
    install_path: String,
    file_path: String,
) {
    let app = match ah.upgrade() {
        Some(a) => a,
        None => return,
    };

    app.set_is_installing(true);
    app.set_install_status(i18n::t("install.checking").into());
    app.set_install_success(false);
    app.set_terminal_output("".into());
    app.set_name_error("".into());

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
        app.set_name_error(i18n::t("dialog.name_required").into());
        app.set_is_installing(false);
        app.set_install_status(i18n::t("install.error").into());
        return;
    }

    let is_valid_chars = final_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if !is_valid_chars || final_name.len() > 25 {
        app.set_name_error(i18n::t("dialog.install_name_invalid").into());
        app.set_is_installing(false);
        app.set_install_status(i18n::t("install.error").into());
        return;
    }

    let distros = app.get_distros();
    let mut name_exists = false;
    for i in 0..distros.row_count() {
        if let Some(d) = distros.row_data(i) {
            if d.name == final_name {
                name_exists = true;
                break;
            }
        }
    }

    if name_exists {
        let new_suggested_name = sanitize_instance_name(&generate_random_suffix(&final_name));
        app.set_new_instance_name(new_suggested_name.clone().into());
        
        let distro_location = app.get_distro_location().to_string();
        let new_path = std::path::Path::new(&distro_location)
            .join(&new_suggested_name)
            .to_string_lossy()
            .to_string();
        app.set_new_instance_path(new_path.into());

        app.set_name_error(i18n::tr("dialog.install_name_exists", &[final_name.clone()]).into());
        app.set_is_installing(false);
        app.set_install_status(i18n::t("install.conflict_error").into());
        return;
    }

    let ah_inner = ah.clone();
    let executor = {
        let state = as_ptr.lock().await;
        state.wsl_dashboard.executor().clone()
    };

    let mut success = false;
    let mut error_msg = String::new();

    match source_idx {
        2 => { // Store
            let mut real_id = String::new();
            let installables = app.get_installable_distros();
            for i in 0..installables.row_count() {
                if let Some(d) = installables.row_data(i) {
                    if d.friendly_name == friendly_name {
                        real_id = d.name.to_string();
                        break;
                    }
                }
            }

            if real_id.is_empty() {
                app.set_install_status(i18n::t("install.unknown_distro").into());
                app.set_is_installing(false);
                return;
            }

            app.set_install_status(i18n::t("install.installing").into());
            app.set_terminal_output(format!("{}\n", i18n::tr("install.step_1", &[real_id.clone()])).into());
            info!("Starting store installation for distribution ID: {}", real_id);
            
            let _ = {
                let config_manager = {
                    let state = as_ptr.lock().await;
                    state.config_manager.clone()
                };
                executor.delete_distro(&config_manager, &real_id).await
            };
            
            let mut current = app.get_terminal_output().to_string();
            current.push_str(&format!("{}\n", i18n::t("install.step_2")));
            app.set_terminal_output(current.into());

            let use_web_download = executor.detect_fastest_source().await;
            
            let mut install_args = vec!["--install", "-d", &real_id, "--no-launch"];
            if use_web_download {
                install_args.push("--web-download");
            }
            let cmd_str = format!("wsl {}", install_args.join(" "));
            
            let mut current = app.get_terminal_output().to_string();
            current.push_str(&format!("{}\n", i18n::tr("install.step_3", &[cmd_str.clone()])));
            app.set_terminal_output(current.into());

            let source_text = if use_web_download { "GitHub" } else { "Microsoft" };
            let mut current = app.get_terminal_output().to_string();
            current.push_str(&i18n::tr("install.step_4", &[source_text.to_string()])); 
            app.set_terminal_output(current.into());

            let timer = slint::Timer::default();
            let ah_timer = ah_inner.clone();
            let start_time = std::time::Instant::now();
            let base_content = app.get_terminal_output().to_string();
            let mut frame = 0;

            timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
                 if let Some(app) = ah_timer.upgrade() {
                     let elapsed = start_time.elapsed().as_secs();
                     let max_dots = (3 + (elapsed / 5)) as usize;
                     let max_dots = max_dots.min(18);
                     
                     frame += 1;
                     let cycle = 2 * (max_dots - 1);
                     let current_dots = if cycle == 0 {
                         1
                     } else {
                         let x = frame % cycle;
                         (max_dots as isize - (max_dots as isize - 1 - x as isize).abs()) as usize
                     };

                     let mut new_output = base_content.clone();
                     new_output.push_str(&".".repeat(current_dots));
                     app.set_terminal_output(new_output.into());
                 }
            });
            
            let result = executor.execute_command_streaming(&install_args, |_| {}).await;
            timer.stop();
            let mut current = app.get_terminal_output().to_string();
            current.push('\n');
            app.set_terminal_output(current.into());

            if result.success {
                let mut distro_registered = false;
                app.set_install_status(i18n::t("install.verifying").into());
                for _ in 0..15 {
                     let list_result = executor.list_distros().await;
                     if let Some(distros) = list_result.data {
                        if distros.iter().any(|d| d.name == real_id) {
                            distro_registered = true;
                            break;
                        }
                     }
                     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }

                if !distro_registered {
                     error_msg = i18n::tr("install.verify_failed", &[real_id.clone()]);
                } else {
                    let mut current = app.get_terminal_output().to_string();
                     current.push_str(&format!("{}\n", i18n::t("install.step_5")));
                    app.set_terminal_output(current.into());

                    if final_name != real_id || !install_path.is_empty() {
                        info!("Customization requested (Name: {}, Path: {}). Starting relocation...", final_name, install_path);
                         app.set_install_status(i18n::t("install.customizing").into());
                         
                         let mut current = app.get_terminal_output().to_string();
                         current.push_str(&format!("{}\n", i18n::t("install.step_6")));
                        app.set_terminal_output(current.into());

                        let (temp_dir, temp_file_str) = {
                            let state = as_ptr.lock().await;
                            let temp_dir = PathBuf::from(&state.config_manager.get_settings().temp_location);
                            let temp_file = temp_dir.join(format!("wsl_move_{}.tar", uuid::Uuid::new_v4()));
                            (temp_dir, temp_file.to_string_lossy().to_string())
                        };
                        
                        let _ = std::fs::create_dir_all(&temp_dir);
                        let target_path = install_path.clone();

                        executor.execute_command(&["--export", &real_id, &temp_file_str]).await;
                        
                        let mut current = app.get_terminal_output().to_string();
                        current.push_str(&format!("{}\n", i18n::t("install.step_7")));
                        app.set_terminal_output(current.into());

                        executor.execute_command(&["--unregister", &real_id]).await;
                        
                        let final_path = if target_path.is_empty() {
                            let distro_location = {
                                let state = as_ptr.lock().await;
                                state.config_manager.get_settings().distro_location.clone()
                            };
                            let base = PathBuf::from(&distro_location);
                            base.join(&final_name).to_string_lossy().to_string()
                        } else {
                            target_path
                        };
                        let _ = std::fs::create_dir_all(&final_path);
                        info!("Importing to final path: {}", final_path);
                        
                        let import_res = executor.execute_command(&["--import", &final_name, &final_path, &temp_file_str]).await;
                        let _ = std::fs::remove_file(&temp_file_str);
                        
                        success = import_res.success;
                        if success {
                            let mut current = app.get_terminal_output().to_string();
                            current.push_str(&format!("{}\n", i18n::t("install.step_8")));
                            app.set_terminal_output(current.into());

                            let list_result = executor.list_distros().await;
                             if let Some(distros) = list_result.data {
                                if distros.iter().any(|d| d.name == final_name) {
                                     let mut current = app.get_terminal_output().to_string();
                                     current.push_str(&format!("{}\n", i18n::t("install.step_9")));
                                     app.set_terminal_output(current.into());
                                }
                             }
                        } else {
                            error_msg = import_res.error.unwrap_or_else(|| i18n::t("install.import_failed_custom"));
                        }
                    } else {
                        success = true;
                        let mut current = app.get_terminal_output().to_string();
                        current.push_str(&format!("{}\n", i18n::t("install.step_9")));
                        app.set_terminal_output(current.into());
                    }
                }
            } else {
                if !result.output.trim().is_empty() {
                     let mut current = app.get_terminal_output().to_string();
                     current.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                     app.set_terminal_output(current.into());
                }
                error_msg = result.error.unwrap_or_else(|| i18n::t("install.install_failed"));
            }
        },
        0 | 1 => { // RootFS or VHDX
            if file_path.is_empty() {
                error_msg = i18n::t("install.select_file");
            } else {
                app.set_terminal_output(format!("{}\n", i18n::t("install.step_1_3")).into());
                
                let mut target_path = install_path.clone();
                if target_path.is_empty() {
                    let distro_location = {
                        let state = as_ptr.lock().await;
                        state.config_manager.get_settings().distro_location.clone()
                    };
                    let base = PathBuf::from(&distro_location);
                    target_path = base.join(&final_name).to_string_lossy().to_string();
                }
                if let Err(e) = std::fs::create_dir_all(&target_path) {
                    error_msg = format!("Failed to create directory: {}", e);
                    app.set_install_success(false);
                    app.set_install_status(format!("Error: {}", error_msg).into());
                    app.set_is_installing(false);
                    return;
                }

                info!("Importing file '{}' to '{}' as '{}'...", file_path, target_path, final_name);

                let mut import_args = vec!["--import", &final_name, &target_path, &file_path];
                if source_idx == 1 {
                    import_args.push("--vhd");
                }

                app.set_install_status(i18n::t("install.importing").into());
                
                let cmd_str = format!("wsl {}", import_args.join(" "));
                let mut current = app.get_terminal_output().to_string();
                current.push_str(&format!("{}\n", i18n::tr("install.step_2_3", &[cmd_str.clone()])));
                app.set_terminal_output(current.into());

                let result = executor.execute_command_streaming(&import_args, |_| {}).await;

                success = result.success;
                if !success {
                     if !result.output.trim().is_empty() {
                         let mut current = app.get_terminal_output().to_string();
                         current.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                         app.set_terminal_output(current.into());
                    }
                    error_msg = result.error.unwrap_or_else(|| i18n::t("install.import_failed"));
                } else {
                    let mut current = app.get_terminal_output().to_string();
                    current.push_str(&format!("{}\n", i18n::tr("install.step_3_3", &[final_name.clone()])));
                    app.set_terminal_output(current.into());
                }
            }
        },
        _ => {
            error_msg = i18n::t("install.unknown_source");
        }
    }

    if success {
        app.set_install_success(true);
        app.set_install_status(i18n::tr("install.created_success", &[final_name.clone()]).into());
        refresh_distros_ui(ah.clone(), as_ptr.clone()).await;
    } else {
        app.set_install_success(false);
        app.set_install_status(format!("{}: {}", i18n::t("install.error"), error_msg).into());
    }
    app.set_is_installing(false);
}
