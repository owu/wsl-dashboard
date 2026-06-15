// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};

// `[install]` section of debug.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugInstallConfig {
    // Path to a local JSON file containing `MirrorListResponse` data.
    // When non-empty, this file is used instead of fetching from the network.
    #[serde(rename = "online-distros", default)]
    pub online_distros: String,
}

// `[distro]` section of debug.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugDistroConfig {
    // Path to a local bash script (.sh) for cleanup during compression.
    // When non-empty, this script is executed inside the distro instead of downloading the default script.
    #[serde(rename = "cleanup-script", default)]
    pub cleanup_script: String,
}

// Root structure for `~/.wsldashboard/debug.toml`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugConfig {
    #[serde(default)]
    pub install: DebugInstallConfig,
    #[serde(default)]
    pub distro: DebugDistroConfig,
}

impl DebugConfig {
    // Return the path to the debug config file: `~/.wsldashboard/debug.toml`
    pub fn path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".wsldashboard").join("debug.toml")
    }

    // Load the debug config from disk.
    //
    // - If the file does not exist, a `Default` instance is returned silently.
    // - If the file exists but cannot be parsed, a warning is logged and `Default` is returned.
    pub fn load() -> Self {
        let path = Self::path();
        if !path.exists() {
            info!("[DebugConfig] debug.toml not found at {}, using defaults", path.display());
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<DebugConfig>(&content) {
                Ok(cfg) => {
                    info!(
                        "[DebugConfig] Loaded debug.toml from {}: install.online-distros='{}'",
                        path.display(),
                        cfg.install.online_distros
                    );
                    cfg
                }
                Err(e) => {
                    warn!(
                        "[DebugConfig] Failed to parse debug.toml at {}: {}. Using defaults.",
                        path.display(),
                        e
                    );
                    Self::default()
                }
            },
            Err(e) => {
                warn!(
                    "[DebugConfig] Failed to read debug.toml at {}: {}. Using defaults.",
                    path.display(),
                    e
                );
                Self::default()
            }
        }
    }
}
