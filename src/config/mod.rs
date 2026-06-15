// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::path::PathBuf;
use tracing::{info, error};

mod migration;
pub mod instances;
pub mod models;
pub mod debug;

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

                    // Refresh basic information (startup_time, version)
                    // Force refresh system information if more than 7 days or data is missing
                    let force_system = should_refresh_system || config.system.system_language.is_empty();
                    
                    Self::refresh_system_info(&mut config, force_system).await;
                    
                    // Ensure critical directories exist
                    let _ = fs::create_dir_all(&config.settings.distro_location);
                    let _ = fs::create_dir_all(&config.settings.logs_location);
                    let _ = fs::create_dir_all(&config.settings.temp_location);

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
        config
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

    // --- USB Config Management ---

    pub fn get_usb_config(&self) -> &UsbConfig {
        &self.config.usb
    }

    #[allow(dead_code)]
    pub fn update_usb_config(&mut self, usb: UsbConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.config.usb = usb;
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("USB configuration saved successfully");
        Ok(())
    }

    // --- Debug Config ---

    // Load `~/.wsldashboard/debug.toml` (best-effort, never panics).
    pub fn load_debug_config() -> DebugConfig {
        DebugConfig::load()
    }

    pub fn toggle_usb_auto_attach(&mut self, bus_id: &str, vid_pid: &str, distro: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let list = &mut self.config.usb.auto_attach_list;
        // Use bus_id for exact matching during toggle as the click is specific to a bus-id
        let index = list.iter().position(|d| d.bus_id == bus_id);
        
        let is_enabled = if let Some(i) = index {
            list.remove(i);
            false
        } else {
            list.push(UsbAutoAttachDevice {
                bus_id: bus_id.to_string(),
                vid_pid: vid_pid.to_string(),
                distribution: distro.to_string(),
            });
            true
        };
        
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("USB auto-attach for {} is now {}", bus_id, if is_enabled { "enabled" } else { "disabled" });
        Ok(is_enabled)
    }
}
