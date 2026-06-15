// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Popup notification business handler
//
// Called by SyncMonitorTask, handles popup data filtering and dialog display.

use tracing::{debug, warn};
use crate::api::models::PopupConfig;
use crate::AppWindow;

// Download and decode popup image (runs on a blocking thread)
fn load_popup_image_pixels(url: &str) -> Result<(Vec<u8>, u32, u32), String> {
    use std::io::Read;
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

// Handle popup notification business
pub async fn handle_popup(popup: &PopupConfig, app_handle: &slint::Weak<AppWindow>) {
    // Return immediately when enable == 0
    if popup.enable != 1 {
        return;
    }

    // Get system language and timezone
    let (sys_lang, timezone) = {
        let ah = app_handle.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = slint::invoke_from_event_loop(move || {
            let lang = if let Some(app) = ah.upgrade() {
                app.get_system_language().to_string()
            } else {
                "en".to_string()
            };
            let _ = tx.send((lang, String::new()));
        });
        rx.recv_timeout(std::time::Duration::from_millis(100))
            .unwrap_or(("en".to_string(), String::new()))
    };

    debug!("popup: checking with sys_lang={}, timezone={}", sys_lang, timezone);

    // Check notice popup
    if let Some((notice, detail)) = crate::app::popup::should_show_notice(
        popup, &sys_lang, &timezone,
    ) {
        debug!("popup: showing notice popup, id={}", notice.id);

        // Prevent duplicates: skip if a popup is already showing
        let already_showing = {
            let ah = app_handle.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            let _ = slint::invoke_from_event_loop(move || {
                let showing = if let Some(app) = ah.upgrade() {
                    app.get_show_popup_notice_dialog()
                } else {
                    false
                };
                let _ = tx.send(showing);
            });
            rx.recv_timeout(std::time::Duration::from_millis(100)).unwrap_or(false)
        };

        if already_showing {
            debug!("popup: notice dialog already showing, skip");
            return;
        }

        // Set popup properties and display
        let ah = app_handle.clone();
        let url = notice.url.clone();
        let highlight = notice.highlight.clone();
        let notice_id = notice.id.clone();
        let confirm = notice.confirm;
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = ah.upgrade() {
                app.set_popup_notice_title(detail.title.into());
                app.set_popup_notice_msg1(detail.msg1.into());
                app.set_popup_notice_msg2(detail.msg2.into());
                app.set_popup_notice_msg3(detail.msg3.into());
                app.set_popup_notice_hint(detail.hint.into());
                app.set_popup_notice_link(detail.link.into());
                app.set_popup_notice_url(url.into());
                app.set_popup_notice_highlight(highlight.into());
                app.set_popup_notice_confirm(confirm);
                app.set_show_popup_notice_dialog(true);
            }
        });

        // Record popup shown time to sync.toml
        crate::app::popup::record_popup_shown("notice", &notice_id);
        return;
    }

    // Check image popup
    if let Some((image, detail)) = crate::app::popup::should_show_image(
        popup, &sys_lang, &timezone,
    ) {
        debug!("popup: showing image popup, id={}", image.id);

        let already_showing = {
            let ah = app_handle.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            let _ = slint::invoke_from_event_loop(move || {
                let showing = if let Some(app) = ah.upgrade() {
                    app.get_show_popup_image_dialog()
                } else {
                    false
                };
                let _ = tx.send(showing);
            });
            rx.recv_timeout(std::time::Duration::from_millis(100)).unwrap_or(false)
        };

        if already_showing {
            debug!("popup: image dialog already showing, skip");
            return;
        }

        // Load image asynchronously
        let img_url = detail.img.clone();
        let img_title = detail.title.clone();
        let img_width = image.width;
        let img_height = image.height;
        let img_url2 = image.url.clone();
        let image_id = image.id.clone();
        let ah = app_handle.clone();

        match tokio::task::spawn_blocking(move || {
            load_popup_image_pixels(&img_url)
        })
        .await
        {
            Ok(Ok((pixels, w, h))) => {
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        let buf = slint::SharedPixelBuffer::clone_from_slice(&pixels, w, h);
                        let img = slint::Image::from_rgba8(buf);
                        app.set_popup_image_source(img);
                        app.set_popup_image_title(img_title.into());
                        app.set_popup_image_url(img_url2.into());
                        app.set_popup_image_width(img_width);
                        app.set_popup_image_height(img_height);
                        app.set_show_popup_image_dialog(true);
                    }
                });

                // Record popup shown time to sync.toml
                crate::app::popup::record_popup_shown("image", &image_id);
            }
            Ok(Err(e)) => {
                warn!("popup: image load failed: {}", e);
            }
            Err(e) => {
                warn!("popup: image load task failed: {}", e);
            }
        }
    }
}
