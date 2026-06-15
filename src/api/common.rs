// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::LazyLock;
use crate::api::cache::{CacheEntry, CACHE_TTL_MEDIUM, CACHE_TTL_LONG, try_get_cache, try_get_stale_cache, set_cache};
use crate::api::client::WslUiClient;
use tracing::{debug, error, warn};
use chrono;

use crate::api::models::*;

// --- Cache storage ---

static LATEST_VERSION_CACHE: LazyLock<std::sync::Mutex<Option<CacheEntry<ReleaseData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_ABOUT_CACHE:   LazyLock<std::sync::Mutex<Option<CacheEntry<HelperAboutData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_DISTRO_CACHE:  LazyLock<std::sync::Mutex<Option<CacheEntry<HelperDistroData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_INSTALL_CACHE: LazyLock<std::sync::Mutex<Option<CacheEntry<HelperInstallData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_MIRRORS_CACHE: LazyLock<std::sync::Mutex<Option<CacheEntry<MirrorListResponse>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_DONATE_CACHE:  LazyLock<std::sync::Mutex<Option<CacheEntry<DonateData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_SYNC_CACHE:   LazyLock<std::sync::Mutex<Option<CacheEntry<HelperSyncData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));
static HELPER_BOOTSTRAP_CACHE: LazyLock<std::sync::Mutex<Option<CacheEntry<HelperBootstrapData>>>> = LazyLock::new(|| std::sync::Mutex::new(None));

// Get wslui latest version
pub fn wslui_latest_version() -> Result<ReleaseData, String> {
    // 1. Check cache
    if let Some(data) = try_get_cache(&LATEST_VERSION_CACHE) {
        debug!("Returning cached latest version data: {:?}", data);
        return Ok(data);
    }

    // 2. Fetch from API
    let client = WslUiClient::new();
    let result = match client.request_api2_with_timeout::<ReleaseData>("GET", "/common/v1/releases/version", None, Some(10000)) {
        Ok((resp, _)) => {
            debug!("Obtained latest release from wslui: {:?}", resp.data);
            Ok(resp.data)
        }
        Err(e) => {
            error!("Failed to get latest release from wslui: {}", e);
            Err(e)
        }
    };

    match result {
        Ok(ref data) => {
            set_cache(&LATEST_VERSION_CACHE, data.clone(), CACHE_TTL_LONG);
            result
        }
        Err(ref _e) => {
            if let Some(stale) = try_get_stale_cache(&LATEST_VERSION_CACHE) {
                warn!("Request failed, falling back to stale cached latest version");
                return Ok(stale);
            }
            result
        }
    }
}

