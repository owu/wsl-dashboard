// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Cached Windows / WSL version information, shared by startup logging and the API User-Agent.
//
// The values are collected once per process (`wsl --version`) and kept in a [`OnceLock`], so
// neither the startup log nor the request header construction spawns a child process repeatedly.

use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::OnceLock;

use crate::utils::machine::current_arch;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const FALLBACK_VERSION: &str = "N/A";

// Windows and WSL version strings, e.g. `10.0.26300.9818` and `3.0.3.0`.
#[derive(Debug, Clone)]
pub struct SystemVersions {
    pub windows_version: String,
    pub wsl_version: String,
}

static SYSTEM_VERSIONS: OnceLock<SystemVersions> = OnceLock::new();

// Collect Windows / WSL versions once; subsequent calls return the cached values.
// Blocking on first call (runs `wsl --version`), so call it from a background task when possible.
pub fn versions() -> &'static SystemVersions {
    SYSTEM_VERSIONS.get_or_init(|| {
        let stdout = run_wsl_version();
        SystemVersions {
            wsl_version: stdout
                .as_deref()
                .and_then(parse_wsl_version)
                .unwrap_or_else(|| FALLBACK_VERSION.to_string()),
            windows_version: stdout
                .as_deref()
                .and_then(parse_windows_version)
                .or_else(windows_version_from_ver)
                .unwrap_or_else(|| FALLBACK_VERSION.to_string()),
        }
    })
}

// User-Agent sent with every API request, e.g.
// `Mozilla/5.0 (Windows NT 10.0.26300.9818; Win64; x64; WSL/3.0.3.0) AppleWebKit/537.36`
// The app version is intentionally omitted here; it is carried by the `App-Version` header.
pub fn user_agent() -> String {
    let versions = versions();
    format_user_agent(
        &versions.windows_version,
        &versions.wsl_version,
        current_arch().as_str(),
    )
}

fn format_user_agent(windows_version: &str, wsl_version: &str, arch: &str) -> String {
    format!(
        "Mozilla/5.0 (Windows NT {}; Win64; {}; WSL/{}) AppleWebKit/537.36",
        windows_version, arch, wsl_version
    )
}

// Run `wsl --version` and return its stdout when the command succeeds.
fn run_wsl_version() -> Option<String> {
    let output = Command::new("wsl.exe")
        .arg("--version")
        .env("WSL_UTF8", "1")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

// Extract the `WSL version:` value while ignoring WSLg / kernel / Windows / Direct3D / DXCore / MSRDC lines.
fn parse_wsl_version(stdout: &str) -> Option<String> {
    stdout.lines().find_map(|line| {
        let lower = line.to_lowercase();
        if lower.contains("wsl")
            && !lower.contains("wslg")
            && !lower.contains("kernel")
            && !lower.contains("windows")
            && !lower.contains("direct3d")
            && !lower.contains("dxcore")
            && !lower.contains("msrdc")
        {
            extract_version_number(line)
        } else {
            None
        }
    })
}

// Extract the `Windows version:` value from `wsl --version` output.
fn parse_windows_version(stdout: &str) -> Option<String> {
    stdout.lines().find_map(|line| {
        if line.to_lowercase().contains("windows version") {
            extract_version_number(line)
        } else {
            None
        }
    })
}

// Fallback for `Windows version:`: parse the version number from `cmd /c ver`.
fn windows_version_from_ver() -> Option<String> {
    let output = Command::new("cmd.exe")
        .args(["/c", "ver"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    extract_version_number(&String::from_utf8_lossy(&output.stdout))
}

// Pick the first digit-and-dot token (e.g. `10.0.26300.9818`), which also guarantees an ASCII-only
// value suitable for an HTTP header.
fn extract_version_number(s: &str) -> Option<String> {
    s.split(|c: char| !c.is_ascii_digit() && c != '.')
        .find(|token| {
            !token.is_empty()
                && token.contains('.')
                && token.chars().all(|c| c.is_ascii_digit() || c == '.')
        })
        .map(|token| token.trim_matches('.').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "WSL version: 3.0.3.0\nKernel version: 6.18.54.1-1\nWSLg version: 1.0.79\nMSRDC version: 1.2.7391\nDirect3D version: 1.611.1-81528511\nDXCore version: 10.0.26100.1-240331-1435.ge-release\nWindows version: 10.0.26300.9818";

    #[test]
    fn test_parse_wsl_version() {
        assert_eq!(parse_wsl_version(SAMPLE).as_deref(), Some("3.0.3.0"));
    }

    #[test]
    fn test_parse_windows_version() {
        assert_eq!(parse_windows_version(SAMPLE).as_deref(), Some("10.0.26300.9818"));
    }

    #[test]
    fn test_extract_from_localized_ver_output() {
        assert_eq!(
            extract_version_number("Microsoft Windows [版本 10.0.26100.9818]").as_deref(),
            Some("10.0.26100.9818")
        );
    }

    #[test]
    fn test_format_user_agent() {
        assert_eq!(
            format_user_agent("10.0.26300.9818", "3.0.3.0", "x64"),
            "Mozilla/5.0 (Windows NT 10.0.26300.9818; Win64; x64; WSL/3.0.3.0) AppleWebKit/537.36"
        );
    }
}
