use tokio::task;
use tracing::{info, debug, error};
use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::{WslCommandResult, WslDistro, WslInformation, WslStatus};

pub async fn list_distros(executor: &WslCommandExecutor) -> WslCommandResult<Vec<WslDistro>> {
    let result = executor.execute_command(&["-l", "-v"]).await;
    if !result.success {
        return WslCommandResult::error(result.output, result.error.unwrap_or_default());
    }

    let distros = crate::wsl::parser::parse_distros_list(&result.output);
    WslCommandResult::success(result.output, Some(distros))
}

pub async fn list_available_distros(executor: &WslCommandExecutor) -> WslCommandResult<String> {
    executor.execute_command(&["-l", "-o"]).await
}

pub async fn detect_fastest_source(_executor: &WslCommandExecutor) -> bool {
    info!("Probing network connection to https://github.com");

    let result = task::spawn_blocking(|| {
        // Check https://github.com with 5 seconds timeout
        match ureq::head("https://github.com")
            .timeout(std::time::Duration::from_secs(5))
            .call() 
        {
            Ok(response) => {
                response.status() == 200
            }
            Err(e) => {
                debug!("GitHub probe failed: {}", e);
                false
            }
        }
    }).await;

    match result {
        Ok(is_accessible) => {
            if is_accessible {
                info!("GitHub is accessible (HTTP 200). Using WebDownload.");
                true
            } else {
                info!("GitHub is not accessible or timed out. Using default (Windows Update).");
                false
            }
        }
        Err(e) => {
            error!("Failed to execute network probe: {}. Defaulting to Windows Update.", e);
            false
        }
    }
}

