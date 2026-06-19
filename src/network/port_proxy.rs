// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tracing::{info, error};
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::collections::HashSet;
use super::models::PortProxyRule;
use super::tracker::{get_distro_ip, is_distro_running};

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn execute_netsh(args: &[&str]) -> Result<String, String> {
    let output = Command::new("netsh")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to execute netsh: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if output.status.success() {
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Netsh error ({}): {}", output.status, stderr))
    }
}

pub fn get_active_listen_ports() -> Result<HashSet<(String, u16)>, String> {
    let output = execute_netsh(&["interface", "portproxy", "show", "all"])?;
    let mut active_sets = HashSet::new();

    let mut in_data_section = false;
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // Detect separator line (e.g., "--------------- ----------  --------------- ----------")
        // This works regardless of the language of the headers
        if line.chars().all(|c| c == '-' || c == ' ') && line.contains('-') {
            in_data_section = true;
            continue;
        }

        if in_data_section {
            // Expected line: "0.0.0.0         3306        172.30.12.166   3306"
            // Parse the first two whitespace-separated parts as address and port
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let addr = parts[0].to_string();
                if let Ok(port) = parts[1].parse::<u16>() {
                    active_sets.insert((addr, port));
                }
            }
        }
    }
    
    Ok(active_sets)
}


pub fn add_port_proxy(listen_addr: &str, listen_port: u16, connect_addr: &str, connect_port: u16) -> Result<(), String> {
    let port_str = listen_port.to_string();
    let conn_port_str = connect_port.to_string();

    info!("Executing PortProxy Mapping: {}:{} -> {}:{}", listen_addr, listen_port, connect_addr, connect_port);

    execute_netsh(&[
        "interface", "portproxy", "add", "v4tov4",
        &format!("listenaddress={}", listen_addr),
        &format!("listenport={}", port_str),
        &format!("connectaddress={}", connect_addr),
        &format!("connectport={}", conn_port_str),
    ])?;
    Ok(())
}

pub fn delete_port_proxy(listen_addr: &str, listen_port: u16) -> Result<(), String> {
    let port_str = listen_port.to_string();
    // Ignore deletion failures as the rule might not exist
    let _ = execute_netsh(&[
        "interface", "portproxy", "delete", "v4tov4",
        &format!("listenaddress={}", listen_addr),
        &format!("listenport={}", port_str),
    ]);
    Ok(())
}

// Elevated version of add_port_proxy (triggers UAC)
pub fn add_port_proxy_elevated(listen_addr: &str, listen_port: u16, connect_addr: &str, connect_port: u16, _enable_firewall: bool, _distro_name: &str) -> Result<(), String> {
    info!("Requesting elevation for netsh add portproxy (Direct: {}:{} -> {}:{})", listen_addr, listen_port, connect_addr, connect_port);
    let full_command = format!(
        "netsh interface portproxy add v4tov4 listenaddress={} listenport={} connectaddress={} connectport={}",
        listen_addr, listen_port, connect_addr, connect_port
    );
    crate::utils::system::run_invisible_elevated_command(&full_command)
}

// Elevated version of add_port_proxy (Single cmd /c line, combined with optional firewall)
pub fn add_port_proxy_and_firewall_elevated(
    listen_addr: &str, listen_port: u16, 
    connect_addr: &str, connect_port: u16, 
    enable_firewall: bool, distro_name: &str
) -> Result<(), String> {
    let rule_name = format!("WSL_Dashboard_{}_{}", distro_name, listen_port);
    let mut commands = Vec::new();

    // 1. Add portproxy
    commands.push(format!(
        "netsh interface portproxy add v4tov4 listenaddress={} listenport={} connectaddress={} connectport={}",
        listen_addr, listen_port, connect_addr, connect_port
    ));

    // 2. Firewall (if enabled)
    if enable_firewall {
        commands.push(super::firewall::get_delete_rule_cmd_netsh(&rule_name));
        commands.push(super::firewall::get_add_rule_cmd_netsh(&rule_name, listen_port, listen_addr));
    }

    crate::utils::system::run_invisible_elevated_commands(commands)
}

// Elevated version of adding firewall rule ONLY (used when distro is not started)
pub fn add_firewall_rule_elevated(listen_addr: &str, listen_port: u16, distro_name: &str) -> Result<(), String> {
    let rule_name = format!("WSL_Dashboard_{}_{}", distro_name, listen_port);
    let mut commands = Vec::new();
    commands.push(super::firewall::get_delete_rule_cmd_netsh(&rule_name));
    commands.push(super::firewall::get_add_rule_cmd_netsh(&rule_name, listen_port, listen_addr));
    
    crate::utils::system::run_invisible_elevated_commands(commands)
}


