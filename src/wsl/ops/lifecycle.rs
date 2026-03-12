use tokio::task;
use tokio::process::Command;
use tracing::{info, warn, error, debug};
use std::time::Duration;
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
    // Use native registry access instead of slow PowerShell
    let all_distros_reg = task::spawn_blocking(|| {
        crate::utils::registry::get_wsl_distros_from_reg()
    }).await.unwrap_or_default();
    
    let target_distro_info = all_distros_reg.iter().find(|d| d.name == distro_name);
    let exists_in_reg = target_distro_info.is_some();
    
    let mut pfn_to_remove = None;
    if let Some(info) = target_distro_info {
        let pfn = &info.package_family_name;
        if !pfn.is_empty() {
            // Count how many distros use this same PFN
            let pfn_count = all_distros_reg.iter().filter(|d| &d.package_family_name == pfn).count();
            if pfn_count == 1 {
                pfn_to_remove = Some(pfn.clone());
                info!("Distribution '{}' is associated with package '{}' and is the only instance using it.", distro_name, pfn);
            } else {
                info!("Distribution '{}' is associated with package '{}', but {} other instances still use this launcher.", distro_name, pfn, pfn_count - 1);
            }
        }
    }

    // 2. Extra Cleanups: config file and autostart vbs (Parallelized to reduce I/O wait)
    info!("Cleaning up configurations for '{}' before unregistering", distro_name);
    
    let cm = config_manager.clone();
    let dn1 = distro_name.to_string();
    let dn2 = distro_name.to_string();

    debug!("Starting parallel cleanup tasks for '{}' with 15s timeout...", distro_name);
    let cleanup_future = async {
        tokio::join!(
            // a. Remove from instances.toml
            async {
                debug!("Removing instance config for '{}'...", distro_name);
                let res = task::spawn_blocking(move || {
                    cm.remove_instance_config(&dn1).map_err(|e| e.to_string())
                }).await;
                debug!("Instance config removal for '{}' complete", distro_name);
                res
            },
            // b. Remove from wsl-dashboard.vbs
            async {
                debug!("Updating VBS autostart for '{}' (Removing entries)...", distro_name);
                let res = update_windows_autostart(&dn2, false).await;
                debug!("VBS autostart update for '{}' complete", distro_name);
                res
            }
        )
    };

    let cleanup_result = tokio::time::timeout(Duration::from_secs(15), cleanup_future).await;
    
    if cleanup_result.is_err() {
        warn!("Parallel cleanup tasks for '{}' timed out after 15s. Proceeding.", distro_name);
    }
    
    debug!("Finished parallel cleanup tasks attempt for '{}'", distro_name);
    {
        use tokio::task::JoinError;
        let (config_res, autostart_res): (Result<Result<(), String>, JoinError>, Result<(), Box<dyn std::error::Error + Send + Sync>>) = cleanup_result.unwrap_or((
            Ok(Ok(())), // Mock success to continue
            Ok(())
        ));

        match config_res {
            Ok(Err(e)) => warn!("Failed to remove instance config for '{}': {}", distro_name, e),
            Err(e) => warn!("Task join error during instance config removal: {}", e),
            _ => {}
        }

        if let Err(e) = autostart_res {
            warn!("Failed to remove autostart line for '{}' from VBS: {}", distro_name, e);
        }
    }

    // 3. WSL Operations (Skip if not in registry to avoid unnecessary errors/hangs)
    if !exists_in_reg {
        debug!("Distribution '{}' not found in registry, skipping WSL terminate/unregister.", distro_name);
    } else {
        // Pre-termination to prevent unregister hangs
        debug!("Terminating '{}' before unregistration to avoid hangs (10s timeout)...", distro_name);
        let _ = tokio::time::timeout(
            Duration::from_secs(10),
            executor.execute_command(&["--terminate", distro_name])
        ).await;

        // Perform wsl --unregister with specific timeout to avoid permanent hanging
        debug!("Executing WSL command: wsl --unregister {} (20s timeout)...", distro_name);
        let result = match tokio::time::timeout(
            Duration::from_secs(20),
            executor.execute_command(&["--unregister", distro_name])
        ).await {
            Ok(res) => res,
            Err(_) => {
                let err = format!("WSL unregister timed out for '{}' after 20s", distro_name);
                warn!("{}", err);
                return WslCommandResult::error(String::new(), err);
            }
        };
        
        if !result.success {
            warn!("Failed to unregister WSL distro '{}': {:?}", distro_name, result.error);
            return result;
        }
        info!("Successfully unregistered WSL distro '{}'", distro_name);
    }

    info!("Successfully unregistered WSL distro '{}'", distro_name);

    // 3. Remove Appx package if needed (Run in background to avoid blocking distro removal)
    if let Some(pfn) = pfn_to_remove {
        info!("Initiating launcher cleanup for PackageFamilyName: {} (Background)", pfn);
        
        let bg_sem = executor.background_semaphore().clone();
        tokio::spawn(async move {
            let _permit = bg_sem.acquire().await;
            debug!("Launcher cleanup permit acquired for '{}'", pfn);
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
