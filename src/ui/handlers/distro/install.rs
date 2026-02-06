use std::sync::Arc;
use tokio::sync::Mutex;
use slint::Model;
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_installable_distros;
use super::sanitize_instance_name;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Folder selection
    let ah = app_handle.clone();
    app.on_select_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_install_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                let path_str = path.display().to_string();
                app.set_new_instance_path(path_str.clone().into());
                
                let p = std::path::Path::new(&path_str);
                if p.exists() {
                    if let Ok(entries) = std::fs::read_dir(p) {
                        if entries.count() > 0 {
                            app.set_path_error(i18n::t("dialog.dir_not_empty").into());
                        } else {
                            app.set_path_error("".into());
                        }
                    }
                } else {
                    app.set_path_error("".into());
                }
            }
        }
    });

    let ah = app_handle.clone();
    app.on_check_install_path(move |path| {
        if let Some(app) = ah.upgrade() {
            if path.is_empty() {
                app.set_path_error("".into());
                return;
            }
            let p = std::path::Path::new(path.as_str());
            if p.exists() && p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    if entries.count() > 0 {
                        app.set_path_error(i18n::t("dialog.dir_not_empty").into());
                        return;
                    }
                }
            }
            app.set_path_error("".into());
        }
    });

    let ah = app_handle.clone();
    app.on_select_install_file(move |source_idx| {
        let mut dialog = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_install_file"));
        
        dialog = match source_idx {
            0 => dialog.add_filter(i18n::t("dialog.archive"), &["tar", "tar.gz", "tar.xz", "wsl"]),
            1 => dialog.add_filter(i18n::t("dialog.vhdx"), &["vhdx"]),
            _ => dialog,
        };

        if let Some(path) = dialog.pick_file() {
            if let Some(app) = ah.upgrade() {
                app.set_install_file_path(path.display().to_string().into());
                
                if let Some(name_os) = path.file_name() {
                    let mut full_stem = name_os.to_string_lossy().to_string();

                    // Optimize: Remove specific suffixes first to get clean name
                    if full_stem.ends_with(".tar.gz") {
                        full_stem.truncate(full_stem.len() - 7);
                    } else if full_stem.ends_with(".tar.xz") {
                        full_stem.truncate(full_stem.len() - 7);
                    } else if full_stem.ends_with(".tar") {
                        full_stem.truncate(full_stem.len() - 4);
                    } else if full_stem.ends_with(".wsl") {
                        full_stem.truncate(full_stem.len() - 4);
                    } else if full_stem.ends_with(".vhdx") {
                        full_stem.truncate(full_stem.len() - 5);
                    }
                    // Remove "rootfs" case-insensitively
                    while let Some(idx) = full_stem.to_lowercase().find("rootfs") {
                        full_stem.replace_range(idx..idx+6, "");
                    }

                    let parts: Vec<&str> = full_stem.split('-').collect();
                    let mut filtered_parts = Vec::new();
                    let stop_keywords = ["wsl", "amd64", "arm64", "x86_64", "with", "docker", "vhdx", "image"];
                    
                    for part in parts {
                        let lower_part = part.to_lowercase();
                        if stop_keywords.iter().any(|&k| lower_part.contains(k)) {
                            break;
                        }
                        if !part.is_empty() && part != "." {
                             filtered_parts.push(part);
                        }
                    }
                    
                    let suggested_name = if filtered_parts.is_empty() {
                        full_stem
                    } else {
                        filtered_parts.join("-")
                    };
                    
                    let mut sanitized = sanitize_instance_name(&suggested_name);
                    
                    while sanitized.ends_with(|c| c == '-' || c == '_' || c == '.') {
                        sanitized.pop();
                    }

                    app.set_new_instance_name(sanitized.clone().into());
                    
                    // Sync path
                    let distro_location = app.get_distro_location().to_string();
                    let new_path = std::path::Path::new(&distro_location)
                        .join(&sanitized)
                        .to_string_lossy()
                        .to_string();
                    app.set_new_instance_path(new_path.into());
                }
            }
        }
    });

    let ah = app_handle.clone();
    app.on_distro_selected(move |val| {
        if let Some(app) = ah.upgrade() {
            let sanitized = sanitize_instance_name(&val);
            app.set_new_instance_name(sanitized.clone().into());
            
            let distro_location = app.get_distro_location().to_string();
            let new_path = std::path::Path::new(&distro_location)
                .join(&sanitized)
                .to_string_lossy()
                .to_string();
            app.set_new_instance_path(new_path.into());
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_source_selected(move |idx| {
        if let Some(app) = ah.upgrade() {
            app.set_name_error("".into());
            app.set_path_error("".into());
            app.set_install_status("".into());
            app.set_terminal_output("".into());
        }

        if idx == 2 {
             let ah_inner = ah.clone();
             let as_ptr = as_ptr.clone();
             let _ = slint::spawn_local(async move {
                 if let Some(app) = ah_inner.upgrade() {
                     if app.get_installable_distro_names().row_count() == 0 {
                        app.set_task_status_text(i18n::t("operation.fetching_distros").into());
                        app.set_task_status_visible(true);

                        refresh_installable_distros(ah_inner.clone(), as_ptr).await;

                        if let Some(app) = ah_inner.upgrade() {
                            app.set_task_status_visible(false);
                        }
                     } else {
                        if let Some(first) = app.get_installable_distro_names().row_data(0) {
                            let first_str: slint::SharedString = first; // Explicit type
                            app.set_selected_install_distro(first_str.clone());
                            app.invoke_distro_selected(first_str);
                        }
                     }
                 }
             });
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_install_distro(move |source_idx, name, friendly_name, install_path, file_path| {
        let name = name.to_string();
        let friendly_name = friendly_name.to_string();
        let install_path = install_path.to_string();
        let file_path = file_path.to_string();
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        
        let _ = slint::spawn_local(async move {
            super::install_logic::perform_install(ah, as_ptr, source_idx, name, friendly_name, install_path, file_path).await;
        });
    });
}
