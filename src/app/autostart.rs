// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only


use tracing::{info, warn, trace};







#[cfg(windows)]

pub fn get_startup_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let path = dirs::data_dir()
        .ok_or("Could not find AppData directory")?
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");
    Ok(path)
}







// Sets the dashboard itself to start automatically on Windows logon using the registry (HKCU).
// If start_minimized is true, adds /silent parameter to the command line.
// Fallbacks to VBS in Startup folder if registry access is denied.
pub async fn set_dashboard_autostart(enable: bool, start_minimized: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let exe_path = std::env::current_exe()?;
    let path_str = exe_path.to_str().ok_or("Invalid executable path")?;
    let run_subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let value_name = "WSLDashboard";

    if enable {
        let command = if start_minimized {
            format!("\"{}\" /silent", path_str)
        } else {
            format!("\"{}\"", path_str)
        };
        
        // 1. Check if it's already set to the same value to avoid redundant writes
        if let Some(current_val) = crate::utils::registry::read_reg_string_ext(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name) {
            if current_val == command {
                trace!("Dashboard autostart is already correctly set in registry, skipping write.");
                return Ok(());
            }
        }

        info!("Enabling dashboard autostart in registry: {}", command);
        // Try native registry first
        match crate::utils::registry::write_reg_string(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name, &command) {
            Ok(_) => {
                info!("Dashboard autostart set in registry.");
                Ok(())
            }
            Err(e) => {
                warn!("Registry autostart failed ({})", e);
                Err(e.into())
            }
        }
    } else {
        info!("Disabling dashboard autostart in registry...");
        let reg_res = crate::utils::registry::delete_reg_value(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name);
        
        if let Err(e) = reg_res {
            let err_msg = e.to_string();
            if !err_msg.contains("not found") && !err_msg.contains("system cannot find the file") {
                return Err(err_msg.into());
            }
        }

        info!("Dashboard autostart disabled.");
        Ok(())
    }
}




pub fn check_autostart_valid(start_minimized: bool) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let exe_path = std::env::current_exe()?;
    let path_str = exe_path.to_str().ok_or("Invalid executable path")?;
    let run_subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let value_name = "WSLDashboard";

    let target_command = if start_minimized {
        format!("\"{}\" /silent", path_str)
    } else {
        format!("\"{}\"", path_str)
    };

    if let Some(current_val) = crate::utils::registry::read_reg_string_ext(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name) {
        Ok(current_val == target_command)
    } else {
        Ok(false)
    }
}

// Automatically repairs the autostart path if the software has been moved (portable mode).
// - If autostart is enabled, updates the registry with current path (and /silent if start_minimized)
// - If autostart is disabled, removes the registry entry if it exists
pub async fn repair_autostart_path(autostart_enabled: bool, start_minimized: bool) {
    // 1. Repair registry autostart
    if autostart_enabled {
        // Only log and update if something actually changed
        match check_autostart_valid(start_minimized) {
            Ok(true) => {
                // Path is valid, do nothing
            }
            _ => {
                info!("System check: Autostart is enabled but path is invalid or missing, updating...");
                if let Err(e) = set_dashboard_autostart(true, start_minimized).await {
                    warn!("Failed to repair autostart path: {}", e);
                }
            }
        }
    } else {
        // If autostart is disabled in config, ensure entries are removed
        info!("System check: Autostart is disabled, ensuring entry is removed...");
        let _ = set_dashboard_autostart(false, false).await;
    }

    // 2. Repair scheduled task /TR path if exe has moved
    repair_task_scheduler_path().await;
}

// Checks whether the scheduled task's intermediary script points to the current exe.
// If the exe has moved (portable install), silently updates the script content (no UAC needed).
pub async fn repair_task_scheduler_path() {
    use crate::network::scheduler;
    use std::fs;

    // Only act if the task already exists (don't create scripts if user hasn't initialized the task)
    if !scheduler::check_task_exists() {
        return;
    }

    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            warn!("Failed to get current EXE path: {}", e);
            return;
        }
    };

    let exe_str = exe_path.to_string_lossy();
    let script_path = scheduler::get_script_path();

    if !script_path.exists() {
        info!("Task script missing, recreating...");
        if let Err(e) = scheduler::ensure_task_script_exists() {
            warn!("Failed to recreate task script: {}", e);
        }
        return;
    }

    // Read existing script to check if path is correct
    let content = match fs::read_to_string(&script_path) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read task script: {}", e);
            return;
        }
    };

    // Check if the current exe path is already in the script
    // Simple containment check for the path string
    if content.contains(&format!("$exePath = \"{}\"", exe_str)) {
        trace!("Scheduled task script is up-to-date, no repair needed.");
        return;
    }

    info!("Scheduled task script is stale (exe has moved). Updating script content silently...");
    if let Err(e) = scheduler::ensure_task_script_exists() {
        warn!("Failed to update task script: {}", e);
    } else {
        info!("Scheduled task script updated successfully.");
    }
}
