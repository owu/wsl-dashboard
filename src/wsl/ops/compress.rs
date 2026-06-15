// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::path::Path;
use tokio::process::Command;
use tracing::{info, warn, error};
use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::WslCommandResult;

// Check if enough free disk space is available for compression (at least VHDX size + 2GB buffer)
pub fn check_disk_space(vhdx_path: &str) -> WslCommandResult<bool> {
    let clean_path = if vhdx_path.starts_with(r"\\?\") {
        &vhdx_path[4..]
    } else {
        vhdx_path
    };
    if let Some(parent) = Path::new(clean_path).parent() {
        let drive = parent.to_string_lossy();
        if drive.len() >= 3 {
            let root = &drive[..3];
            let free_bytes = crate::utils::system::get_disk_free_space(root);
            
            if let Ok(metadata) = fs::metadata(vhdx_path) {
                let vhdx_bytes = metadata.len();
                // Compression requires: 1x VHDX size (Export TAR) + 2GB buffer
                let required = vhdx_bytes + (2 * 1024 * 1024 * 1024);
                
                if free_bytes < required {
                    let free_gb = free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let req_gb = required as f64 / (1024.0 * 1024.0 * 1024.0);
                    return WslCommandResult::error(String::new(), format!(
                        "Insufficient disk space. Free: {:.2} GB, Required: {:.2} GB", 
                        free_gb, req_gb
                    ));
                }
            }
        }
    }
    WslCommandResult::success(String::new(), Some(true))
}

// Execute fstrim inside Linux, handling missing tool scenarios
pub async fn fstrim_rootfs(executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    info!("Executing fstrim on distro: {}", distro_name);
    
    // Wrap with sh -c to avoid WSL low-level Relay Error logs when the tool is missing and exec fails directly
    let cmd = "fstrim /";
    let result = executor.execute_command(&["-d", distro_name, "-u", "root", "-e", "sh", "-c", cmd]).await;
    
    if result.success {
        info!("fstrim completed successfully for distro: {}", distro_name);
        return result;
    } else {
        let err_msg = result.error.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
        // Detect typical "command not found" error patterns
        if err_msg.contains("not found") || err_msg.contains("no such file") {
            warn!("fstrim not found in {}. Please install 'util-linux' (e.g., sudo apt install util-linux) inside the distro to improve compression efficiency.", distro_name);
            return WslCommandResult::success("fstrim skipped (not installed)".to_string(), None);
        } else {
            warn!("fstrim failed: {:?}", result.error);
            return result;
        }
    }
}

// Clean up temporary files and package caches inside Linux
pub async fn cleanup_temp_files(executor: &WslCommandExecutor, distro_name: &str, script_url: &str, local_script_path: &str) -> WslCommandResult<String> {
    if !local_script_path.is_empty() {
        info!("Executing local cleanup script from: {}", local_script_path);
        let wsl_cmd = format!("sh \"$(wslpath -u '{}')\"", local_script_path.replace("'", "'\\''"));
        let result = executor.execute_command(&["-d", distro_name, "-u", "root", "-e", "sh", "-c", &wsl_cmd]).await;
        
        if !result.success {
            warn!("Local cleanup script failed: {:?}", result.error);
        }
        return result;
    }

    if script_url.is_empty() {
        info!("Compress script URL is empty, skipping cleanup for distro: {}", distro_name);
        return WslCommandResult::success("cleanup skipped (no script URL)".to_string(), None);
    }

    info!("Cleaning up temp files in distro: {} (script URL: {})", distro_name, script_url);
    
    // Try to download and run the script; if neither curl nor wget exists, mark via echo and exit
    let cmd = format!(
        "if command -v curl >/dev/null 2>&1; then curl -sL {0} | sudo sh; \
         elif command -v wget >/dev/null 2>&1; then wget -qO- {0} | sudo sh; \
         else echo 'DOWNLOAD_TOOLS_MISSING'; exit 127; fi", script_url);
         
    let result = executor.execute_command(&["-d", distro_name, "-u", "root", "-e", "sh", "-c", &cmd]).await;
    
    if !result.success {
        let out_msg = result.output.to_lowercase();
        let err_msg = result.error.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
        
        if out_msg.contains("download_tools_missing") || err_msg.contains("not found") {
            warn!("Cleanup failed: No download tools (curl or wget) found in {}. Please install them (e.g., sudo apt install curl) to enable the cleanup script.", distro_name);
        } else {
            warn!("Remote cleanup script failed: {:?}", result.error);
        }
    }
    
    result
}

