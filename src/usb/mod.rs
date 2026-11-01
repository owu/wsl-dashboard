// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::{info, trace};

pub mod detail;
pub mod vendor_ids;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsbDeviceModel {
    #[serde(default, rename = "BusId")]
    pub bus_id: Option<String>,
    #[serde(default, rename = "Vid")]
    pub vid: Option<String>,
    #[serde(default, rename = "Pid")]
    pub pid: Option<String>,
    #[serde(default, rename = "InstanceId")]
    pub instance_id: Option<String>,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default = "default_state", rename = "State")]
    pub state: String, 
    #[serde(default, rename = "PersistedGuid")]
    pub persisted_guid: Option<String>,
    #[serde(default, rename = "ClientIPAddress")]
    pub client_ip_address: Option<String>,
    #[serde(default, rename = "StubInstanceId")]
    pub stub_instance_id: Option<String>,
    #[serde(default, rename = "IsForced")]
    pub is_forced: bool,
}

fn default_state() -> String {
    "Not shared".to_string()
}

#[derive(Debug, Deserialize)]
pub struct UsbStateResponse {
    #[serde(rename = "Devices")]
    pub devices: Vec<UsbDeviceModel>,
}

pub struct UsbManager;
impl UsbManager {
    // Get the usbipd-win version
    pub async fn get_version() -> Result<String, String> {
        let mut cmd = Command::new("usbipd");
        cmd.arg("--version");
        #[cfg(windows)]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        
        trace!("Executing command: usbipd --version");
        // Use a fixed internal error key instead of localized OS error messages
        let output = cmd.output().map_err(|_| "cmd_not_found".to_string())?;

        if !output.status.success() {
            return Err("cmd_not_found".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        // Check if output looks like a version (contains digits) as per user suggestion
        if !stdout.chars().any(|c| c.is_ascii_digit()) {
            return Err("cmd_not_found".to_string());
        }

        // Extract the main version number, e.g. "5.3.0-54+Branch..." -> "5.3.0"
        let version = stdout.split(|c: char| !c.is_ascii_digit() && c != '.')
            .next()
            .unwrap_or(&stdout)
            .to_string();
            
        Ok(version)
    }

    // Get the device list using 'usbipd state' (JSON)
    pub async fn list_devices() -> Result<Vec<UsbDeviceModel>, String> {
        let mut cmd = Command::new("usbipd");
        cmd.arg("state");
        #[cfg(windows)]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        trace!("Executing command: usbipd state");

        let output = cmd.output().map_err(|_| "cmd_not_found".to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if let Some(start) = stdout.find(|c| c == '{' || c == '[') {
            let end = stdout.rfind(|c| c == '}' || c == ']').map(|i| i + 1).unwrap_or(stdout.len());
            let json_part = &stdout[start..end];
            
            if json_part.starts_with('{') {
                match serde_json::from_str::<UsbStateResponse>(json_part) {
                    Ok(res) => {
                        trace!("Successfully parsed USB state JSON ({} devices)", res.devices.len());
                        return Ok(res.devices);
                    }
                    Err(e) => {
                        return Err(format!("USB JSON parse error: {}", e));
                    }
                }
            } else if json_part.starts_with('[') {
                match serde_json::from_str::<Vec<UsbDeviceModel>>(json_part) {
                    Ok(devices) => {
                        trace!("Successfully parsed USB list JSON ({} devices)", devices.len());
                        return Ok(devices);
                    }
                    Err(e) => {
                        return Err(format!("USB JSON array parse error: {}", e));
                    }
                }
            }
        }

        Err("program not found: No valid JSON output from 'usbipd state'".to_string())
    }

    // Perform the bind operation (directly with elevation as it always requires it)
    pub async fn bind(bus_id: &str, force: bool) -> Result<(), String> {
        info!("Binding device with elevation: {} (force={})", bus_id, force);
        let mut args = vec!["bind".to_string(), "--busid".to_string(), bus_id.to_string()];
        if force {
            args.push("--force".to_string());
        }
        crate::utils::system::run_command_with_elevation("usbipd", args)
    }

    // Perform the unbind operation (directly with elevation as it always requires it)
    pub async fn unbind(bus_id: &str) -> Result<(), String> {
        info!("Unbinding device with elevation: {}", bus_id);
        crate::utils::system::run_command_with_elevation("usbipd", vec!["unbind".to_string(), "--busid".to_string(), bus_id.to_string()])
    }

    // Perform the attach operation (directly with elevation)
    // This now includes an implicit 'bind' step to support "Not Shared" -> "Attached" in one click.
    pub async fn attach(bus_id: &str, distro: &str, force: bool, auto_attach: bool, is_usbipd_outdated: bool) -> Result<(), String> {
        info!("Attaching device {} to distro {} (with implicit bind check, force={}, auto_attach={}, is_usbipd_outdated={})", bus_id, distro, force, auto_attach, is_usbipd_outdated);

        // Pre-check: Ensure at least one WSL 2 distribution is running.
        // usbipd attach requires a running WSL 2 instance to work.
        let is_running = {
            let mut cmd = Command::new("wsl");
            cmd.args(["-l", "-v"]);
            #[cfg(windows)]
            {
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            cmd.env("WSL_UTF8", "1");
            
            match cmd.output() {
                Ok(out) => {
                    let stdout = crate::wsl::decoder::decode_output(&out.stdout);
                    stdout.lines()
                        .skip(1) // Skip header
                        .any(|line| {
                            let lower = line.to_lowercase();
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            // Must be Running AND Version 2
                            lower.contains("running") && parts.iter().any(|&p| p == "2")
                        })
                }
                Err(_) => false,
            }
        };

        if !is_running {
            return Err("no_wsl2_running".to_string());
        }
        
        // Chain bind and attach so it works even if the device is currently "Not Shared"
        // We use 'cmd /c' to run both commands under a single UAC prompt.
        let force_flag = if force { " --force" } else { "" };
        // Version compatibility: usbipd-win < 4.0.0 does not support --auto-attach, silently downgrade
        let auto_attach_flag = if auto_attach && !is_usbipd_outdated { " --auto-attach" } else { "" };
        let display_command = if distro.is_empty() {
            format!("usbipd bind --busid {0}{1} & usbipd attach --wsl --busid {0}{2}", bus_id, force_flag, auto_attach_flag)
        } else {
            format!("usbipd bind --busid {0}{1} & usbipd attach --wsl \"{2}\" --busid {0}{3}", bus_id, force_flag, distro, auto_attach_flag)
        };

        // Capture output to a temp file so we can detect errors
        let output_file = std::env::temp_dir().join(format!("wsld_usbipd_attach_{}.log", bus_id.replace('-', "_")));
        let output_path = output_file.to_string_lossy().to_string();

        // Redirect stdout+stderr to the temp file
        let cmd_with_output = format!("{} > \"{}\" 2>&1", display_command, output_path);

        // NOTE: do NOT short-circuit on the elevation result here. `usbipd attach`
        // reports its failure via the redirected output file AND a non-zero exit
        // code, so propagating the exit-code error first would discard the captured
        // output and the user-facing "attach failed" hint would never be shown.
        let elevation_result = crate::utils::system::run_command_with_elevation("cmd", vec!["/c".to_string(), cmd_with_output]);

        // Read the captured output
        let captured = std::fs::read_to_string(&output_file).unwrap_or_default();
        let _ = std::fs::remove_file(&output_file);

        // Check if the output indicates an error (case-insensitive)
        if captured.to_lowercase().contains("error") {
            return Err(format!(
                "usb_attach_failed:COMMAND:{}\n\nOUTPUT:{}",
                display_command, captured
            ));
        }

        // No error text captured: surface the elevation/exit-code failure (if any),
        // e.g. UAC denied or a non-zero exit code without a recognizable message.
        if let Err(e) = elevation_result {
            return Err(e);
        }

        Ok(())
    }

    // Perform the detach operation
    pub async fn detach(bus_id: &str) -> Result<(), String> {
        info!("Attempting to detach device: {}", bus_id);
        
        let mut cmd = Command::new("usbipd");
        cmd.args(["detach", "--busid", bus_id]);
        #[cfg(windows)]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        trace!("Executing command: usbipd detach --busid {}", bus_id);
        let output = cmd.output()
            .map_err(|e| format!("Failed to execute detach: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Detach failed: {}", stderr));
        }

        Ok(())
    }

    // Get device detailed info from SetupAPI
    pub async fn get_device_detail(instance_id: &str) -> Result<Option<detail::UsbDeviceDetail>, String> {
        detail::get_device_detail(instance_id).await
    }
}
