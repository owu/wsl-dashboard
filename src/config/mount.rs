// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::app::mount_disk::models::MountedDiskRecord;

pub const MOUNT_DISK_CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountDiskCommonConfig {
    #[serde(rename = "setting-version", default = "default_mount_disk_version")]
    pub setting_version: u32,
    #[serde(rename = "modify-time", default = "default_modify_time")]
    pub modify_time: String,
}

fn default_mount_disk_version() -> u32 {
    MOUNT_DISK_CONFIG_VERSION
}

fn default_modify_time() -> String {
    chrono::Utc::now().timestamp_millis().to_string()
}

impl Default for MountDiskCommonConfig {
    fn default() -> Self {
        Self {
            setting_version: MOUNT_DISK_CONFIG_VERSION,
            modify_time: default_modify_time(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountDiskConfig {
    #[serde(default)]
    pub common: MountDiskCommonConfig,
    #[serde(rename = "mounted-disks", alias = "mounted_disks", default)]
    pub mounted_disks: Vec<MountedDiskRecord>,
}

impl Default for MountDiskConfig {
    fn default() -> Self {
        Self {
            common: MountDiskCommonConfig::default(),
            mounted_disks: Vec::new(),
        }
    }
}

impl MountDiskConfig {
    // Get path to mount.toml (~/.wsldashboard/mount.toml)
    pub fn config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let dir = home_dir.join(".wsldashboard");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        dir.join("mount.toml")
    }

    // Load mount.toml configuration file
    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            let mut cfg = Self::default();
            let _ = cfg.save();
            return cfg;
        }
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(e) => {
                tracing::warn!("Failed to read mount.toml: {}, using default", e);
                Self::default()
            }
        }
    }

    // Save mount.toml configuration file (auto-updates modify-time)
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
        let path = Self::config_path();
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    // Add or update a mounted disk record
    pub fn add_or_update(&mut self, record: MountedDiskRecord) {
        self.mounted_disks.retain(|item| item.disk_path != record.disk_path || item.mount_id != record.mount_id);
        self.mounted_disks.push(record);
        let _ = self.save();
    }

    // Remove a mounted disk record by path or mount_id
    pub fn remove(&mut self, disk_path_or_id: &str) {
        let path_clean = disk_path_or_id.replace('/', "\\");
        self.mounted_disks.retain(|item| {
            item.mount_id != disk_path_or_id && item.disk_path != disk_path_or_id && item.disk_path != path_clean
        });
        let _ = self.save();
    }

    // Set is_mounted status for a disk record by path or mount_id
    pub fn set_mount_status(&mut self, disk_path_or_id: &str, is_mounted: bool) {
        let path_clean = disk_path_or_id.replace('/', "\\");
        if let Some(record) = self.mounted_disks.iter_mut().find(|item| {
            item.mount_id == disk_path_or_id || item.disk_path == disk_path_or_id || item.disk_path == path_clean
        }) {
            record.is_mounted = is_mounted;
            let _ = self.save();
        }
    }
}