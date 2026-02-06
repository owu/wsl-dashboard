use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, Icon, TrayIconEvent,
};
use crate::AppWindow;
use crate::i18n;
use tracing::{info, error};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// TrayIcon from tray-icon crate is not Send/Sync on some platforms (like Windows due to Rc/RefCell)
// But we are only accessing it from the main UI thread via Slint/Winit anyway.
struct TrayIconWrapper(#[allow(dead_code)] tray_icon::TrayIcon);
unsafe impl Send for TrayIconWrapper {}
unsafe impl Sync for TrayIconWrapper {}

static TRAY_ICON: Lazy<Mutex<Option<TrayIconWrapper>>> = Lazy::new(|| Mutex::new(None));

pub struct SystemTray;

impl SystemTray {
    pub fn initialize(app_weak: slint::Weak<AppWindow>, initially_visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing system tray via tray-icon...");

        let icon = Icon::from_resource(1, None)
            .or_else(|_| Icon::from_resource(101, None))
            .or_else(|_| Icon::from_path("assets/logo/logo.ico", None))
            .or_else(|_| Icon::from_resource(32512, None))
            .map_err(|e| {
                error!("Critical: All tray icon loading attempts failed: {}", e);
                format!("Failed to load tray icon: {}", e)
            })?;

        let tray_menu = Menu::new();
        let show_item = MenuItem::with_id("show", i18n::tr("tray.show_window", &[]), true, None);
        let exit_item = MenuItem::with_id("exit", i18n::tr("tray.exit", &[]), true, None);

        tray_menu.append(&show_item)?;
        tray_menu.append(&PredefinedMenuItem::separator())?;
        tray_menu.append(&exit_item)?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_menu_on_left_click(false)
            .with_tooltip(format!("{} v{}", crate::app::constants::APP_NAME, env!("CARGO_PKG_VERSION")))
            .with_icon(icon)
            .build()?;
            
        // Store the tray icon in the global static. 
        // If a previous icon existed, it will be dropped here, which removes it from the system tray.
        let mut global_tray = TRAY_ICON.lock().map_err(|e| format!("Failed to lock tray icon: {}", e))?;
        *global_tray = Some(TrayIconWrapper(tray));

        let app_weak_clone = app_weak.clone();

        // Ensure initial state represents the start_minimized configuration
        if let Some(app) = app_weak.upgrade() {
            app.set_is_window_visible(initially_visible);
        }

        let timer = slint::Timer::default();
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
            // Poll Tray Events (Toggle visibility on DoubleClick)
            while let Ok(event) = TrayIconEvent::receiver().try_recv() {
                match event {
                    TrayIconEvent::DoubleClick { .. } => {
                        if let Some(app) = app_weak_clone.upgrade() {
                            let is_visible = app.get_is_window_visible();
                            if is_visible {
                                info!("Tray double-clicked: hiding window");
                                app.set_is_window_visible(false);
                                crate::app::window::set_skip_taskbar(&app, true);
                            } else {
                                info!("Tray double-clicked: showing window");
                                crate::app::window::show_and_center(&app);
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Poll Menu Events
            while let Ok(event) = MenuEvent::receiver().try_recv() {
                match event.id.as_ref() {
                    "show" => {
                        if let Some(app) = app_weak_clone.upgrade() {
                            info!("Tray menu 'show' clicked");
                            crate::app::window::show_and_center(&app);
                        }
                    }
                    "exit" => {
                        info!("Exit requested from tray menu");
                        slint::quit_event_loop().unwrap();
                    }
                    _ => {}
                }
            }
        });
        
        std::mem::forget(timer);

        Ok(())
    }
}
