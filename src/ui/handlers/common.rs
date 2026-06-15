// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{AppWindow, AppState};
use tracing::debug;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_select_tab(move |tab| {
        if let Some(app) = ah.upgrade() {
            app.set_selected_tab(tab);

            // Tab 1 is "Add an instance"
            if tab == 1 {
                let as_ptr = as_ptr.clone();
                let ah = ah.clone();
                slint::spawn_local(async move {
                    let state = as_ptr.lock().await;
                    let settings = state.config_manager.get_settings();
                    let location = settings.distro_location.clone();
                    drop(state);

                    if let Some(app) = ah.upgrade() {
                        app.set_distro_location(location.clone().into());
                        
                        let current_name = app.get_new_instance_name().to_string();
                        let final_path = if !current_name.is_empty() {
                            std::path::Path::new(&location).join(&current_name).to_string_lossy().to_string()
                        } else {
                            location
                        };

                        app.set_new_instance_path(final_path.into());
                    }
                }).unwrap();
            }

            // Tab 2 is "USB Devices"
            if tab == 2 {
                app.invoke_refresh_usb(true);
            }

            // Tab 3 is "Network"
            if tab == 3 {
                app.invoke_check_network_task_status();
                let ah = ah.clone();
                let as_ptr = as_ptr.clone();
                tokio::spawn(async move {
                    crate::ui::handlers::network::refresh_network_view_data(ah, as_ptr).await;
                });
            }

            // Tab 6 is "About" — fetch helper about info once on first visit
            if tab == 6 {
                super::about::trigger_fetch(ah.clone(), as_ptr.clone());
            }

            // Tab 4 is "Donate" — fetch donate payment methods once on first visit
            if tab == 4 {
                super::donate::trigger_fetch(ah.clone(), as_ptr.clone());
            }
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_save_sidebar_state(move || {
        if let Some(app) = ah.upgrade() {
            let collapsed = app.get_sidebar_collapsed();
            let as_ptr = as_ptr.clone();
            debug!("Saving sidebar state: {}", collapsed);
            tokio::spawn(async move {
                let mut state = as_ptr.lock().await;
                let mut settings = state.config_manager.get_settings().clone();
                if settings.sidebar_collapsed != collapsed {
                    settings.sidebar_collapsed = collapsed;
                    let _ = state.config_manager.update_settings(settings);
                }
            });
        }
    });

    app.on_open_url(move |url| {
        let _ = open::that(url.as_str());
    });

    let ah = app_handle.clone();
    app.on_close_task_status(move || {
        if let Some(app) = ah.upgrade() {
            app.set_task_status_visible(false);
        }
    });
}
