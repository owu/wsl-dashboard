use std::path::PathBuf;
use tracing::{info, error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalWslConfig {
    pub memory: String,
    pub processors: String,
    pub networking_mode: String,
    pub swap: String,
}

pub fn get_global_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".wslconfig"))
}

pub fn load_global_config() -> GlobalWslConfig {
    let mut config = GlobalWslConfig::default();
    let path = match get_global_config_path() {
        Some(p) => p,
        None => return config,
    };

    if !path.exists() {
        return config;
    }

    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() { continue; }
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "memory" => config.memory = value.trim().to_string(),
                    "processors" => config.processors = value.trim().to_string(),
                    "networkingMode" => config.networking_mode = value.trim().to_string(),
                    "swap" => config.swap = value.trim().to_string(),
                    _ => {}
                }
            }
        }
    }
    config
}

pub fn save_global_config(config: GlobalWslConfig) -> Result<(), String> {
    let path = get_global_config_path().ok_or("Failed to get home directory")?;
    
    let mut lines = Vec::new();
    lines.push("[wsl2]".to_string());
    if !config.memory.is_empty() { lines.push(format!("memory={}", config.memory)); }
    if !config.processors.is_empty() { lines.push(format!("processors={}", config.processors)); }
    if !config.networking_mode.is_empty() { lines.push(format!("networkingMode={}", config.networking_mode)); }
    if !config.swap.is_empty() { lines.push(format!("swap={}", config.swap)); }

    std::fs::write(path, lines.join("
")).map_err(|e| e.to_string())
}
