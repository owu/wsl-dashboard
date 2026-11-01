// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tracing::info;

// Collect and log system version information asynchronously at startup.
// This runs in a background tokio task and does not block the startup flow.
pub async fn log_startup_system_info() {
    let versions = crate::utils::system_info::versions();

    info!(
        "[STARTUP] App: v{} | WSL: {} | Windows: {}",
        env!("CARGO_PKG_VERSION"),
        versions.wsl_version,
        versions.windows_version
    );
}
