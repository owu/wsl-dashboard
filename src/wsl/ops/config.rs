// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use ini::Ini;
use tracing::{info, warn, error, trace};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::wsl::executor::WslCommandExecutor;

static VERSION_CACHE: Lazy<Mutex<Option<WslVersionMeta>>> = Lazy::new(|| Mutex::new(None));

// ===== Data Models =====

// Represents the full /etc/wsl.conf configuration
#[derive(Debug, Clone, Default)]
pub struct WslConf {
    pub automount: AutomountSection,
    pub network: NetworkSection,
    pub interop: InteropSection,
    pub user: UserSection,
    pub boot: BootSection,
    pub gpu: GpuSection,
    pub time: TimeSection,
}

#[derive(Debug, Clone)]
pub struct AutomountSection {
    pub enabled: Option<bool>,
    pub mount_fs_tab: Option<bool>,
    pub root: Option<String>,
    pub options: Option<String>,
}

impl Default for AutomountSection {
    fn default() -> Self {
        Self {
            enabled: None,
            mount_fs_tab: None,
            root: None,
            options: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkSection {
    pub generate_hosts: Option<bool>,
    pub generate_resolv_conf: Option<bool>,
    pub hostname: Option<String>,
}

impl Default for NetworkSection {
    fn default() -> Self {
        Self {
            generate_hosts: None,
            generate_resolv_conf: None,
            hostname: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InteropSection {
    pub enabled: Option<bool>,
    pub append_windows_path: Option<bool>,
}

impl Default for InteropSection {
    fn default() -> Self {
        Self {
            enabled: None,
            append_windows_path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserSection {
    pub default: Option<String>,
}

impl Default for UserSection {
    fn default() -> Self {
        Self { default: None }
    }
}

#[derive(Debug, Clone)]
pub struct BootSection {
    pub systemd: Option<bool>,
    pub command: Option<String>,
    pub protect_binfmt: Option<bool>,
}

impl Default for BootSection {
    fn default() -> Self {
        Self {
            systemd: None,
            command: None,
            protect_binfmt: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuSection {
    pub enabled: Option<bool>,
}

impl Default for GpuSection {
    fn default() -> Self {
        Self { enabled: None }
    }
}

#[derive(Debug, Clone)]
pub struct TimeSection {
    pub use_windows_timezone: Option<bool>,
}

impl Default for TimeSection {
    fn default() -> Self {
        Self { use_windows_timezone: None }
    }
}

// Version metadata returned to the frontend for UI display
#[derive(Debug, Clone)]
pub struct WslVersionMeta {
    pub version_string: String,
    pub boot_supported: bool,
    pub gpu_supported: bool,
    pub time_supported: bool,
    pub detection_failed: bool,
}

impl Default for WslVersionMeta {
    fn default() -> Self {
        Self {
            version_string: String::new(),
            boot_supported: true,
            gpu_supported: true,
            time_supported: true,
            detection_failed: false,
        }
    }
}

// Result of validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub user_error: Option<String>,
    pub command_error: Option<String>,
}

// ===== Parsing =====

fn parse_bool_opt(ini: &Ini, section: &str, key: &str) -> Option<bool> {
    ini.get_from(Some(section), key).map(|v| {
        v.eq_ignore_ascii_case("true") || v == "1" || v.eq_ignore_ascii_case("yes")
    })
}

fn parse_string_opt(ini: &Ini, section: &str, key: &str) -> Option<String> {
    ini.get_from(Some(section), key)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
}

// Parse /etc/wsl.conf content into WslConf struct
pub fn parse_wsl_conf(content: &str) -> WslConf {
    let ini = match Ini::load_from_str(content) {
        Ok(ini) => ini,
        Err(e) => {
            warn!("Failed to parse wsl.conf: {}", e);
            return WslConf::default();
        }
    };

    WslConf {
        automount: AutomountSection {
            enabled: parse_bool_opt(&ini, "automount", "enabled"),
            mount_fs_tab: parse_bool_opt(&ini, "automount", "mountFsTab"),
            root: parse_string_opt(&ini, "automount", "root"),
            options: parse_string_opt(&ini, "automount", "options"),
        },
        network: NetworkSection {
            generate_hosts: parse_bool_opt(&ini, "network", "generateHosts"),
            generate_resolv_conf: parse_bool_opt(&ini, "network", "generateResolvConf"),
            hostname: parse_string_opt(&ini, "network", "hostname"),
        },
        interop: InteropSection {
            enabled: parse_bool_opt(&ini, "interop", "enabled"),
            append_windows_path: parse_bool_opt(&ini, "interop", "appendWindowsPath"),
        },
        user: UserSection {
            default: parse_string_opt(&ini, "user", "default"),
        },
        boot: BootSection {
            systemd: parse_bool_opt(&ini, "boot", "systemd"),
            command: parse_string_opt(&ini, "boot", "command"),
            protect_binfmt: parse_bool_opt(&ini, "boot", "protectBinfmt"),
        },
        gpu: GpuSection {
            enabled: parse_bool_opt(&ini, "gpu", "enabled"),
        },
        time: TimeSection {
            use_windows_timezone: parse_bool_opt(&ini, "time", "useWindowsTimezone"),
        },
    }
}

// ===== Serialization =====

fn bool_to_ini(val: bool) -> &'static str {
    if val { "true" } else { "false" }
}

// Serialize WslConf to INI string, optionally filtering unsupported sections
pub fn serialize_wsl_conf(conf: &WslConf, version_meta: &WslVersionMeta) -> String {
    let mut ini = Ini::new();

    // [automount] - always supported
    if let Some(v) = conf.automount.enabled {
        ini.set_to(Some("automount"), "enabled".to_string(), bool_to_ini(v).to_string());
    }
    if let Some(v) = conf.automount.mount_fs_tab {
        ini.set_to(Some("automount"), "mountFsTab".to_string(), bool_to_ini(v).to_string());
    }
    if let Some(ref v) = conf.automount.root {
        if !v.is_empty() {
            ini.set_to(Some("automount"), "root".to_string(), v.clone());
        }
    }
    if let Some(ref v) = conf.automount.options {
        if !v.is_empty() {
            ini.set_to(Some("automount"), "options".to_string(), v.clone());
        }
    }

    // [network] - always supported
    if let Some(v) = conf.network.generate_hosts {
        ini.set_to(Some("network"), "generateHosts".to_string(), bool_to_ini(v).to_string());
    }
    if let Some(v) = conf.network.generate_resolv_conf {
        ini.set_to(Some("network"), "generateResolvConf".to_string(), bool_to_ini(v).to_string());
    }
    if let Some(ref v) = conf.network.hostname {
        if !v.is_empty() {
            ini.set_to(Some("network"), "hostname".to_string(), v.clone());
        }
    }

    // [interop] - always supported
    if let Some(v) = conf.interop.enabled {
        ini.set_to(Some("interop"), "enabled".to_string(), bool_to_ini(v).to_string());
    }
    if let Some(v) = conf.interop.append_windows_path {
        ini.set_to(Some("interop"), "appendWindowsPath".to_string(), bool_to_ini(v).to_string());
    }

    // [user] - always supported
    if let Some(ref v) = conf.user.default {
        if !v.is_empty() {
            ini.set_to(Some("user"), "default".to_string(), v.clone());
        }
    }

    // [boot] - requires WSL 0.67.6+
    if version_meta.boot_supported {
        // Note: systemd is read-only in UI, we preserve its original value
        if let Some(v) = conf.boot.systemd {
            ini.set_to(Some("boot"), "systemd".to_string(), bool_to_ini(v).to_string());
        }
        if let Some(ref v) = conf.boot.command {
            if !v.is_empty() {
                ini.set_to(Some("boot"), "command".to_string(), v.clone());
            }
        }
        if let Some(v) = conf.boot.protect_binfmt {
            ini.set_to(Some("boot"), "protectBinfmt".to_string(), bool_to_ini(v).to_string());
        }
    }

    // [gpu] - requires newer WSL
    if version_meta.gpu_supported {
        if let Some(v) = conf.gpu.enabled {
            ini.set_to(Some("gpu"), "enabled".to_string(), bool_to_ini(v).to_string());
        }
    }

    // [time] - requires newer WSL
    if version_meta.time_supported {
        if let Some(v) = conf.time.use_windows_timezone {
            ini.set_to(Some("time"), "useWindowsTimezone".to_string(), bool_to_ini(v).to_string());
        }
    }

    // Write to string
    let mut buf = Vec::new();
    ini.write_to(&mut buf).unwrap_or_default();
    let result = String::from_utf8(buf).unwrap_or_default();

    // Ensure LF line endings (no CRLF)
    result.replace("\r\n", "\n")
}

// ===== Core Operations =====

// Read /etc/wsl.conf from a distribution
pub async fn get_wsl_conf(executor: &WslCommandExecutor, distro_name: &str) -> WslConf {
    trace!("Reading wsl.conf for '{}'", distro_name);
    let result = executor.execute_command(&["-d", distro_name, "-e", "cat", "/etc/wsl.conf"]).await;

    if result.success && !result.output.trim().is_empty() {
        parse_wsl_conf(&result.output)
    } else {
        trace!("wsl.conf not found or empty for '{}', using defaults", distro_name);
        WslConf::default()
    }
}

// Validate the wsl.conf configuration
pub async fn validate_wsl_conf(
    executor: &WslCommandExecutor,
    distro_name: &str,
    conf: &WslConf,
) -> ValidationResult {
    let mut result = ValidationResult {
        success: true,
        user_error: None,
        command_error: None,
    };

    // Validate [user].default - check if user exists
    if let Some(ref username) = conf.user.default {
        if !username.is_empty() {
            let check = executor.execute_command(&["-d", distro_name, "-e", "id", "-u", username]).await;
            if !check.success {
                result.success = false;
                result.user_error = Some(format!("User '{}' does not exist", username));
            }
        }
    }

    // Validate [boot].command - check if command/path exists
    if let Some(ref command) = conf.boot.command {
        if !command.is_empty() {
            // Extract the first token (command name or path)
            let cmd_path = command.split_whitespace().next().unwrap_or("");
            if !cmd_path.is_empty() {
                // Check if file exists
                let check = executor.execute_command(&[
                    "-d", distro_name, "-u", "root", "-e", "test", "-e", cmd_path
                ]).await;
                if !check.success {
                    // Also try `command -v` for built-in commands
                    let check2 = executor.execute_command(&[
                        "-d", distro_name, "-u", "root", "-e", "sh", "-c",
                        &format!("command -v {}", cmd_path)
                    ]).await;
                    if !check2.success {
                        result.success = false;
                        result.command_error = Some(format!("Command '{}' not found", cmd_path));
                    }
                }
            }
        }
    }

    result
}

// Detect WSL version and determine feature support
pub async fn check_wsl_version_support(executor: &WslCommandExecutor) -> WslVersionMeta {
    {
        let cache = VERSION_CACHE.lock().unwrap();
        if let Some(meta) = &*cache {
            return meta.clone();
        }
    }

    info!("Checking WSL version support (Cache miss)");
    let result = executor.execute_command(&["--version"]).await;

    if !result.success {
        warn!("Failed to run 'wsl --version'");
        return WslVersionMeta {
            version_string: String::new(),
            boot_supported: false,
            gpu_supported: false,
            time_supported: false,
            detection_failed: true,
        };
    }

    let output = result.output.trim().to_string();

    // Parse the WSL version line: "WSL version: x.y.z.w"
    let version_string = output.lines()
        .find(|line| {
            let lower = line.to_lowercase();
            // Look for lines that contain 'wsl' but NOT 'wslg' or 'kernel' or 'windows' or 'direct3d' etc.
            // The goal is to find the line that explicitly describes the WSL version itself.
            lower.contains("wsl") 
                && !lower.contains("wslg") 
                && !lower.contains("kernel")
                && !lower.contains("windows")
                && !lower.contains("direct3d")
                && !lower.contains("dxcore")
                && !lower.contains("msrdc")
        })
        .and_then(|line| {
            // Find the token that looks like a version number (contains dots and digits)
            line.split(|c: char| !c.is_ascii_digit() && c != '.')
                .find(|token| {
                    token.contains('.') && token.chars().all(|c| c.is_ascii_digit() || c == '.')
                        && token.split('.').all(|part| !part.is_empty())
                })
                .map(|s| s.to_string())
        })
        .unwrap_or_default();

    if version_string.is_empty() {
        warn!("Could not parse WSL version from output: {}", output);
        return WslVersionMeta {
            version_string: output,
            boot_supported: false,
            gpu_supported: false,
            time_supported: false,
            detection_failed: true,
        };
    }

    info!("Detected WSL version: {}", version_string);

    // Parse version components for comparison
    let parts: Vec<u64> = version_string
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    // Compare version: major.minor.patch
    let (major, minor, patch) = (
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    );

    // [boot] requires WSL 0.67.6+ (but modern WSL is 2.x.x)
    // 0.67.6 -> (0, 67, 6)
    // 2.0.0+ is obviously newer
    let boot_supported = if major >= 2 {
        true
    } else if major == 0 {
        if minor > 67 {
            true
        } else if minor == 67 {
            patch >= 6
        } else {
            false
        }
    } else {
        // major == 1, which shouldn't happen in practice
        true
    };

    // [gpu] and [time] require relatively modern WSL (2.0.0+)
    let gpu_supported = major >= 1;
    let time_supported = gpu_supported;

    let meta = WslVersionMeta {
        version_string,
        boot_supported,
        gpu_supported,
        time_supported,
        detection_failed: false,
    };

    let mut cache = VERSION_CACHE.lock().unwrap();
    *cache = Some(meta.clone());
    meta
}

// Save wsl.conf to a distribution
pub async fn save_wsl_conf(
    executor: &WslCommandExecutor,
    distro_name: &str,
    conf: &WslConf,
    version_meta: &WslVersionMeta,
) -> Result<(), String> {
    info!("Saving wsl.conf for '{}'", distro_name);

    // 1. Create backup
    let backup_result = executor.execute_command(&[
        "-d", distro_name, "-u", "root", "-e", "sh", "-c",
        "[ -f /etc/wsl.conf ] && cp /etc/wsl.conf /etc/wsl.conf.bak || true"
    ]).await;

    if !backup_result.success {
        warn!("Failed to create backup of wsl.conf for '{}': {:?}", distro_name, backup_result.error);
        // Continue anyway - backup failure shouldn't block save
    }

    // 2. Serialize the configuration (filter unsupported sections)
    let content = serialize_wsl_conf(conf, version_meta);

    // 3. Write to file using heredoc

    let write_cmd = format!(
        "cat << 'WSLCONF_EOF' > /etc/wsl.conf\n{}\nWSLCONF_EOF",
        content.trim_end()
    );

    let write_result = executor.execute_command(&[
        "-d", distro_name, "-u", "root", "-e", "sh", "-c", &write_cmd
    ]).await;

    if !write_result.success {
        let err = write_result.error.unwrap_or_else(|| "Unknown error".to_string());
        error!("Failed to save wsl.conf for '{}': {}", distro_name, err);
        return Err(err);
    }

    info!("Successfully saved wsl.conf for '{}'", distro_name);
    Ok(())
}
