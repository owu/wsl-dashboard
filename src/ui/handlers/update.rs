// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
use slint::ComponentHandle;
use crate::{AppWindow, AppState, AppInfo};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, _app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let _as_download = _app_state.clone();
    
    app.on_check_update(move || {
        info!("Manual check update triggered from UI");
        let ah = ah.clone();
        
        // Directly use the compile-time version number constant, or obtain the version number on the UI thread.
        // This eliminates the need to call ah.upgrade() on a background thread.
        let current_v = env!("CARGO_PKG_VERSION").to_string();
        
        tokio::spawn(async move {
            info!("Starting manual check for version: {}", current_v);

            // Set checking_update to true on the UI thread
            let _ = slint::invoke_from_event_loop({
                let ah = ah.clone();
                move || {
                    if let Some(app) = ah.upgrade() {
                        app.global::<AppInfo>().set_checking_update(true);
                    }
                }
            });

            match crate::app::updater::check_update(&current_v).await {
                Ok(result) => {
                    let has_update = result.has_update;
                    let latest_version = result.latest_version.clone();
                    let release_date = result.release_date.clone();
                    
                    let _ = slint::invoke_from_event_loop({
                        let ah = ah.clone();
                        move || {
                            if let Some(app) = ah.upgrade() {
                                info!("Update check result: has_update={}", has_update);
                                app.global::<AppInfo>().set_checking_update(false);
                                app.global::<AppInfo>().set_has_update(has_update);
                                app.global::<AppInfo>().set_latest_version(latest_version.clone().into());
                                app.global::<AppInfo>().set_latest_release_date(release_date.clone().into());
                                app.global::<AppInfo>().set_update_download_url(format!("{}{}", crate::app::PROJECT_WEBSITE, crate::app::DOWNLOAD_URI).into());
                                
                                if has_update {
                                    app.set_show_update_dialog(true);
                                } else {
                                    app.set_current_message(crate::i18n::tr("dialog.update_latest", &[latest_version.clone(), release_date.clone()]).into());
                                    app.set_show_message_dialog(true);
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    warn!("Manual check update failed: {}", e);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = ah.upgrade() {
                            app.global::<AppInfo>().set_checking_update(false);
                            
                            let message = if e == "RequestTimeOut" {
                                crate::i18n::t("dialog.update_timeout")
                            } else {
                                crate::i18n::tr("dialog.update_failed", &[e])
                            };

                            app.set_current_message(message.into());
                            app.set_show_message_dialog(true);
                        }
                    });
                }
            }
        });
    });

    let ah_download = app_handle.clone();
    app.on_download_update(move || {
        let ah = ah_download.clone();
        slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                let update_url = app.global::<AppInfo>().get_update_download_url().to_string();
                if !update_url.is_empty() {
                    info!("Downloading update from update_download_url: {}", update_url);
                    let _ = open::that(update_url);
                } else {
                    let url = format!("{}{}", crate::app::PROJECT_WEBSITE, crate::app::DOWNLOAD_URI);
                    let _ = open::that(url);
                }
            }
        }).unwrap();
    });

    let ah_close = app_handle.clone();
    app.on_close_expire_dialog(move || {
        if let Some(app) = ah_close.upgrade() {
            app.set_show_expire_dialog(false);
        }
    });
}
