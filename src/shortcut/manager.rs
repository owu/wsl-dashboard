// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Global shortcut registration, unregistration and action dispatch.
//
// Windows `RegisterHotKey` binds a hotkey to the **calling thread's message queue**: if it is
// registered on the UI thread, `WM_HOTKEY` is consumed by Slint's winit event loop and can no
// longer be retrieved. This module therefore starts a
// dedicated message thread (`shortcut-worker`):
//
// 1. That thread creates its own message queue and calls `RegisterHotKey` on it;
// 2. `WM_HOTKEY` is retrieved by that thread's `GetMessageW` and dispatched to the action recorded at registration;
// 3. The UI thread requests register / unregister through a command queue and wakes the thread with `PostThreadMessageW` to process commands.
//
// Shortcuts stay active for the whole app runtime (including while the main window is hidden in the tray),
// so the worker thread is never stopped once started.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use once_cell::sync::Lazy;
use tracing::{error, info, warn};

use super::keymap::MOD_NOREPEAT;

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageW, PeekMessageW, PostThreadMessageW, MSG, PM_NOREMOVE, WM_HOTKEY,
};

// Message used to wake up the worker thread (WM_APP + 1)
const WAKE_MESSAGE: u32 = 0x8001;
// Maximum time to wait for a registration result
const REGISTER_TIMEOUT: Duration = Duration::from_millis(800);

// Action triggered by a shortcut
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutAction {
    // Toggle main window visibility
    ToggleWindow,
    // Open the terminal of the given distro
    Terminal(String),
}

// Registration result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    // Registration succeeded
    Registered,
    // Conflicts with another shortcut inside this app
    ConflictInApp,
    // Occupied by the system or another application
    ConflictOs,
    // Shortcut feature is currently unavailable (worker not started, etc.)
    Unavailable,
}

// Action callback (runs on the worker thread; switch back to the UI thread yourself)
pub type ActionCallback = Arc<dyn Fn(ShortcutAction) + Send + Sync + 'static>;

static ACTION_CALLBACK: Lazy<Mutex<Option<ActionCallback>>> = Lazy::new(|| Mutex::new(None));
// Whether the shortcut feature is ready (worker thread started successfully)
static ACTIVE: AtomicBool = AtomicBool::new(false);

struct WorkerHandle {
    thread_id: u32,
    cmd_tx: Sender<Cmd>,
    reply_rx: Receiver<Reply>,
}

#[derive(Default)]
struct WorkerState {
    worker: Option<WorkerHandle>,
    next_id: i32,
    // Occupied combos (`modifiers << 32 | key`) -> owning action id
    used: HashMap<u64, String>,
    // Successfully registered action ids -> (system hotkey id, modifiers, key)
    registered: HashMap<String, (i32, u32, u32)>,
}

static STATE: Lazy<Mutex<WorkerState>> = Lazy::new(|| Mutex::new(WorkerState::default()));

enum Cmd {
    Register {
        id: i32,
        action_key: String,
        modifiers: u32,
        key: u32,
        distro: Option<String>,
    },
    Unregister {
        id: i32,
    },
}

enum Reply {
    Register { id: i32, ok: bool },
}

// Store the action callback (injected by the UI layer at startup)
pub fn set_action_callback(callback: ActionCallback) {
    if let Ok(mut slot) = ACTION_CALLBACK.lock() {
        *slot = Some(callback);
    }
}

// Whether the shortcut feature is ready
pub fn is_active() -> bool {
    ACTIVE.load(Ordering::SeqCst)
}

fn lock_state() -> std::sync::MutexGuard<'static, WorkerState> {
    STATE.lock().unwrap_or_else(|e| e.into_inner())
}

// Start the worker thread (idempotent). Returns whether it is available.
pub fn start() -> bool {
    let mut state = lock_state();
    if state.worker.is_some() {
        ACTIVE.store(true, Ordering::SeqCst);
        return true;
    }
    let callback = match ACTION_CALLBACK.lock() {
        Ok(slot) => slot.clone(),
        Err(e) => e.into_inner().clone(),
    };
    let Some(callback) = callback else {
        error!("Shortcut worker not started: action callback is missing");
        return false;
    };

    let (cmd_tx, cmd_rx) = mpsc::channel::<Cmd>();
    let (reply_tx, reply_rx) = mpsc::channel::<Reply>();
    let (thread_id_tx, thread_id_rx) = mpsc::channel::<u32>();

    if let Err(e) = std::thread::Builder::new()
        .name("shortcut-worker".to_string())
        .spawn(move || {
            // Create this thread's message queue first, then report the thread id, so the UI thread does not call PostThreadMessageW too early
            let mut msg = MSG::default();
            unsafe {
                let _ = PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
            }
            let thread_id = unsafe { GetCurrentThreadId() };
            let _ = thread_id_tx.send(thread_id);
            worker_main(cmd_rx, reply_tx, callback);
        })
    {
        error!("Failed to spawn shortcut worker thread: {}", e);
        return false;
    }

    let thread_id = match thread_id_rx.recv_timeout(Duration::from_secs(2)) {
        Ok(id) => id,
        Err(e) => {
            error!("Shortcut worker did not report thread id: {}", e);
            return false;
        }
    };

    state.worker = Some(WorkerHandle {
        thread_id,
        cmd_tx,
        reply_rx,
    });
    ACTIVE.store(true, Ordering::SeqCst);
    info!("Shortcut worker started (thread id: {})", thread_id);
    true
}

fn wake(thread_id: u32) {
    if thread_id == 0 {
        return;
    }
    unsafe {
        if let Err(e) = PostThreadMessageW(thread_id, WAKE_MESSAGE, WPARAM(0), LPARAM(0)) {
            warn!("Failed to wake shortcut worker: {}", e);
        }
    }
}

