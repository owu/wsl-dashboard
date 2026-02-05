use tokio::task;
use tokio::process::Command;
use tracing::{info, warn, error};
use serde_json;
use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::WslCommandResult;
use crate::config::ConfigManager;
use crate::app::autostart::update_windows_autostart;

pub async fn start_distro(executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    // Option 1: First try to start and verify by executing a simple command
    // Use --exec to run a simple echo, which will trigger subsystem startup
    let probe_result = executor.execute_command(&["-d", distro_name, "--", "sh", "-c", "echo 'starting'"]).await;
    
    if !probe_result.success {
        warn!("Failed to start WSL distro {}: {:?}", distro_name, probe_result.error);
        return probe_result;
    }

    // After successful detection, we need to maintain the subsystem's running state.
    // WSL automatically stops the subsystem when there are no active processes or terminal connections.
    // We keep it active by running a non-exiting, windowless 'sleep infinity' process in the background.
    let distro_name_owned = distro_name.to_string();
    task::spawn_blocking(move || {
        info!("Starting background keep-alive process for WSL distro: {}", distro_name_owned);
        
        // Start wsl.exe running sleep infinity with CREATE_NO_WINDOW flag to avoid console window popping up
        let mut cmd = std::process::Command::new("wsl.exe");
        cmd.args(&["-d", &distro_name_owned, "--", "sleep", "infinity"]);
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        
        match cmd.spawn() {
            Ok(_child) => {
                info!("Successfully spawned keep-alive process for {}", distro_name_owned);
                // Don't wait for the child process to end
            }
            Err(e) => {
                error!("Failed to spawn keep-alive process for {}: {}", distro_name_owned, e);
            }
        }
    });

    WslCommandResult::success(format!("Distro '{}' started and keep-alive process initiated", distro_name), None)
}

pub async fn stop_distro(executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    executor.execute_command(&["--terminate", distro_name]).await
}

pub async fn shutdown_wsl(executor: &WslCommandExecutor) -> WslCommandResult<String> {
    executor.execute_command(&["--shutdown"]).await
}

