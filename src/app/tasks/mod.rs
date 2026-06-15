// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod sync_task;
pub mod popup_handler;
pub mod message_handler;
pub mod expiry_task;
pub mod update_task;
pub mod compat_task;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{trace};
use crate::{AppState, AppWindow};
use crate::ui::data::refresh_distros_ui;
use crate::config::models::CachedDistro;

pub use sync_task::{PopupSyncTask, MessageSyncTask};
pub use expiry_task::VersionExpiryTask;
pub use update_task::UpdateCheckTask;
pub use compat_task::WslCompatTask;

// Start WSL status monitoring task
pub fn spawn_wsl_monitor(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let ah = app_handle.clone();
            let as_ptr = app_state.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah.upgrade() {
                    let is_visible = app.get_is_window_visible();
                    // Only refresh if the Home tab (index 0) is selected to save resources
                    if app.get_selected_tab() == 0 {
                        let as_ptr_res = as_ptr.clone();
                        tokio::spawn(async move {
                            if crate::ui::data::should_refresh_wsl("periodic trigger", is_visible) {
                                let dashboard = {
                                    let state = as_ptr_res.lock().await;
                                    state.wsl_dashboard.clone()
                                };
                                let _ = dashboard.refresh_distros().await;
                            }
                        });
                    }
                }
            });
        }
    });
}

// Start USB status monitoring task
pub fn spawn_usb_monitor(app_handle: slint::Weak<AppWindow>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let ah = app_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah.upgrade() {
                    // Only refresh if the USB tab (index 2) is selected to save resources
                    if app.get_selected_tab() == 2 {
                        app.invoke_refresh_usb(false);
                    }
                }
            });
        }
    });
}

// Listen for distribution state changes and automatically update UI
pub fn spawn_state_listener(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        let mut last_refresh = std::time::Instant::now();
        const MIN_REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);
        
        loop {
            {
                let lock_timeout = std::time::Duration::from_millis(500);
                let state_changed = match tokio::time::timeout(lock_timeout, app_state.lock()).await {
                    Ok(state) => state.wsl_dashboard.state_changed().clone(),
                    Err(_) => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        continue;
                    }
                };
                state_changed.notified().await;
            }
            
            // Debounce: limit minimum refresh interval to reduce memory pressure
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(last_refresh);
            if elapsed < MIN_REFRESH_INTERVAL {
                tokio::time::sleep(MIN_REFRESH_INTERVAL - elapsed).await;
            }
            
            trace!("WSL state changed, updating UI...");
            let _ = refresh_distros_ui(app_handle.clone(), app_state.clone()).await;
            
            // Save updated distro list to cache for fast startup next time
            let app_state_for_cache = app_state.clone();
            tokio::spawn(async move {
                let lock_timeout = std::time::Duration::from_millis(500);
                let (distros, config_manager) = match tokio::time::timeout(lock_timeout, app_state_for_cache.lock()).await {
                    Ok(state) => (state.wsl_dashboard.get_distros().await, state.config_manager.clone()),
                    Err(_) => return,
                };
                
                let cached: Vec<CachedDistro> = distros.into_iter().map(|d| {
                    CachedDistro {
                        name: d.name,
                        status: format!("{:?}", d.status),
                        version: format!("{:?}", d.version),
                        is_default: d.is_default,
                    }
                }).collect();
                
                let _ = config_manager.update_cached_distros(cached);
                trace!("WSL distro list cache updated.");
            });
            
            last_refresh = std::time::Instant::now();
        }
    });
}

// Processing after application exit
pub async fn handle_app_exit(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    let auto_shutdown = app.get_auto_shutdown();
    if auto_shutdown {
        trace!("Auto-shutdown on exit is enabled, shutting down WSL...");
        let manager = {
            let state = app_state.lock().await;
            state.wsl_dashboard.clone()
        };
        manager.shutdown_wsl().await;
        trace!("WSL shut down completed");
    }
}

// Listen for wakeup requests from secondary instances (Named Pipe IPC)
pub fn spawn_wakeup_listener(app_handle: slint::Weak<AppWindow>) {
    #[cfg(target_os = "windows")]
    tokio::spawn(async move {
        use tokio::net::windows::named_pipe::ServerOptions;
        let pipe_name = r"\\.\pipe\wsldashboard_wakeup_pipe_v0.3";
        loop {
            let server = match ServerOptions::new().create(pipe_name) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to create named pipe server: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            if match server.connect().await {
                Ok(_) => true,
                Err(_) => false,
            } {
                tracing::info!("Received wakeup request via Named Pipe");
                let ah = app_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        crate::app::window::show_and_center(&app, false);
                    }
                });
            }
        }
    });
}

/// Block until the main window becomes visible
pub async fn wait_for_window_visible(app_handle: &slint::Weak<crate::AppWindow>) {
    loop {
        let is_visible = {
            let ah = app_handle.clone();
            let (tx, rx) = tokio::sync::oneshot::channel();
            // Safely read window visibility on the Slint UI thread
            let _ = slint::invoke_from_event_loop(move || {
                let visible = if let Some(app) = ah.upgrade() {
                    app.get_is_window_visible()
                } else {
                    false
                };
                let _ = tx.send(visible);
            });
            rx.await.unwrap_or(false)
        };
        
        if is_visible {
            break; // Window is visible, proceed immediately
        }
        // Window not visible, sleep 500ms before retrying (negligible overhead)
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}