fn encode(modifiers: u32, key: u32) -> u64 {
    ((modifiers as u64) << 32) | (key as u64)
}

// Request to register a global shortcut.
//
// - `action_key`: unique action id (`toggle-window` or a distro name), also used for occupancy detection
// - `distro`: when `Some`, this hotkey triggers opening the terminal of the given distro
pub fn request_register(action_key: &str, modifiers: u32, key: u32, distro: Option<String>) -> Outcome {
    if !is_active() {
        return Outcome::Unavailable;
    }
    let mut state = lock_state();
    let code = encode(modifiers, key);
    if let Some(owner) = state.used.get(&code) {
        return if owner == action_key {
            Outcome::Registered
        } else {
            Outcome::ConflictInApp
        };
    }
    // When the same action is rebound to another combo, unregister the old registration first to avoid leftovers in the system
    if let Some((old_id, old_modifiers, old_key)) = state.registered.remove(action_key) {
        state.used.remove(&encode(old_modifiers, old_key));
        if let Some(worker) = state.worker.as_ref() {
            // Commands are processed in FIFO order, so Unregister takes effect before the following Register
            let _ = worker.cmd_tx.send(Cmd::Unregister { id: old_id });
            wake(worker.thread_id);
        }
    }

    state.next_id += 1;
    let id = state.next_id;
    let cmd = Cmd::Register {
        id,
        action_key: action_key.to_string(),
        modifiers,
        key,
        distro,
    };

    // Send the command and wake the worker thread; the borrow ends with this block so `state.used` can be modified afterwards
    let reply = {
        let Some(worker) = state.worker.as_ref() else {
            return Outcome::Unavailable;
        };
        if worker.cmd_tx.send(cmd).is_err() {
            return Outcome::Unavailable;
        }
        wake(worker.thread_id);
        worker.reply_rx.recv_timeout(REGISTER_TIMEOUT)
    };

    match reply {
        Ok(Reply::Register { id: replied_id, ok }) if replied_id == id => {
            if ok {
                state.used.insert(code, action_key.to_string());
                state.registered.insert(action_key.to_string(), (id, modifiers, key));
                Outcome::Registered
            } else {
                Outcome::ConflictOs
            }
        }
        Ok(_) => Outcome::Unavailable,
        Err(_) => {
            warn!("Timed out waiting for shortcut registration result: {}", action_key);
            Outcome::Unavailable
        }
    }
}

// Request to unregister the shortcut of the given action. Returns whether a binding was actually removed.
pub fn request_unregister(action_key: &str) -> bool {
    let mut state = lock_state();
    let Some((id, modifiers, key)) = state.registered.remove(action_key) else {
        return false;
    };
    state.used.remove(&encode(modifiers, key));
    if let Some(worker) = state.worker.as_ref() {
        if worker.cmd_tx.send(Cmd::Unregister { id }).is_err() {
            return true;
        }
        wake(worker.thread_id);
    }
    true
}

// Worker thread main loop
fn worker_main(cmd_rx: Receiver<Cmd>, reply_tx: Sender<Reply>, callback: ActionCallback) {
    // Registered system hotkeys: (id, distro name (None means window toggle), action key)
    let mut registrations: Vec<(i32, Option<String>, String)> = Vec::new();

    loop {
        // 1) Process commands sent by the UI thread (non-blocking)
        loop {
            match cmd_rx.try_recv() {
                Ok(Cmd::Register {
                    id,
                    action_key,
                    modifiers,
                    key,
                    distro,
                }) => {
                    let ok = unsafe {
                        RegisterHotKey(None, id, HOT_KEY_MODIFIERS(modifiers | MOD_NOREPEAT), key)
                            .is_ok()
                    };
                    if ok {
                        info!("Registered global shortcut: {} (id: {})", action_key, id);
                        registrations.push((id, distro, action_key));
                    } else {
                        warn!("Failed to register global shortcut: {} (id: {})", action_key, id);
                    }
                    let _ = reply_tx.send(Reply::Register { id, ok });
                }
                Ok(Cmd::Unregister { id }) => {
                    if let Some((_, _, action_key)) = registrations
                        .iter()
                        .find(|(reg_id, _, _)| *reg_id == id)
                    {
                        info!("Unregistered global shortcut: {} (id: {})", action_key, id);
                    }
                    unsafe {
                        let _ = UnregisterHotKey(None, id);
                    }
                    registrations.retain(|(reg_id, _, _)| *reg_id != id);
                }
                Err(TryRecvError::Empty) => break,
                // Command channel disconnected (app exiting) -> end the thread
                Err(TryRecvError::Disconnected) => {
                    unregister_all(&registrations);
                    info!("Shortcut worker thread exited");
                    return;
                }
            }
        }

        // 2) Block waiting for a message (WM_HOTKEY lands in this thread's message queue)
        let mut msg = MSG::default();
        let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        if ret.0 == 0 || ret.0 == -1 {
            unregister_all(&registrations);
            info!("Shortcut worker thread exited");
            return;
        }
        if msg.message == WM_HOTKEY {
            let id = msg.wParam.0 as i32;
            if let Some((_, distro, _)) = registrations
                .iter()
                .find(|(reg_id, _, _)| *reg_id == id)
            {
                let action = match distro {
                    Some(name) => ShortcutAction::Terminal(name.clone()),
                    None => ShortcutAction::ToggleWindow,
                };
                callback(action);
            }
        }
    }
}

fn unregister_all(registrations: &[(i32, Option<String>, String)]) {
    if !registrations.is_empty() {
        info!(
            "Unregistering {} global shortcut(s) before worker exit",
            registrations.len()
        );
    }
    for (id, _, _) in registrations {
        unsafe {
            let _ = UnregisterHotKey(None, *id);
        }
    }
}
