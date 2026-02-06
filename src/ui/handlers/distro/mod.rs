use std::sync::Arc;
use tokio::sync::Mutex;
use rand::{Rng, distr::Alphanumeric};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::{AppWindow, AppState, i18n};

pub mod lifecycle;
pub mod manage;
pub mod export;
pub mod clone;
pub mod clone_logic;
pub mod install;
pub mod install_logic;
pub mod move_distro;
pub mod move_logic;
pub mod settings_logic;

pub fn sanitize_instance_name(name: &str) -> String {
    let mut sanitized: String = name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect();
    
    if sanitized.len() > 25 {
        sanitized.truncate(25);
    }
    sanitized
}

pub fn generate_random_suffix(name: &str) -> String {
    let random_suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    format!("{}_{}", name, random_suffix)
}

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    lifecycle::setup(app, app_handle.clone(), app_state.clone());
    manage::setup(app, app_handle.clone(), app_state.clone());
    export::setup(app, app_handle.clone(), app_state.clone());
    clone::setup(app, app_handle.clone(), app_state.clone());
    install::setup(app, app_handle.clone(), app_state.clone());
    move_distro::setup(app, app_handle.clone(), app_state.clone());
}

pub fn spawn_file_size_monitor(
    ah: slint::Weak<AppWindow>,
    file_path: String,
    distro_name: String,
    i18n_key: String,
    stop_signal: Arc<AtomicBool>,
) {
    tokio::spawn(async move {
        while !stop_signal.load(Ordering::Relaxed) {
            let size_mb = if let Ok(metadata) = std::fs::metadata(&file_path) {
                metadata.len() / (1024 * 1024)
            } else {
                0
            };
            
            let ah_inner = ah.clone();
            let distro_name_inner = distro_name.clone();
            let i18n_key_inner = i18n_key.clone();
            
            let size_str = format!("{} MB", size_mb);
            
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_inner.upgrade() {
                    let msg = i18n::tr(&i18n_key_inner, &[distro_name_inner, size_str]);
                    app.set_task_status_text(msg.into());
                }
            });
            
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
}
