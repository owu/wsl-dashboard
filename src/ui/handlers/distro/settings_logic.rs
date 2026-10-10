// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::{AppState, AppWindow};
use crate::ui::data::refresh_distros_ui;

pub async fn perform_save_settings(
    ah: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    name: String,
    terminal_dir: String,
    vscode_dir: String,
    distro_notes: String,
    is_default: bool,
    autostart: bool,
    startup_script: String,
    terminal_proxy: bool,
) {
    info!("Operation: Save settings - {}", name);

    // Normalize user input: strip accidental leading/trailing whitespace so
    // both the existence check and the persisted value use the same path.
    let terminal_dir = terminal_dir.trim().to_string();
    let vscode_dir = vscode_dir.trim().to_string();

    let executor = {
        let lock_timeout = std::time::Duration::from_millis(500);
        match tokio::time::timeout(lock_timeout, as_ptr.lock()).await {
            Ok(state) => state.wsl_dashboard.executor().clone(),
            Err(_) => {
                error!("perform_save_settings: AppState lock timeout");
                return;
            }
        }
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

        if distro_notes.chars().count() > 24 {
            app.set_settings_distro_notes_error(crate::i18n::t("dialog.distro_notes_error").into());
            has_error = true;
        } else {
            app.set_settings_distro_notes_error("".into());
        }

        if autostart && !startup_script.trim().is_empty() {
            let startup_script_trimmed = startup_script.trim();
            if startup_script_trimmed == crate::app::WSL_INIT_SCRIPT {
                app.set_settings_startup_script_error(crate::i18n::t("dialog.startup_script_forbidden").into());
                has_error = true;
            } else {
                let (exists, executable) = executor.check_file_executable(&name, startup_script_trimmed).await;
                if !exists {
                    app.set_settings_startup_script_error(crate::i18n::t("dialog.script_not_found").into());
                    has_error = true;
                } else if !executable {
                    app.set_settings_startup_script_error(crate::i18n::t("dialog.script_not_executable").into());
                    has_error = true;
                } else {
                    app.set_settings_startup_script_error("".into());
                }
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
        distro_notes,
        auto_startup: autostart,
        startup_script: startup_script.clone(),
        terminal_proxy,
    };

    {
        let lock_timeout = std::time::Duration::from_millis(500);
        if let Ok(state) = tokio::time::timeout(lock_timeout, as_ptr.lock()).await {
            if let Err(e) = state.config_manager.update_instance_config(&name, config) {
                error!("Failed to save instance settings for '{}': {}", name, e);
            }
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

        let setup_cmd = format!("printf '{}' > {} && chmod +x {}", script_content, crate::app::WSL_INIT_SCRIPT, crate::app::WSL_INIT_SCRIPT);
        let _ = executor.execute_command(&["-d", &name, "-u", "root", "-e", "sh", "-c", &setup_cmd]).await;
        
        // 5. Check and register task if needed
        if !crate::network::scheduler::check_task_exists() {
            info!("Auto-start task not found, attempting to register with elevation...");
            match crate::network::scheduler::register_task_with_elevation() {
                Ok(_) => {
                    info!("Successfully registered auto-start task via elevation.");
                    // Show a toast success message
                    let ah_toast = ah.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_toast.upgrade() {
                            app.set_task_status_text("Task successfully scheduled".into());
                            app.set_task_status_visible(true);
                            
                            let ah_hide = ah_toast.clone();
                            slint::Timer::single_shot(std::time::Duration::from_secs(3), move || {
                                if let Some(app_h) = ah_hide.upgrade() {
                                    app_h.set_task_status_visible(false);
                                }
                            });
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to register auto-start task: {}", e);
                    let err_msg = if e.contains("InvalidOperation") || e.contains("denied") {
                        "Operation denied (UAC)".to_string()
                    } else {
                        format!("Failed to schedule: {}", e)
                    };
                    // Show a toast error message
                    let ah_toast = ah.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah_toast.upgrade() {
                            app.set_task_status_text(err_msg.into());
                            app.set_task_status_visible(true);
                            
                            let ah_hide = ah_toast.clone();
                            slint::Timer::single_shot(std::time::Duration::from_secs(3), move || {
                                if let Some(app_h) = ah_hide.upgrade() {
                                    app_h.set_task_status_visible(false);
                                }
                            });
                        }
                    });
                }
            }
        }
    }


    refresh_distros_ui(ah, as_ptr).await;
}