pub async fn get_distro_information(executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<WslInformation> {
    let distro_name_owned = distro_name.to_string();
    
    // Get registry information and VHDX size
    let ps_script = format!(r#"
        $distro = "{}"
        $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
        $subkeys = Get-ChildItem $regPath -ErrorAction SilentlyContinue
        
        $res = @{{
            distro_name = $distro
            wsl_version = "Unknown"
            install_location = "Unknown"
            vhdx_path = "Unknown"
            vhdx_size = "0 GB"
            package_family_name = ""
        }}

        foreach ($subkey in $subkeys) {{
            $props = Get-ItemProperty $subkey.PSPath -ErrorAction SilentlyContinue
            if ($props.DistributionName -eq $distro) {{
                $res.install_location = $props.BasePath
                $res.wsl_version = if ($props.Version -eq 2) {{ "WSL2" }} else {{ "WSL1" }}
                $res.package_family_name = if ($props.PackageFamilyName) {{ $props.PackageFamilyName }} else {{ "" }}
                
                $vhdxPaths = @(
                    (Join-Path $props.BasePath "ext4.vhdx"),
                    (Join-Path $props.BasePath "LocalState\ext4.vhdx")
                )
                foreach ($p in $vhdxPaths) {{
                    if (Test-Path $p) {{
                        $res.vhdx_path = $p
                        $size = (Get-Item $p).Length
                        $res.vhdx_size = "$([math]::Round($size / 1GB, 2)) GB"
                        break
                    }}
                }}
                break
            }}
        }}
        $res | ConvertTo-Json
    "#, distro_name_owned);

    let mut cmd = tokio::process::Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        #[allow(unused_imports)]
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let mut information = WslInformation::default();
    information.distro_name = distro_name_owned.clone();

    if let Ok(output) = cmd.output().await {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&stdout) {
            information.wsl_version = parsed["wsl_version"].as_str().unwrap_or("Unknown").to_string();
            information.install_location = parsed["install_location"].as_str().unwrap_or("Unknown").to_string();
            information.vhdx_path = parsed["vhdx_path"].as_str().unwrap_or("Unknown").to_string();
            information.vhdx_size = parsed["vhdx_size"].as_str().unwrap_or("0 GB").to_string();
            information.package_family_name = parsed["package_family_name"].as_str().unwrap_or("").to_string();
        }
    }

    // Get running status
    let distros_result = list_distros(executor).await;
    let mut is_running = false;
    if let Some(distros) = distros_result.data {
        if let Some(d) = distros.iter().find(|d| d.name == distro_name_owned) {
            is_running = d.status == WslStatus::Running;
            information.status = match d.status {
                WslStatus::Running => "Running",
                WslStatus::Stopped => "Stopped",
            }.to_string();
        }
    }

    // Get statistics only if running
    if is_running {
        // IP Address
        let ip_result = executor.execute_command(&["-d", &distro_name_owned, "--exec", "hostname", "-I"]).await;
        if ip_result.success {
            information.ip_address = ip_result.output.trim().split_whitespace().next().unwrap_or("Unknown").to_string();
        } else {
            information.ip_address = "Error".to_string();
        }

        // Docker Containers
        let docker_result = executor.execute_command(&["-d", &distro_name_owned, "--exec", "docker", "ps", "--format", "{{.Names}} ({{.Status}})"]).await;
        if docker_result.success {
            let containers: Vec<String> = docker_result.output.lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            information.docker_containers = containers;
        }

        let df_result = executor.execute_command(&["-d", &distro_name_owned, "--exec", "df", "-B1M", "/"]).await;
        if df_result.success {
            let output = df_result.output.trim();
            if let Some(second_line) = output.lines().nth(1) {
                let parts: Vec<&str> = second_line.split_whitespace().collect();
                if parts.len() >= 3 {
                    if let Ok(mb_val) = parts[2].parse::<f64>() {
                        let gb_val = mb_val / 1024.0;
                        information.actual_used = format!("{:.2} GB", gb_val);
                    } else {
                        information.actual_used = format!("{} MB", parts[2]); 
                    }
                } else {
                    information.actual_used = "Parse Error".to_string();
                }
            } else {
                information.actual_used = "No Output".to_string();
            }
        } else {
            information.actual_used = "Error".to_string();
        }
    } else {
        information.actual_used = "Unknown (Stopped)".to_string();
    }

    WslCommandResult::success(String::new(), Some(information))
}

#[allow(dead_code)]
pub async fn get_distro_install_location(_executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    let distro_name_owned = distro_name.to_string();
    
    // Minimal PowerShell script to only get the BasePath (install location)
    let ps_script = format!(r#"
        $distro = "{}"
        $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
        $subkeys = Get-ChildItem $regPath -ErrorAction SilentlyContinue
        foreach ($subkey in $subkeys) {{
            $props = Get-ItemProperty $subkey.PSPath -ErrorAction SilentlyContinue
            if ($props.DistributionName -eq $distro) {{
                $props.BasePath
                break
            }}
        }}
    "#, distro_name_owned);

    let mut cmd = tokio::process::Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        // use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    if let Ok(output) = cmd.output().await {
        let location = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !location.is_empty() {
            return WslCommandResult::success(String::new(), Some(location));
        }
    }

    WslCommandResult::error("".into(), "Failed to get install location".into())
}

pub async fn check_windows_features(_executor: &WslCommandExecutor) -> WslCommandResult<Vec<(String, bool)>> {
    let ps_script = r#"
        $features = @("Microsoft-Windows-Subsystem-Linux", "VirtualMachinePlatform")
        $results = @()
        foreach ($f in $features) {
            $state = Get-WindowsOptionalFeature -Online -FeatureName $f -ErrorAction SilentlyContinue
            $results += @{ name = $f; enabled = ($state.State -eq "Enabled") }
        }
        $results | ConvertTo-Json
    "#;

    let mut cmd = tokio::process::Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    if let Ok(output) = cmd.output().await {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
            let mut features = Vec::new();
            for item in parsed {
                let name = item["name"].as_str().unwrap_or("").to_string();
                let enabled = item["enabled"].as_bool().unwrap_or(false);
                features.push((name, enabled));
            }
            return WslCommandResult::success(String::new(), Some(features));
        }
    }

    WslCommandResult::error("".into(), "Failed to check Windows features".into())
}

pub async fn get_distro_default_uid(_executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<u32> {
    let distro_name_owned = distro_name.to_string();
    
    let ps_script = format!(r#"
        $distro = "{}"
        $regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
        $subkeys = Get-ChildItem $regPath -ErrorAction SilentlyContinue
        foreach ($subkey in $subkeys) {{
            $props = Get-ItemProperty $subkey.PSPath -ErrorAction SilentlyContinue
            if ($props.DistributionName -eq $distro) {{
                if ($props.DefaultUid) {{
                    $props.DefaultUid
                }} else {{
                    0
                }}
                break
            }}
        }}
    "#, distro_name_owned);

    let mut cmd = tokio::process::Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    if let Ok(output) = cmd.output().await {
        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Ok(uid) = output_str.parse::<u32>() {
            return WslCommandResult::success(String::new(), Some(uid));
        }
    }

    // Default to 0 (root) if not found or error
    WslCommandResult::success(String::new(), Some(0))
}
