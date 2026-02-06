use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use crate::{AppState, AppWindow};
use crate::ui::data::refresh_distros_ui;

// Start WSL status monitoring task
pub fn spawn_wsl_monitor(app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        let manager = {
            let app_state = app_state.lock().await;
            app_state.wsl_dashboard.clone()
        };
        manager.start_monitoring().await;
    });
}

// Listen for distribution state changes and automatically update UI
pub fn spawn_state_listener(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        let mut last_refresh = std::time::Instant::now();
        const MIN_REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);
        
        loop {
            {
                let state = app_state.lock().await;
                let state_changed = state.wsl_dashboard.state_changed().clone();
                drop(state);
                state_changed.notified().await;
            }
            
            // Debounce: limit minimum refresh interval to reduce memory pressure
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(last_refresh);
            if elapsed < MIN_REFRESH_INTERVAL {
                tokio::time::sleep(MIN_REFRESH_INTERVAL - elapsed).await;
            }
            
            debug!("WSL state changed, updating UI...");
            let _ = refresh_distros_ui(app_handle.clone(), app_state.clone()).await;
            last_refresh = std::time::Instant::now();
        }
    });
}

// Processing after application exit
pub async fn handle_app_exit(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    let auto_shutdown = app.get_auto_shutdown();
    if auto_shutdown {
        debug!("Auto-shutdown on exit is enabled, shutting down WSL...");
        let manager = {
            let state = app_state.lock().await;
            state.wsl_dashboard.clone()
        };
        manager.shutdown_wsl().await;
        debug!("WSL shut down completed");
    }
}