pub async fn delete_distro(executor: &WslCommandExecutor, config_manager: &ConfigManager, distro_name: &str) -> WslCommandResult<String> {
    info!("Operation: Delete WSL distribution - {}", distro_name);
    
    // 1. Determine PackageFamilyName and if it's the only instance before unregistering
    let ps_script = format!(r#"
        $distro = "{}"
        $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
        $subkeys = Get-ChildItem $regPath -ErrorAction SilentlyContinue
        
        $targetPfn = ""
        $pfnCounts = @{{}}
        
        # First Pass: Identify the target's PFN and all Pfns in use
        foreach ($subkey in $subkeys) {{
            $props = Get-ItemProperty $subkey.PSPath -ErrorAction SilentlyContinue
            $pfn = ""
            
            if ($props.PackageFamilyName) {{
                $pfn = $props.PackageFamilyName.Trim()
            }} elseif ($props.BasePath -match "LocalState$") {{
                # Heuristic: Find PFN in BasePath if registry key is missing
                if ($props.BasePath -match "Packages\\([^\\]+)\\LocalState") {{
                    $pfn = $matches[1]
                }}
            }}
            
            if ($pfn) {{
                $pfnCounts[$pfn] = [int]$pfnCounts[$pfn] + 1
                if ($props.DistributionName.Trim() -eq $distro) {{
                    $targetPfn = $pfn
                }}
            }}
        }}
        
        $shouldRemove = $false
        if ($targetPfn -and ($pfnCounts[$targetPfn] -eq 1)) {{
            $shouldRemove = $true
        }}
        
        @{{ pfn = $targetPfn; should_remove = $shouldRemove }} | ConvertTo-Json
    "#, distro_name);

    let mut cmd = Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
        // Set kill_on_drop so the process is terminated if wait_with_output times out and the future is dropped
        cmd.kill_on_drop(true);
    }

    let mut pfn_to_remove = None;
    
    // Spawn and wait for output with timeout
    let output_res = tokio::time::timeout(
        std::time::Duration::from_secs(15), 
        async {
            match cmd.spawn() {
                Ok(child) => child.wait_with_output().await,
                Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }
        }
    ).await;

    match output_res {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&stdout) {
                let pfn = parsed["pfn"].as_str().unwrap_or("").to_string();
                let should_remove = parsed["should_remove"].as_bool().unwrap_or(false);
                if !pfn.is_empty() && should_remove {
                    pfn_to_remove = Some(pfn);
                    info!("Distribution '{}' is associated with package '{}' and is the only instance using it.", distro_name, pfn_to_remove.as_ref().unwrap());
                } else if !pfn.is_empty() {
                    info!("Distribution '{}' is associated with package '{}', but other instances still use this launcher.", distro_name, pfn);
                }
            }
        }
        Ok(Err(e)) => {
            warn!("Failed to get output from PowerShell PFN detection: {}", e);
        }
        Err(_) => {
            warn!("PowerShell PFN detection timed out after 15s (process killed by kill_on_drop)");
        }
    }

    // 2. Extra Cleanups: config file and autostart vbs
    info!("Cleaning up configurations for '{}' before unregistering", distro_name);
    
    // a. Remove from instances.toml (Use spawn_blocking for sync config management)
    let cm = config_manager.clone();
    let dn = distro_name.to_string();
    let removal_res = task::spawn_blocking(move || {
        cm.remove_instance_config(&dn).map_err(|e| e.to_string())
    }).await;

    if let Err(e) = removal_res {
        warn!("Task join error during instance config removal: {}", e);
    } else if let Ok(Err(e)) = removal_res {
        warn!("Failed to remove instance config for '{}': {}", distro_name, e);
    }

    // b. Remove from wsl-dashboard.vbs
    if let Err(e) = update_windows_autostart(distro_name, false).await {
        warn!("Failed to remove autostart line for '{}' from VBS: {}", distro_name, e);
    }

    // 3. Perform wsl --unregister
    let result = executor.execute_command(&["--unregister", distro_name]).await;
    
    if !result.success {
        warn!("Failed to unregister WSL distro '{}': {:?}", distro_name, result.error);
        return result;
    }

    info!("Successfully unregistered WSL distro '{}'", distro_name);

    // 3. Remove Appx package if needed (Run in background to avoid blocking distro removal)
    if let Some(pfn) = pfn_to_remove {
        info!("Initiating launcher cleanup for PackageFamilyName: {} (Background)", pfn);
        
        tokio::spawn(async move {
            let uninstall_script = format!(r#"
                $pfn = "{}"
                # Faster search by splitting PFN and using Name wildcard
                $namePart = $pfn.Split('_')[0]
                $packages = Get-AppxPackage -Name "*$namePart*" | Where-Object {{ 
                    $_.PackageFamilyName -eq $pfn -or 
                    $_.PackageFullName -like "*$pfn*"
                }}

                if ($packages) {{
                    foreach ($pkg in $packages) {{
                        Write-Host "Found associated package: $($pkg.PackageFullName). Uninstalling..."
                        Remove-AppxPackage -Package $pkg.PackageFullName -ErrorAction SilentlyContinue
                    }}
                }} else {{
                    Write-Host "No associated Appx package could be matches for: $pfn"
                }}
            "#, pfn);

            let mut uninstall_cmd = Command::new("powershell");
            uninstall_cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &uninstall_script]);
            #[cfg(windows)]
            {
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                uninstall_cmd.creation_flags(CREATE_NO_WINDOW);
                uninstall_cmd.kill_on_drop(true);
            }

            let cleanup_res = tokio::time::timeout(
                std::time::Duration::from_secs(15), 
                async {
                    match uninstall_cmd.spawn() {
                        Ok(child) => child.wait_with_output().await,
                        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
                    }
                }
            ).await;

            match cleanup_res {
                Ok(Ok(output)) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !stdout.is_empty() { 
                        info!("Launcher cleanup detail: {}", stdout); 
                    }
                }
                Ok(Err(e)) => {
                    error!("Failed to execute launcher cleanup: {}", e);
                }
                Err(_) => {
                    warn!("Launcher cleanup timed out after 15s (process killed by kill_on_drop)");
                }
            }
        });
    }

    WslCommandResult::success(format!("Distro '{}' deleted and launcher cleanup initiated", distro_name), None)
}

