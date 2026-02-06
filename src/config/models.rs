use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Configuration file version constant
pub const SETTINGS_VERSION: u32 = 3;

// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub homepage: String,
    #[serde(rename = "app-version", alias = "version")]
    pub app_version: String,
    #[serde(rename = "setting-version", default)]
    pub setting_version: u8,
    #[serde(rename = "startup-time")]
    pub startup_time: String,
}

// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    #[serde(rename = "system-language")]
    pub system_language: String,
    #[serde(rename = "timezone")]
    pub timezone: String,
}

// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "modify-time", default)]
    pub modify_time: String,
    #[serde(rename = "check-time", default)]
    pub check_time: String,
    #[serde(rename = "check-update", default = "default_check_update")]
    pub check_update: u8,
    #[serde(rename = "distro-location")]
    pub distro_location: String,
    #[serde(rename = "logs-location")]
    pub logs_location: String,
    #[serde(rename = "temp-location", default)]
    pub temp_location: String,
    #[serde(rename = "ui-language")]
    pub ui_language: String,
    #[serde(rename = "auto-shutdown")]
    pub auto_shutdown: bool,
    #[serde(rename = "dark-mode", default)]
    pub dark_mode: bool,
    #[serde(rename = "log-level", default = "default_log_level")]
    pub log_level: u8,
    #[serde(rename = "log-days", default = "default_log_days")]
    pub log_days: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraySettings {
    #[serde(default)]
    pub autostart: bool,
    #[serde(rename = "start-minimized", default)]
    pub start_minimized: bool,
    #[serde(rename = "close-to-tray", default = "default_close_to_tray")]
    pub close_to_tray: bool,
}

pub fn default_close_to_tray() -> bool { true }

impl Default for TraySettings {
    fn default() -> Self {
        Self {
            autostart: false,
            start_minimized: false,
            close_to_tray: true,
        }
    }
}

pub fn default_log_level() -> u8 { 3 }
pub fn default_log_days() -> u8 { 7 }
pub fn default_check_update() -> u8 { 7 }

// Complete configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub application: ApplicationConfig,
    pub system: SystemConfig,
    pub settings: UserSettings,
    #[serde(default)]
    pub tray: TraySettings,
}

impl Config {
    // Get default distribution installation path (prefer D drive)
    pub fn get_default_distro_location() -> String {
        if std::path::Path::new("D:\\").exists() {
            "D:\\linux".to_string()
        } else {
            "C:\\linux".to_string()
        }
    }

    // Create default configuration
    pub fn default() -> Self {
        let home_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .to_string_lossy()
            .to_string();
        
        Self {
            application: ApplicationConfig {
                name: crate::app::APP_NAME.to_string(),
                homepage: crate::app::GITHUB_URL.to_string(),
                app_version: env!("CARGO_PKG_VERSION").to_string(),
                setting_version: SETTINGS_VERSION as u8,
                startup_time: chrono::Utc::now().timestamp_millis().to_string(),
            },
            system: SystemConfig {
                system_language: String::new(),
                timezone: String::new(),
            },
            settings: UserSettings {
                modify_time: chrono::Utc::now().timestamp_millis().to_string(),
                check_time: "0".to_string(),
                check_update: 7,
                distro_location: Self::get_default_distro_location(),
                logs_location: format!("{}\\.wsldashboard\\logs", home_dir),
                temp_location: format!("{}\\.wsldashboard\\temp", home_dir),
                ui_language: "auto".to_string(),
                auto_shutdown: false,
                dark_mode: false,
                log_level: 3,
                log_days: 7,
            },
            tray: TraySettings::default(),
        }
    }
}

// --- Instance-specific configuration (instances.toml) ---

pub const INSTANCES_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceCommonConfig {
    #[serde(rename = "setting-version")]
    pub setting_version: u32,
    #[serde(rename = "modify-time")]
    pub modify_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistroInstanceConfig {
    #[serde(rename = "terminal-dir", default = "default_terminal_dir")]
    pub terminal_dir: String,
    #[serde(rename = "vscode-dir", default = "default_vscode_dir")]
    pub vscode_dir: String,
    #[serde(rename = "auto-startup", default)]
    pub auto_startup: bool,
    #[serde(rename = "startup-script", default)]
    pub startup_script: String,
}

pub fn default_terminal_dir() -> String { "~".to_string() }
pub fn default_vscode_dir() -> String { "/home".to_string() }

impl Default for DistroInstanceConfig {
    fn default() -> Self {
        Self {
            terminal_dir: default_terminal_dir(),
            vscode_dir: default_vscode_dir(),
            auto_startup: false,
            startup_script: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancesContainer {
    pub common: InstanceCommonConfig,
    pub instances: std::collections::HashMap<String, DistroInstanceConfig>,
}

impl InstancesContainer {
    pub fn new() -> Self {
        Self {
            common: InstanceCommonConfig {
                setting_version: INSTANCES_VERSION,
                modify_time: chrono::Utc::now().timestamp_millis().to_string(),
            },
            instances: std::collections::HashMap::new(),
        }
    }
}
