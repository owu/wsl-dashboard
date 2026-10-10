// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ReleaseData {
    pub version: String,
    pub release_date: String,
}

// 1. wslui_helper_about data structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct OfficialGroup {
    pub system_language: String,
    pub timezone: String,
    pub name: String,
    pub pic: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperAboutLink {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperAboutData {
    pub official_group: Vec<OfficialGroup>,
    pub documents: Option<HelperAboutLink>,
    pub discussions: Option<HelperAboutLink>,
}

// 2. wslui_helper_distro data structure
#[derive(Debug, Deserialize, Clone)]
pub struct VSCodeExtension {
    pub name: String,
    pub url: String,
}

impl Default for VSCodeExtension {
    fn default() -> Self {
        Self {
            name: "Microsoft WSL(Identifier:ms-vscode-remote.remote-wsl)".to_string(),
            url: crate::app::VSCODE_MARKETPLACE_URL.to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct StartupScript {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

impl Default for StartupScript {
    fn default() -> Self {
        Self {
            name: "Distro startup script".to_string(),
            url: crate::app::PROJECT_DOCS.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompressScript {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

impl Default for CompressScript {
    fn default() -> Self {
        Self {
            name: "Linux disk cleanup".to_string(),
            url: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompressSource {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

impl Default for CompressSource {
    fn default() -> Self {
        Self {
            name: "WSL Community".to_string(),
            url: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperDistroData {
    pub setup_config: Option<HelperHomeLink>,
    pub vscode_extension: VSCodeExtension,
    pub startup_script: StartupScript,
    pub compress_script: CompressScript,
    pub compress_source: CompressSource,
}

// 3. wslui_helper_install data structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct RootfsHelp {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct OnlineDistroLink {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperInstallData {
    pub rootfs_help: Vec<RootfsHelp>,
    pub online_distros: Option<OnlineDistroLink>,
    pub online_source: Option<OnlineDistroLink>,
}

// 4. wslui_helper_distro setup_config (shared link structure)
#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct HelperHomeLink {
    pub name: String,
    pub url: String,
}

// 5. wslui_helper_mirrors data structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MirrorSource {
    pub url: String,
    pub mirror: String,
    pub format: String,
    #[allow(dead_code)]
    pub last_modified: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DistroInfo {
    pub name: String,
    pub version: String,
    pub sources: Vec<MirrorSource>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MirrorListResponse {
    #[allow(dead_code)]
    pub update_time: String,
    pub distros: Vec<DistroInfo>,
    // Mirror list used on the ARM64 platform (sibling of `distros`; choose one based on the OS platform).
    #[serde(default)]
    pub mirrors: Vec<DistroInfo>,
}

impl MirrorListResponse {
    // Choose the mirror list based on the OS platform:
    // - OS is arm64: use `mirrors`
    // - otherwise (x64 or unknown, default): use `distros`
    //
    // Also logs the detected platform and the chosen array for debugging.
    pub fn select_distros(mut self) -> Vec<DistroInfo> {
        let os_arch = crate::utils::machine::os_arch();
        let (array_name, list) = if os_arch == crate::utils::machine::Arch::Arm64 {
            ("mirrors", std::mem::take(&mut self.mirrors))
        } else {
            ("distros", std::mem::take(&mut self.distros))
        };
        tracing::info!(
            "MirrorListResponse: os_arch={} -> using '{}' array ({} items)",
            os_arch.as_str(),
            array_name,
            list.len()
        );
        list
    }
}

// 6. wslui_helper_donate data structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct DonatePaymentMethod {
    pub name: String,
    pub enable: i32,
    pub icon: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub ext: String,
    #[serde(default)]
    pub format: i32,
    #[serde(default)]
    #[allow(dead_code)]
    pub region: i32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DonationRecord {
    #[allow(dead_code)]
    pub name: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DonateData {
    pub payment_methods: Vec<DonatePaymentMethod>,
    #[serde(default)]
    pub donation_record: Option<DonationRecord>,
}

// ============================================================
// 7. wslui_helper_sync data structure (popup notifications)
// ============================================================

// Multilingual detail item for notice-type popup
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PopupNoticeDetail {
    pub system_language: String,
    pub title: String,
    pub msg1: String,
    pub msg2: String,
    pub msg3: String,
    #[serde(default)]
    pub hint: String,
    #[serde(default)]
    pub link: String,
}

// Notice-type popup configuration
#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct PopupNotice {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub system_language: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub detail: Vec<PopupNoticeDetail>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub highlight: String,
    #[serde(default)]
    pub interval: i64,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub create_time: i64,
    #[serde(default)]
    pub confirm: i32,
}
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PopupImageDetail {
    pub system_language: String,
    pub title: String,
    pub img: String,
}

// Image-type popup configuration
#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct PopupImage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub system_language: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub detail: Vec<PopupImageDetail>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub interval: i64,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub create_time: i64,
}

// Popup configuration (notice or image)
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PopupConfig {
    #[serde(default)]
    pub enable: i32,
    #[serde(default)]
    #[serde(rename = "type")]
    pub popup_type: String,
    #[serde(default)]
    pub notice: Option<PopupNotice>,
    #[serde(default)]
    pub image: Option<PopupImage>,
}

// Data field of the sync API
#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperSyncData {
    #[serde(default)]
    pub popup: PopupConfig,
    #[serde(default)]
    pub messages: Vec<MessageItem>,
}

// Multilingual title item for system message
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MessageTitle {
    pub system_language: String,
    pub title: String,
}

// System message item
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MessageItem {
    pub id: String,
    #[serde(default)]
    pub system_language: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    #[serde(rename = "type")]
    // Supported types: info | warning | error | success | update
    pub msg_type: String,
    pub title: Vec<MessageTitle>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub create_time: i64,
}

// ============================================================
// 8. wslui_helper_bootstrap data structure
// ============================================================

// WSL support version range configuration
#[derive(Debug, Deserialize, Clone, Default)]
pub struct WslSupport {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub since_version: String,
    #[serde(default)]
    pub until_version: String,
    #[serde(default)]
    pub confirm: i32,
    #[serde(default)]
    pub url: String,
}

// Data field of the bootstrap API
#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperBootstrapData {
    #[serde(default)]
    pub wsl_support: WslSupport,
    #[serde(default)]
    pub unix_time: i64,
}

// ============================================================
// 9. wslui_helper_scheduler data structure
// ============================================================

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CronExpressionLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct HelperSchedulerData {
    pub cron_expression: Option<CronExpressionLink>,
    pub command_docs: Option<CronExpressionLink>,
}

// ============================================================
// 10. wslui_helper_mount data structure
// ============================================================

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MountDiskLink {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperMountDiskData {
    pub mount_disk: MountDiskLink,
}

// ============================================================
// 11. wslui_utils_ping data structure
// ============================================================

// Data field of the utils ping API (currently an empty object: {})
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct UtilsPingData {}

