// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

mod types;
mod probe;
mod download;

pub use types::{DownloadProgress, DownloadProgressOwned};
pub use probe::select_fastest_mirrors;
pub use download::download_with_fallback;

use types::replace_last_line;
use std::sync::Arc;
use tracing::{info, error, debug};
use crate::{AppWindow, i18n};

pub async fn install_from_mirror(
    ah: slint::Weak<AppWindow>,
    executor: Arc<crate::wsl::executor::WslCommandExecutor>,
    config_manager: Arc<crate::config::ConfigManager>,
    internal_id: String,
    final_name: String,
    install_path: String,
) -> Result<String, (String, String)> {
    let mut terminal_buffer = String::new();

    let distro_info = {
        let cache = crate::ui::data::MIRROR_LIST_CACHE.lock().unwrap();
        cache.iter().find(|d| format!("{} {}", d.name, d.version) == internal_id).cloned()
    };

    debug!("install_from_mirror: looking up internal_id='{}' in MIRROR_LIST_CACHE (entries: {})",
        internal_id,
        crate::ui::data::MIRROR_LIST_CACHE.lock().unwrap().len()
    );

    let distro_info = match distro_info {
        Some(d) => {
            info!("install_from_mirror: found distro '{}' with {} sources", d.name, d.sources.len());
            d
        }
        None => {
            error!("install_from_mirror: distro '{}' not found in cache", internal_id);
            return Err((i18n::t("install.unknown_distro"), terminal_buffer));
        }
    };

    let ah_cb = ah.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah_cb.upgrade() {
            app.set_install_status(i18n::t("install.status_testing_mirrors").into());
        }
    });

    terminal_buffer.push_str(&format!("{}\n", i18n::t("install.probing_mirrors")));
    let ah_cb = ah.clone();
    let tb = terminal_buffer.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah_cb.upgrade() {
            app.set_terminal_output(tb.into());
        }
    });

    let fastest_mirrors = select_fastest_mirrors(&distro_info.sources).await;
    if fastest_mirrors.is_empty() {
        return Err((i18n::tr("install.all_mirrors_failed", &["No mirrors available".to_string()]), terminal_buffer));
    }

    let fastest_mirror_name = &fastest_mirrors[0].mirror;
    replace_last_line(&mut terminal_buffer, &i18n::tr("install.mirror_selected", &[fastest_mirror_name.clone()]));

    let ah_cb = ah.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah_cb.upgrade() {
            app.set_install_status(i18n::t("install.status_downloading").into());
        }
    });

    let temp_location = config_manager.get_settings().temp_location.clone();
    let temp_dir = std::path::PathBuf::from(temp_location.clone());
    let _ = tokio::task::spawn_blocking(move || std::fs::create_dir_all(&temp_dir)).await;
    let temp_file = std::path::PathBuf::from(temp_location).join(format!("mirror_download_{}.tmp", uuid::Uuid::new_v4()));

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let ah_ui = ah.clone();
    let initial_tb = terminal_buffer.clone();

    let ui_task = tokio::spawn(async move {
        let mut buffer = initial_tb;
        let mut current_mirror = String::new();
        while let Some(msg) = rx.recv().await {
            match msg {
                DownloadProgressOwned::TryingMirror { mirror, .. } => {
                    current_mirror = mirror.clone();
                    let text = i18n::tr("install.trying_mirror", &[mirror]);
                    let t = buffer.trim_end_matches('\n');
                    let last_is_step2 = match t.rfind('\n') {
                        Some(i) => t[i + 1..].starts_with("[2/4]"),
                        None => t.starts_with("[2/4]"),
                    };
                    if last_is_step2 {
                        replace_last_line(&mut buffer, &text);
                    } else {
                        buffer.push_str(&text);
                        buffer.push('\n');
                    }
                }
                DownloadProgressOwned::Downloading { percent, speed } => {
                    let progress = format!("{:.1}% ({})", percent, speed);
                    let text = i18n::tr("install.downloading", &[current_mirror.clone(), progress]);
                    replace_last_line(&mut buffer, &text);
                }
                DownloadProgressOwned::MirrorFailed { mirror, error } => {
                    let short_error = if error.len() > 60 {
                        format!("{}...", &error[..60])
                    } else {
                        error.clone()
                    };
                    let text = i18n::tr("install.mirror_failed", &[mirror, short_error]);
                    replace_last_line(&mut buffer, &text);
                }
                DownloadProgressOwned::MirrorFileInvalid { mirror } => {
                    let text = i18n::tr("install.file_invalid", &[mirror]);
                    replace_last_line(&mut buffer, &text);
                }
            }

            let ah_cb = ah_ui.clone();
            let text_to_set = buffer.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_cb.upgrade() {
                    app.set_terminal_output(text_to_set.into());
                }
            });
        }
        buffer
    });

    let download_res = download_with_fallback(&fastest_mirrors, &temp_file, |progress| {
        // Send and drop Owned strings
        let owned_progress = match progress {
            DownloadProgress::TryingMirror { mirror, url } => DownloadProgressOwned::TryingMirror { mirror: mirror.to_string(), url: url.to_string() },
            DownloadProgress::Downloading { percent, speed } => DownloadProgressOwned::Downloading { percent, speed: speed.to_string() },
            DownloadProgress::MirrorFailed { mirror, error } => DownloadProgressOwned::MirrorFailed { mirror: mirror.to_string(), error: error.to_string() },
            DownloadProgress::MirrorFileInvalid { mirror } => DownloadProgressOwned::MirrorFileInvalid { mirror: mirror.to_string() },
        };
        let _ = tx.try_send(owned_progress);
    }).await;

    drop(tx);
    terminal_buffer = ui_task.await.unwrap_or(terminal_buffer);

    match download_res {
        Ok(downloaded_file) => {
            if !terminal_buffer.ends_with('\n') { terminal_buffer.push('\n'); }
            terminal_buffer.push_str(&format!("{}\n", i18n::t("install.download_complete")));
            
            let ah_cb = ah.clone();
            let tb = terminal_buffer.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_cb.upgrade() {
                    app.set_install_status(i18n::t("install.importing").into());
                    app.set_terminal_output(tb.into());
                }
            });

            let mut target_path = install_path.clone();
            if target_path.is_empty() {
                let distro_location = config_manager.get_settings().distro_location.clone();
                let base = std::path::PathBuf::from(&distro_location);
                target_path = base.join(&final_name).to_string_lossy().to_string();
            }
            
            let tp_clone = target_path.clone();
            if let Err(e) = tokio::task::spawn_blocking(move || std::fs::create_dir_all(&tp_clone)).await.unwrap() {
                let err = format!("Failed to create directory: {}", e);
                return Err((err, terminal_buffer));
            }

            let downloaded_file_str = downloaded_file.to_string_lossy().to_string();
            let import_args = vec!["--import", &final_name, &target_path, &downloaded_file_str];
            let display_file = if downloaded_file_str.len() > 30 {
                format!("{}...", &downloaded_file_str[..30])
            } else {
                downloaded_file_str.clone()
            };
            let cmd_display = format!("wsl --import {} {} {}", final_name, target_path, display_file);
            terminal_buffer.push_str(&i18n::tr("install.mirror_step_import", &[cmd_display]));
            terminal_buffer.push('\n');
            
            let (tx_out, mut rx_out) = tokio::sync::mpsc::channel::<String>(100);
            let ah_ui_2 = ah.clone();
            let initial_tb_2 = terminal_buffer.clone();
            let ui_task_2 = tokio::spawn(async move {
                let mut buffer = initial_tb_2;
                while let Some(msg) = rx_out.recv().await {
                    // Filter out the annoying sparse VHD warnings and success messages from WSL command stdout/stderr
                    let filtered: String = msg
                        .lines()
                        .filter(|line| {
                            let l = line.trim().to_lowercase();
                            !l.contains("sparse vhd support is currently disabled")
                                && !l.contains("to force a distribution to use a sparse vhd")
                                && !l.contains("wsl.exe --manage")
                                && !l.contains("allow-unsafe")
                                && !l.contains("the operation completed successfully.")
                        })
                        .map(|line| format!("{}\n", line))
                        .collect();

                    if !filtered.trim().is_empty() {
                        buffer.push_str(&filtered);
                        let ah_cb = ah_ui_2.clone();
                        let tb = buffer.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_cb.upgrade() {
                                app.set_terminal_output(tb.into());
                            }
                        });
                    }
                }
                buffer
            });

            let tx_callback = tx_out.clone();
            let result = executor.execute_command_streaming(&import_args, move |text| {
                let _ = tx_callback.try_send(text);
            }).await;

            drop(tx_out);
            terminal_buffer = ui_task_2.await.unwrap_or(terminal_buffer);
            let _ = tokio::fs::remove_file(&downloaded_file).await;

            if result.success {
                info!("mirror_install: wsl --import succeeded for '{}'", final_name);
                if !terminal_buffer.ends_with('\n') { terminal_buffer.push('\n'); }
                terminal_buffer.push_str(&format!("{}\n", i18n::tr("install.mirror_step_done", &[final_name.clone()])));
                Ok(terminal_buffer)
            } else {
                error!("mirror_install: wsl --import failed for '{}': {:?}", final_name, result.error);
                if !result.output.trim().is_empty() {
                    terminal_buffer.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                }
                Err((result.error.unwrap_or_else(|| i18n::t("install.import_failed")), terminal_buffer))
            }
        }
        Err(e) => {
            Err((e.to_string(), terminal_buffer))
        }
    }
}
