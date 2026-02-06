#[cfg(target_os = "windows")]
use tracing::info;
use crate::AppWindow;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowRect, GetWindowThreadProcessId, 
    SetWindowPos, GetWindow, GW_OWNER, SWP_NOSIZE, SWP_NOZORDER, HWND_TOP,
    GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_APPWINDOW,
    ShowWindow, SW_HIDE, SW_SHOW, SWP_FRAMECHANGED, SWP_NOMOVE, GetWindowTextW,
    GetClassNameW, SetForegroundWindow, SW_RESTORE, SetWindowTextW, SetLayeredWindowAttributes,
    LWA_ALPHA
};
use windows::Win32::UI::WindowsAndMessaging::WS_EX_LAYERED;
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTOPRIMARY};

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

            // ABSOLUTE IDENTIFIER: Slint UI is the only one we care about.
            if owner.0.is_null() && (title.contains("WINDOW_UI") || title == "WSL Dashboard Main" || class_name.contains("Slint")) {
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
            if owner.0.is_null() && (class_name.contains("Slint") || class_name.contains("Window")) {
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
pub fn set_skip_taskbar(_app: &crate::AppWindow, skip: bool) {
    info!("set_skip_taskbar(skip={})", skip);
    std::thread::spawn(move || {
        for _i in 0..15 {
            if let Some(hwnd) = find_main_window() {
                unsafe {
                    // Hide first to ensure taskbar refresh
                    let _ = ShowWindow(hwnd, SW_HIDE);
                    
                    let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                    if skip {
                        ex_style |= WS_EX_TOOLWINDOW.0;
                        ex_style &= !WS_EX_APPWINDOW.0;
                    } else {
                        ex_style &= !WS_EX_TOOLWINDOW.0;
                        ex_style |= WS_EX_APPWINDOW.0;
                    }
                    SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
                    
                    let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, 0, 0, 
                                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
                    
                    if !skip {
                        let _ = ShowWindow(hwnd, SW_RESTORE);
                        let _ = ShowWindow(hwnd, SW_SHOW);
                        let _ = SetForegroundWindow(hwnd);
                    }
                    return;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
    });
}

#[cfg(target_os = "windows")]
pub fn set_window_opacity(opacity: u8) {
    if let Some(hwnd) = find_main_window() {
        unsafe {
            let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
            ex_style |= WS_EX_LAYERED.0;
            let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
            let _ = SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), opacity, LWA_ALPHA);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_window_opacity(_opacity: u8) {}

#[cfg(not(target_os = "windows"))]
pub fn set_skip_taskbar(_app: &crate::AppWindow, _skip: bool) {}

pub fn show_and_center(app: &AppWindow) {
    use slint::ComponentHandle;
    info!("show_and_center requested");
    
    #[cfg(target_os = "windows")]
    {
        // 1. Try to find the window handle BEFORE it's shown
        // We poll aggressively for a short time
        for _ in 0..20 {
            if let Some(hwnd) = find_main_window() {
                // Instantly hide it via opacity and move it off-screen 
                // just in case Slint's show() is faster than our next steps
                set_window_opacity(0);
                unsafe {
                    let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), -32000, -32000, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
                }
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        
        // 2. Initial show (it will be invisible at -32000 if step 1 succeeded)
        app.set_is_window_visible(true);
        let _ = app.show();

        set_skip_taskbar(app, false);
        rename_app_windows();

        std::thread::spawn(move || {
            // We give it a few more retries to ensure we have the final window handle and correct size
            for _ in 0..50 {
                if let Some(hwnd) = find_main_window() {
                    unsafe {
                        let mut rect = RECT::default();
                        if GetWindowRect(hwnd, &mut rect).is_ok() {
                            let w = rect.right - rect.left;
                            let h = rect.bottom - rect.top;
                            
                            // Only center if we have a valid size (Slint might take a moment to layout)
                            if w > 100 && h > 100 { 
                                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                                let mut monitor_info = MONITORINFO {
                                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                                    ..Default::default()
                                };
                                
                                if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() {
                                    let mr = monitor_info.rcWork;
                                    let x = mr.left + (mr.right - mr.left - w) / 2;
                                    let y = mr.top + (mr.bottom - mr.top - h) / 2;
                                    
                                    // 3. Move window to the ACTUAL center
                                    let _ = SetWindowPos(hwnd, HWND_TOP, x, y, 0, 0, SWP_NOSIZE);
                                    
                                    // 4. Final reveal
                                    let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                                    ex_style |= WS_EX_LAYERED.0;
                                    let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
                                    let _ = SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), 255, LWA_ALPHA);
                                    
                                    let _ = ShowWindow(hwnd, SW_SHOW);
                                    let _ = SetForegroundWindow(hwnd);
                                    info!("Window centered and revealed at {}, {}", x, y);
                                    return;
                                }
                            }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        app.set_is_window_visible(true);
        let _ = app.show();
    }
}

pub fn activate_window_by_hwnd(hwnd: HWND) {
    #[cfg(target_os = "windows")]
    unsafe {
        let _ = ShowWindow(hwnd, SW_HIDE);
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        ex_style &= !WS_EX_TOOLWINDOW.0;
        ex_style |= WS_EX_APPWINDOW.0;
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style as i32);
        let _ = SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
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
        let is_slint_ui = owner.0.is_null() && (title.contains("WINDOW_UI") || title == "WSL Dashboard Main" || class_name.contains("Slint"));

        if is_slint_ui {
            let new_title: Vec<u16> = "WSL Dashboard Main\0".encode_utf16().collect();
            let _ = unsafe { SetWindowTextW(hwnd, windows::core::PCWSTR(new_title.as_ptr())) };
            
            // Ensure Main UI has APPWINDOW and NOT TOOLWINDOW (unless specifically skipped)
            // But let set_skip_taskbar handle the skip logic later.
        } else {
            // FORCE CLEANUP OF TRAY WINDOWS
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
