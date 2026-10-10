// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Runtime architecture (machine) detection utility.
//!
//! Provides two kinds of architecture information:
//! - [`current_arch`]: the running architecture of the current exe itself (equivalent to `std::env::consts::ARCH`).
//! - [`os_arch`]: the native architecture of the operating system (host). Even if an x64 exe runs
//!   under emulation on arm64 Windows, `os_arch` still returns [`Arch::Arm64`], differing from `current_arch`.

use windows::Win32::System::SystemInformation::{
    GetNativeSystemInfo, PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM64, SYSTEM_INFO,
};

// Architecture identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    // 64-bit x86.
    X64,
    // 64-bit ARM.
    Arm64,
    // Other/unknown architecture.
    Unknown,
}

impl Arch {
    // Compact identifier, e.g. `x64`, `arm64`.
    pub fn as_str(self) -> &'static str {
        match self {
            Arch::X64 => "x64",
            Arch::Arm64 => "arm64",
            Arch::Unknown => "unknown",
        }
    }

    // Display suffix appended after the version number, e.g. `.x64`, `.arm64`; returns an empty string for an unknown architecture.
    pub fn suffix(self) -> &'static str {
        match self {
            Arch::X64 => ".x64",
            Arch::Arm64 => ".arm64",
            Arch::Unknown => "",
        }
    }
}

// Read the running architecture of the current exe (the process's own architecture).
//
// An x64 build returns [`Arch::X64`], an arm64 build returns [`Arch::Arm64`];
// an x64 exe emulated on arm64 Windows still returns [`Arch::X64`].
pub fn current_arch() -> Arch {
    match std::env::consts::ARCH {
        "x86_64" => Arch::X64,
        "aarch64" => Arch::Arm64,
        _ => Arch::Unknown,
    }
}

// Read the native architecture of the operating system (host).
//
// Unlike [`current_arch`], this function queries the system itself, not the current process.
// For example, an x64 exe running under emulation on arm64 Windows returns [`Arch::Arm64`].
pub fn os_arch() -> Arch {
    let mut info = SYSTEM_INFO::default();
    unsafe { GetNativeSystemInfo(&mut info) };
    let processor_arch = unsafe { info.Anonymous.Anonymous.wProcessorArchitecture };
    if processor_arch == PROCESSOR_ARCHITECTURE_ARM64 {
        Arch::Arm64
    } else if processor_arch == PROCESSOR_ARCHITECTURE_AMD64 {
        Arch::X64
    } else {
        Arch::Unknown
    }
}

// Convenience method: directly returns the version display suffix, e.g. `.x64`, `.arm64`.
pub fn arch_suffix() -> &'static str {
    current_arch().suffix()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_str_and_suffix() {
        assert_eq!(Arch::X64.as_str(), "x64");
        assert_eq!(Arch::Arm64.as_str(), "arm64");
        assert_eq!(Arch::Unknown.as_str(), "unknown");
        assert_eq!(Arch::X64.suffix(), ".x64");
        assert_eq!(Arch::Arm64.suffix(), ".arm64");
        assert_eq!(Arch::Unknown.suffix(), "");
    }

    #[test]
    fn test_current_arch_matches_target() {
        // The build target matches the running architecture; the test environment is x64 or arm64.
        #[cfg(target_arch = "x86_64")]
        assert_eq!(current_arch(), Arch::X64);
        #[cfg(target_arch = "aarch64")]
        assert_eq!(current_arch(), Arch::Arm64);
    }

    #[test]
    fn test_os_arch_is_detected() {
        // A normal Windows environment should be detected as x64 or arm64.
        assert!(matches!(os_arch(), Arch::X64 | Arch::Arm64));
    }
}