pub async fn move_distro(executor: &WslCommandExecutor, distro_name: &str, new_path: &str) -> WslCommandResult<String> {
    info!("Operation: Move WSL distribution - {} to {}", distro_name, new_path);
    executor.execute_command(&["--manage", distro_name, "--move", new_path]).await
}

pub async fn set_distro_default_uid(_executor: &WslCommandExecutor, distro_name: &str, uid: u32) -> WslCommandResult<String> {
    info!("Operation: Set Default UID - {} to {}", distro_name, uid);
    
    let ps_script = format!(r#"
        $distro = "{}"
        $uid = {}
        $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
        $subkeys = Get-ChildItem $regPath -ErrorAction SilentlyContinue
        
        foreach ($subkey in $subkeys) {{
            $props = Get-ItemProperty $subkey.PSPath -ErrorAction SilentlyContinue
            if ($props.DistributionName -eq $distro) {{
                Set-ItemProperty -Path $subkey.PSPath -Name "DefaultUid" -Value $uid -Type DWord
                "Success"
                break
            }}
        }}
    "#, distro_name, uid);

    let mut cmd = Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    match cmd.output().await {
        Ok(output) => {
             let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
             if stdout == "Success" {
                 WslCommandResult::success("Default UID updated".to_string(), None)
             } else {
                 WslCommandResult::error(stdout, "Failed to find or update registry key".to_string())
             }
        },
        Err(e) => WslCommandResult::error("".to_string(), e.to_string()),
    }
}

pub async fn reset_wsl_network(executor: &WslCommandExecutor) -> WslCommandResult<String> {
    info!("Operation: Reset WSL Network (Shutdown)");
    executor.execute_command(&["--shutdown"]).await
}

pub async fn compact_distro_disk(executor: &WslCommandExecutor, distro_name: &str, vhdx_path: &str) -> WslCommandResult<String> {
    info!("Operation: Compact VHDX - {} at {}", distro_name, vhdx_path);
    
    // 1. Ensure distro is stopped
    let _ = stop_distro(executor, distro_name).await;
    
    // 2. Prepare diskpart script
    // Note: This requires absolute Windows path. vhdx_path should already be one.
    let diskpart_script = format!(
        "select vdisk file=\"{}\"\r\nattach vdisk readonly\r\ncompact vdisk\r\ndetach vdisk\r\n",
        vhdx_path
    );
    
    let temp_script = std::env::temp_dir().join(format!("compact_{}.txt", distro_name));
    if let Err(e) = std::fs::write(&temp_script, diskpart_script) {
        return WslCommandResult::error("".into(), format!("Failed to create diskpart script: {}", e));
    }

    // 3. Execute diskpart
    // We use powershell to run diskpart to handle potential path issues or elevation hints
    let ps_script = format!(
        "Start-Process diskpart.exe -ArgumentList '/s \"{}\"' -Wait -WindowStyle Hidden",
        temp_script.display()
    );

    let mut cmd = Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let result = match cmd.output().await {
        Ok(output) => {
            if output.status.success() {
                WslCommandResult::success("Disk compacted successfully".into(), None)
            } else {
                WslCommandResult::error(String::from_utf8_lossy(&output.stdout).into(), "Diskpart failed".into())
            }
        },
        Err(e) => WslCommandResult::error("".into(), e.to_string()),
    };

    let _ = std::fs::remove_file(temp_script);
    result
}
