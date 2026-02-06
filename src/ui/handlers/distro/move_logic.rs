use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;
use std::path::PathBuf;

pub fn run_move_process(
    ah_move: slint::Weak<AppWindow>, 
    as_ptr: Arc<Mutex<AppState>>, 
    source_name: String, 
    target_name: String, 
    target_path: String, 
    version: String
) {
    let _ = slint::spawn_local(async move {
        if let Some(app) = ah_move.upgrade() {
            app.set_is_moving(true);
            app.set_task_status_visible(true);
        }

        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.set_manual_operation(true);

        if let Some(app) = ah_move.upgrade() {
            let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), i18n::t("dialog.processing")]);
            app.set_task_status_text(msg.into());
        }

        let mut size_str = "0 MB".to_string();
        if version == "2" {
             let info_res = crate::wsl::ops::info::get_distro_information(dashboard.executor(), &source_name).await;
             if info_res.success {
                 if let Some(info) = info_res.data {
                     if !info.vhdx_size.is_empty() {
                         size_str = info.vhdx_size;
                     }
                 }
             }
        }
        
        if let Some(app) = ah_move.upgrade() {
            let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), size_str.clone()]);
            app.set_task_status_text(msg.into());
        }

        if version == "2" {
            let _ = dashboard.shutdown_wsl().await;
            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        } else {
            let _ = dashboard.stop_distro(&source_name).await;
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }

        let result = if version == "2" {
            if let Some(app) = ah_move.upgrade() {
                let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), size_str]);
                app.set_task_status_text(msg.into());
            }

            let mut move_res = crate::wsl::models::WslCommandResult::error("".into(), "".into());
            for attempt in 1..=5 {
                if attempt > 1 {
                    info!("WSL 2 Move: Retry attempt {} after delay...", attempt);
                    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
                }
                
                move_res = dashboard.move_distro(&source_name, &target_path).await;
                
                if move_res.success {
                    break;
                }
                
                if move_res.output.contains("ERROR_SHARING_VIOLATION") || move_res.output.contains("0x80070020") {
                    continue;
                }
            }
            move_res
        } else {
            move_wsl1(ah_move.clone(), as_ptr.clone(), &source_name, &target_name, &target_path).await
        };

        if let Some(app) = ah_move.upgrade() {
            app.set_task_status_visible(false);
            app.set_is_moving(false);
            if result.success {
                app.set_current_message(i18n::tr("dialog.move_success", &[source_name, target_path]).into());
            } else {
                let err = result.error.unwrap_or_else(|| i18n::t("dialog.error"));
                if result.output == "BACKUP_SAVED" {
                    app.set_current_message(i18n::tr("dialog.move_failed_backup", &[err.clone()]).into());
                    app.set_current_message_link(i18n::t("distro.explorer").into());
                    let backup_path = std::path::Path::new(&err);
                    if let Some(parent) = backup_path.parent() {
                            app.set_current_message_url(parent.to_string_lossy().to_string().into());
                    } else {
                            app.set_current_message_url(err.into());
                    }
                } else {
                    app.set_current_message(i18n::tr("dialog.move_failed", &[err]).into());
                }
            }
            app.set_show_message_dialog(true);
            dashboard.set_manual_operation(false);
            refresh_distros_ui(ah_move.clone(), as_ptr.clone()).await;
        } else {
            dashboard.set_manual_operation(false);
        }
    });
}

async fn move_wsl1(
    ah: slint::Weak<AppWindow>, 
    as_ptr: Arc<Mutex<AppState>>, 
    source_name: &str, 
    target_name: &str, 
    target_path: &str
) -> crate::wsl::models::WslCommandResult<String> {
    use crate::wsl::models::WslCommandResult;
    
    let (temp_dir, temp_file_str) = {
        let state = as_ptr.lock().await;
        let temp_dir = PathBuf::from(&state.config_manager.get_settings().temp_location);
        let temp_file = temp_dir.join(format!("wsl_move_{}.tar", uuid::Uuid::new_v4()));
        (temp_dir, temp_file.to_string_lossy().to_string())
    };

    let _ = std::fs::create_dir_all(&temp_dir);

    info!("WSL1 Move: Exporting '{}' to '{}'...", source_name, temp_file_str);
    let stop_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
    
    if let Some(app) = ah.upgrade() {
        let msg = i18n::tr("operation.moving_wsl1_step1", &[source_name.to_string(), "0 MB".to_string()]);
        app.set_task_status_text(msg.into());
    }

    super::spawn_file_size_monitor(
        ah.clone(),
        temp_file_str.clone(),
        source_name.to_string(),
        "operation.moving_wsl1_step1".into(),
        stop_signal.clone()
    );

    let export_result = {
        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.export_distro(source_name, &temp_file_str).await
    };

    stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

    if !export_result.success {
        let _ = std::fs::remove_file(&temp_file_str);
        return export_result;
    }

    if let Ok(metadata) = std::fs::metadata(&temp_file_str) {
        if metadata.len() == 0 {
            let _ = std::fs::remove_file(&temp_file_str);
            return WslCommandResult::error("".into(), "Exported file is empty".into());
        }
    } else {
        return WslCommandResult::error("".into(), "Failed to verify exported file".into());
    }

    info!("WSL1 Move: Unregistering '{}'...", source_name);
    let unregister_result = {
        let executor = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        executor.execute_command(&["--unregister", source_name]).await
    };

    if !unregister_result.success {
        let _ = std::fs::remove_file(&temp_file_str);
        return unregister_result;
    }

    info!("WSL1 Move: Importing to '{}' at '{}'...", target_name, target_path);
    if let Some(app) = ah.upgrade() {
        let msg = i18n::tr("operation.moving_wsl1_step2", &[source_name.to_string()]);
        app.set_task_status_text(msg.into());
    }
    let import_result = {
        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.import_distro(target_name, target_path, &temp_file_str).await
    };

    if !import_result.success {
        return WslCommandResult {
            success: false,
            output: "BACKUP_SAVED".into(),
            error: Some(temp_file_str),
            data: None,
        };
    }

    let verify_result = {
        let executor = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        executor.execute_command(&["-l", "-v"]).await
    };

    if verify_result.success && verify_result.output.contains(target_name) {
        info!("WSL1 Move: Success, cleaning up temp file");
        let _ = std::fs::remove_file(&temp_file_str);
        WslCommandResult::success("Move successful".into(), None)
    } else {
        warn!("WSL1 Move: Import appeared successful but distro not found in list");
        WslCommandResult {
            success: false,
            output: "BACKUP_SAVED".into(),
            error: Some(temp_file_str),
            data: None,
        }
    }
}
