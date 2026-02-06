use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use slint::{ComponentHandle, Model};
use crate::{AppWindow, AppState, i18n};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let _as_ptr_open = app_state.clone();
    app.on_open_move_dialog(move |name| {
        info!("Operation: Open move dialog - {}", name);
        if let Some(app) = ah.upgrade() {
            if app.get_is_installing() || app.get_is_exporting() || app.get_is_cloning() || app.get_is_moving() {
                app.set_current_message(i18n::t("dialog.operation_in_progress").into());
                app.set_show_message_dialog(true);
                return;
            }

            let name_str = name.to_string();
            let distro_location = app.get_distro_location().to_string();
            let target_path = std::path::Path::new(&distro_location)
                .join(&name_str)
                .to_string_lossy()
                .to_string();

            app.set_move_source_name(name_str.clone().into());
            // Note: move_target_name is still used internally but the input is removed from UI
            app.set_move_target_name(name_str.clone().into());
            app.set_move_target_path(target_path.into());
            app.set_move_original_path("".into());
            app.set_move_error("".into());
            app.set_show_move_dialog(true);
        }
    });

    let ah = app_handle.clone();
    app.on_cancel_move_confirm(move || {
        if let Some(app) = ah.upgrade() {
            app.set_show_move_confirm(false);
            info!("Operation: Move confirm cancelled");
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_move_action(move || {
        let ah = match ah.upgrade() {
            Some(a) => a,
            None => return,
        };
        ah.set_show_move_confirm(false);

        let source_name = ah.get_move_source_name().to_string();
        let target_name = ah.get_move_target_name().to_string();
        let target_path = ah.get_move_target_path().to_string();

        info!("Operation: Move confirmed - Starting WSL2 Move for {}", source_name);
        
        run_move_process(
            ah.as_weak(), 
            as_ptr.clone(), 
            source_name, 
            target_name, 
            target_path, 
            "2".to_string()
        );
    });

    let ah = app_handle.clone();
    app.on_select_move_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_move_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                app.set_move_target_path(path.to_string_lossy().to_string().into());
            }
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_move(move |source_name, _target_name, target_path| {
        info!("Operation: Confirm move - Source: {}, Target: {}, Path: {}", source_name, _target_name, target_path);
        let ah = match ah.upgrade() {
            Some(a) => a,
            None => return,
        };

        if ah.get_is_installing() || ah.get_is_exporting() || ah.get_is_cloning() || ah.get_is_moving() {
            return;
        }

        // 1. Sync Validations
        let p = std::path::Path::new(target_path.as_str());
        if p.exists() {
            if p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    if entries.count() > 0 {
                        ah.set_move_error(i18n::t("dialog.dir_not_empty").into());
                        return;
                    }
                }
            } else {
                ah.set_move_error(i18n::t("dialog.path_is_not_dir").into());
                return;
            }
        } else {
            if let Err(e) = std::fs::create_dir_all(p) {
                ah.set_move_error(i18n::tr("dialog.mkdir_failed", &[e.to_string()]).into());
                return;
            }
        }

        // Get distro version (Sync)
        let mut version = "2".to_string();
        let distros = ah.get_distros();
        for i in 0..distros.row_count() {
            if let Some(d) = distros.row_data(i) {
                if d.name == source_name {
                    version = d.version.to_string();
                    break;
                }
            }
        }

        ah.set_move_error("".into());
        // Do not close dialog yet, we need to check current path asynchronously
        
        let ah_move = ah.as_weak(); // Still used if we pass to run_move_process
        let as_ptr = as_ptr.clone();
        let source_name = source_name.to_string();
        let target_path = target_path.to_string();
        let target_name = _target_name.to_string();

        if version == "2" {
            // Check if there are other running distros
            let distros = ah.get_distros();
            let mut running_names = Vec::new();
            
            for i in 0..distros.row_count() {
                if let Some(d) = distros.row_data(i) {
                    if d.status.as_str() == "Running" && d.name != source_name {
                        running_names.push(d.name.to_string());
                    }
                }
            }
            
            // Show confirmation dialog for WSL2 move
            let warning_msg = if running_names.is_empty() {
                i18n::t("dialog.move_wsl2_shutdown_warning_no_running")
            } else {
                let other_distros = running_names.join(", ");
                i18n::tr("dialog.move_wsl2_shutdown_warning", &[other_distros])
            };
            
            ah.set_move_confirm_message(warning_msg.into());
            ah.set_show_move_confirm(true);
            ah.set_show_move_dialog(false); 
        } else {
             ah.set_show_move_dialog(false);
             run_move_process(
                ah_move,
                as_ptr,
                source_name,
                target_name,
                target_path,
                version
             );
        }
    });
}

fn run_move_process(
    ah_move: slint::Weak<AppWindow>, 
    as_ptr: Arc<Mutex<AppState>>, 
    source_name: String, 
    target_name: String, 
    target_path: String, 
    version: String
) {
    super::move_logic::run_move_process(ah_move, as_ptr, source_name, target_name, target_path, version);
}
