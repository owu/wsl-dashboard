// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use std::io::Read;
use tracing::{debug, warn};
use crate::{AppWindow, AppState};
use crate::utils::system::copy_to_clipboard;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, _app_state: Arc<tokio::sync::Mutex<AppState>>) {
    let ah = app_handle.clone();
    app.on_donate_qr_clicked(move |url| {
        if let Some(app) = ah.upgrade() {
            app.set_donate_popup_loading(true);
            app.set_show_donate_popup(true);
            
            let url_str = url.to_string();
            let ah2 = ah.clone();
            tokio::spawn(async move {
                let img_res = tokio::task::spawn_blocking(move || {
                    load_image_pixels(&url_str)
                }).await.unwrap_or_else(|_| Err("Task failed".to_string()));
                
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app2) = ah2.upgrade() {
                        match img_res {
                            Ok((pixels, w, h)) => {
                                let buf = slint::SharedPixelBuffer::clone_from_slice(&pixels, w, h);
                                let img = slint::Image::from_rgba8(buf);
                                app2.set_donate_popup_image(img);
                            }
                            Err(e) => {
                                warn!("Failed to load donate QR: {}", e);
                            }
                        }
                        app2.set_donate_popup_loading(false);
                    }
                });
            });
        }
    });

    let ah = app_handle.clone();
    app.on_donate_copy_clicked(move |text| {
        if let Some(app) = ah.upgrade() {
            let _ = copy_to_clipboard(&text.to_string());
            app.set_task_status_text(app.get_donate_strings().copied);
            app.set_task_status_visible(true);
            
            let ah2 = ah.clone();
            slint::Timer::single_shot(std::time::Duration::from_secs(2), move || {
                if let Some(app2) = ah2.upgrade() {
                    app2.set_task_status_visible(false);
                }
            });
        }
    });
}

pub fn trigger_fetch(app_handle: slint::Weak<AppWindow>, _app_state: Arc<tokio::sync::Mutex<AppState>>) {
    debug!("donate: triggering donate data fetch");

    tokio::spawn(async move {
        // 1. Async API request (spawn_blocking wraps synchronous ureq)
        let resp = tokio::task::spawn_blocking(|| {
            crate::api::common::wslui_helper_donate()
        }).await.unwrap_or_default();

        // 2. Only keep items where enable == 1 and format is 1, 2, or 3
        let methods: Vec<_> = resp.payment_methods
            .into_iter()
            .filter(|m| m.enable == 1 && (m.format == 1 || m.format == 2 || m.format == 3 || m.format == 4))
            .collect();

        if methods.is_empty() {
            debug!("donate: no enabled payment methods");
            return;
        }

        // 3. Concurrently download icons for all supported channels
        let mut handles = Vec::new();
        for method in &methods {
            let url = method.icon.clone();
            handles.push(tokio::task::spawn_blocking(move || {
                load_image_pixels(&url)
            }));
        }

        // 4. Wait for parallel downloads, collect raw pixel bytes (slint::Image does not implement Send, must be constructed on UI thread)
        let mut raw_images: Vec<(Vec<u8>, u32, u32)> = Vec::new();
        let mut names = Vec::new();
        let mut urls = Vec::new();
        let mut exts = Vec::new();
        let mut formats = Vec::new();
        let mut regions = Vec::new();

        for (i, handle) in handles.into_iter().enumerate() {
            match handle.await {
                Ok(Ok((pixels, w, h))) => {
                    raw_images.push((pixels, w, h));
                    names.push(methods[i].name.clone());
                    urls.push(methods[i].url.clone());
                    exts.push(methods[i].ext.clone());
                    formats.push(methods[i].format);
                    regions.push(methods[i].region);
                }
                Ok(Err(e)) => {
                    warn!("donate: failed to load icon for {}: {}", methods[i].name, e);
                }
                Err(e) => {
                    warn!("donate: task panicked for {}: {}", methods[i].name, e);
                }
            }
        }

        // 5. Send raw pixel bytes to UI thread, construct slint::Image within event loop and refresh display
        if !raw_images.is_empty() {
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_handle.upgrade() {
                    let icons: Vec<slint::Image> = raw_images
                        .into_iter()
                        .map(|(pixels, w, h)| {
                            let buf = slint::SharedPixelBuffer::clone_from_slice(&pixels, w, h);
                            slint::Image::from_rgba8(buf)
                        })
                        .collect();
                    let names_slint: Vec<slint::SharedString> = names
                        .iter()
                        .map(|n| n.as_str().into())
                        .collect();
                    let urls_slint: Vec<slint::SharedString> = urls
                        .iter()
                        .map(|n| n.as_str().into())
                        .collect();
                    let exts_slint: Vec<slint::SharedString> = exts
                        .iter()
                        .map(|n| n.as_str().into())
                        .collect();
                    
                    app.set_donate_method_icons(
                        std::rc::Rc::new(slint::VecModel::from(icons)).into()
                    );
                    app.set_donate_method_names(
                        std::rc::Rc::new(slint::VecModel::from(names_slint)).into()
                    );
                    app.set_donate_method_urls(
                        std::rc::Rc::new(slint::VecModel::from(urls_slint)).into()
                    );
                    app.set_donate_method_exts(
                        std::rc::Rc::new(slint::VecModel::from(exts_slint)).into()
                    );
                    app.set_donate_method_formats(
                        std::rc::Rc::new(slint::VecModel::from(formats)).into()
                    );

                    // Build region badge images (SVG loaded at compile time)
                    let region_0_svg = include_bytes!("../../../assets/region/0.svg");
                    let region_86_svg = include_bytes!("../../../assets/region/86.svg");
                    let region_images: Vec<slint::Image> = regions
                        .iter()
                        .map(|&r| {
                            let svg_data: &[u8] = match r {
                                86 => region_86_svg,
                                _ => region_0_svg,
                            };
                            slint::Image::load_from_svg_data(svg_data).unwrap_or_default()
                        })
                        .collect();
                    app.set_donate_method_regions(
                        std::rc::Rc::new(slint::VecModel::from(region_images)).into()
                    );
                    app.set_donate_methods_ready(true);
                    debug!("donate: payment methods UI updated");
                }
            });
        }
    });
}

// Helper: thread-safe image download and decoding
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
