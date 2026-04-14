use crate::AppWindow;
#[cfg(target_os = "windows")]
use tracing::{error, info};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::WindowsAndMessaging::WS_EX_LAYERED;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GW_OWNER, GWL_EXSTYLE, GetClassNameW, GetWindow, GetWindowLongW, GetWindowRect,
    GetWindowTextW, GetWindowThreadProcessId, HWND_TOP, IsIconic, LWA_ALPHA, SW_HIDE, SW_RESTORE,
    SW_SHOW, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetForegroundWindow,
    SetLayeredWindowAttributes, SetWindowLongW, SetWindowPos, SetWindowTextW, ShowWindow,
    WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

#[cfg(target_os = "windows")]
struct EnumWindowData {
    target_pid: u32,
    main_window: Option<HWND>,
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_main_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let data = &mut *(lparam.0 as *mut EnumWindowData);
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if pid == data.target_pid {
            let mut title_buf: [u16; 512] = [0; 512];
            let title_len = GetWindowTextW(hwnd, &mut title_buf);
            let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

            let mut class_buf: [u16; 256] = [0; 256];
            let class_len = GetClassNameW(hwnd, &mut class_buf);
            let class_name = String::from_utf16_lossy(&class_buf[..class_len as usize]);

            let owner = GetWindow(hwnd, GW_OWNER).unwrap_or(HWND(std::ptr::null_mut()));

            // LOG EVERY MATCHING PID WINDOW
            info!("Found window for PID {}: Title='{}', Class='{}', Owner={:?}", data.target_pid, title, class_name, owner);

            // ABSOLUTE IDENTIFIER: Slint UI is the only one we care about.
            // Exclude tray helper windows and event targets
            let is_tray = title.contains("Tray") || class_name.contains("tray_icon_app") || class_name.contains("Target");
            if !is_tray && owner.0.is_null() && (title.contains("WINDOW_UI") || title == "WSL Dashboard Main" || class_name.contains("Slint")) {
                data.main_window = Some(hwnd);
                return BOOL(0); // Stop enumeration
            }
        }
        BOOL(1)
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_fallback_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let data = &mut *(lparam.0 as *mut EnumWindowData);
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if pid == data.target_pid {
            let mut class_buf: [u16; 256] = [0; 256];
            let class_len = GetClassNameW(hwnd, &mut class_buf);
            let class_name = String::from_utf16_lossy(&class_buf[..class_len as usize]);

            let owner = GetWindow(hwnd, GW_OWNER).unwrap_or(HWND(std::ptr::null_mut()));

            // In extreme cold start, title might be empty, so we trust class name + no owner
            // Strictly exclude known background helper classes
            let is_helper = class_name.contains("tray_icon_app") || class_name.contains("Target");
            if !is_helper && owner.0.is_null() && (class_name.contains("Slint") || class_name.contains("Window")) {
                data.main_window = Some(hwnd);
                return BOOL(0); // Stop enumeration
            }
        }
        BOOL(1)
    }
}

#[cfg(target_os = "windows")]
fn find_main_window() -> Option<HWND> {
    let mut data = EnumWindowData {
        target_pid: std::process::id(),
        main_window: None,
    };
    unsafe {
        let _ = EnumWindows(Some(enum_main_window_proc), LPARAM(&mut data as *mut _ as isize));
    }
    
    // Fallback: If not found by specific criteria, take the first top-level window of this process
    // that is likely the Slint UI (not a tray helper)
    if data.main_window.is_none() {
        unsafe {
            let _ = EnumWindows(Some(enum_fallback_window_proc), LPARAM(&mut data as *mut _ as isize));
        }
    }
    data.main_window
}

#[cfg(target_os = "windows")]
fn hide_window_completely(hwnd: HWND) {
    unsafe {
        // 1. Hide it first
        let _ = ShowWindow(hwnd, SW_HIDE);
        
        // 2. Set opacity to 0 and move off-screen
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        ex_style |= WS_EX_LAYERED.0;
        let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
        let _ = SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), 0, LWA_ALPHA);
        let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), -32000, -32000, 0, 0, SWP_NOSIZE | SWP_NOZORDER);

        // 3. Remove from taskbar
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        ex_style |= WS_EX_TOOLWINDOW.0;
        ex_style &= !WS_EX_APPWINDOW.0;
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
        
        let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, 0, 0, 
                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
        info!("Window logically and physically hidden from desktop and taskbar.");
    }
}

