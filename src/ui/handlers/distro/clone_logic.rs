use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info};
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;
use std::io::{Read, Write};

pub async fn perform_clone(
    ah_clone: slint::Weak<AppWindow>,
    as_ptr: Arc<Mutex<AppState>>,
    source_name: String,
    target_name: String,
    target_path: String,
) {
    // Synchronously upgrade and set initial status to prevent UI lag
    if let Some(app) = ah_clone.upgrade() {
        app.set_is_cloning(true);
        app.set_task_status_visible(true);
        let initial_msg = i18n::tr("operation.cloning_step1_wsl2", &[source_name.clone(), "0 MB".to_string()]);
        app.set_task_status_text(initial_msg.into());
    }

    // 1. Get executor and dashboard info without holding the lock during await
    let (executor, dashboard) = {
        let state = as_ptr.lock().await;
        (state.wsl_dashboard.executor().clone(), state.wsl_dashboard.clone())
    };

    let distro_info = crate::wsl::ops::info::get_distro_information(&executor, &source_name).await;
    let old_install_location = executor.get_distro_install_location(&source_name).await.data;

    let is_wsl2 = distro_info.success && distro_info.data.as_ref().map_or(false, |info| info.wsl_version == "WSL2");
    let vhdx_path = distro_info.data.as_ref().map(|info| info.vhdx_path.clone()).unwrap_or_default();

    // Lock manual operation to prevent status bar from being hidden by auto-refresh
    dashboard.increment_manual_operation();
    
    // Ensure we decrement when finished
    let dashboard_clone = dashboard.clone();
    let _manual_op_guard = scopeguard::guard((), |_| {
        dashboard_clone.decrement_manual_operation();
    });

    if is_wsl2 && !vhdx_path.is_empty() {
        // Acquire global lock for heavy operations to prevent conflicts
        let _heavy_lock = dashboard.heavy_op_lock().lock().await;

        // Optimized WSL2 Clone: Manual VHDX Copy to TMP + Direct Import

        // Step 1: Terminate source distro to release VHDX lock
        info!("WSL2 Clone: Terminating '{}' to release VHDX lock...", source_name);
        let _ = dashboard.executor().execute_command(&["--terminate", &source_name]).await;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // Step 2: Manual Copy to Temporary File
        let (_, temp_vhdx_file) = super::resolve_temp_path(as_ptr.clone(), &source_name, "clone_tmp", "vhdx").await;
        info!("WSL2 Clone: Copying VHDX from '{}' to temp '{}'...", vhdx_path, temp_vhdx_file);
        
        let copy_result: Result<u64, String> = tokio::task::spawn_blocking({
            let source_path = vhdx_path.clone();
            let target_path = temp_vhdx_file.clone();
            let ah_clone = ah_clone.clone();
            let source_name = source_name.clone();
            move || {
                let mut source_file = std::fs::File::open(&source_path).map_err(|e| format!("Open source failed: {}", e))?;
                let mut target_file = std::fs::File::create(&target_path).map_err(|e| format!("Create target failed: {}", e))?;
                let total_size = source_file.metadata().map_err(|e| e.to_string())?.len();
                let mut copied = 0u64;
                let mut buffer = vec![0u8; 8 * 1024 * 1024]; // 8MB buffer on heap
                let mut last_update = 0u64;

                while copied < total_size {
                    let bytes_read = source_file.read(&mut buffer).map_err(|e| e.to_string())?;
                    if bytes_read == 0 { break; }
                    target_file.write_all(&buffer[..bytes_read]).map_err(|e| e.to_string())?;
                    copied += bytes_read as u64;

                    // Update UI every ~64MB
                    if copied - last_update > 64 * 1024 * 1024 || copied == total_size {
                        last_update = copied;
                        let ah_inner = ah_clone.clone();
                        let source_name_inner = source_name.clone();
                        let mb = copied / 1024 / 1024;
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = ah_inner.upgrade() {
                                let msg = i18n::tr("operation.cloning_step1_wsl2", &[source_name_inner, format!("{} MB", mb)]);
                                app.set_task_status_text(msg.into());
                                app.set_task_status_visible(true);
                            }
                        });
                    }
                }
                Ok(total_size)
            }
        }).await.map_err(|e| e.to_string()).and_then(|r| r);

        let expected_total_size = match copy_result {
            Ok(size) => size,
            Err(e) => {
                if let Some(app) = ah_clone.upgrade() {
                    app.set_task_status_visible(false);
                    app.set_is_cloning(false);
                    app.set_current_message(i18n::tr("dialog.clone_failed_export", &[e]).into());
                    app.set_show_message_dialog(true);
                }
                let _ = std::fs::remove_file(&temp_vhdx_file);
                return;
            }
        };

        // Step 3: Direct Import using --vhd from temp file
        info!("WSL2 Clone: Finalizing import for '{}' to '{}'...", target_name, target_path);
        let ah_inner = ah_clone.clone();
        let source_name_inner = source_name.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_inner.upgrade() {
                let msg = i18n::tr("operation.cloning_step2_wsl2", &[source_name_inner]);
                app.set_task_status_text(msg.into());
                app.set_task_status_visible(true);
            }
        });
        
        let _ = std::fs::create_dir_all(&target_path); // Ensure target dir exists
        let import_result = dashboard.executor().execute_command(&[
            "--import", 
            &target_name, 
            &target_path, 
            &temp_vhdx_file, 
            "--version", "2", 
            "--vhd"
        ]).await;

        // Cleanup temp file
        let _ = std::fs::remove_file(&temp_vhdx_file);

        if import_result.success {
            // Monitor VHDX growth/stability after "success" return
            let target_vhdx = std::path::Path::new(&target_path).join("ext4.vhdx");
            let mut last_size = 0u64;
            let mut stable_count = 0;
            let start_time = std::time::Instant::now();
            
            info!("WSL2 Clone: Import command returned, monitoring growth for '{}'...", target_vhdx.display());
            
            while start_time.elapsed().as_secs() < 120 {
                let current_size = std::fs::metadata(&target_vhdx).map(|m| m.len()).unwrap_or(0);
                
                let ah_inner = ah_clone.clone();
                let source_name_inner = source_name.clone();
                let mb = current_size / 1024 / 1024;
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah_inner.upgrade() {
                         let msg = i18n::tr("operation.cloning_step2_wsl2", &[source_name_inner]);
                         app.set_task_status_text(format!("{}: {} MB", msg, mb).into());
                         app.set_task_status_visible(true);
                    }
                });

                if current_size > 0 && current_size == last_size {
                    stable_count += 1;
                } else {
                    stable_count = 0;
                }
                
                last_size = current_size;
                
                // If size is 98% of expected total_size, or stable for 6 checks (3s)
                if (expected_total_size > 0 && current_size >= (expected_total_size * 98 / 100)) || stable_count >= 6 {
                    info!("WSL2 Clone: VHDX growth stabilized at {} bytes (expected {}).", current_size, expected_total_size);
                    break;
                }
                
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }

            // 1. Set Success Message
            let ah_inner = ah_clone.clone();
            let source = source_name.clone();
            let target = target_name.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_current_message(i18n::tr("dialog.clone_success", &[source, target]).into());
                }
            });

            // shortcut.ico
            if let Some(src_path) = old_install_location.clone() {
                let ico_src = std::path::Path::new(&src_path).join("shortcut.ico");
                if ico_src.exists() {
                    let ico_dst = std::path::Path::new(&target_path).join("shortcut.ico");
                    info!("Cloning shortcut.ico from {:?} to {:?}", ico_src, ico_dst);
                    let _ = std::fs::copy(ico_src, ico_dst);
                }
            }

            // 2. Refresh UI (Async)
            refresh_distros_ui(ah_clone.clone(), as_ptr.clone()).await;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            // 3. Cleanup UI
            let ah_inner = ah_clone.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_task_status_visible(false);
                    app.set_is_cloning(false);
                    app.set_show_message_dialog(true);
                }
            });
        } else {
            let ah_inner = ah_clone.clone();
            let err = import_result.error.unwrap_or_else(|| i18n::t("dialog.error"));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_current_message(i18n::tr("dialog.clone_failed_import", &[err]).into());
                    app.set_task_status_visible(false);
                    app.set_is_cloning(false);
                    app.set_show_message_dialog(true);
                }
            });
            let _ = std::fs::remove_dir_all(&target_path); // Cleanup target dir on fail
        }
    } else {
        // Fallback or WSL1: Classic Export/Import flow
        let ah_inner = ah_clone.clone();
        let source_name_inner = source_name.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_inner.upgrade() {
                let initial_msg = i18n::tr("operation.cloning_step1", &[source_name_inner, "0 MB".to_string()]);
                app.set_task_status_text(initial_msg.into());
                app.set_task_status_visible(true);
            }
        });

        let (temp_dir, temp_file_str) = super::resolve_temp_path(as_ptr.clone(), &source_name, "wsl_clone", "tar").await;

        let _ = std::fs::create_dir_all(&temp_dir);

        // Step 1: Export
        let stop_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
        super::spawn_file_size_monitor(
            ah_clone.clone(),
            temp_file_str.clone(),
            source_name.clone(),
            "operation.cloning_step1".into(),
            stop_signal.clone()
        );

        tokio::task::yield_now().await;
        let export_result = {
            let dashboard = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.clone()
            };
            info!("WSL1/Fallback Clone: exporting source '{}' to temp file '{}'...", source_name, temp_file_str);
            dashboard.export_distro(&source_name, &temp_file_str).await
        };

        stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

        if !export_result.success {
            let ah_inner = ah_clone.clone();
            let err = export_result.error.unwrap_or_else(|| i18n::t("dialog.export_failed").replace("{0}", ""));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_task_status_visible(false);
                    app.set_is_cloning(false);
                    app.set_current_message(i18n::tr("dialog.clone_failed_export", &[err]).into());
                    app.set_show_message_dialog(true);
                }
            });
            let _ = std::fs::remove_file(&temp_file_str);
            return;
        }

        // Step 2: Import
        let ah_inner = ah_clone.clone();
        let source_name_inner = source_name.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_inner.upgrade() {
                let msg = i18n::tr("operation.cloning_step2", &[source_name_inner]);
                app.set_task_status_text(msg.into());
                app.set_task_status_visible(true);
            }
        });

        tokio::task::yield_now().await;
        let import_result = {
            let dashboard = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.clone()
            };
            info!("WSL1/Fallback Clone: importing as '{}' to '{}'...", target_name, target_path);
            dashboard.import_distro(&target_name, &target_path, &temp_file_str).await
        };

        let _ = std::fs::remove_file(&temp_file_str);

        if import_result.success {
            // Success Path
            let ah_inner = ah_clone.clone();
            let source = source_name.clone();
            let target = target_name.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_current_message(i18n::tr("dialog.clone_success", &[source, target]).into());
                }
            });

            // shortcut.ico
            if let Some(src_path) = old_install_location {
                let ico_src = std::path::Path::new(&src_path).join("shortcut.ico");
                if ico_src.exists() {
                    let ico_dst = std::path::Path::new(&target_path).join("shortcut.ico");
                    info!("Cloning shortcut.ico from {:?} to {:?}", ico_src, ico_dst);
                    let _ = std::fs::copy(ico_src, ico_dst);
                }
            }

            refresh_distros_ui(ah_clone.clone(), as_ptr.clone()).await;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        } else {
            // Failure Path
            let ah_inner = ah_clone.clone();
            let err = import_result.error.unwrap_or_else(|| i18n::t("dialog.error"));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    app.set_current_message(i18n::tr("dialog.clone_failed_import", &[err]).into());
                }
            });
            let _ = std::fs::remove_dir_all(&target_path);
        }

        // Shared Cleanup
        let ah_inner = ah_clone.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah_inner.upgrade() {
                app.set_task_status_visible(false);
                app.set_is_cloning(false);
                app.set_show_message_dialog(true);
            }
        });
    }
}
