use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::info;
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;

pub async fn perform_clone(
    ah_clone: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    source_name: String,
    target_name: String,
    target_path: String,
) {
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
            app.set_current_message(i18n::tr("dialog.clone_success", &[source_name.clone(), target_name.clone()]).into());
            refresh_distros_ui(ah_clone.clone(), as_ptr.clone()).await;
        } else {
            let err = import_result.error.unwrap_or_else(|| i18n::t("dialog.error"));
            app.set_current_message(i18n::tr("dialog.clone_failed_import", &[err]).into());
        }
        app.set_show_message_dialog(true);
    }
}
