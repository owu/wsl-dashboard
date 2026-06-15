// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Popup notification condition logic + sync.toml read/write

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::{debug, warn};
use crate::api::models::*;
use crate::api::cache::now_ms;

// ============================================================
// sync.toml version control and storage
// ============================================================

// sync.toml version number
const SYNC_VERSION: u32 = 1;

// sync.toml root structure
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SyncConfig {
    #[serde(default)]
    pub common: SyncCommonConfig,
    #[serde(default)]
    pub popup: PopupRecord,
    #[serde(default)]
    pub message: MessageRecord,
}

// Common config section (version + modify time)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncCommonConfig {
    #[serde(rename = "setting-version")]
    pub setting_version: u32,
    #[serde(rename = "modify-time")]
    pub modify_time: String,
}

impl Default for SyncCommonConfig {
    fn default() -> Self {
        Self {
            setting_version: SYNC_VERSION,
            modify_time: chrono::Utc::now().timestamp_millis().to_string(),
        }
    }
}

// Popup record (flat structure, only one popup type at a time)
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PopupRecord {
    // Popup unique identifier (notice.id or image.id)
    #[serde(default)]
    pub id: String,
    // Popup type ("notice" or "image")
    #[serde(default)]
    #[serde(rename = "type")]
    pub popup_type: String,
    // Last shown timestamp (milliseconds)
    #[serde(default, rename = "shown-time")]
    pub shown_time: i64,
}

// Message read record
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MessageRecord {
    // Read message ID list, comma-separated
    #[serde(default, rename = "read-list")]
    pub read_list: String,
    // Last updated timestamp (milliseconds)
    #[serde(default, rename = "updated-time")]
    pub updated_time: i64,
}

// Get sync.toml file path
fn get_sync_config_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home_dir.join(".wsldashboard").join("sync.toml")
}

// Load sync.toml config file
//
// Returns default config if file does not exist or parsing fails.
// Migrates and writes back to disk if file version is lower than current.
pub fn load_sync_config() -> SyncConfig {
    let path = get_sync_config_path();
    if !path.exists() {
        return SyncConfig::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => match toml::from_str::<SyncConfig>(&content) {
            Ok(mut config) => {
                let old_version = config.common.setting_version;
                if old_version < SYNC_VERSION {
                    migrate_sync_config(&mut config);
                    config.common.setting_version = SYNC_VERSION;
                    let _ = save_sync_config_inner(&path, &config);
                }
                config
            }
            Err(e) => {
                warn!("Failed to parse sync.toml: {}", e);
                SyncConfig::default()
            }
        },
        Err(e) => {
            warn!("Failed to read sync.toml: {}", e);
            SyncConfig::default()
        }
    }
}

// sync.toml version migration
fn migrate_sync_config(config: &mut SyncConfig) {
    let old_version = config.common.setting_version;
    if old_version >= SYNC_VERSION {
        return;
    }
    // Add future version migration logic here
    config.common.setting_version = SYNC_VERSION;
}

// Internal save function (does not update modify-time)
fn save_sync_config_inner(path: &PathBuf, config: &SyncConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let toml_string = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(path, toml_string).map_err(|e| e.to_string())
}

// Save sync.toml config file (auto-updates modify-time)
pub fn save_sync_config(config: &SyncConfig) -> Result<(), String> {
    let path = get_sync_config_path();
    let mut config = config.clone();
    config.common.modify_time = chrono::Utc::now().timestamp_millis().to_string();
    save_sync_config_inner(&path, &config)
}

// Record popup shown time
pub fn record_popup_shown(popup_type: &str, id: &str) {
    let mut config = load_sync_config();
    config.popup = PopupRecord {
        id: id.to_string(),
        popup_type: popup_type.to_string(),
        shown_time: now_ms() as i64,
    };
    if let Err(e) = save_sync_config(&config) {
        warn!("Failed to save sync config: {}", e);
    }
}

// Check if popup was shown within the interval
// Returns true if the popup should be shown (never shown or interval exceeded)
pub fn should_show_by_interval(popup_type: &str, id: &str, interval_ms: i64) -> bool {
    let config = load_sync_config();
    let record = &config.popup;

    if record.id.is_empty() || record.id != id || record.popup_type != popup_type {
        debug!("popup: {} id={} is new, can show", popup_type, id);
        return true;
    }

    let now = now_ms() as i64;
    let elapsed = now - record.shown_time;
    if elapsed >= interval_ms {
        debug!("popup: {} id={} elapsed {}ms >= interval {}ms, can show", popup_type, id, elapsed, interval_ms);
        true
    } else {
        debug!("popup: {} id={} elapsed {}ms < interval {}ms, skip", popup_type, id, elapsed, interval_ms);
        false
    }
}

// ============================================================
// Core decision logic
// ============================================================

// Determine whether to show a notice-type popup
pub fn should_show_notice(
    popup: &PopupConfig,
    sys_lang: &str,
    timezone: &str,
) -> Option<(PopupNotice, PopupNoticeDetail)> {
    if popup.enable != 1 {
        debug!("popup: enable != 1, skip");
        return None;
    }

    if popup.popup_type != "notice" {
        debug!("popup: type != notice, skip");
        return None;
    }

    let notice = match &popup.notice {
        Some(n) => n,
        None => {
            debug!("popup: notice is None, skip");
            return None;
        }
    };

    if !is_in_time_range(notice.start_time, notice.end_time) {
        debug!("popup: notice not in time range [{}, {}]", notice.start_time, notice.end_time);
        return None;
    }

    if !matches_locale(&notice.system_language, &notice.timezone, sys_lang, timezone) {
        debug!("popup: notice locale mismatch");
        return None;
    }

    if !should_show_by_interval("notice", &notice.id, notice.interval) {
        return None;
    }

    let detail = match_detail_by_lang(&notice.detail, sys_lang);
    Some((notice.clone(), detail))
}