// Elevated version of delete_port_proxy (triggers UAC) - NO firewall
pub fn delete_port_proxy_elevated(listen_addr: &str, listen_port: u16, _distro_name: &str) -> Result<(), String> {
    info!("Requesting elevation to delete portproxy (Direct): {}:{}", listen_addr, listen_port);
    let full_command = format!(
        "netsh interface portproxy delete v4tov4 listenaddress={} listenport={}",
        listen_addr, listen_port
    );
    crate::utils::system::run_invisible_elevated_command(&full_command)
}

// Elevated version of delete_port_proxy AND firewall (Physical deletion)
pub fn delete_port_proxy_and_firewall_elevated(listen_addr: &str, listen_port: u16, distro_name: &str) -> Result<(), String> {
    let rule_name = format!("WSL_Dashboard_{}_{}", distro_name, listen_port);
    let mut commands = Vec::new();
    commands.push(format!("netsh interface portproxy delete v4tov4 listenaddress={} listenport={}", listen_addr, listen_port));
    commands.push(super::firewall::get_delete_rule_cmd_netsh(&rule_name));
    
    crate::utils::system::run_invisible_elevated_commands(commands)
}

// Elevated version of applying multiple rules at once (Single UAC) - NO firewall
pub fn apply_port_proxies_elevated(rules_with_ips: Vec<(PortProxyRule, String)>) -> Result<(), String> {
    if rules_with_ips.is_empty() { return Ok(()); }
    
    info!("Batch applying {} port proxy rules via elevation", rules_with_ips.len());
    let mut all_cmd_parts = Vec::new();
    for (rule, target_ip) in rules_with_ips {
        // Add portproxy only
        all_cmd_parts.push(format!("netsh interface portproxy add v4tov4 listenaddress={} listenport={} connectaddress={} connectport={}", 
            rule.listen_address, rule.listen_port, target_ip, rule.target_port));
    }
    
    crate::utils::system::run_invisible_elevated_commands(all_cmd_parts)
}

// Elevated version of deleting multiple rules at once (Single UAC) - NO firewall
pub fn delete_port_proxies_elevated(rules: Vec<PortProxyRule>) -> Result<(), String> {
    if rules.is_empty() { return Ok(()); }
    
    info!("Batch deleting {} port proxy rules via elevation", rules.len());
    let mut all_cmd_parts = Vec::new();
    for rule in rules {
        all_cmd_parts.push(format!("netsh interface portproxy delete v4tov4 listenaddress={} listenport={}", 
            rule.listen_address, rule.listen_port));
    }
    
    crate::utils::system::run_invisible_elevated_commands(all_cmd_parts)
}

pub struct SyncResult {}

// Execute the main port synchronization flow (PortProxySyncFlow).
// This call requires elevation and should only be executed by privileged tasks or elevated instances.
pub fn sync_port_proxies(distro_name: &str, rules: &[PortProxyRule]) -> Result<Vec<SyncResult>, String> {
    info!("Starting port proxy synchronization for: {}", distro_name);
    
    // 1. Check if the distribution is running
    let is_running = is_distro_running(distro_name);
    
    // 2. Obtain current IP if running
    let target_ip = if is_running {
        match get_distro_ip(distro_name) {
            Ok(ip) => Some(ip),
            Err(e) => {
                error!("Failed to obtain IP for running distro {}: {}", distro_name, e);
                None
            }
        }
    } else {
        info!("Distro '{}' is NOT running. Skipping rule application (Applying step), but will still clean up existing rules.", distro_name);
        None
    };
    
    if let Some(ref ip) = target_ip {
        info!("Obtained Target IP for {}: {}", distro_name, ip);
    }
    
    let mut results = Vec::new();

    // 3. Iterate and process rules
    for rule in rules {
        if rule.distro_name != distro_name {
            continue;
        }
        
        // 3.a Delete existing rules (if any) - Always executed
        info!("Step 1/2: Deleting existing rule (if any) for {}:{}", rule.listen_address, rule.listen_port);
        let _ = delete_port_proxy(&rule.listen_address, rule.listen_port);
        
        // 3.b Add new rule - Executed only if distro is running and IP is available
        if let Some(ref ip) = target_ip {
            info!("Step 2/2: Applying port proxy rule: {}:{} -> {}:{}", rule.listen_address, rule.listen_port, ip, rule.target_port);
            match add_port_proxy(&rule.listen_address, rule.listen_port, ip, rule.target_port) {
                Ok(_) => {
                    info!("Successfully applied rule: {}:{} -> {}:{}", rule.listen_address, rule.listen_port, ip, rule.target_port);
                    results.push(SyncResult {});
                },
                Err(e) => {
                    error!("CRITICAL: Failed to apply port proxy for {}: {}. Ensure the app is running with Administrator privileges.", rule.listen_port, e);
                    results.push(SyncResult {});
                }
            }
        } else {
            info!("Step 2/2: Skipping Applying step for {}:{} (Reason: Distro not running or IP unavailable)", rule.listen_address, rule.listen_port);
            results.push(SyncResult {});
        }
    }
    
    info!("Synchronization completed for {}. {} rules processed.", distro_name, results.len());
    Ok(results)
}