#[cfg(target_os = "windows")]
pub fn set_skip_taskbar(_app: &crate::AppWindow, skip: bool) {
    info!("set_skip_taskbar(skip={})", skip);
    std::thread::spawn(move || {
        for _i in 0..20 {
            if let Some(hwnd) = find_main_window() {
                unsafe {
                    // Hide first to ensure taskbar refresh
                    let _ = ShowWindow(hwnd, SW_HIDE);

                    if skip {
                        hide_window_completely(hwnd);
                    } else {
                        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                        ex_style &= !WS_EX_TOOLWINDOW.0;
                        ex_style |= WS_EX_APPWINDOW.0;
                        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
                        
                        let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, 0, 0, 
                                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
                        
                        let _ = ShowWindow(hwnd, SW_RESTORE);
                        let _ = ShowWindow(hwnd, SW_SHOW);
                        let _ = SetForegroundWindow(hwnd);
                    }
                }
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

#[cfg(not(target_os = "windows"))]
pub fn set_window_opacity(_opacity: u8) {}

#[cfg(target_os = "windows")]
fn set_window_opacity_by_hwnd(hwnd: HWND, opacity: u8) {
    unsafe {
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        ex_style |= WS_EX_LAYERED.0;
        let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
        let _ = SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), opacity, LWA_ALPHA);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_skip_taskbar(_app: &crate::AppWindow, _skip: bool) {}

pub fn show_and_center(app: &AppWindow, silent: bool) {
    use slint::ComponentHandle;
    info!("show_and_center requested (silent={})", silent);
    
    #[cfg(target_os = "windows")]
    {
        if silent {
            info!("Silent mode: Skipping app.show() to keep window hidden.");
            app.window().set_minimized(true);
            app.window().set_position(slint::LogicalPosition::new(-32000.0, -32000.0));
            app.set_is_window_visible(false);
            
            // CRITICAL: Even in silent mode, we must rename and hide any existing helper windows (like tray) 
            // so they don't appear as blank icons in the taskbar. 
            // Since windows might appear asynchronously, we use a background sweep for the first 2 seconds.
            std::thread::spawn(move || {
                for i in 0..6 {
                    rename_app_windows();
                    let delay = match i {
                        0 => 50,
                        1 => 200,
                        2 => 500,
                        _ => 1000,
                    };
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                }
            });
            return;
        }

        // 1. Initial manual hide: opacity 0 + offscreen to prevent flicker during Slint's show()
        // We poll briefly for a handle if it exists, otherwise Slint's app.show() will create it.
        for _ in 0..10 {
            if let Some(hwnd) = find_main_window() {
                set_window_opacity_by_hwnd(hwnd, 0);
                unsafe {
                    let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), -32000, -32000, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
                }
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        app.set_is_window_visible(true);
        let _ = app.show();

        // 2. CONSOLIDATED REVEAL FLOW: ONE thread to handle positioning, styling, and activation
        std::thread::spawn(move || {
            // We give it plenty of retries to ensure Slint has fully initialized the window
            for _ in 0..60 {
                if let Some(hwnd) = find_main_window() {
                    unsafe {
                        // If the window is minimized (iconic), restore it first so GetWindowRect returns real dimensions
                        if IsIconic(hwnd).as_bool() {
                            let _ = ShowWindow(hwnd, SW_RESTORE);
                            // Give Windows a moment to process the restore
                            std::thread::sleep(std::time::Duration::from_millis(50));
                        }

                        let mut rect = RECT::default();
                        if GetWindowRect(hwnd, &mut rect).is_ok() {
                            let w = rect.right - rect.left;
                            let h = rect.bottom - rect.top;
                            
                            // Only proceed if Slint has performed layout (size > 100x100)
                            if w > 100 && h > 100 { 
                                // A. Set correct taskbar style (APPWINDOW)
                                let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                                ex_style &= !WS_EX_TOOLWINDOW.0;
                                ex_style |= WS_EX_APPWINDOW.0;
                                SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
                                
                                // B. Calculate center position on current monitor
                                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                                let mut monitor_info = MONITORINFO {
                                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                                    ..Default::default()
                                };
                                
                                if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() {
                                    let mr = monitor_info.rcWork;
                                    let x = mr.left + (mr.right - mr.left - w) / 2;
                                    let y = mr.top + (mr.bottom - mr.top - h) / 2;
                                    
                                    // C. Move to center AND bring to top layer
                                    let _ = SetWindowPos(hwnd, HWND_TOP, x, y, 0, 0, SWP_NOSIZE | SWP_FRAMECHANGED);
                                    
                                    // D. RENAME (Internal bookkeeping)
                                    rename_app_windows();

                                    // E. FINAL Activation: Show, Restore, and FORCE Foreground
                                    let _ = ShowWindow(hwnd, SW_RESTORE);
                                    let _ = ShowWindow(hwnd, SW_SHOW);
                                    let _ = SetForegroundWindow(hwnd);
                                    
                                    // F. Reveal: Transition opacity back to 255
                                    set_window_opacity_by_hwnd(hwnd, 255);

                                    info!("Window successfully centered at {}, {} and robustly activated.", x, y);
                                    return;
                                }
                            }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            error!("Failed to find or activate main window within timeout.");
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        if !silent {
            app.set_is_window_visible(true);
            let _ = app.show();
        }
    }
}

pub fn activate_window_by_hwnd(hwnd: HWND) {
    #[cfg(target_os = "windows")]
    unsafe {
        // STYLE FIRST: Ensure it's not a tool window and is an app window so it appears in taskbar correctly
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        ex_style &= !WS_EX_TOOLWINDOW.0;
        ex_style |= WS_EX_APPWINDOW.0;
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
        
        // Refresh frame and bring to top position
        let _ = SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED);
        
        // Activation: Restore if minimized, then show and focus
        let _ = ShowWindow(hwnd, SW_RESTORE);
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = SetForegroundWindow(hwnd);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn activate_window_by_hwnd(_hwnd: windows::Win32::Foundation::HWND) {}

#[cfg(target_os = "windows")]
unsafe extern "system" fn rename_windows_final_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let target_pid = lparam.0 as u32;
    let mut pid: u32 = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)); }

    if pid == target_pid {
        let mut title_buf: [u16; 512] = [0; 512];
        let title_len = unsafe { GetWindowTextW(hwnd, &mut title_buf) };
        let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

        let mut class_buf: [u16; 256] = [0; 256];
        let class_len = unsafe { GetClassNameW(hwnd, &mut class_buf) };
        let class_name = String::from_utf16_lossy(&class_buf[..class_len as usize]);

        let owner = unsafe { GetWindow(hwnd, GW_OWNER).unwrap_or(HWND(std::ptr::null_mut())) };

        // EXPLICIT IDENTIFICATION
        let is_tray = title.contains("Tray") || class_name.contains("tray_icon_app") || class_name.contains("Target");
        let is_slint_ui = !is_tray && owner.0.is_null() && (title.contains("WINDOW_UI") || title == "WSL Dashboard Main" || class_name.contains("Slint"));

        if is_slint_ui {
            let new_title: Vec<u16> = "WSL Dashboard Main\0".encode_utf16().collect();
            let _ = unsafe { SetWindowTextW(hwnd, windows::core::PCWSTR(new_title.as_ptr())) };
            
            // Ensure Main UI has APPWINDOW and NOT TOOLWINDOW (unless specifically skipped)
            // But let set_skip_taskbar handle the skip logic later.
        } else {
            // FORCE CLEANUP OF TRAY WINDOWS - but be careful not to hide things that look like Slint windows
            // even if they don't have the final title yet.
            if !class_name.contains("Slint") && !class_name.contains("Window") {
                let new_title: Vec<u16> = "WSL Dashboard Tray\0".encode_utf16().collect();
                let _ = unsafe { SetWindowTextW(hwnd, windows::core::PCWSTR(new_title.as_ptr())) };
                
                // CRITICAL: Force helper windows to NOT show in taskbar
                let mut ex_style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 };
                ex_style |= WS_EX_TOOLWINDOW.0;
                ex_style &= !WS_EX_APPWINDOW.0;
                unsafe {
                    SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
                    let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, 0, 0, 
                                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
                    
                    // Explicitly hide to force taskbar refresh
                    let _ = ShowWindow(hwnd, SW_HIDE);
                }
            } else {
                info!("Skipping HIDE for potential Slint window: title='{}', class='{}'", title, class_name);
            }
        }
    }
    BOOL(1)
}

#[cfg(target_os = "windows")]
pub fn rename_app_windows() {
    let pid = std::process::id();
    unsafe {
        let _ = EnumWindows(Some(rename_windows_final_proc), LPARAM(pid as isize));
    }
}

#[cfg(not(target_os = "windows"))]
pub fn rename_app_windows() {}