// Determine whether to show an image-type popup
pub fn should_show_image(
    popup: &PopupConfig,
    sys_lang: &str,
    timezone: &str,
) -> Option<(PopupImage, PopupImageDetail)> {
    if popup.enable != 1 {
        return None;
    }

    if popup.popup_type != "image" {
        return None;
    }

    let image = match &popup.image {
        Some(i) => i,
        None => {
            debug!("popup: image is None, skip");
            return None;
        }
    };

    if !is_in_time_range(image.start_time, image.end_time) {
        debug!("popup: image not in time range");
        return None;
    }

    if !matches_locale(&image.system_language, &image.timezone, sys_lang, timezone) {
        debug!("popup: image locale mismatch");
        return None;
    }

    if !should_show_by_interval("image", &image.id, image.interval) {
        return None;
    }

    let detail = match_image_detail_by_lang(&image.detail, sys_lang);
    Some((image.clone(), detail))
}

// Check if current time is within [start_time, end_time]
pub fn is_in_time_range(start_time: i64, end_time: i64) -> bool {
    let now = now_ms() as i64;

    if start_time > 0 && now < start_time {
        return false;
    }
    if end_time > 0 && now > end_time {
        return false;
    }
    true
}

// Check if system language and timezone match
pub fn matches_locale(
    remote_lang: &str,
    remote_tz: &str,
    sys_lang: &str,
    sys_tz: &str,
) -> bool {
    let lang_empty = remote_lang.is_empty();
    let tz_empty = remote_tz.is_empty();

    if lang_empty && tz_empty {
        return true;
    }

    if !lang_empty && !lang_matches(remote_lang, sys_lang) {
        return false;
    }

    // Skip timezone check when local timezone is empty (no filtering if local timezone is unavailable)
    if !tz_empty && !sys_tz.is_empty() && !timezone_matches(remote_tz, sys_tz) {
        return false;
    }

    true
}

// Language matching (case-insensitive, ignores hyphen/underscore differences)
fn lang_matches(remote: &str, local: &str) -> bool {
    normalize_lang(remote) == normalize_lang(local)
}

// Timezone matching (case-insensitive, spaces removed)
fn timezone_matches(remote: &str, local: &str) -> bool {
    remote.to_lowercase().replace(' ', "")
        == local.to_lowercase().replace(' ', "")
}

// Normalize language code: lowercase + remove hyphens and underscores
pub fn normalize_lang(s: &str) -> String {
    s.to_lowercase()
        .replace('-', "")
        .replace('_', "")
}

// Select the entry matching the current language from the detail array
fn match_detail_by_lang<'a>(
    details: &'a [PopupNoticeDetail],
    sys_lang: &str,
) -> PopupNoticeDetail {
    let nl = normalize_lang(sys_lang);

    for d in details {
        if normalize_lang(&d.system_language) == nl {
            return d.clone();
        }
    }

    for d in details {
        if normalize_lang(&d.system_language) == "enus" {
            return d.clone();
        }
    }

    details.first().cloned().unwrap_or_default()
}

// ============================================================
// System message read management
// ============================================================

// Get the set of read message IDs
pub fn get_read_message_ids() -> HashSet<String> {
    let config = load_sync_config();
    if config.message.read_list.is_empty() {
        return HashSet::new();
    }
    config.message.read_list
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// Mark a message as read (also cleans up expired IDs)
pub fn mark_message_read(id: &str, active_ids: &[String]) {
    let mut config = load_sync_config();
    let mut ids: HashSet<String> = config.message.read_list
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Clean up historical IDs not in the current API response list
    let active_set: HashSet<String> = active_ids.iter().cloned().collect();
    ids.retain(|k| active_set.contains(k));

    ids.insert(id.to_string());
    config.message.read_list = ids.iter().cloned().collect::<Vec<_>>().join(",");
    config.message.updated_time = now_ms() as i64;

    if let Err(e) = save_sync_config(&config) {
        warn!("Failed to save message read status: {}", e);
    }
}

// Mark all messages as read (replaces with current list, naturally cleans up expired IDs)
pub fn mark_all_messages_read(ids: &[String]) {
    let mut config = load_sync_config();
    config.message.read_list = ids.join(",");
    config.message.updated_time = now_ms() as i64;

    if let Err(e) = save_sync_config(&config) {
        warn!("Failed to save message read status: {}", e);
    }
}

// Image popup detail matching logic
fn match_image_detail_by_lang<'a>(
    details: &'a [PopupImageDetail],
    sys_lang: &str,
) -> PopupImageDetail {
    let nl = normalize_lang(sys_lang);

    for d in details {
        if normalize_lang(&d.system_language) == nl {
            return d.clone();
        }
    }

    for d in details {
        if normalize_lang(&d.system_language) == "enus" {
            return d.clone();
        }
    }

    details.first().cloned().unwrap_or_default()
}
