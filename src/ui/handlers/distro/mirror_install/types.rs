// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, Clone)]
pub enum DownloadError {
    HumanVerification { mirror: String, reason: String },
    NetworkError { mirror: String, error: String },
    FileError { mirror: String, error: String },
    AllMirrorsFailed { #[allow(dead_code)] errors: Vec<String> },
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HumanVerification { mirror, reason } => write!(f, "Human verification detected from {}: {}", mirror, reason),
            Self::NetworkError { mirror, error } => write!(f, "Network error from {}: {}", mirror, error),
            Self::FileError { mirror, error } => write!(f, "File error from {}: {}", mirror, error),
            Self::AllMirrorsFailed { .. } => {
                write!(f, "All mirrors failed")
            }
        }
    }
}

impl std::error::Error for DownloadError {}

pub enum DownloadProgress<'a> {
    TryingMirror { mirror: &'a str, url: &'a str },
    Downloading { percent: f32, speed: &'a str },
    MirrorFailed { mirror: &'a str, error: &'a str },
    MirrorFileInvalid { mirror: &'a str },
}

pub enum DownloadProgressOwned {
    #[allow(dead_code)]
    TryingMirror { mirror: String, url: String },
    Downloading { percent: f32, speed: String },
    MirrorFailed { mirror: String, error: String },
    MirrorFileInvalid { mirror: String },
}

pub fn replace_last_line(buffer: &mut String, new_line: &str) {
    if buffer.ends_with('\n') {
        buffer.pop();
    }
    if let Some(last_nl) = buffer.rfind('\n') {
        buffer.truncate(last_nl + 1);
    } else {
        buffer.clear();
    }
    buffer.push_str(new_line);
    buffer.push('\n');
}
