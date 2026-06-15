// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::wsl::models::{WslDistro, WslStatus, WslVersion};
use tracing::trace;

// Parse output of wsl -l -v command to extract WSL subsystem list
pub fn parse_distros_list(output: &str) -> Vec<WslDistro> {
    trace!("Parsing WSL distributions list from output (length: {})", output.len());
    let mut distros = Vec::new();
    let lines = output.lines();

    // Skip header line
    let mut is_header_skipped = false;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !is_header_skipped {
            // Header line format: NAME STATE VERSION (may have leading spaces)
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with("NAME") {
                is_header_skipped = true;
            }
            continue;
        }

        // Parse line content
        // Format example: Ubuntu Running 2
        // Or: Ubuntu-20.04 Stopped 1
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        // Handle case where name contains spaces (though WSL subsystem names usually don't)
        let name_parts = parts[0..parts.len()-2].join(" ");
        let state_part = parts[parts.len()-2];
        let version_part = parts[parts.len()-1];

        // Parse status
        let status = match state_part.to_lowercase().as_str() {
            "running" => WslStatus::Running,
            _ => WslStatus::Stopped,
        };

        // Parse version
        let version = match version_part.parse::<u32>() {
            Ok(1) => WslVersion::V1,
            Ok(2) => WslVersion::V2,
            _ => WslVersion::V1, // Default V1
        };

        // Check if it's the default subsystem (default subsystem has * before name)
        let (is_default, name) = if name_parts.starts_with("*") {
            (true, name_parts[1..].trim().to_string())
        } else {
            (false, name_parts.trim().to_string())
        };

        distros.push(WslDistro {
            name,
            status,
            version,
            is_default,
            last_start_time: None,
        });
    }

    distros
}

// Parse output of wsl -l -o command to extract installable WSL distribution list
pub fn parse_available_distros(output: &str) -> Vec<(String, String)> {
    let mut available = Vec::new();
    let lines = output.lines();
    let mut is_header_skipped = false;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !is_header_skipped {
            if line.contains("NAME") {
                is_header_skipped = true;
            }
            continue;
        }

        // Parse line: NAME FRIENDLY_NAME
        // Example: Ubuntu Ubuntu
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let name = parts[0].to_string();
        let friendly_name = parts[1..].join(" ");
        let friendly_name = if friendly_name.is_empty() {
            name.clone()
        } else {
            friendly_name
        };

        available.push((name, friendly_name));
    }

    available
}