// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use tracing::{info, error, debug};

mod migration;
pub mod instances;
pub mod models;
pub mod debug;
pub mod mount;

pub use models::*;
pub use debug::DebugConfig;

// Configuration manager, responsible for loading, saving, and managing application configuration
#[derive(Clone)]
pub struct ConfigManager {
    // Configuration file path
    config_path: PathBuf,
    // Current configuration data
    config: Config,
}

impl ConfigManager {
    // Get configuration file path
    fn get_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("settings.toml")
    }

    pub fn get_instances_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("instances.toml")
    }

    fn get_network_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("network.toml")
    }

    // Initialize configuration manager
    pub async fn new() -> Self {
        let config_path = Self::get_config_path();
        
        // Check if configuration file exists
        if config_path.exists() {
            info!("Configuration file exists, loading...");
            match Self::load_config(&config_path).await {
                Ok(mut config) => {
                    // Calculate time difference to determine if PowerShell call is needed to refresh [system] information (7-day threshold)
                    let now = chrono::Utc::now().timestamp_millis();
                    let last_modify = config.settings.modify_time.parse::<i64>().unwrap_or(0);
                    let should_refresh_system = (now - last_modify) >= 604_800_000;

                    // Check version and complete fields
                    Self::migrate_config(&mut config);

                    // NOTE: device-id generation happens in ensure_device_id(),
                    // called by bootstrap AFTER logging is initialized, so its
                    // logs are not dropped. Do not generate here.

                    // Refresh basic information (startup_time, version)
                    // Force refresh system information if more than 7 days or data is missing
                    let force_system = should_refresh_system || config.system.system_language.is_empty();
                    
                    Self::refresh_system_info(&mut config, force_system).await;
                    
                    // Ensure critical directories exist
                    let _ = fs::create_dir_all(&config.settings.distro_location);
                    let _ = fs::create_dir_all(&config.settings.logs_location);
                    let _ = fs::create_dir_all(&config.settings.temp_location);

                    // Migrate old USB config from settings.toml to usb.toml (one-time)
                    Self::migrate_usb_config();

                    // Save updated configuration (save_config automatically updates settings.modify_time)
                    if let Err(e) = Self::save_config(&config_path, &mut config) {
                        error!("Failed to save config: {}", e);
                    }
                    
                    Self {
                        config_path,
                        config,
                    }
                }
                Err(e) => {
                    error!("Failed to load configuration file: {}, using default configuration", e);
                    let config = Self::create_default_config().await;
                    Self {
                        config_path,
                        config,
                    }
                }
            }
        } else {
            info!("Configuration file does not exist, initializing...");
            let mut config = Self::create_default_config().await;
            
            // Create configuration directory
            if let Some(parent) = config_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create configuration directory: {}", e);
                } else {
                    // Ensure critical directories exist (according to user's current configuration)
                    let _ = fs::create_dir_all(&config.settings.distro_location);
                    let _ = fs::create_dir_all(&config.settings.logs_location);
                    let _ = fs::create_dir_all(&config.settings.temp_location);
                }
            }
            
            // Save configuration
            if let Err(e) = Self::save_config(&config_path, &mut config) {
                error!("Failed to save initial configuration: {}", e);
            } else {
                info!("Configuration file initialized successfully: {}", config_path.display());
            }
            
            Self {
                config_path,
                config,
            }
        }
    }

    // Create default configuration and populate system information
    async fn create_default_config() -> Config {
        let mut config = Config::default();
        Self::refresh_system_info(&mut config, true).await;
        // device-id is generated by ensure_device_id() after logging is initialized (see launcher::bootstrap)
        config
    }

    // Generate and persist device-id on first use; subsequent runs reuse the stored value.
    // Must be called AFTER tracing/logging is initialized, otherwise its logs are dropped.
    pub async fn ensure_device_id(&mut self) {
        if self.config.application.device_id.is_empty() {
            self.config.application.device_id = crate::utils::device_id::generate_device_id().await;
            info!("Generated device-id: {}", self.config.application.device_id);
            if let Err(e) = Self::save_config(&self.config_path, &mut self.config) {
                error!("Failed to save config after generating device-id: {}", e);
            }
        } else {
            debug!("Reusing existing device-id: {}", self.config.application.device_id);
        }
    }

    // Refresh system information fields
    async fn refresh_system_info(config: &mut Config, refresh_system: bool) {
        // Update startup time field
        config.application.startup_time = chrono::Utc::now().timestamp_millis().to_string();
        config.application.app_version = env!("CARGO_PKG_VERSION").to_string();
        
        if !refresh_system {
            info!("Skipping system environment query (less than 7 days since last update)");
            return;
        }

        info!("Refreshing system language and timezone information...");
        
        config.system.system_language = crate::utils::registry::get_system_locale();
        config.system.timezone = crate::utils::registry::get_system_timezone();
    }

    // Load configuration file
    async fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    // Save configuration file
    fn save_config(path: &PathBuf, config: &mut Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Update modify-time each time saving
        config.settings.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        let toml_string = toml::to_string_pretty(config)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    // Migrate configuration (version compatibility)
    fn migrate_config(config: &mut Config) {
        migration::migrate_config(config);
    }

    // Get configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    // Update user settings and save
    pub fn update_settings(&mut self, settings: UserSettings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Ensure new paths exist
        let _ = fs::create_dir_all(&settings.distro_location);
        let _ = fs::create_dir_all(&settings.logs_location);
        let _ = fs::create_dir_all(&settings.temp_location);

        self.config.settings = settings;
        self.config.application.setting_version = SETTINGS_VERSION as u8;
        
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Configuration saved successfully");
        Ok(())
    }

    // Get user settings
    pub fn get_settings(&self) -> &UserSettings {
        &self.config.settings
    }

    // Get tray settings
    pub fn get_tray_settings(&self) -> &TraySettings {
        &self.config.tray
    }

    // Update tray settings and save
    pub fn update_tray_settings(&mut self, tray: TraySettings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.config.tray = tray;
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Tray configuration saved successfully");
        Ok(())
    }

    // Update sidebar settings and save
    pub fn update_sidebar_settings(&mut self, sidebar: SidebarConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.config.sidebar = sidebar;
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Sidebar configuration saved successfully");
        Ok(())
    }

    // Update popup detection timestamp
    pub fn update_check_time(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.config.settings.check_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Updated check-time to: {}", self.config.settings.check_time);
        Ok(())
    }

    // --- Network Config Management ---
    
    fn load_network_config() -> NetworkConfig {
        let path = Self::get_network_config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                match toml::from_str::<NetworkConfig>(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        error!("Failed to parse network.toml, falling back to default: {}", e);
                    }
                }
            }
        }
        NetworkConfig::default()
    }

    fn save_network_config(network: &NetworkConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = Self::get_network_config_path();
        let toml_string = toml::to_string_pretty(network)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    pub fn get_network_config(&self) -> NetworkConfig {
        Self::load_network_config()
    }

    pub fn update_network_config(&self, mut network: NetworkConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        network.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        let rule_count = network.port_proxies.len();
        Self::save_network_config(&network)?;
        info!("Network configuration ({} rules) saved successfully to network.toml", rule_count);
        Ok(())
    }

    // --- Instances Config Management ---

    fn load_instances() -> InstancesContainer {
        instances::load_instances(&Self::get_instances_path())
    }

    fn save_instances_to_disk(path: &std::path::Path, container: &InstancesContainer) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        instances::save_instances_to_disk(path, container)
    }

    pub fn get_instance_config(&self, distro_name: &str) -> DistroInstanceConfig {
        let mut container = Self::load_instances();
        if let Some(config) = container.instances.get(distro_name) {
            config.clone()
        } else {
            // Initialize with default if not found
            let default_config = DistroInstanceConfig::default();
            container.instances.insert(distro_name.to_string(), default_config.clone());
            // Save immediately as requested
            let _ = self.update_instance_config(distro_name, default_config.clone());
            default_config
        }
    }

    pub fn update_instance_config(&self, distro_name: &str, config: DistroInstanceConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut container = Self::load_instances();
        container.instances.insert(distro_name.to_string(), config);
        container.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        container.common.setting_version = INSTANCES_VERSION;

        let path = Self::get_instances_path();
        Self::save_instances_to_disk(&path, &container)?;
        info!("Instance configuration for '{}' saved successfully", distro_name);
        Ok(())
    }

    pub fn remove_instance_config(&self, distro_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut container = Self::load_instances();
        if container.instances.remove(distro_name).is_some() {
            container.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
            let path = Self::get_instances_path();
            Self::save_instances_to_disk(&path, &container)?;
            info!("Removed instance configuration for '{}'", distro_name);
        }
        Ok(())
    }

    pub fn get_cached_distros(&self) -> Vec<CachedDistro> {
        let container = Self::load_instances();
        container.last_distros
    }

    pub fn update_cached_distros(&self, distros: Vec<CachedDistro>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut container = Self::load_instances();
        container.last_distros = distros;
        container.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        let path = Self::get_instances_path();
        Self::save_instances_to_disk(&path, &container)?;
        Ok(())
    }

    // --- USB Config Management (usb.toml) ---

    fn get_usb_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("usb.toml")
    }

    fn load_usb_config() -> UsbConfigFile {
        let path = Self::get_usb_config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<UsbConfigFile>(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        error!("Failed to parse usb.toml ({}), backing up and creating default", e);
                        let backup_path = path.with_extension("toml.bak");
                        let _ = fs::rename(&path, &backup_path);
                    }
                },
                Err(e) => error!("Failed to read usb.toml: {}", e),
            }
        }
        UsbConfigFile::default()
    }

    fn save_usb_config(config: &UsbConfigFile) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = Self::get_usb_config_path();
        let toml_string = toml::to_string_pretty(config)?;
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml_string)?;
        Ok(())
    }

    pub fn get_usb_config(&self) -> UsbConfigFile {
        Self::load_usb_config()
    }

    // Toggle `boot-attach` for a device identified by bus_id.
    // If the device doesn't exist in config, create a new entry.
    pub fn toggle_usb_boot_attach(&mut self, bus_id: &str, vid_pid: &str, distro: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut config = Self::load_usb_config();
        let entry = config.usb.entry(vid_pid.to_string()).or_insert(UsbDeviceConfig {
            boot_attach: false,
            auto_attach: false,
            force_bind: false,
            bus_id: bus_id.to_string(),
            vid_pid: vid_pid.to_string(),
            distribution: distro.to_string(),
        });
        entry.bus_id = bus_id.to_string();
        entry.distribution = distro.to_string();
        entry.boot_attach = !entry.boot_attach;
        let new_state = entry.boot_attach;
        config.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_usb_config(&config)?;
        Ok(new_state)
    }

    // Set boot-attach for a device identified by vid_pid.
    pub fn set_usb_boot_attach(&mut self, vid_pid: &str, bus_id: &str, enabled: bool) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut config = Self::load_usb_config();
        let entry = config.usb.entry(vid_pid.to_string()).or_insert(UsbDeviceConfig {
            boot_attach: false,
            auto_attach: false,
            force_bind: false,
            bus_id: bus_id.to_string(),
            vid_pid: vid_pid.to_string(),
            distribution: String::new(),
        });
        entry.boot_attach = enabled;
        // Update bus_id if provided (non-empty), to keep config in sync with device listing
        if !bus_id.is_empty() {
            entry.bus_id = bus_id.to_string();
        }
        entry.vid_pid = vid_pid.to_string();
        config.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_usb_config(&config)?;
        Ok(enabled)
    }

    // Set auto-attach for a device identified by vid_pid.
    pub fn set_usb_auto_attach(&mut self, vid_pid: &str, enabled: bool) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut config = Self::load_usb_config();
        let entry = config.usb.entry(vid_pid.to_string()).or_insert(UsbDeviceConfig {
            boot_attach: false,
            auto_attach: false,
            force_bind: false,
            bus_id: String::new(),
            vid_pid: vid_pid.to_string(),
            distribution: String::new(),
        });
        entry.auto_attach = enabled;
        config.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_usb_config(&config)?;
        Ok(enabled)
    }

    // Toggle force-bind for a device identified by vid_pid.
    pub fn toggle_force_bind(&mut self, vid_pid: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut config = Self::load_usb_config();
        let entry = config.usb.entry(vid_pid.to_string()).or_insert(UsbDeviceConfig {
            boot_attach: false,
            auto_attach: false,
            force_bind: false,
            bus_id: String::new(),
            vid_pid: vid_pid.to_string(),
            distribution: String::new(),
        });
        entry.force_bind = !entry.force_bind;
        let new_state = entry.force_bind;
        config.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_usb_config(&config)?;
        Ok(new_state)
    }

    // Check if a device (by VID:PID) has force-bind enabled.
    pub fn is_force_bind(&self, vid_pid: &str) -> bool {
        let config = Self::load_usb_config();
        config.usb.get(vid_pid).map(|d| d.force_bind).unwrap_or(false)
    }

    // Get all devices with boot-attach enabled (for scheduler).
    pub fn get_usb_boot_attach_devices(&self) -> Vec<UsbDeviceConfig> {
        let config = Self::load_usb_config();
        config.usb.values()
            .filter(|d| d.boot_attach)
            .cloned()
            .collect()
    }

    // Migrate old settings.toml USB config to usb.toml (one-time, only if usb.toml doesn't exist).
    pub fn migrate_usb_config() {
        let usb_path = Self::get_usb_config_path();
        if usb_path.exists() {
            return;
        }
        let settings_path = Self::get_config_path();
        if !settings_path.exists() {
            return;
        }
        // Try to read old USB config from settings.toml
        // We need to use a temporary struct that can deserialize the old format
        #[derive(Deserialize)]
        struct OldUsbDevice {
            #[serde(rename = "bus-id")]
            bus_id: String,
            #[serde(rename = "vid-pid")]
            vid_pid: String,
            distribution: String,
        }
        #[derive(Deserialize)]
        struct OldUsbSection {
            #[serde(rename = "auto-attach-list", default)]
            auto_attach_list: Vec<OldUsbDevice>,
        }
        #[derive(Deserialize)]
        struct OldConfig {
            usb: Option<OldUsbSection>,
        }

        let content = match fs::read_to_string(&settings_path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let old: OldConfig = match toml::from_str(&content) {
            Ok(c) => c,
            Err(_) => return,
        };
        let old_usb = match old.usb {
            Some(u) => u,
            None => return,
        };
        if old_usb.auto_attach_list.is_empty() {
            return;
        }

        let mut new_config = UsbConfigFile::default();
        for device in &old_usb.auto_attach_list {
            new_config.usb.insert(device.vid_pid.clone(), UsbDeviceConfig {
                boot_attach: true,
                auto_attach: false,
                force_bind: false,
                bus_id: device.bus_id.clone(),
                vid_pid: device.vid_pid.clone(),
                distribution: device.distribution.clone(),
            });
        }
        if let Err(e) = Self::save_usb_config(&new_config) {
            error!("Failed to migrate USB config to usb.toml: {}", e);
        } else {
            info!("Migrated {} USB auto-attach devices from settings.toml to usb.toml", new_config.usb.len());
        }
    }

    // --- Debug Config ---

    // Load `~/.wsldashboard/debug.toml` (best-effort, never panics).
    pub fn load_debug_config() -> DebugConfig {
        DebugConfig::load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, DateTime};

    // ── P0: serde / serde_json round-trip ──

    #[test]
    fn test_config_toml_roundtrip() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        // Verify top-level fields preserved
        assert_eq!(config.application.app_version, deserialized.application.app_version);
        assert_eq!(config.application.startup_time, deserialized.application.startup_time);
        assert_eq!(config.settings.distro_location, deserialized.settings.distro_location);
        assert_eq!(config.settings.logs_location, deserialized.settings.logs_location);
        assert_eq!(config.settings.temp_location, deserialized.settings.temp_location);
    }

    #[test]
    fn test_config_json_roundtrip() {
        let config = Config::default();
        let json_str = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json_str).unwrap();

        assert_eq!(config.application.app_version, deserialized.application.app_version);
        assert_eq!(config.settings.modify_time, deserialized.settings.modify_time);
    }

    #[test]
    fn test_user_settings_serde_roundtrip() {
        let settings = UserSettings {
            modify_time: "1712345678000".to_string(),
            check_time: "1712345678000".to_string(),
            check_update: 7,
            distro_location: "D:\\linux".to_string(),
            backup_location: "D:\\linux".to_string(),
            logs_location: "C:\\Users\\test\\.wsldashboard\\logs".to_string(),
            temp_location: "C:\\Users\\test\\.wsldashboard\\temp".to_string(),
            ui_language: "zh-CN".to_string(),
            auto_shutdown: true,
            system_color: false,
            dark_mode: true,
            sidebar_collapsed: false,
            log_level: 4,
            log_days: 7,
            colorful_icons: true,
            mail: true,
            hide_pin: false,
            show_drag: true,
            mount_auto_probe: false,
        };

        // Verify TOML round-trip with serde
        let toml_str = toml::to_string_pretty(&settings).unwrap();
        let deserialized: UserSettings = toml::from_str(&toml_str).unwrap();
        assert_eq!(settings.distro_location, deserialized.distro_location);
        assert_eq!(settings.ui_language, deserialized.ui_language);
        assert_eq!(settings.log_level, deserialized.log_level);
        assert_eq!(settings.auto_shutdown, deserialized.auto_shutdown);
        assert_eq!(settings.dark_mode, deserialized.dark_mode);
    }

    // ── P0: chrono timestamp handling ──

    #[test]
    fn test_timestamp_millis_roundtrip() {
        let now_ms = Utc::now().timestamp_millis();
        let dt = DateTime::from_timestamp_millis(now_ms).unwrap();
        assert_eq!(dt.timestamp_millis(), now_ms);
    }

    #[test]
    fn test_config_modify_time_parsing() {
        let time_str = "1712345678000";
        let parsed = time_str.parse::<i64>().unwrap();
        let dt = DateTime::from_timestamp_millis(parsed);
        assert!(dt.is_some());
        assert_eq!(dt.unwrap().timestamp_millis(), 1712345678000);
    }

    #[test]
    fn test_empty_modify_time() {
        // Edge case: empty string should default to 0, which represents Unix epoch
        let parsed = "".parse::<i64>().unwrap_or(0);
        assert_eq!(parsed, 0);
    }

    // ── P0: toml Value API ──

    #[test]
    fn test_toml_value_access() {
        let content = r#"
[package]
name = "wsldashboard"
[package.metadata]
expire = 1234567890
"#;
        let value: toml::Value = toml::from_str(content).unwrap();
        let expire = value.get("package")
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.get("expire"))
            .and_then(|e| e.as_integer());
        assert_eq!(expire, Some(1234567890));
    }

    #[test]
    fn test_toml_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let value: toml::Value = toml::from_str(&toml_str).unwrap();

        // Verify structure via toml::Value
        assert!(value.get("application").is_some());
        assert!(value.get("system").is_some());
        assert!(value.get("settings").is_some());

        // Check settings fields
        let settings = value.get("settings").unwrap();
        assert!(settings.get("distro-location").is_some());
        assert!(settings.get("logs-location").is_some());
        assert!(settings.get("modify-time").is_some());
    }

    // ── P1: dirs path format ──

    #[test]
    fn test_home_dir_format() {
        let home = dirs::home_dir();
        assert!(home.is_some(), "home_dir() should return Some on Windows");
        let path = home.unwrap();
        let path_str = path.to_str().unwrap();
        // Windows home dir should contain a drive letter and Users
        assert!(
            path_str.contains(":\\Users\\") || path_str.contains(":\\Windows\\"),
            "Expected Windows home dir format like C:\\Users\\xxx, got: {}",
            path_str
        );
    }

    #[test]
    fn test_data_dir_non_empty() {
        assert!(dirs::data_dir().is_some(), "data_dir() should return Some on Windows");
        let path = dirs::data_dir().unwrap();
        assert!(!path.as_os_str().is_empty(), "data_dir() path should not be empty");
    }

    // ── P1: rand Alphanumeric sampling ──

    #[test]
    fn test_random_suffix_format() {
        use rand::{RngExt, distr::Alphanumeric};

        let suffix: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();

        assert_eq!(suffix.len(), 4, "Suffix should be exactly 4 characters");
        assert!(
            suffix.chars().all(|c| c.is_ascii_alphanumeric()),
            "All characters should be alphanumeric, got: {}",
            suffix
        );
    }

    #[test]
    fn test_random_suffix_multiple_runs_unique() {
        use rand::RngExt;
        let mut suffixes = std::collections::HashSet::new();
        for _ in 0..100 {
            let suffix: String = rand::rng()
                .sample_iter(&rand::distr::Alphanumeric)
                .take(4)
                .map(char::from)
                .collect();
            suffixes.insert(suffix);
        }
        // With 100 runs of 4-char random strings, we should have at least some variety
        assert!(suffixes.len() > 1, "Random suffixes should produce different values");
    }
}
