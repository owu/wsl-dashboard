use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};
use crate::{AppWindow, AppState, i18n};
use slint::Model;
use crate::ui::data::refresh_distros_ui;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Handle message link click (to open startup folder or generic URL)
    let ah_link = app_handle.clone();
    app.on_message_link_clicked(move || {
        if let Some(app) = ah_link.upgrade() {
            let mut link = app.get_current_message_url().to_string();
            if link.is_empty() {
                link = app.get_current_message_link().to_string();
            }
            
            if link.starts_with("http://") || link.starts_with("https://") {
                let _ = open::that(link);
            } else if let Ok(startup_dir) = crate::app::autostart::get_startup_dir() {
                // Legacy logic or fallback to startup dir if no specific link
                let target = if !link.is_empty() && std::path::Path::new(&link).exists() {
                    std::path::PathBuf::from(link)
                } else {
                    startup_dir
                };
                
                let mut cmd = std::process::Command::new("explorer.exe");
                cmd.arg(target);
                
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    cmd.creation_flags(CREATE_NO_WINDOW);
                }
                
                let _ = cmd.spawn();
            }
        }
    });


    // Shortcut
    let ah = app_handle.clone();
    app.on_create_distro_shortcut(move |name| {
        info!("Operation: Create shortcut - {}", name);
        let ah = ah.clone();
        match crate::utils::shortcut::create_distro_shortcut(&name) {
            Ok(_) => {
                if let Some(app) = ah.upgrade() {
                    app.set_current_message(format!("Desktop shortcut for {} created successfully.", name).into());
                    app.set_show_message_dialog(true);
                }
            },
            Err(e) => {
                if let Some(app) = ah.upgrade() {
                    app.set_current_message(format!("Failed to create shortcut: {}", e).into());
                    app.set_show_message_dialog(true);
                }
            }
        }
    });

    // Terminal
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_terminal_distro(move |name| {
        info!("Operation: Open terminal - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            {
                let app_state = as_ptr.lock().await;
                let executor = app_state.wsl_dashboard.executor().clone();
                let working_dir = app_state.config_manager.get_instance_config(&name).terminal_dir;
                drop(app_state);
                let _ = executor.open_distro_terminal(&name, &working_dir).await;
            }
            // Refresh status immediately after opening terminal
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Folder
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_folder_distro(move |name| {
        info!("Operation: Open folder - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            {
                let app_state = as_ptr.lock().await;
                let executor = app_state.wsl_dashboard.executor().clone();
                drop(app_state);
                let _ = executor.open_distro_folder(&name).await;
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // VS Code
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_vscode_distro(move |name| {
        info!("Operation: Try open VS Code - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            let ah_timer = ah.clone();
            
            // 1. Pre-check: Calling wsl exec execute code --version
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };
            
            let check_result = crate::wsl::ops::ui::check_vscode_version(&executor, &name).await;
            
            // 2. Logic based on pre-check result
            // A valid version output must succeed and start with a digit (e.g., "1.108.2")
            let is_valid_version = check_result.success && {
                let stdout = check_result.output.trim();
                !stdout.is_empty() && stdout.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            };

            if is_valid_version {
                // Success: proceed with existing logic
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
                // Failure: show prompt to install extension
                // Fetch VS Code extension info from AppState or try to fetch if missing
                let ext_info = {
                    let state = as_ptr.lock().await;
                    state.vscode_extension.clone()
                };

                let (ext_name, ext_url) = if let Some(info) = ext_info {
                    (info.name, info.url)
                } else {
                    // Fallback to default values
                    ("WSL".to_string(), "https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl".to_string())
                };

                if let Some(app) = ah.upgrade() {
                    // Use the extension name as the link text, and the URL for the background
                    app.set_current_message(i18n::t("dialog.vscode_extension_required").into());
                    app.set_current_message_link(ext_name.into());
                    app.set_current_message_url(ext_url.into());
                    app.set_show_message_dialog(true);
                }
            }
        });
    });

    // Edit configuration
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_edit_bashrc_distro(move |name| {
        info!("Operation: Edit .bashrc - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            let app_state = as_ptr.lock().await;
            app_state.wsl_dashboard.open_distro_bashrc(&name).await;
            drop(app_state);
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Instance information
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_compact_distro(move |name, vhdx_path| {
        info!("Operation: Compact disk - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let name = name.to_string();
        let vhdx_path = vhdx_path.to_string();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(format!("Compacting disk for {}...", name).into());
                app.set_task_status_visible(true);
            }
            let result = {
                let state = as_ptr.lock().await;
                let executor = state.wsl_dashboard.executor().clone();
                drop(state);
                executor.compact_distro_disk(&name, &vhdx_path).await
            };
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
                if result.success {
                    app.set_current_message(format!("Disk for {} compacted successfully.", name).into());
                } else {
                    app.set_current_message(format!("Failed to compact disk: {}", result.error.unwrap_or_default()).into());
                }
                app.set_show_message_dialog(true);
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_information_clicked(move |name| {
        info!("Operation: Fetch information - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let name = name.to_string();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(i18n::t("operation.fetching_info").into());
                app.set_task_status_visible(true);
            }
            let result = {
                let app_state = as_ptr.lock().await;
                let executor = app_state.wsl_dashboard.executor().clone();
                drop(app_state);
                executor.get_distro_information(&name).await
            };
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
                if result.success {
                    if let Some(data) = result.data {
                        let mut slint_data = app.get_information();
                        slint_data.distro_name = data.distro_name.into();
                        slint_data.wsl_version = data.wsl_version.into();
                        slint_data.status = data.status.into();
                        slint_data.ip_address = data.ip_address.into();
                        slint_data.install_location = data.install_location.into();
                        slint_data.vhdx_path = data.vhdx_path.into();
                        slint_data.vhdx_size = data.vhdx_size.into();
                        slint_data.actual_used = data.actual_used.into();
                        
                        let containers: Vec<slint::SharedString> = data.docker_containers.into_iter().map(|s| s.into()).collect();
                        slint_data.docker_containers = slint::ModelRc::from(std::sync::Arc::new(slint::VecModel::from(containers)));
                        
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

    // Instance settings
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_settings_clicked(move |name| {
        info!("Operation: Show settings - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let name = name.to_string();
        let _ = slint::spawn_local(async move {
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

                // Load from instances.toml
                let instance_config = {
                    let state = as_ptr.lock().await;
                    state.config_manager.get_instance_config(&name)
                };
                
                // Load Default UID
                let default_uid = {
                    let state = as_ptr.lock().await;
                    let executor = state.wsl_dashboard.executor().clone();
                    match executor.get_distro_default_uid(&name).await.data {
                         Some(uid) => uid,
                         None => 0
                    }
                };

                // Load wsl.conf Configuration
                let wsl_conf = {
                    let state = as_ptr.lock().await;
                    let executor = state.wsl_dashboard.executor().clone();
                    executor.get_wsl_conf(&name).await
                };

                app.set_settings_distro_name(name.clone().into());
                app.set_settings_is_default(is_default);
                app.set_settings_lock_default(is_default);
                app.set_settings_default_uid(default_uid.to_string().into());
                app.set_settings_systemd_enabled(wsl_conf.systemd);
                app.set_settings_generate_hosts(wsl_conf.generate_hosts);
                app.set_settings_generate_resolv_conf(wsl_conf.generate_resolv_conf);
                app.set_settings_interop_enabled(wsl_conf.interop_enabled);
                app.set_settings_append_windows_path(wsl_conf.append_windows_path);
                app.set_settings_terminal_dir(instance_config.terminal_dir.into());
                app.set_settings_vscode_dir(instance_config.vscode_dir.into());
                app.set_settings_startup_script(instance_config.startup_script.into());
                let is_vbs_enabled = crate::app::autostart::is_autostart_enabled(&name);
                app.set_settings_autostart(instance_config.auto_startup && is_vbs_enabled);
                app.set_settings_terminal_dir_error("".into());
                app.set_settings_vscode_dir_error("".into());
                app.set_settings_startup_script_error("".into());
                app.set_settings_default_error("".into());
                app.set_show_settings(true);
            }
        });
    });

    // Handle settings confirmation
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_distro_settings(move |name, terminal_dir, vscode_dir, is_default, default_uid, systemd, hosts, resolv, interop, path, autostart, startup_script| {
        info!("Operation: Save settings - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let name = name.to_string();
        let terminal_dir = terminal_dir.to_string();
        let vscode_dir = vscode_dir.to_string();
        let startup_script = startup_script.to_string();
        let default_uid_str = default_uid.to_string();

        let _ = slint::spawn_local(async move {
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };

            
            // Check if it was already default
            let was_default = {
                let distros = executor.list_distros().await;
                distros.data.map(|list| list.iter().any(|d| d.name == name && d.is_default)).unwrap_or(false)
            };

            if let Some(app) = ah.upgrade() {
                let mut has_error = false;
                
                // 1. Validate paths and default status
                let terminal_exists = executor.check_path_exists(&name, &terminal_dir).await;
                let vscode_exists = executor.check_path_exists(&name, &vscode_dir).await;
                
                // a. Terminal & VS Code paths
                if !terminal_exists {
                    app.set_settings_terminal_dir_error(crate::i18n::t("dialog.path_not_found").into());
                    has_error = true;
                } else {
                    app.set_settings_terminal_dir_error("".into());
                }

                if !vscode_exists {
                    app.set_settings_vscode_dir_error(crate::i18n::t("dialog.path_not_found").into());
                    has_error = true;
                } else {
                    app.set_settings_vscode_dir_error("".into());
                }

                // b. Startup Script Validation (If enabled)
                if autostart && !startup_script.trim().is_empty() {
                    let (exists, executable) = executor.check_file_executable(&name, &startup_script).await;
                    if !exists {
                        app.set_settings_startup_script_error(crate::i18n::t("dialog.script_not_found").into());
                        has_error = true;
                    } else if !executable {
                        app.set_settings_startup_script_error(crate::i18n::t("dialog.script_not_executable").into());
                        has_error = true;
                    } else {
                        app.set_settings_startup_script_error("".into());
                    }
                } else {
                    app.set_settings_startup_script_error("".into());
                }

                // c. Default distro error - UI handles this by hiding the option if already default
                app.set_settings_default_error("".into());

                if has_error {
                    return;
                }

                // If no error, close dialog and continue saving
                app.set_show_settings(false);
            }

            // 2. Save to instances.toml
            let config = crate::config::DistroInstanceConfig {
                terminal_dir,
                vscode_dir,
                auto_startup: autostart,
                startup_script: startup_script.clone(),
            };

            {
                let state = as_ptr.lock().await;
                if let Err(e) = state.config_manager.update_instance_config(&name, config) {
                    error!("Failed to save instance settings for '{}': {}", name, e);
                }
            }

            // 3a. Save Default UID
            if let Ok(uid) = default_uid_str.parse::<u32>() {
                 let _ = executor.set_distro_default_uid(&name, uid).await;
            }

            // 3b. Save wsl.conf Configuration
            let conf = crate::wsl::models::WslConf {
                systemd,
                generate_hosts: hosts,
                generate_resolv_conf: resolv,
                interop_enabled: interop,
                append_windows_path: path,
            };
            let _ = executor.set_wsl_conf(&name, conf).await;

            // 3c. Handle Default Distro (CLI)
            if is_default && !was_default {
                let _ = executor.execute_command(&["--set-default", &name]).await;
            }

            // 4. Handle Autostart
            if autostart {
                // a. Setup init script in Linux
                let mut script_content = String::from("#! /bin/sh\\n\\n");
                
                if !startup_script.trim().is_empty() {
                    // Execute user script in background, ignore all errors
                    script_content.push_str("# Execute user script in background\\n");
                    script_content.push_str(&format!("( {} ) > /dev/null 2>&1 &\\n\\n", startup_script.trim()));
                }

                script_content.push_str("# WSL Dashboard Keep-alive\\n");
                script_content.push_str("exec sleep infinity\\n");

                let setup_cmd = format!("printf '{}' > /etc/init.wsl-dashboard && chmod +x /etc/init.wsl-dashboard", script_content);
                let _ = executor.execute_command(&["-d", &name, "-u", "root", "-e", "sh", "-c", &setup_cmd]).await;
            }
            
            
            // b. Update Windows startup VBS (handles both true and false)
            let write_result = crate::app::autostart::update_windows_autostart(&name, autostart).await;
            
            // If write fails or times out, try backing up to the desktop
            if let Err(e) = &write_result {
                error!("Failed to update Windows autostart for '{}': {}", name, e);
                
                // If enabling autostart failed, back up to the desktop
                if autostart {
                    if let Some(desktop_dir) = dirs::desktop_dir() {
                        let backup_path = desktop_dir.join("wsl-dashboard.vbs");
                        
                        // Build VBS content and write to the desktop
                        let vbs_content = format!(
                            "Set ws = WScript.CreateObject(\"WScript.Shell\")\r\nws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide",
                            name
                        );
                        
                        if let Ok(_) = std::fs::write(&backup_path, vbs_content) {
                            info!("VBS file backed up to desktop: {}", backup_path.display());
                        }
                    }
                }
            }

            // c. Unified verification logic: Regardless of whether write succeeded, check if the file actually exists in the startup directory
            if autostart {
                let ah_verify = ah.clone();
                let name_clone = name.clone();
                let _ = slint::spawn_local(async move {
                    // Wait for 5 seconds to give anti-virus software and the system some time
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    
                    if let Ok(vbs_path) = crate::app::autostart::get_vbs_path() {
                        if !vbs_path.exists() {
                            warn!("Verification failed: VBS file does not exist in the startup directory, possibly intercepted or deleted by anti-virus software");
                            
                            // Ensure there is a backup file on the desktop
                            if let Some(desktop_dir) = dirs::desktop_dir() {
                                let backup_path = desktop_dir.join("wsl-dashboard.vbs");
                                
                                // If not on the desktop either, create one
                                if !backup_path.exists() {
                                    let vbs_content = format!(
                                        "Set ws = WScript.CreateObject(\"WScript.Shell\")\r\nws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide",
                                        name_clone
                                    );
                                    
                                    let _ = std::fs::write(&backup_path, vbs_content);
                                    info!("VBS file backed up to desktop: {}", backup_path.display());
                                }
                            }
                            
                            // Show a unified prompt dialog
                            if let Some(app) = ah_verify.upgrade() {
                                app.set_current_message(crate::i18n::t("dialog.autostart_timeout").into());
                                
                                // Add a clickable link to open the startup directory
                                if let Ok(startup_dir) = crate::app::autostart::get_startup_dir() {
                                    app.set_current_message_link(
                                        crate::i18n::tr("dialog.click_to_open_startup", &[startup_dir.to_string_lossy().to_string()]).into()
                                    );
                                }
                                
                                app.set_current_message_action("".into());
                                app.set_copy_script_content("".into());
                                app.set_show_message_dialog(true);
                            }
                        } else {
                            info!("✅ VBS file successfully created and exists in the startup directory");
                        }
                    }
                });
            }

            // 5. Refresh UI
            refresh_distros_ui(ah, as_ptr).await;
        });
    });
}