// Get the default user of the distro (used to restore after Import)
pub async fn get_default_user(executor: &WslCommandExecutor, distro_name: &str) -> String {
    let res = executor.execute_command(&["-d", distro_name, "-e", "whoami"]).await;
    if res.success {
        res.output.trim().to_string()
    } else {
        "root".to_string()
    }
}

// Restore the default user of the distro (by modifying /etc/wsl.conf)
pub async fn restore_default_user(executor: &WslCommandExecutor, distro_name: &str, user: &str) {
    if user == "root" || user.is_empty() { return; }
    
    info!("Restoring default user '{}' for distro: {}", user, distro_name);
    let conf_cmd = format!(
        "if [ ! -f /etc/wsl.conf ] || ! grep -q '\\[user\\]' /etc/wsl.conf; then \
            echo -e '\\n[user]\\ndefault={}' >> /etc/wsl.conf; \
         else \
            sed -i 's/^default=.*/default={}/' /etc/wsl.conf; \
         fi", user, user);
         
    let _ = executor.execute_command(&["-d", distro_name, "-u", "root", "-e", "sh", "-c", &conf_cmd]).await;
}

// Compress VHDX using PowerShell Optimize-VHD (requires Hyper-V module, auto-elevates)
// Uses inline command mode to avoid creating temporary .ps1 script files
pub async fn optimize_vhd_elevated(vhdx_path: &str) -> WslCommandResult<String> {
    info!("Attempting VHDX optimization via elevated inline command: {}", vhdx_path);

    if !Path::new(vhdx_path).exists() {
        return WslCommandResult::error(String::new(), format!("VHDX file not found: {}", vhdx_path));
    }

    let temp_dir = std::env::temp_dir();
    let result_path = temp_dir.join(format!("wsl_opt_res_{}.txt", uuid::Uuid::new_v4()));

    // Build inline PowerShell command: execute compression and write result to a temp file
    // Use Ascii encoding to avoid BOM issues with PS 5.1 default UTF-8 that cause Rust read failures
    let inner_cmd = format!(
        "try {{ Optimize-VHD -Path '{}' -Mode Full; 'OK' | Out-File -FilePath '{}' -Encoding Ascii }} catch {{ $_.Exception.Message | Out-File -FilePath '{}' -Encoding Ascii; exit 1 }}",
        vhdx_path.replace("'", "''"), // Escape single quotes
        result_path.display(),
        result_path.display()
    );

    // Elevate privileges using Start-Process -Verb RunAs
    let ps_command = format!(
        "Start-Process powershell -Verb RunAs -Wait -WindowStyle Hidden -ArgumentList '-NoProfile','-NonInteractive','-Command',\"{}\"",
        inner_cmd.replace("\"", "`\"") // Escape double quotes for ArgumentList
    );

    let mut cmd = Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_command]);

    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let result = cmd.output().await;

    // Read and clean up the result file
    let result_content = fs::read_to_string(&result_path).unwrap_or_default();
    let _ = fs::remove_file(&result_path);
    let result_trimmed = result_content.trim();

    match result {
        Ok(output) => {
            // Use contains to avoid possible BOM or invisible character interference
            if output.status.success() && result_trimmed.contains("OK") {
                info!("VHDX optimization completed via inline Optimize-VHD");
                WslCommandResult::success("Optimize-VHD completed".to_string(), None)
            } else {
                let err = if result_trimmed.is_empty() {
                    "Elevation cancelled or Optimize-VHD failed (check Hyper-V module)".to_string()
                } else {
                    result_trimmed.to_string()
                };
                warn!("Optimize-VHD failed: {}", err);
                WslCommandResult::error(String::new(), err)
            }
        }
        Err(e) => {
            error!("Failed to execute powershell: {}", e);
            WslCommandResult::error(String::new(), e.to_string())
        }
    }
}

