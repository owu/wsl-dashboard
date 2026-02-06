use tokio::fs;
use std::time::Duration;
use tracing::{info, warn};
#[cfg(windows)]

pub fn get_startup_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let path = dirs::data_dir()
        .ok_or("Could not find AppData directory")?
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");
    Ok(path)
}

pub fn get_vbs_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    Ok(get_startup_dir()?.join("wsl-dashboard.vbs"))
}

/// Writes to a file with a timeout mechanism to avoid hanging for a long time if intercepted by anti-virus software
/// 
/// If the write operation does not complete within 5 seconds, it returns a timeout error
async fn write_with_timeout(
    path: &std::path::Path,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_path_buf();
    
    // Use tokio::time::timeout to set a 5-second timeout
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        fs::write(&path, content)
    ).await;
    
    match result {
        Ok(Ok(())) => {
            // Write succeeded
            Ok(())
        }
        Ok(Err(e)) => {
            // Write failed
            Err(Box::new(e))
        }
        Err(_) => {
            // Timeout
            warn!("File write timed out (5s), possibly intercepted by anti-virus software");
            Err("File write timed out, possibly intercepted by anti-virus software. Please check your anti-virus settings.".into())
        }
    }
}

/// Updates the Windows startup VBS file to add or remove autostart for a WSL distribution.
pub async fn update_windows_autostart(distro_name: &str, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let startup_dir = get_startup_dir()?;
    
    if !startup_dir.exists() {
        fs::create_dir_all(&startup_dir).await?;
    }
    
    let vbs_path = get_vbs_path()?;
    let line_to_manage = format!("ws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide", distro_name);
    let header = "Set ws = WScript.CreateObject(\"WScript.Shell\")";

    let mut lines: Vec<String> = if vbs_path.exists() {
        let content = fs::read_to_string(&vbs_path).await?;
        content.lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![header.to_string()]
    };

    // Ensure header is always at the top
    if !lines.iter().any(|l: &String| l.contains("WScript.CreateObject")) {
        lines.insert(0, header.to_string());
    }

    if enable {
        // Add if not present
        if !lines.iter().any(|l| l == &line_to_manage) {
            lines.push(line_to_manage);
            info!("✅ Added autostart line for '{}' to VBS", distro_name);
        }
    } else {
        // Remove strictly matching lines
        let old_count = lines.len();
        lines.retain(|l| l != &line_to_manage);
        if lines.len() < old_count {
            info!("✅ Removed autostart line for '{}' from VBS", distro_name);
        }
    }

    // Write back with timeout protection
    let content = lines.join("\r\n");
    write_with_timeout(&vbs_path, content).await?;
    info!("📂 Updated autostart VBS: {}", vbs_path.display());

    Ok(())
}

pub fn is_autostart_enabled(distro_name: &str) -> bool {
    let vbs_path = match get_vbs_path() {
        Ok(p) => p,
        Err(_) => return false,
    };

    if !vbs_path.exists() {
        return false;
    }

    let line_to_check = format!("ws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide", distro_name);
    
    // Explicitly use std::fs for sync read to avoid any tokio::fs async/sync confusion
    if let Ok(content) = std::fs::read_to_string(&vbs_path) {
        content.lines().any(|l| l.trim() == line_to_check)
    } else {
        false
    }
}

/// Sets the dashboard itself to start automatically on Windows logon using the registry (HKCU).
/// If start_minimized is true, adds /silent parameter to the command line.
/// Fallbacks to VBS in Startup folder if registry access is denied.
pub async fn set_dashboard_autostart(enable: bool, start_minimized: bool) -> Result<(), Box<dyn std::error::Error>> {
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
        
        info!("Enabling dashboard autostart in registry: {}", command);
        // Try native registry first
        match crate::utils::registry::write_reg_string(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name, &command) {
            Ok(_) => {
                info!("✅ Dashboard autostart set in registry.");
                // Clean up any old fallback VBS if it exists
                if let Ok(p) = get_dashboard_vbs_path() { let _ = fs::remove_file(p).await; }
                Ok(())
            }
            Err(e) => {
                warn!("Registry autostart failed ({}), trying fallback to Startup folder...", e);
                set_dashboard_autostart_vbs_fallback(&command).await
            }
        }
    } else {
        info!("Disabling dashboard autostart...");
        // Remove from registry
        let reg_res = crate::utils::registry::delete_reg_value(windows::Win32::System::Registry::HKEY_CURRENT_USER, run_subkey, value_name);
        
        // Also remove from Startup folder if exists
        let vbs_res: Result<(), Box<dyn std::error::Error>> = if let Ok(p) = get_dashboard_vbs_path() {
            if p.exists() { fs::remove_file(p).await.map_err(|e| e.into()) } else { Ok(()) }
        } else { Ok(()) };

        if reg_res.is_err() && vbs_res.is_err() {
            // Only error if both failed and it's not a "not found" error
            let err_msg = format!("Failed to disable autostart: Reg({:?}), VBS({:?})", reg_res, vbs_res);
            if !err_msg.contains("not found") && !err_msg.contains("system cannot find the file") {
                return Err(err_msg.into());
            }
        }
        info!("✅ Dashboard autostart disabled.");
        Ok(())
    }
}

pub fn get_dashboard_vbs_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    Ok(get_startup_dir()?.join("dashboard-autostart.vbs"))
}

async fn set_dashboard_autostart_vbs_fallback(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vbs_path = get_dashboard_vbs_path()?;
    let content = format!(
        "Set ws = WScript.CreateObject(\"WScript.Shell\")\r\nws.run \"{}\", 0\r\n",
        command.replace("\"", "\"\"") // Escape quotes for VBS
    );
    write_with_timeout(&vbs_path, content).await?;
    info!("✅ Dashboard autostart fallback VBS created at: {}", vbs_path.display());
    Ok(())
}

/// Automatically repairs the autostart path if the software has been moved (portable mode).
/// - If autostart is enabled, updates the registry with current path (and /silent if start_minimized)
/// - If autostart is disabled, removes the registry entry if it exists
pub async fn repair_autostart_path(autostart_enabled: bool, start_minimized: bool) {
    if autostart_enabled {
        info!("System check: Autostart is enabled, verifying/updating path...");
        if let Err(e) = set_dashboard_autostart(true, start_minimized).await {
            warn!("Failed to repair autostart path: {}", e);
        }
    } else {
        // If autostart is disabled in config, ensure entries are removed
        info!("System check: Autostart is disabled, ensuring entry is removed...");
        let _ = set_dashboard_autostart(false, false).await;
    }
}
