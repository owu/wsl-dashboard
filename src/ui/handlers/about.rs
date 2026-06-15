// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use std::io::Read;
use tokio::sync::Mutex;
use tracing::{debug, warn};
use crate::{AppWindow, AppState};
use crate::api::models::OfficialGroup;
use slint::ComponentHandle;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, _app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();

    // group_clicked: pop up only when the image is ready, otherwise no response
    app.on_group_clicked(move || {
        if let Some(app) = ah.upgrade() {
            if app.get_about_group_pic_ready() {
                app.set_show_group_popup(true);
            }
            // Do not respond if the image is not ready
        }
    });
}

// Called when the user opens the About page for the first time (from the select_tab hook in common.rs)
pub fn trigger_fetch(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    debug!("about: triggering helper about info fetch");

    tokio::spawn(async move {
        // Read user configuration (language + timezone)
        let (timezone, sys_lang) = {
            let state = app_state.lock().await;
            let cfg = state.config_manager.get_config();
            (cfg.system.timezone.clone(), cfg.system.system_language.clone())
        };

        let resp = tokio::task::spawn_blocking(move || {
            crate::api::common::wslui_helper_about()
        }).await.unwrap_or_default();

        debug!("about: wslui_helper_about success, official-group count={}", resp.official_group.len());
        
        let app_handle_urls = app_handle.clone();
        let documents_url = resp.documents.as_ref().map(|d| d.url.clone()).unwrap_or_default();
        let talk_url = resp.discussions.as_ref().map(|d| d.url.clone()).unwrap_or_default();

        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = app_handle_urls.upgrade() {
                app.global::<crate::AppInfo>().set_documents_url(documents_url.into());
                app.global::<crate::AppInfo>().set_talk_url(talk_url.into());
            }
        });

        if let Some(item) = match_group(&resp.official_group, &sys_lang, &timezone) {
            let pic_url = item.pic.clone();
            debug!("about: matched group='{}', pic_url={}", item.name, pic_url);
                    // In the child thread, keep only raw pixel bytes + width/height (all implement Send)
                    // slint::Image holds VRc<OpaqueImageVTable> (contains *mut ()), which does not implement Send,
                    // so it must be reconstructed in the UI thread (inside invoke_from_event_loop).
                    match load_image_pixels(&pic_url) {
                        Ok((pixels, w, h)) => {
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(app) = app_handle.upgrade() {
                                    let buf = slint::SharedPixelBuffer::clone_from_slice(&pixels, w, h);
                                    let img = slint::Image::from_rgba8(buf);
                                    app.set_about_group_pic(img);
                                    app.set_about_group_pic_ready(true);
                                    debug!("about: group pic loaded and ready");
                                }
                            });
                        }
                        Err(e) => {
                            warn!("about: group pic load failed: {}", e);
                        }
                    }
                } else {
                    debug!("about: no matching group item found");
                }
    });
}

// Standardize language code: to lowercase + remove hyphens and underscores
// Example: zh-CN / zh_CN / zh-cn => zhcn
fn normalize_lang(s: &str) -> String {
    s.to_lowercase()
        .replace('-', "")
        .replace('_', "")
}

// Match an appropriate entry from the official-group array
//
// Priority rule: both system-language and timezone are non-empty → AND match (case/separator insensitive)
// Fallback rule: entry where both system-language and timezone are empty strings (general fallback)
fn match_group<'a>(
    items: &'a [OfficialGroup],
    sys_lang: &str,
    timezone: &str,
) -> Option<&'a OfficialGroup> {
    let nl = normalize_lang(sys_lang);
    let nt = timezone.to_lowercase();

    // Priority: both language && timezone are non-empty, perform AND match
    for item in items {
        if !item.system_language.is_empty() && !item.timezone.is_empty() {
            if normalize_lang(&item.system_language) == nl
                && item.timezone.to_lowercase() == nt
            {
                return Some(item);
            }
        }
    }

    // Fallback: general entry where both fields are empty
    items
        .iter()
        .find(|i| i.system_language.is_empty() && i.timezone.is_empty())
}


// Download the image and decode to raw RGBA8 pixel bytes + width/height
// Returns (rgba_bytes, width, height), all implement Send, can be safely passed across threads
fn load_image_pixels(url: &str) -> Result<(Vec<u8>, u32, u32), String> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .map_err(|e| e.to_string())?;

    let mut bytes = Vec::new();
    resp.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;

    let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Ok((rgba.into_raw(), w, h))
}
