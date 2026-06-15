// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tracing::{debug, info, warn};

fn wsl_guide_url() -> &'static std::sync::RwLock<String> {
    static WSL_GUIDE_URL: std::sync::LazyLock<std::sync::RwLock<String>> = std::sync::LazyLock::new(|| {
        std::sync::RwLock::new(crate::app::PROJECT_DOCS.to_string())
    });
    &WSL_GUIDE_URL
}

// Returns the current wsl guide URL (from API or default PROJECT_DOCS).
#[allow(dead_code)]
pub fn get_wsl_guide_url() -> String {
    wsl_guide_url().read().unwrap().clone()
}

pub fn trigger_fetch_home_api(app_handle: slint::Weak<crate::AppWindow>) {
    debug!("wsl_guide: triggering distro API fetch");

    tokio::spawn(async move {
        let resp = tokio::task::spawn_blocking(|| {
            crate::api::common::wslui_helper_distro()
        }).await.unwrap_or_default();

        let url = match resp.setup_config {
            Some(ref config) if !config.url.is_empty() => {
                info!("wsl_guide: setup_config URL from API = {}", config.url);
                config.url.clone()
            }
            _ => {
                warn!("wsl_guide: API returned empty setup_config, using default");
                crate::app::PROJECT_DOCS.to_string()
            }
        };

        if let Ok(mut guard) = wsl_guide_url().write() {
            *guard = url.clone();
        }

        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = app_handle.upgrade() {
                app.set_wsl_guide_url(url.into());
            }
        });
    });
}
