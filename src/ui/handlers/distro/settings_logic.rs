use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};
use crate::{AppState, AppWindow};
use crate::ui::data::refresh_distros_ui;

pub async fn perform_save_settings(
    ah: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    name: String,
    terminal_dir: String,
    vscode_dir: String,
    is_default: bool,
    autostart: bool,
    startup_script: String,
) {
    info!("Operation: Save settings - {}", name);

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

        app.set_settings_default_error("".into());

        if has_error {
            return;
        }

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

    // 3. Handle Default Distro (CLI)
    if is_default && !was_default {
        let _ = executor.execute_command(&["--set-default", &name]).await;
    }

    // 4. Handle Autostart
    if autostart {
        let mut script_content = String::from("#! /bin/sh\\n\\n");
        if !startup_script.trim().is_empty() {
            script_content.push_str("# Execute user script in background\\n");
            script_content.push_str(&format!("( {} ) > /dev/null 2>&1 &\\n\\n", startup_script.trim()));
        }

        script_content.push_str("# WSL Dashboard Keep-alive\\n");
        script_content.push_str("exec sleep infinity\\n");

        let setup_cmd = format!("printf '{}' > /etc/init.wsl-dashboard && chmod +x /etc/init.wsl-dashboard", script_content);
        let _ = executor.execute_command(&["-d", &name, "-u", "root", "-e", "sh", "-c", &setup_cmd]).await;
        
        let write_result = crate::app::autostart::update_windows_autostart(&name, autostart).await;
        
        if let Err(e) = &write_result {
            error!("Failed to update Windows autostart for '{}': {}", name, e);
            if let Some(desktop_dir) = dirs::desktop_dir() {
                let backup_path = desktop_dir.join("wsl-dashboard.vbs");
                let vbs_content = format!(
                    "Set ws = WScript.CreateObject(\"WScript.Shell\")\r\nws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide",
                    name
                );
                let _ = std::fs::write(&backup_path, vbs_content);
            }
        }

        let ah_verify = ah.clone();
        let name_clone = name.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            if let Ok(vbs_path) = crate::app::autostart::get_vbs_path() {
                if !vbs_path.exists() {
                    warn!("Verification failed: VBS file does not exist in the startup directory");
                    if let Some(desktop_dir) = dirs::desktop_dir() {
                        let backup_path = desktop_dir.join("wsl-dashboard.vbs");
                        if !backup_path.exists() {
                             let vbs_content = format!(
                                "Set ws = WScript.CreateObject(\"WScript.Shell\")\r\nws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide",
                                name_clone
                            );
                            let _ = std::fs::write(&backup_path, vbs_content);
                        }
                    }
                    
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_verify.upgrade() {
                            app.set_current_message(crate::i18n::t("dialog.autostart_timeout").into());
                            if let Ok(startup_dir) = crate::app::autostart::get_startup_dir() {
                                app.set_current_message_link(
                                    crate::i18n::tr("dialog.click_to_open_startup", &[startup_dir.to_string_lossy().to_string()]).into()
                                );
                            }
                            app.set_show_message_dialog(true);
                        }
                    });
                }
            }
        });
    } else {
        let _ = crate::app::autostart::update_windows_autostart(&name, false).await;
    }

    refresh_distros_ui(ah, as_ptr).await;
}
