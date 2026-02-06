use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
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
                        slint_data.install_location = data.install_location.into();
                        slint_data.vhdx_path = data.vhdx_path.into();
                        slint_data.vhdx_size = data.vhdx_size.into();
                        slint_data.actual_used = data.actual_used.into();
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
                
                app.set_settings_distro_name(name.clone().into());
                app.set_settings_is_default(is_default);
                app.set_settings_lock_default(is_default);
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
    app.on_confirm_distro_settings(move |name, terminal_dir, vscode_dir, is_default, autostart, startup_script| {
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let name = name.to_string();
        let terminal_dir = terminal_dir.to_string();
        let vscode_dir = vscode_dir.to_string();
        let startup_script = startup_script.to_string();

        let _ = slint::spawn_local(async move {
            super::settings_logic::perform_save_settings(ah, as_ptr, name, terminal_dir, vscode_dir, is_default, autostart, startup_script).await;
        });
    });
}
