use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::info;
use rand::Rng;
use rand::distr::Alphanumeric;
use slint::{ComponentHandle, Model};
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Clone process
    let ah = app_handle.clone();
    app.on_open_clone_dialog(move |name| {
        info!("Operation: Open clone dialog - {}", name);
        if let Some(app) = ah.upgrade() {
            if app.get_is_cloning() || app.get_is_exporting() || app.get_is_moving() {
                app.set_current_message(i18n::t("dialog.operation_in_progress").into());
                app.set_show_message_dialog(true);
                return;
            }
            // Generate 4-character random alphanumeric string
            let random_suffix: String = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(4)
                .map(char::from)
                .collect();
            
            let target_name = format!("{}_{}", name, random_suffix);
            let distro_location = app.get_distro_location();
            let target_path = std::path::Path::new(&distro_location.to_string())
                .join(&target_name)
                .to_string_lossy()
                .to_string();

            app.set_clone_source_name(name.clone().into());
            app.set_clone_target_name(target_name.into());
            app.set_clone_target_path(target_path.into());
            app.set_clone_error("".into());
            app.set_show_clone_dialog(true);
        }
    });

    let ah = app_handle.clone();
    app.on_select_clone_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_clone_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                let target_name = app.get_clone_target_name().to_string();
                let final_path = path.join(target_name).to_string_lossy().to_string();
                app.set_clone_target_path(final_path.into());
            }
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_clone(move |source_name, target_name, target_path, start_after_clone| {
        info!("Operation: Confirm clone - Source: {}, Target: {}, Path: {}, Start: {}", source_name, target_name, target_path, start_after_clone);
        let ah = match ah.upgrade() {
            Some(a) => a,
            None => return,
        };

        if ah.get_is_cloning() || ah.get_is_exporting() || ah.get_is_moving() {
            return;
        }

        // 1. Validation: Name length <= 24
        if target_name.len() > 24 {
            ah.set_clone_error(i18n::t("dialog.name_too_long").into());
            return;
        }

        // 2. Validation: ASCII Alphanumeric and -_. (Reject Chinese/Unicode)
        let is_valid_name = target_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
        if !is_valid_name {
            ah.set_clone_error(i18n::t("dialog.name_invalid").into());
            return;
        }

        // 3. Validation: Instance exists
        let distros = ah.get_distros();
        for i in 0..distros.row_count() {
            if let Some(d) = distros.row_data(i) {
                if d.name == target_name {
                    ah.set_clone_error(i18n::t("dialog.name_exists").into());
                    return;
                }
            }
        }

        // 4. Validation: Directory emptiness
        let p = std::path::Path::new(target_path.as_str());
        if p.exists() {
            if p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    if entries.count() > 0 {
                        ah.set_clone_error(i18n::t("dialog.dir_not_empty").into());
                        return;
                    }
                }
            } else {
                ah.set_clone_error(i18n::t("dialog.path_is_not_dir").into());
                return;
            }
        } else {
            // Create directory if not exists
            if let Err(e) = std::fs::create_dir_all(p) {
                ah.set_clone_error(i18n::tr("dialog.mkdir_failed", &[e.to_string()]).into());
                return;
            }
        }

        ah.set_clone_error("".into());
        ah.set_show_clone_dialog(false);
        
        let ah_clone = ah.as_weak();
        let as_ptr = as_ptr.clone();
        let source_name = source_name.to_string();
        let target_name = target_name.to_string();
        let target_path = target_path.to_string();

        let _ = slint::spawn_local(async move {
            // Setup monitor and indicator
            let stop_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
            if let Some(app) = ah_clone.upgrade() {
                app.set_is_cloning(true);
                let initial_msg = i18n::tr("operation.cloning_step1", &[source_name.clone(), "0 MB".to_string()]);
                app.set_task_status_text(initial_msg.into());
                app.set_task_status_visible(true);
            }

            let (temp_dir, temp_file_str) = {
                let state = as_ptr.lock().await;
                let temp_dir = PathBuf::from(&state.config_manager.get_settings().temp_location);
                let temp_file = temp_dir.join(format!("wsl_clone_{}.tar", uuid::Uuid::new_v4()));
                (temp_dir, temp_file.to_string_lossy().to_string())
            };

            let _ = std::fs::create_dir_all(&temp_dir);

            // Step 1: Export
            super::spawn_file_size_monitor(
                ah_clone.clone(),
                temp_file_str.clone(),
                source_name.clone(),
                "operation.cloning_step1".into(),
                stop_signal.clone()
            );

            let export_result = {
                let dashboard = {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.clone()
                };
                info!("Cloning distribution: exporting source '{}' to temp file '{}'...", source_name, temp_file_str);
                dashboard.export_distro(&source_name, &temp_file_str).await
            };

            // Stop indicator for Step 1
            stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

            if !export_result.success {
                if let Some(app) = ah_clone.upgrade() {
                    app.set_task_status_visible(false);
                    app.set_is_cloning(false);
                    let err = export_result.error.unwrap_or_else(|| i18n::t("dialog.export_failed").replace("{0}", ""));
                    app.set_current_message(i18n::tr("dialog.clone_failed_export", &[err]).into());
                    app.set_show_message_dialog(true);
                }
                let _ = std::fs::remove_file(&temp_file_str);
                return;
            }

            // Step 2: Import
            if let Some(app) = ah_clone.upgrade() {
                let msg = i18n::tr("operation.cloning_step2", &[source_name.clone()]);
                app.set_task_status_text(msg.into());
            }

            let import_result = {
                let dashboard = {
                    let state = as_ptr.lock().await;
                    state.wsl_dashboard.clone()
                };
                info!("Cloning distribution: importing as '{}' to '{}'...", target_name, target_path);
                dashboard.import_distro(&target_name, &target_path, &temp_file_str).await
            };

            // Step 3: Cleanup
            let _ = std::fs::remove_file(&temp_file_str);

            if let Some(app) = ah_clone.upgrade() {
                app.set_task_status_visible(false);
                app.set_is_cloning(false);
                if import_result.success {
                    if start_after_clone {
                        // Start the new distro
                        let state = as_ptr.lock().await;
                        let executor = state.wsl_dashboard.executor().clone();
                        drop(state);
                        let _ = executor.execute_command(&["-d", &target_name, "--", "echo", "started"]).await;
                    }

                    app.set_current_message(i18n::tr("dialog.clone_success", &[source_name.clone(), target_name.clone()]).into());
                    refresh_distros_ui(ah_clone.clone(), as_ptr.clone()).await;
                } else {
                    let err = import_result.error.unwrap_or_else(|| i18n::t("dialog.error"));
                    app.set_current_message(i18n::tr("dialog.clone_failed_import", &[err]).into());
                }
                app.set_show_message_dialog(true);
            }
        });
    });

    let ah = app_handle.clone();
    app.on_clone_name_changed(move |new_name| {
        if let Some(app) = ah.upgrade() {
            let current_path = app.get_clone_target_path().to_string();
            if current_path.is_empty() { return; }
            
            let path = std::path::Path::new(&current_path);
            if let Some(parent) = path.parent() {
                let new_path = parent.join(new_name.to_string()).to_string_lossy().to_string();
                app.set_clone_target_path(new_path.into());
            }
        }
    });

    app.on_close_message_dialog(move || {
        // Placeholder
    });
}
