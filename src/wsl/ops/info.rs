// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tokio::task;
use tracing::{info, trace, error};
use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::{WslCommandResult, WslDistro, WslInformation, WslStatus};

pub async fn list_distros(executor: &WslCommandExecutor) -> WslCommandResult<Vec<WslDistro>> {
    let result = executor.execute_command(&["-l", "-v"]).await;
    if !result.success {
        let distros = crate::wsl::parser::parse_distros_list(&result.output);
        if distros.is_empty() {
            let version_check = executor.execute_command(&["--version"]).await;
            if version_check.success {
                return WslCommandResult::success(result.output, Some(distros));
            }
        }
        return WslCommandResult::error(result.output, result.error.unwrap_or_default());
    }

    let distros = crate::wsl::parser::parse_distros_list(&result.output);
    WslCommandResult::success(result.output, Some(distros))
}

pub async fn list_available_distros(executor: &WslCommandExecutor) -> WslCommandResult<String> {
    executor.execute_command(&["-l", "-o"]).await
}

pub async fn detect_fastest_source(_executor: &WslCommandExecutor) -> bool {
    info!("Probing network connection to {}", crate::app::GITHUB_DOMAIN);

    let result = task::spawn_blocking(|| {
        // Check GitHub domain with 5 seconds timeout
        match ureq::head(crate::app::GITHUB_DOMAIN)
            .timeout(std::time::Duration::from_secs(5))
            .call() 
        {
            Ok(response) => {
                response.status() == 200
            }
            Err(e) => {
                trace!("GitHub probe failed: {}", e);
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
    let mut information = WslInformation::default();
    information.distro_name = distro_name_owned.clone();

    // Use native registry access instead of PowerShell
    let distros_reg = crate::utils::registry::get_wsl_distros_from_reg();
    if let Some(reg_info) = distros_reg.into_iter().find(|d| d.name == distro_name_owned) {
        information.install_location = reg_info.base_path.clone();
        information.wsl_version = format!("WSL{}", reg_info.version);
        information.package_family_name = reg_info.package_family_name;

        // VHDX Logic (ported from PS heuristic)
        if reg_info.version == 2 {
            if let Some(p) = get_vhdx_path(&reg_info.base_path) {
                information.vhdx_path = p.to_string_lossy().to_string();
                if let Ok(metadata) = std::fs::metadata(&p) {
                    let size_gb = metadata.len() as f64 / (1024.0 * 1024.0 * 1024.0);
                    information.vhdx_size = format!("{:.2} GB", size_gb);
                    
                    // Check sparse status
                    information.is_sparse = crate::utils::system::is_sparse_file(&p.to_string_lossy());
                }
            }
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

    // Get df -B1M / statistics only if running
    if is_running {
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

        // Get IP
        let distro_name_for_ip = distro_name_owned.clone();
        if let Ok(Ok(ip)) = task::spawn_blocking(move || {
            crate::network::tracker::get_distro_ip(&distro_name_for_ip)
        }).await {
            information.ip = ip;
        }
    } else {
        information.actual_used = "Unknown (Stopped)".to_string();
    }

    WslCommandResult::success(String::new(), Some(information))
}

pub async fn get_distro_install_location(_executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    // Replace minimal PowerShell script with native registry access
    let distros_reg = crate::utils::registry::get_wsl_distros_from_reg();
    if let Some(reg_info) = distros_reg.into_iter().find(|d| d.name == distro_name) {
        if !reg_info.base_path.is_empty() {
            return WslCommandResult::success(String::new(), Some(reg_info.base_path));
        }
    }

    WslCommandResult::error("".into(), "Failed to find install location in registry".into())
}
pub fn get_vhdx_path(base_path_str: &str) -> Option<std::path::PathBuf> {
    let base_path = std::path::PathBuf::from(base_path_str);
    
    // Common locations
    let probe_paths = vec![
        base_path.join("ext4.vhdx"),
        base_path.join("LocalState\\ext4.vhdx"),
    ];

    for p in probe_paths {
        if p.exists() {
            return Some(p);
        }
    }

    // Fallback: search in base path
    if base_path.exists() {
        if let Ok(entries) = std::fs::read_dir(&base_path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() && entry.path().extension().map_or(false, |ext| ext == "vhdx") {
                        return Some(entry.path());
                    }
                }
            }
        }
    }

    None
}
