// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use rand::{Rng, distr::Alphanumeric};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::{AppWindow, AppState, i18n};
use uuid;

pub mod lifecycle;
pub mod manage;
pub mod export;
pub mod clone;
pub mod clone_logic;
pub mod install;
pub mod install_logic;
pub mod move_distro;
pub mod move_logic;
pub mod settings_logic;
pub mod config_logic;
pub mod compress;
pub mod mirror_install;

pub fn sanitize_instance_name(name: &str) -> String {
    let mut sanitized = String::new();
    let mut last_was_hyphen = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '.' {
            sanitized.push(c);
            last_was_hyphen = false;
        } else if c == '-' || c == '_' || c.is_whitespace() {
            if !last_was_hyphen && !sanitized.is_empty() {
                sanitized.push('-');
                last_was_hyphen = true;
            }
        }
    }
    
    while sanitized.ends_with('-') || sanitized.ends_with('.') {
        sanitized.pop();
    }
    
    if sanitized.len() > 25 {
        sanitized.truncate(25);
        while sanitized.ends_with('-') || sanitized.ends_with('.') {
            sanitized.pop();
        }
    }
    sanitized
}

pub fn generate_random_suffix(name: &str) -> String {
    let random_suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    format!("{}_{}", name, random_suffix)
}

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    lifecycle::setup(app, app_handle.clone(), app_state.clone());
    manage::setup(app, app_handle.clone(), app_state.clone());
    export::setup(app, app_handle.clone(), app_state.clone());
    clone::setup(app, app_handle.clone(), app_state.clone());
    install::setup(app, app_handle.clone(), app_state.clone());
    move_distro::setup(app, app_handle.clone(), app_state.clone());
    compress::setup(app, app_handle.clone(), app_state.clone());
}

pub fn spawn_file_size_monitor(
    ah: slint::Weak<AppWindow>,
    file_path: String,
    distro_name: String,
    i18n_key: String,
    stop_signal: Arc<AtomicBool>,
) {
    tokio::spawn(async move {
        while !stop_signal.load(Ordering::Relaxed) {
            let size_mb = if let Ok(metadata) = std::fs::metadata(&file_path) {
                metadata.len() / (1024 * 1024)
            } else {
                0
            };
            
            let ah_inner = ah.clone();
            let distro_name_inner = distro_name.clone();
            let i18n_key_inner = i18n_key.clone();
            
            let size_str = format!("{} MB", size_mb);
            
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    let msg = i18n::tr(&i18n_key_inner, &[distro_name_inner, size_str]);
                    app.set_task_status_text(msg.into());
                }
            });
            
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
}

pub async fn resolve_temp_path(
    as_ptr: Arc<Mutex<AppState>>,
    distro_name: &str,
    prefix: &str,
    extension: &str,
) -> (std::path::PathBuf, String) {
    let (config_temp, distro_loc) = {
        let state = as_ptr.lock().await;
        let settings = state.config_manager.get_settings();
        (settings.temp_location.clone(), settings.distro_location.clone())
    };

    // 1. Get distro size
    let mut vhdx_bytes: u64 = 0;
    {
        let executor = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        let info_res = crate::wsl::ops::info::get_distro_information(&executor, distro_name).await;
        if info_res.success {
            if let Some(info) = info_res.data {
                // Try to parse vhdx_size first (WSL2)
                let size_str = if !info.vhdx_size.is_empty() {
                    info.vhdx_size
                } else {
                    info.actual_used // Fallback to actual_used (WSL1)
                };

                if !size_str.is_empty() && !size_str.contains("Unknown") {
                    let parts: Vec<&str> = size_str.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(val) = parts[0].parse::<f64>() {
                            let unit = parts[1].to_uppercase();
                            vhdx_bytes = if unit.contains("GB") {
                                (val * 1024.0 * 1024.0 * 1024.0) as u64
                            } else if unit.contains("MB") {
                                (val * 1024.0 * 1024.0) as u64
                            } else {
                                (val * 1024.0) as u64
                            };
                        }
                    }
                }
            }
        }
    }

    // 2. Check C: drive space
    let c_free_bytes = crate::utils::system::get_c_drive_free_space();
    
    // Safety margin (e.g., 1GB) since the export might be slightly larger or we want to leave some space
    let required_bytes = vhdx_bytes + 1024 * 1024 * 1024;

    let temp_dir = if c_free_bytes < required_bytes && !distro_loc.is_empty() {
        // Use distro_location/temp
        let path = std::path::PathBuf::from(distro_loc).join("temp");
        let _ = std::fs::create_dir_all(&path);
        path
    } else {
        std::path::PathBuf::from(config_temp)
    };

    let temp_file = temp_dir.join(format!("{}_{}.{}", prefix, uuid::Uuid::new_v4(), extension));
    (temp_dir, temp_file.to_string_lossy().to_string())
}
