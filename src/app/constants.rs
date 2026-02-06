// Application constants definition
#[allow(dead_code)]
pub const APP_NAME: &str = "WSL Dashboard";
#[allow(dead_code)]
pub const APP_ID: &str = "wsldashboard";
#[allow(dead_code)]
pub const COMPANY_NAME: &str = APP_NAME;
#[allow(dead_code)]
pub const LEGAL_COPYRIGHT: &str = "2026 WSL Dashboard. All rights reserved.";

// FREE
#[allow(dead_code)]
pub const GITHUB_URL: &str = "https://github.com/owu/wsl-dashboard";

// CN
#[allow(dead_code)]
pub const GITEE_URL: &str = "https://gitee.com/bye/wsl-dashboard";

#[allow(dead_code)]
pub const GITHUB_ISSUES: &str = "/issues";
#[allow(dead_code)]
pub const GITHUB_DISCUSSIONS: &str = "/discussions";
#[allow(dead_code)]
pub const GITHUB_RELEASES: &str = "/releases";

// FREE
#[allow(dead_code)]
pub const STATIC_API_FREE: &str = "https://raw.githubusercontent.com/owu/oss/refs/heads";

// CN
#[allow(dead_code)]
pub const STATIC_API: &str = "https://gitee.com/bye/oss/raw";

#[allow(dead_code)]
pub const UPDATE_CHECK_API: &str = "/main/wsldashboard/api/base.json";

#[allow(dead_code)]
pub const INSTANCE_API: &str = "/main/wsldashboard/api/instance.json";

#[allow(dead_code)]
pub const ZH_TIMEZONE: &str = "UTC+08:00";

// Compatibility of Chinese and Japanese character display on Western language operating systems
// Font constants
// #[allow(dead_code)]
// pub const FONT_ZH: &str = "Microsoft YaHei UI";
// #[allow(dead_code)]
// pub const FONT_EN_FALLBACK: &str = "Segoe UI, Microsoft YaHei UI";


// Font constants
#[allow(dead_code)]
pub const FONT_ZH: &str = "Segoe UI";
#[allow(dead_code)]
pub const FONT_EN_FALLBACK: &str = "Segoe UI";

/// Check if a language code represents Chinese
#[allow(dead_code)]
pub fn is_chinese_lang(lang: &str) -> bool {
    lang.to_lowercase().starts_with("zh")
}

