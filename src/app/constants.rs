// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Application constants definition
#[allow(dead_code)]
pub const APP_NAME: &str = "WSL Dashboard";
#[allow(dead_code)]
pub const APP_ID: &str = "wsldashboard";
#[allow(dead_code)]
pub const COMPANY_NAME: &str = APP_NAME;
#[allow(dead_code)]
pub const LEGAL_COPYRIGHT: &str = "2026 WSL Dashboard. All rights reserved.";


#[allow(dead_code)]
pub const PROJECT_REPOSITORY: &str = "https://github.com/owu/wsl-dashboard";

#[allow(dead_code)]
pub const PROJECT_WEBSITE: &str = "https://www.wslui.com";

#[allow(dead_code)]
pub const GITHUB_ISSUES: &str = "/issues";

#[allow(dead_code)]
pub const GITHUB_RELEASES: &str = "/releases";

#[allow(dead_code)]
pub const DOWNLOAD_URI: &str = "/download/";

#[allow(dead_code)]
pub const DONATE_URI: &str = "/donate/";

#[allow(dead_code)]
pub const PRIVACY_URI: &str = "/privacy/";

#[allow(dead_code)]
pub const TERMS_URI: &str = "/terms/";

#[allow(dead_code)]
pub const GITHUB_DOMAIN: &str = "https://github.com";

#[allow(dead_code)]
pub const WSL_GITHUB_RELEASES: &str = "https://github.com/microsoft/WSL/releases/latest";

#[allow(dead_code)]
pub const VSCODE_MARKETPLACE_URL: &str = "https://marketplace.visualstudio.com";

#[allow(dead_code)]
pub const PROJECT_DOCS: &str = "https://docs.wslui.com";

#[allow(dead_code)]
pub const API1_URL: &str = "https://api1.wslui.com";

#[allow(dead_code)]
pub const API2_URL: &str = "https://api2.wslui.com";

// Compatibility of Chinese and Japanese character display on Western language operating systems
// Font constants
#[allow(dead_code)]
pub const FONT_ZH: &str = "Microsoft YaHei UI";
#[allow(dead_code)]
pub const FONT_EN_FALLBACK: &str = "Segoe UI, Microsoft YaHei UI";

// Check if a language code represents Chinese
#[allow(dead_code)]
pub fn is_chinese_lang(lang: &str) -> bool {
    lang.to_lowercase().starts_with("zh")
}

// Check if the Windows system locale supports CJK natively.
// When the system locale is Chinese, Windows font fallback (DirectWrite) automatically
// renders CJK characters using the system's built-in CJK fonts, even when the primary
// font (e.g., Segoe UI) doesn't contain those glyphs. This means we don't need to
// explicitly specify CJK fonts like "Microsoft YaHei UI", saving ~16MB memory by
// avoiding the CJK glyph atlas allocation in Skia.
#[allow(dead_code)]
pub fn system_has_cjk_support(system_lang: &str) -> bool {
    system_lang.to_lowercase().starts_with("zh")
}

// Get the appropriate font for the given app language and Windows system locale.
//
// On Chinese Windows, Windows DirectWrite font fallback handles CJK characters
// automatically, so we use "Segoe UI" and let the system handle CJK rendering.
// This avoids loading the large CJK font glyph atlas in Skia, saving ~16MB memory.
//
// On non-Chinese Windows, we must explicitly specify CJK fonts to ensure correct rendering.
#[allow(dead_code)]
pub fn get_font_for(app_lang: &str, system_lang: &str) -> &'static str {
    if is_chinese_lang(app_lang) {
        if system_has_cjk_support(system_lang) {
            // Chinese Windows + Chinese app: Windows fallback handles CJK → save memory
            "Segoe UI"
        } else {
            // Non-Chinese Windows + Chinese app: must specify YaHei explicitly
            FONT_ZH
        }
    } else {
        if system_has_cjk_support(system_lang) {
            // Chinese Windows + non-Chinese app: Windows fallback handles any CJK content (e.g., distro names)
            "Segoe UI"
        } else {
            // Non-Chinese Windows + non-Chinese app: need YaHei as fallback for CJK content
            FONT_EN_FALLBACK
        }
    }
}

// WSL distribution initialization script path
#[allow(dead_code)]
pub const WSL_INIT_SCRIPT: &str = "/etc/init.wsl-dashboard";

