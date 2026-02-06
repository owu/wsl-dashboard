use std::fs;
use std::path::PathBuf;
use tracing::{info, error};

mod migration;
mod instances;
pub mod models;

pub use models::*;

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

    fn get_instances_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("instances.toml")
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
                info!("✅ Configuration file initialized successfully: {}", config_path.display());
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
    async fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    // Save configuration file
    fn save_config(path: &PathBuf, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn update_settings(&mut self, settings: UserSettings) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure new paths exist
        let _ = fs::create_dir_all(&settings.distro_location);
        let _ = fs::create_dir_all(&settings.logs_location);
        let _ = fs::create_dir_all(&settings.temp_location);

        self.config.settings = settings;
        self.config.application.setting_version = SETTINGS_VERSION as u8;
        
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("✅ Configuration saved successfully");
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
    pub fn update_tray_settings(&mut self, tray: TraySettings) -> Result<(), Box<dyn std::error::Error>> {
        self.config.tray = tray;
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("✅ Tray configuration saved successfully");
        Ok(())
    }

    // Update popup detection timestamp
    pub fn update_check_time(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.settings.check_time = chrono::Utc::now().timestamp_millis().to_string();
        Self::save_config(&self.config_path, &mut self.config)?;
        info!("Updated check-time to: {}", self.config.settings.check_time);
        Ok(())
    }

    // --- Instances Config Management ---

    fn load_instances() -> InstancesContainer {
        instances::load_instances(&Self::get_instances_path())
    }

    fn save_instances_to_disk(path: &std::path::Path, container: &InstancesContainer) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn update_instance_config(&self, distro_name: &str, config: DistroInstanceConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut container = Self::load_instances();
        container.instances.insert(distro_name.to_string(), config);
        container.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        container.common.setting_version = INSTANCES_VERSION;

        let path = Self::get_instances_path();
        Self::save_instances_to_disk(&path, &container)?;
        info!("✅ Instance configuration for '{}' saved successfully", distro_name);
        Ok(())
    }

    pub fn remove_instance_config(&self, distro_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut container = Self::load_instances();
        if container.instances.remove(distro_name).is_some() {
            container.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
            let path = Self::get_instances_path();
            Self::save_instances_to_disk(&path, &container)?;
            info!("✅ Removed instance configuration for '{}'", distro_name);
        }
        Ok(())
    }
}
