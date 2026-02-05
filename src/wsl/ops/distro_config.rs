use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::{WslCommandResult, WslConf};
use tracing::{info, warn};

pub async fn get_wsl_conf(executor: &WslCommandExecutor, distro_name: &str) -> WslConf {
    // Read the entire file
    let result = executor.execute_command(&[
        "-d", distro_name, 
        "-u", "root", 
        "--", "cat", "/etc/wsl.conf"
    ]).await;

    let mut conf = WslConf {
        systemd: false,
        generate_hosts: true, // Default is true in WSL
        generate_resolv_conf: true, // Default is true
        interop_enabled: true, // Default is true
        append_windows_path: true, // Default is true
    };
    
    if result.success {
        let content = result.output;
        // Simple INI parsing
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() { continue; }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "systemd" => conf.systemd = value == "true",
                    "generateHosts" => conf.generate_hosts = value == "true",
                    "generateResolvConf" => conf.generate_resolv_conf = value == "true",
                    "enabled" => conf.interop_enabled = value == "true",
                    "appendWindowsPath" => conf.append_windows_path = value == "true",
                    _ => {}
                }
            }
        }
    }
    
    conf
}

pub async fn set_wsl_conf(executor: &WslCommandExecutor, distro_name: &str, conf: WslConf) -> WslCommandResult<String> {
    info!("Operation: Update wsl.conf for {}", distro_name);

    // We reconstruct the file content. 
    // Note: This approach overwrites existing custom comments/other settings not tracked here.
    // For a production app, a sed-based approach or full TOML parser preservation would be safer, 
    // but for this dashboard, ensuring the state matches UI is acceptable.
    let content = format!(
        "[boot]\nsystemd={}\n\n[network]\ngenerateHosts={}\ngenerateResolvConf={}\n\n[interop]\nenabled={}\nappendWindowsPath={}\n",
        conf.systemd,
        conf.generate_hosts,
        conf.generate_resolv_conf,
        conf.interop_enabled,
        conf.append_windows_path
    );

    let script = format!("printf '{}' > /etc/wsl.conf", content);

    executor.execute_command(&[
        "-d", distro_name,
        "-u", "root",
        "--", "sh", "-c", &script
    ]).await
}

// Keep legacy single-field updater for backward compat if needed, or remove. 
// We will focus on the full config object now.
pub async fn get_systemd_status(executor: &WslCommandExecutor, distro_name: &str) -> bool {
    let conf = get_wsl_conf(executor, distro_name).await;
    conf.systemd
}

pub async fn set_systemd_status(executor: &WslCommandExecutor, distro_name: &str, enabled: bool) -> WslCommandResult<String> {
    let mut conf = get_wsl_conf(executor, distro_name).await;
    conf.systemd = enabled;
    set_wsl_conf(executor, distro_name, conf).await
}