// Get helper about information
pub fn wslui_helper_about() -> HelperAboutData {
    if let Some(data) = try_get_cache(&HELPER_ABOUT_CACHE) {
        debug!("Returning cached helper about data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<HelperAboutData>("GET", "/desktop/v1/helper/about", None) {
        Ok((resp, _)) => {
            debug!("Obtained helper about from wslui: {:?}", resp.data);
            set_cache(&HELPER_ABOUT_CACHE, resp.data.clone(), CACHE_TTL_LONG);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper about from wslui: {}. Using default data.", e);
            HelperAboutData::default()
        }
    }
}

// Get helper distro information
pub fn wslui_helper_distro() -> HelperDistroData {
    if let Some(data) = try_get_cache(&HELPER_DISTRO_CACHE) {
        debug!("Returning cached helper distro data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<HelperDistroData>("GET", "/desktop/v1/helper/distro", None) {
        Ok((resp, _)) => {
            debug!("Obtained helper distro from wslui: {:?}", resp.data);
            set_cache(&HELPER_DISTRO_CACHE, resp.data.clone(), CACHE_TTL_MEDIUM);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper distro from wslui: {}. Using default data.", e);
            HelperDistroData::default()
        }
    }
}

// Get helper installation information
pub fn wslui_helper_install() -> HelperInstallData {
    if let Some(data) = try_get_cache(&HELPER_INSTALL_CACHE) {
        debug!("Returning cached helper install data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<HelperInstallData>("GET", "/desktop/v1/helper/install", None) {
        Ok((resp, _)) => {
            debug!("Obtained helper install from wslui: {:?}", resp.data);
            set_cache(&HELPER_INSTALL_CACHE, resp.data.clone(), CACHE_TTL_MEDIUM);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper install from wslui: {}. Using default data.", e);
            HelperInstallData::default()
        }
    }
}

// Get helper mirrors information
// Note: url parameter is always a fixed address (from wslui_helper_install().online_distros.url)
pub fn wslui_helper_mirrors(url: &str) -> MirrorListResponse {
    if let Some(data) = try_get_cache(&HELPER_MIRRORS_CACHE) {
        debug!("Returning cached helper mirrors data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_full_url::<MirrorListResponse>("GET", url, None, None) {
        Ok((resp, _)) => {
            debug!("Obtained helper mirror from wslui: {:?}", resp.data);
            set_cache(&HELPER_MIRRORS_CACHE, resp.data.clone(), CACHE_TTL_MEDIUM);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper mirror from wslui: {}. Using default data.", e);
            MirrorListResponse::default()
        }
    }
}

// Get helper donate information
pub fn wslui_helper_donate() -> DonateData {
    if let Some(data) = try_get_cache(&HELPER_DONATE_CACHE) {
        debug!("Returning cached helper donate data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<DonateData>("GET", "/desktop/v1/helper/donate", None) {
        Ok((resp, _)) => {
            debug!("Obtained helper donate from wslui: {:?}", resp.data);
            set_cache(&HELPER_DONATE_CACHE, resp.data.clone(), CACHE_TTL_LONG);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper donate from wslui: {}. Using default data.", e);
            DonateData::default()
        }
    }
}

// Get helper sync data (popup notifications)
pub fn wslui_helper_sync() -> HelperSyncData {
    if let Some(data) = try_get_cache(&HELPER_SYNC_CACHE) {
        debug!("Returning cached helper sync data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<HelperSyncData>("GET", "/desktop/v1/helper/sync", None) {
        Ok((resp, _)) => {
            debug!("Obtained helper sync from wslui: {:?}", resp.data);
            set_cache(&HELPER_SYNC_CACHE, resp.data.clone(), CACHE_TTL_LONG);
            resp.data
        }
        Err(e) => {
            error!("Failed to get helper sync from wslui: {}. Using default data.", e);
            HelperSyncData::default()
        }
    }
}

// Get helper bootstrap configuration (also provides internet standard time)
pub fn wslui_helper_bootstrap() -> HelperBootstrapData {
    if let Some(data) = try_get_cache(&HELPER_BOOTSTRAP_CACHE) {
        debug!("Returning cached helper bootstrap data");
        return data;
    }

    let client = WslUiClient::new();
    match client.request_api1::<HelperBootstrapData>("GET", "/desktop/v1/helper/bootstrap", None) {
        Ok((resp, date_header)) => {
            let mut data = resp.data;
            // Resolve unix_time: prefer JSON field, fallback to Date header
            if data.unix_time <= 0 {
                if let Some(date_str) = date_header {
                    match chrono::DateTime::parse_from_rfc2822(&date_str) {
                        Ok(dt) => {
                            let ts = dt.timestamp_millis();
                            debug!("unix_time was 0, obtained time from Date header: {}", ts);
                            data.unix_time = ts;
                        }
                        Err(e) => {
                            error!("Failed to parse Date header from wslui: {}", e);
                        }
                    }
                }
            } else {
                debug!("Obtained unix_time from bootstrap JSON: {}", data.unix_time);
            }
            debug!("Obtained helper bootstrap from wslui: {:?}", data);
            set_cache(&HELPER_BOOTSTRAP_CACHE, data.clone(), CACHE_TTL_LONG);
            data
        }
        Err(e) => {
            error!("Failed to get helper bootstrap from wslui: {}. Using default data.", e);
            HelperBootstrapData::default()
        }
    }
}