// Core compression workflow:
// 1. Force Export to .tar file: serves as both "safety backup" and "compression source".
// 2. Try Optimize-VHD: the fastest path.
// 3. If that fails, re-Import from the existing .tar: the most robust path.
pub async fn compress_vhdx<F>(
    executor: &WslCommandExecutor, 
    distro_name: &str, 
    cleanup_first: bool, 
    backup_first: bool, 
    enable_sparse_flag: bool,
    compress_script_url: &str,
    local_cleanup_script: &str,
    progress_callback: F
) -> WslCommandResult<String> 
where F: Fn(&str) {
    info!("Starting optimized distro compression workflow for: {}", distro_name);
    
    // 1. Get distro information
    progress_callback("task.compress_starting");
    let info = crate::wsl::ops::info::get_distro_information(executor, distro_name).await;
    
    // Record initial states
    let is_default = crate::utils::registry::is_default_distro(distro_name);
    let initial_is_sparse = info.data.as_ref().map(|d| d.is_sparse).unwrap_or(false);
    info!("Initial state for {}: default={}, sparse={}", distro_name, is_default, initial_is_sparse);

    let initial_status = if let Some(ref data) = info.data {
        data.status.clone()
    } else {
        "Stopped".to_string()
    };

    let install_dir = if !info.data.as_ref().map(|d| d.install_location.is_empty()).unwrap_or(true) {
        info.data.as_ref().unwrap().install_location.clone()
    } else {
        return WslCommandResult::error(String::new(), "Install location not found".to_string());
    };

    let vhdx_path = info.data.as_ref().map(|d| d.vhdx_path.clone()).unwrap_or_default();

    // 2. Start distro for preparation
    if initial_status != "Running" {
        info!("Starting distro for preparation...");
        let _ = crate::wsl::ops::lifecycle::start_distro(executor, distro_name).await;
    }

    // Record the default user
    let default_user = get_default_user(executor, distro_name).await;

    if cleanup_first {
        progress_callback("task.compress_cleaning");
        let _ = cleanup_temp_files(executor, distro_name, compress_script_url, local_cleanup_script).await;
    }

    progress_callback("task.compress_fstrim");
    let _ = fstrim_rootfs(executor, distro_name).await;

    // 3. Stop the distro
    info!("Stopping distro for compression...");
    let _ = crate::wsl::ops::lifecycle::stop_distro(executor, distro_name).await;
    
    let mut stopped = false;
    for i in 0..5 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let check = crate::wsl::ops::info::get_distro_information(executor, distro_name).await;
        if let Some(data) = check.data {
            if data.status == "Stopped" {
                stopped = true;
                break;
            }
        }
        info!("Waiting for distro to stop... (attempt {})", i + 1);
    }
    if !stopped {
        return WslCommandResult::error(String::new(), "Failed to stop distro. VHDX might be locked.".to_string());
    }

    // 4. Disk space check
    let space_check = check_disk_space(if !vhdx_path.is_empty() { &vhdx_path } else { &install_dir });
    if !space_check.success {
        return space_check.map(|_| String::new());
    }

    // 5. Force Export backup (instead of previous VHDX copy)
    progress_callback("task.compress_export");
    let tar_path = if !vhdx_path.is_empty() {
        Path::new(&vhdx_path).with_extension("tar")
    } else {
        Path::new(&install_dir).join(format!("{}.tar", distro_name))
    };
    
    // If a backup with the same name already exists, rename the old one to preserve history (e.g., ext4_20260512_2114.tar)
    if tar_path.exists() {
        if let Ok(metadata) = fs::metadata(&tar_path) {
            let timestamp = metadata.modified().ok()
                .map(|t| {
                    let dt: chrono::DateTime<chrono::Local> = t.into();
                    dt.format("%Y%m%d_%H%M%S").to_string()
                })
                .unwrap_or_else(|| "old".to_string());
            
            let stem = tar_path.file_stem().unwrap_or_default().to_string_lossy();
            let new_name = format!("{}_{}.tar", stem, timestamp);
            if let Some(parent) = tar_path.parent() {
                let new_path = parent.join(new_name);
                info!("Existing backup found. Renaming to: {}", new_path.display());
                let _ = fs::rename(&tar_path, new_path);
            }
        }
    }

    let tar_path_str = tar_path.to_string_lossy().to_string();

    info!("Exporting distro as both backup and compression source: {}", tar_path_str);
    let export_result = executor.execute_command(&["--export", distro_name, &tar_path_str]).await;
    if !export_result.success {
        let _ = fs::remove_file(&tar_path);
        return WslCommandResult::error(String::new(), 
            format!("Backup export failed: {}", export_result.error.unwrap_or_default()));
    }

    // Record size before compression
    let size_before = if !vhdx_path.is_empty() {
        fs::metadata(&vhdx_path).map(|m| m.len()).unwrap_or(0)
    } else {
        // For WSL 1 there is no single VHDX file; approximate via the exported tar size,
        // or simply set to 0 to force Tier 2
        0 
    };

    // ========== 6. Compression strategy ==========
    
    // --- Tier 1: Try Optimize-VHD (fast shrink) ---
    // Only applies to WSL 2 (VHDX path exists)
    let mut opt_result = WslCommandResult::error(String::new(), "Not a VHDX-based distro".to_string());
    let mut saved_opt = 0;

    if !vhdx_path.is_empty() {
        progress_callback("task.compress_diskpart");
        info!("=== Trying Tier 1: Optimize-VHD ===");
        opt_result = optimize_vhd_elevated(&vhdx_path).await;

        let size_after_opt = fs::metadata(&vhdx_path).map(|m| m.len()).unwrap_or(0);
        saved_opt = if size_before > size_after_opt { size_before - size_after_opt } else { 0 };

        // Smart fallback logic:
        // If Tier 1 succeeded and freed more than 100MB, consider the result satisfactory and finish.
        if opt_result.success && saved_opt > 100 * 1024 * 1024 {
            let saved_gb = saved_opt as f64 / (1024.0 * 1024.0 * 1024.0);
            info!("Compression finished via Optimize-VHD (Tier 1). Saved: {:.2} GB", saved_gb);
            
            if !backup_first {
                info!("User did not request to keep backup, removing tar file.");
                let _ = fs::remove_file(&tar_path);
            }
            
            // Restore/Apply sparse mode
            crate::wsl::ops::sparse::apply_sparse_vhdx(executor, distro_name, enable_sparse_flag, initial_is_sparse).await;

            return WslCommandResult::success(format!("{:.2} GB", saved_gb), None);
        }
    }

    // --- Tier 2: Re-Import from the existing tar ---
    if opt_result.success {
        info!("Tier 1 saved only {:.2} MB. Falling back to Tier 2 for thorough compression.", 
            saved_opt as f64 / (1024.0 * 1024.0));
    } else {
        warn!("Tier 1 failed. Falling back to Tier 2: Import from backup tar");
    }
    
    // Unregister
    info!("Unregistering distro: {}", distro_name);
    let unreg_result = executor.execute_command(&["--unregister", distro_name]).await;
    if !unreg_result.success {
        // If unregister fails, keep the tar just in case
        return WslCommandResult::error(String::new(), 
            format!("Unregister failed: {}", unreg_result.error.unwrap_or_default()));
    }

    // Import
    progress_callback("task.compress_import");
    info!("Importing distro from: {} to: {}", tar_path_str, install_dir);
    let import_result = executor.execute_command(&["--import", distro_name, &install_dir, &tar_path_str, "--version", "2"]).await;
    if !import_result.success {
        error!("Import failed! Recovery tar preserved at: {}", tar_path_str);
        return WslCommandResult::error(String::new(), 
            format!("Import failed! Distribution unregistered. Recovery tar preserved at: {}", tar_path_str));
    }

    // Restore the default user
    restore_default_user(executor, distro_name, &default_user).await;

    // ========== 7. Restoration ==========
    
    // 7.1 Apply sparse mode if requested OR if it was originally sparse
    // Do this BEFORE set-default to minimize potential file locks
    crate::wsl::ops::sparse::apply_sparse_vhdx(executor, distro_name, enable_sparse_flag, initial_is_sparse).await;

    // 7.2 Restore default status if it was lost during re-import
    if is_default {
        info!("Restoring default distro status for {}...", distro_name);
        let _ = executor.execute_command(&["--set-default", distro_name]).await;
    }

    // 8. Post-processing
    let size_after = fs::metadata(&vhdx_path).map(|m| m.len()).unwrap_or(0);
    let saved_bytes = if size_before > size_after { size_before - size_after } else { 0 };
    let saved_gb = saved_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

    // If the user did not request to keep the backup, remove the tar
    if !backup_first {
        info!("User did not request to keep backup, removing tar file.");
        let _ = fs::remove_file(&tar_path);
    } else {
        info!("Backup preserved at: {}", tar_path_str);
    }

    info!("Compression finished via export/import. Saved: {:.2} GB", saved_gb);
    WslCommandResult::success(format!("{:.2} GB", saved_gb), None)
}
