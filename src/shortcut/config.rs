// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Model and I/O for the global shortcut configuration file (`%USERPROFILE%\.wsldashboard\shortcuts.toml`).
//
// Configuration example:
//
// ```toml
// [common]
// setting-version = 1
// modify-time = "1761974868205"
//
// [global]
// toggle-window = { shortcut = "Ctrl+Alt+W", modifiers = 3, key = 87, enabled = true }
//
// [terminals]
// "Ubuntu-24.04" = { shortcut = "Ctrl+Alt+U", modifiers = 3, key = 85, enabled = true }
// ```

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

// Shortcut configuration file version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutVersion;

impl ShortcutVersion {
    pub const CURRENT: u32 = 1;
}

fn now_millis() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default()
}

// Common configuration info (maps to the `[common]` TOML table)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutCommonConfig {
    #[serde(rename = "setting-version")]
    pub setting_version: u32,
    #[serde(rename = "modify-time")]
    pub modify_time: String,
}

impl Default for ShortcutCommonConfig {
    fn default() -> Self {
        Self {
            setting_version: ShortcutVersion::CURRENT,
            modify_time: now_millis(),
        }
    }
}

// A single shortcut binding
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutEntry {
    // Display text, e.g. `Ctrl+Alt+W`
    pub shortcut: String,
    // Win32 modifier bitmask (`MOD_CONTROL` / `MOD_ALT` / `MOD_SHIFT` / `MOD_WIN`)
    pub modifiers: u32,
    // Win32 virtual key code
    pub key: u32,
    // Whether enabled
    pub enabled: bool,
}

impl ShortcutEntry {
    // Whether empty (not bound)
    pub fn is_empty(&self) -> bool {
        self.key == 0 || self.shortcut.trim().is_empty()
    }
}

// Window-level shortcuts (maps to the `[global]` TOML table)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutGlobalConfig {
    // Show / hide the main window
    #[serde(rename = "toggle-window")]
    pub toggle_window: Option<ShortcutEntry>,
}

// Root object of `shortcuts.toml`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutConfigFile {
    pub common: ShortcutCommonConfig,
    pub global: ShortcutGlobalConfig,
    // Terminal shortcuts per distro, keyed by distro name
    pub terminals: HashMap<String, ShortcutEntry>,
}

// Shortcut config file path: `%USERPROFILE%\.wsldashboard\shortcuts.toml`
pub fn shortcut_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".wsldashboard")
        .join("shortcuts.toml")
}

// Configuration load result
pub struct LoadResult {
    pub config: ShortcutConfigFile,
    // The config file was corrupted or unreadable and was rebuilt with defaults (the UI must notify the user)
    pub rebuilt: bool,
}

// Load the config file; return the default config and try to rebuild it when the file is missing or cannot be parsed.
pub fn load(path: &Path) -> LoadResult {
    let mut rebuilt = false;
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<ShortcutConfigFile>(&content) {
                Ok(config) => {
                    return LoadResult { config, rebuilt: false };
                }
                Err(e) => {
                    error!("Failed to parse shortcuts.toml: {}, rebuilding default config", e);
                    rebuilt = true;
                }
            },
            Err(e) => {
                error!("Failed to read shortcuts.toml: {}, rebuilding default config", e);
                rebuilt = true;
            }
        }
    } else {
        info!("shortcuts.toml not found, creating default config at {:?}", path);
    }

    let config = ShortcutConfigFile::default();
    if let Err(e) = save(path, &config) {
        warn!("Failed to create default shortcuts.toml: {}", e);
    }
    LoadResult { config, rebuilt }
}

// Write the config file (content is generated entirely by this module, ensuring a stable format with no leftover empty tables).
pub fn save(path: &Path, config: &ShortcutConfigFile) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create dir: {}", e))?;
    }
    let content = compose(config);
    fs::write(path, content).map_err(|e| format!("failed to write file: {}", e))?;
    let toggle = usize::from(
        config
            .global
            .toggle_window
            .as_ref()
            .is_some_and(|e| !e.is_empty()),
    );
    let bindings = config.terminals.values().filter(|e| !e.is_empty()).count() + toggle;
    info!("Saved shortcuts.toml: {} binding(s) -> {:?}", bindings, path);
    Ok(())
}

// Generate the config text. Only the known sections are emitted, and unbound entries leave no empty tables behind.
fn compose(config: &ShortcutConfigFile) -> String {
    let mut out = String::new();
    out.push_str("[common]\n");
    out.push_str(&format!("setting-version = {}\n", config.common.setting_version));
    out.push_str(&format!("modify-time = \"{}\"\n", config.common.modify_time));

    if let Some(entry) = config.global.toggle_window.as_ref().filter(|e| !e.is_empty()) {
        out.push_str("\n[global]\n");
        out.push_str(&format!("toggle-window = {}\n", entry_inline(entry)));
    }

    // Distro entries are kept even when no shortcut is bound yet, so previously added rows still exist after a restart
    let mut names: Vec<&String> = config
        .terminals
        .keys()
        .filter(|name| !name.trim().is_empty())
        .collect();
    names.sort();

    if !names.is_empty() {
        out.push('\n');
        out.push_str("[terminals]\n");
        for name in names {
            let entry = &config.terminals[name];
            out.push_str(&format!("{} = {}\n", toml_key(name), entry_inline(entry)));
        }
    }

    out
}

fn entry_inline(entry: &ShortcutEntry) -> String {
    format!(
        "{{ shortcut = \"{}\", modifiers = {}, key = {}, enabled = {} }}",
        entry.shortcut.replace('\\', "\\\\").replace('"', "\\\""),
        entry.modifiers,
        entry.key,
        entry.enabled
    )
}

// Build a valid TOML key: use a bare key when it contains only safe characters, otherwise a quoted key.
fn toml_key(name: &str) -> String {
    let bare = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if bare {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

// Refresh the `modify-time` field
pub fn touch(config: &mut ShortcutConfigFile) {
    config.common.setting_version = ShortcutVersion::CURRENT;
    config.common.modify_time = now_millis();
}
