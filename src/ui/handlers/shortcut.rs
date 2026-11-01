// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Business logic for the shortcuts (Global Shortcut) tab.
//
// Responsible for: recording keys, validating conflicts, writing `shortcuts.toml`, registering /
// unregistering global shortcuts, dispatching shortcut actions (toggle window, open distro terminal),
// and cleaning up validation on distro delete and startup.
//
// The shortcut tab has no Save button: every recording / clearing takes effect immediately.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use slint::{ComponentHandle, ModelRc, VecModel};
use tokio::sync::Mutex as TokioMutex;
use tracing::{error, info, warn};

use crate::shortcut::config::{self, ShortcutConfigFile, ShortcutEntry};
use crate::shortcut::keymap::{self, ParseError, KeyStroke};
use crate::shortcut::manager::{self, ShortcutAction, Outcome};
use crate::i18n;
use crate::utils::icon_mapper;
use crate::{AppState, AppWindow, DistroOption, ShortcutRow};

// Action id of "show / hide main window"
const ACTION_TOGGLE_WINDOW: &str = "toggle-window";
// Esc pressed during recording (Slint's Esc key text)
const KEY_ESCAPE: &str = "\u{001b}";
// Duration the toast is shown
const TOAST_DURATION: std::time::Duration = std::time::Duration::from_secs(3);

// A single shortcut binding row in the UI state. Slot 0 is always "show / hide main window", followed by distro rows.
#[derive(Clone)]
struct RowState {
    // `None` means "show / hide main window"; `Some(name)` is a distro
    distro: Option<String>,
    shortcut: String,
    modifiers: u32,
    key: u32,
    // Whether it was registered successfully in this session
    registered: bool,
}

impl RowState {
    fn action_key(&self) -> String {
        self.distro
            .clone()
            .unwrap_or_else(|| ACTION_TOGGLE_WINDOW.to_string())
    }

    // Whether a shortcut is bound
    fn has_binding(&self) -> bool {
        self.key != 0 && !self.shortcut.trim().is_empty()
    }

    fn entry(&self) -> Option<ShortcutEntry> {
        self.has_binding().then(|| ShortcutEntry {
            shortcut: self.shortcut.clone(),
            modifiers: self.modifiers,
            key: self.key,
            enabled: true,
        })
    }
}

// UI state of the shortcut tab
#[derive(Default)]
struct UiState {
    // Distros installed on the system
    distros: Vec<String>,
    rows: Vec<RowState>,
    // Whether the initial load has completed
    loaded: bool,
    // Slot currently being recorded
    recording: Option<usize>,
    // Hint text inside the recorder
    message: String,
    // Slot that has the conflict hint
    conflict: Option<usize>,
    // Conflict hint text
    conflict_text: String,
}

static UI_STATE: Lazy<Mutex<UiState>> = Lazy::new(|| Mutex::new(UiState::default()));

fn lock_ui() -> std::sync::MutexGuard<'static, UiState> {
    UI_STATE.lock().unwrap_or_else(|e| e.into_inner())
}

// Purpose of fetching the distro list
#[derive(Clone, Copy)]
enum FetchPurpose {
    // Initial load: read config, clean up stale bindings, register according to config
    InitialLoad,
    // Refresh the distro dropdown options
    RefreshDistros,
}

pub fn setup(
    app: &AppWindow,
    app_handle: slint::Weak<AppWindow>,
    app_state: Arc<TokioMutex<AppState>>,
) {
    // Shortcut action callback: worker thread -> UI thread
    manager::set_action_callback({
        let handle = app_handle.clone();
        Arc::new(move |action: ShortcutAction| dispatch_action(&handle, action))
    });

    let handle = app_handle.clone();
    let state = app_state.clone();
    app.on_shortcut_tab_activated(move || {
        spawn_distro_fetch(handle.clone(), state.clone(), FetchPurpose::RefreshDistros);
    });

    let handle = app_handle.clone();
    app.on_shortcut_start_recording(move |slot| {
        if let Some(app) = handle.upgrade() {
            start_recording(&app, slot);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_key_input(move |text, ctrl, alt, shift, meta| {
        if let Some(app) = handle.upgrade() {
            handle_key_input(&app, text.as_str(), ctrl, alt, shift, meta);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_cancel_recording(move || {
        if let Some(app) = handle.upgrade() {
            cancel_recording(&app);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_clear(move |slot| {
        if let Some(app) = handle.upgrade() {
            clear_binding(&app, slot);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_remove_row(move |slot| {
        if let Some(app) = handle.upgrade() {
            remove_row(&app, slot);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_add_row(move || {
        if let Some(app) = handle.upgrade() {
            add_row(&app);
        }
    });

    let handle = app_handle.clone();
    app.on_shortcut_distro_selected(move |slot, option_index| {
        if let Some(app) = handle.upgrade() {
            select_distro(&app, slot, option_index);
        }
    });

    // Load the config and register the global shortcuts at app startup (also effective while the window is hidden in the tray)
    spawn_distro_fetch(app_handle, app_state, FetchPurpose::InitialLoad);
}

// ---------------------------------------------------------------------------
// Action dispatch
// ---------------------------------------------------------------------------

fn dispatch_action(handle: &slint::Weak<AppWindow>, action: ShortcutAction) {
    let handle = handle.clone();
    let _ = slint::invoke_from_event_loop(move || {
        let Some(app) = handle.upgrade() else {
            warn!("Global shortcut fired but the window is gone, action ignored");
            return;
        };
        match action {
            ShortcutAction::ToggleWindow => toggle_window(&app),
            ShortcutAction::Terminal(name) => {
                info!("Shortcut triggered: open terminal of distro '{}'", name);
                app.invoke_terminal_distro(name.into());
            }
        }
    });
}

// Toggle main window visibility (equivalent to double-clicking the tray icon)
fn toggle_window(app: &AppWindow) {
    let visible = app.get_is_window_visible() && !app.window().is_minimized();
    info!(
        "Shortcut triggered: toggle main window (currently {})",
        if visible { "visible" } else { "hidden" }
    );
    if visible {
        app.set_is_window_visible(false);
        crate::app::window::set_skip_taskbar(app, true);
    } else {
        crate::app::window::show_and_center(app, false);
    }
}

// ---------------------------------------------------------------------------
// Distro list and initial load
// ---------------------------------------------------------------------------

fn spawn_distro_fetch(
    handle: slint::Weak<AppWindow>,
    app_state: Arc<TokioMutex<AppState>>,
    purpose: FetchPurpose,
) {
    tokio::spawn(async move {
        let executor = {
            let state = app_state.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        let result = executor.list_distros().await;
        let error = result.error.clone();
        let distros = result
            .data
            .map(|list| list.into_iter().map(|d| d.name).collect::<Vec<_>>());
        if distros.is_none() {
            warn!("Failed to list distros for shortcut settings: {:?}", error);
        }
        let _ = slint::invoke_from_event_loop(move || {
            let Some(app) = handle.upgrade() else { return };
            match purpose {
                FetchPurpose::InitialLoad => initial_load(&app, distros),
                FetchPurpose::RefreshDistros => {
                    if let Some(distros) = distros {
                        update_distros(&app, distros);
                    }
                }
            }
        });
    });
}

// Initial load: read config -> clean up bindings of distros that no longer exist -> build UI state -> register
//
// `distros` being `None` means the distro list could not be fetched: in that case only the window toggle shortcut is registered and no config is cleaned up.
fn initial_load(app: &AppWindow, distros: Option<Vec<String>>) {
    let path = config::shortcut_config_path();
    let loaded = config::load(&path);
    let mut config = loaded.config;
    if loaded.rebuilt {
        show_toast(app, i18n::t("settings.shortcut.error.config_broken"));
    }

    // Startup validation: keep only bindings of distros that still exist
    if let Some(distros) = distros.as_ref() {
        let stale: Vec<String> = config
            .terminals
            .keys()
            .filter(|name| !distros.iter().any(|d| d == *name))
            .cloned()
            .collect();
        if !stale.is_empty() {
            for name in &stale {
                config.terminals.remove(name);
                info!("Removed stale shortcut binding of missing distro: {}", name);
            }
            persist_config(app, &mut config);
        }
    }

    let mut rows = Vec::with_capacity(config.terminals.len() + 1);
    let toggle = config.global.toggle_window.clone().unwrap_or_default();
    rows.push(RowState {
        distro: None,
        shortcut: toggle.shortcut,
        modifiers: toggle.modifiers,
        key: toggle.key,
        registered: false,
    });
    let mut names: Vec<&String> = config.terminals.keys().collect();
    names.sort();
    for name in names {
        let entry = &config.terminals[name];
        rows.push(RowState {
            distro: Some(name.clone()),
            shortcut: entry.shortcut.clone(),
            modifiers: entry.modifiers,
            key: entry.key,
            registered: false,
        });
    }

    {
        let mut state = lock_ui();
        state.distros = distros.unwrap_or_default();
        state.rows = rows;
        state.loaded = true;
        state.recording = None;
        state.message.clear();
        state.conflict = None;
        state.conflict_text.clear();
    }

    register_all(app);
    let state = lock_ui();
    refresh_ui(app, &state);
}

// Refresh the distro dropdown: drop rows whose distro has been uninstalled
fn update_distros(app: &AppWindow, distros: Vec<String>) {
    let (stale_rows, changed) = {
        let mut state = lock_ui();
        state.distros = distros;
        if !state.loaded {
            return;
        }
        let stale: Vec<usize> = state
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                row.distro
                    .as_ref()
                    .is_some_and(|name| !state.distros.iter().any(|d| d == name))
            })
            .map(|(slot, _)| slot)
            .collect();
        let changed = !stale.is_empty();
        (stale, changed)
    };
    if !changed {
        let state = lock_ui();
        refresh_ui(app, &state);
        return;
    }
    // The distro has been uninstalled: handle it as "remove row" (unregister the OS-level binding first, then clean up the config)
    for slot in stale_rows.into_iter().rev() {
        remove_row(app, slot as i32);
    }
}

// ---------------------------------------------------------------------------
// UI state refresh
// ---------------------------------------------------------------------------

fn refresh_ui(app: &AppWindow, state: &UiState) {
    let rows: Vec<ShortcutRow> = state
        .rows
        .iter()
        .enumerate()
        .skip(1) // Slot 0 is "show / hide main window", carried by a separate property
        .filter(|(_, row)| row.distro.is_some())
        .map(|(slot, row)| {
            let name = row.distro.clone().unwrap_or_default();
            let options = distro_options(state, slot);
            let selected_index = options.iter().position(|option| *option == name).unwrap_or(0);
            ShortcutRow {
                distro: name.clone().into(),
                mono: icon_mapper::get_initial(&name).into(),
                color: name_to_color(&name),
                shortcut: row.shortcut.clone().into(),
                keycaps: keycaps_model(&row.shortcut),
                enabled: row.has_binding(),
                registered: row.registered,
                options: ModelRc::new(VecModel::from(
                    options
                        .iter()
                        .map(|option| distro_option(option))
                        .collect::<Vec<_>>(),
                )),
                selected_index: selected_index as i32,
            }
        })
        .collect();

    let toggle = state
        .rows
        .first()
        .cloned()
        .unwrap_or_else(|| RowState {
            distro: None,
            shortcut: String::new(),
            modifiers: 0,
            key: 0,
            registered: false,
        });

    let total = state.distros.len();
    let bound = state
        .rows
        .iter()
        .filter(|row| row.distro.is_some())
        .count();

    app.set_shortcut_rows(ModelRc::new(VecModel::from(rows)));
    app.set_shortcut_toggle_shortcut(toggle.shortcut.clone().into());
    app.set_shortcut_toggle_keycaps(keycaps_model(&toggle.shortcut));
    app.set_shortcut_recording_slot(state.recording.map(|s| s as i32).unwrap_or(-1));
    app.set_shortcut_recording_text(state.message.clone().into());
    app.set_shortcut_conflict_slot(state.conflict.map(|s| s as i32).unwrap_or(-1));
    app.set_shortcut_conflict_text(state.conflict_text.clone().into());
    app.set_shortcut_can_add(total > bound);
}

// Distro dropdown options for one row: distros not occupied by other rows (always includes this row's current distro)
fn distro_options(state: &UiState, slot: usize) -> Vec<String> {
    let used: HashSet<&str> = state
        .rows
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != slot)
        .filter_map(|(_, row)| row.distro.as_deref())
        .collect();
    let mut options: Vec<String> = state
        .distros
        .iter()
        .filter(|name| !used.contains(name.as_str()))
        .cloned()
        .collect();
    if let Some(current) = state.rows.get(slot).and_then(|row| row.distro.as_ref()) {
        if !options.iter().any(|option| option == current) {
            options.insert(0, current.clone());
        }
    }
    options
}

fn distro_option(name: &str) -> DistroOption {
    DistroOption {
        name: name.into(),
        mono: icon_mapper::get_initial(name).into(),
        color: name_to_color(name),
    }
}

// Distro name -> badge background color (simple match by name, without probing real distro info)
fn name_to_color(name: &str) -> slint::Color {
    parse_hex_color(icon_mapper::map_name_to_color(name))
        .unwrap_or_else(|| slint::Color::from_rgb_u8(0x6B, 0x76, 0x84))
}

// `#RRGGBB` -> `slint::Color`
fn parse_hex_color(hex: &str) -> Option<slint::Color> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(slint::Color::from_rgb_u8(r, g, b))
}

// Keycap list -> Slint model
fn keycaps_model(shortcut: &str) -> ModelRc<slint::SharedString> {
    ModelRc::new(VecModel::from(keycaps_of(shortcut)))
}

// Combo text -> keycap array (e.g. `Ctrl+Alt+W` -> `["Ctrl", "Alt", "W"]`)
fn keycaps_of(shortcut: &str) -> Vec<slint::SharedString> {
    if shortcut.trim().is_empty() {
        return Vec::new();
    }
    shortcut.split('+').map(|part| part.trim().into()).collect()
}

// ---------------------------------------------------------------------------
// Recording and binding
// ---------------------------------------------------------------------------

fn start_recording(app: &AppWindow, slot: i32) {
    let mut state = lock_ui();
    if slot < 0 || slot as usize >= state.rows.len() {
        return;
    }
    state.recording = Some(slot as usize);
    state.message.clear();
    state.conflict = None;
    state.conflict_text.clear();
    refresh_ui(app, &state);
}

fn cancel_recording(app: &AppWindow) {
    {
        let mut state = lock_ui();
        if state.recording.is_none() {
            return;
        }
        state.recording = None;
        state.message.clear();
        state.conflict = None;
        state.conflict_text.clear();
    }
    let state = lock_ui();
    refresh_ui(app, &state);
}

fn handle_key_input(app: &AppWindow, text: &str, ctrl: bool, alt: bool, shift: bool, meta: bool) {
    let slot = {
        let state = lock_ui();
        let Some(slot) = state.recording else { return };
        slot
    };

    if text == KEY_ESCAPE {
        cancel_recording(app);
        return;
    }

    match keymap::parse(text, ctrl, alt, shift, meta) {
        Ok(stroke) => apply_binding(app, slot, &stroke),
        // Only a modifier key was pressed: stay in recording state and wait for the main key
        Err(ParseError::ModifierOnly) => {}
        Err(ParseError::NeedsModifier) => {
            let mut state = lock_ui();
            state.message = i18n::t("settings.shortcut.need_modifier");
            refresh_ui(app, &state);
        }
        Err(ParseError::Unsupported) => {
            let mut state = lock_ui();
            state.message = i18n::t("settings.shortcut.unsupported_key");
            refresh_ui(app, &state);
        }
    }
}

// Apply the recorded combo: duplicate check -> write config -> unregister the old binding -> register the new binding 
fn apply_binding(app: &AppWindow, slot: usize, stroke: &KeyStroke) {
    // In-app conflict check: the same combo must not be bound to two places
    let duplicated = {
        let state = lock_ui();
        state.rows.iter().enumerate().any(|(index, row)| {
            index != slot && row.key == stroke.key && row.modifiers == stroke.modifiers
        })
    };
    if duplicated {
        let mut state = lock_ui();
        state.conflict = Some(slot);
        state.conflict_text = in_app_conflict_text(&state, slot, stroke);
        state.recording = None;
        state.message.clear();
        refresh_ui(app, &state);
        return;
    }

    let (action_key, distro, previous_stroke) = {
        let mut state = lock_ui();
        let row = &mut state.rows[slot];
        let previous = row.has_binding().then(|| (row.modifiers, row.key));
        row.shortcut = stroke.display.clone();
        row.modifiers = stroke.modifiers;
        row.key = stroke.key;
        row.registered = false;
        let action_key = row.action_key();
        let distro = row.distro.clone();
        state.recording = None;
        state.message.clear();
        state.conflict = None;
        state.conflict_text.clear();
        (action_key, distro, previous)
    };

    {
        let state = lock_ui();
        persist_rows(app, &state);
    }

    // Unregister the old binding of the same action first, to avoid leftovers of the old combo in the system
    if previous_stroke.is_some_and(|(m, k)| m != stroke.modifiers || k != stroke.key) {
        manager::request_unregister(&action_key);
    }

    let outcome = if manager::is_active() {
        manager::request_register(&action_key, stroke.modifiers, stroke.key, distro)
    } else {
        Outcome::Unavailable
    };

    let mut state = lock_ui();
    state.rows[slot].registered = outcome == Outcome::Registered;
    match outcome {
        Outcome::Registered => {
            info!("Global shortcut binding saved: {}", stroke.display);
            set_toast_text(app, i18n::tr("settings.shortcut.saved", &[stroke.display.clone()]));
        }
        Outcome::ConflictOs | Outcome::Unavailable => {
            // Combo occupied by the system or another app: keep the config but notify the user in the UI
            let name = row_display_name(&state.rows[slot]);
            state.conflict = Some(slot);
            state.conflict_text =
                i18n::tr("settings.shortcut.occupied", &[stroke.display.clone()]);
            warn!("Failed to register global shortcut for {}: {}", name, stroke.display);
        }
        Outcome::ConflictInApp => {
            state.conflict = Some(slot);
            state.conflict_text = in_app_conflict_text(&state, slot, stroke);
        }
    }
    refresh_ui(app, &state);
}

fn row_display_name(row: &RowState) -> String {
    row.distro
        .clone()
        .unwrap_or_else(|| i18n::t("settings.shortcut.toggle_window_action"))
}

// Build the "already occupied by another row" message (the caller already holds the lock, hence `&UiState`)
fn in_app_conflict_text(state: &UiState, slot: usize, stroke: &KeyStroke) -> String {
    let owner = state
        .rows
        .iter()
        .enumerate()
        .find(|(index, other)| {
            *index != slot && other.key == stroke.key && other.modifiers == stroke.modifiers
        })
        .map(|(_, other)| row_display_name(other));
    match owner {
        Some(owner) => i18n::tr("settings.shortcut.conflict", &[owner, stroke.display.clone()]),
        None => i18n::tr("settings.shortcut.occupied", &[stroke.display.clone()]),
    }
}

fn clear_binding(app: &AppWindow, slot: i32) {
    if slot < 0 {
        return;
    }
    let slot = slot as usize;
    let (action_key, name) = {
        let mut state = lock_ui();
        let Some(row) = state.rows.get_mut(slot) else {
            return;
        };
        let action_key = row.action_key();
        let name = row_display_name(row);
        row.shortcut.clear();
        row.modifiers = 0;
        row.key = 0;
        row.registered = false;
        state.conflict = None;
        state.conflict_text.clear();
        state.message.clear();
        (action_key, name)
    };
    // Release the OS-level registration first, then clean up the config
    manager::request_unregister(&action_key);
    {
        let state = lock_ui();
        persist_rows(app, &state);
    }
    info!("Cleared shortcut binding of {}", name);
    set_toast_text(app, i18n::tr("settings.shortcut.cleared", &[name]));
    let state = lock_ui();
    refresh_ui(app, &state);
}

fn remove_row(app: &AppWindow, slot: i32) {
    // Slot 0 is "show / hide main window"; it can only be cleared, not removed
    if slot <= 0 {
        return;
    }
    let slot = slot as usize;
    {
        let mut state = lock_ui();
        if slot >= state.rows.len() {
            return;
        }
        let row = state.rows.remove(slot);
        if let Some(name) = row.distro.as_ref() {
            // the OS-level registration must be removed before the config is cleaned up
            if row.has_binding() {
                manager::request_unregister(name);
            }
            let mut config = load_config();
            if config.terminals.remove(name).is_some() {
                info!("Removed shortcut binding of distro: {}", name);
                persist_config(app, &mut config);
            }
        }
        state.recording = None;
        state.message.clear();
        state.conflict = None;
        state.conflict_text.clear();
    }
    let state = lock_ui();
    refresh_ui(app, &state);
}

fn add_row(app: &AppWindow) {
    let name = {
        let state = lock_ui();
        let used: HashSet<&str> = state
            .rows
            .iter()
            .filter_map(|row| row.distro.as_deref())
            .collect();
        let Some(name) = state.distros.iter().find(|d| !used.contains(d.as_str())) else {
            return;
        };
        name.clone()
    };

    {
        let mut state = lock_ui();
        state.rows.push(RowState {
            distro: Some(name.clone()),
            shortcut: String::new(),
            modifiers: 0,
            key: 0,
            registered: false,
        });
        state.recording = None;
        state.message.clear();
        state.conflict = None;
        state.conflict_text.clear();
    }

    // New rows are written to the config immediately (not bound yet, waiting for the user to record)
    let mut config = load_config();
    config.terminals.insert(name.clone(), ShortcutEntry::default());
    persist_config(app, &mut config);
    info!("Added distro shortcut row: {}", name);

    let state = lock_ui();
    refresh_ui(app, &state);
}

fn select_distro(app: &AppWindow, slot: i32, option_index: i32) {
    if slot <= 0 || option_index < 0 {
        return;
    }
    let slot = slot as usize;
    let (old_name, new_name, has_binding, modifiers, key) = {
        let state = lock_ui();
        if slot >= state.rows.len() {
            return;
        }
        let Some(new_name) = distro_options(&state, slot)
            .get(option_index as usize)
            .cloned()
        else {
            return;
        };
        let row = &state.rows[slot];
        (
            row.distro.clone().unwrap_or_default(),
            new_name,
            row.has_binding(),
            row.modifiers,
            row.key,
        )
    };
    if old_name == new_name {
        let state = lock_ui();
        refresh_ui(app, &state);
        return;
    }

    // Unregister the old distro's shortcut first, otherwise the leftover binding would trigger an action for a distro that no longer exists
    if has_binding {
        manager::request_unregister(&old_name);
    }

    {
        let mut state = lock_ui();
        state.rows[slot].distro = Some(new_name.clone());
        state.rows[slot].registered = false;
        state.conflict = None;
        state.conflict_text.clear();
        state.message.clear();
    }

    let mut config = load_config();
    let entry = config.terminals.remove(&old_name).unwrap_or_default();
    config.terminals.insert(new_name.clone(), entry);
    persist_config(app, &mut config);
    info!("Changed distro shortcut row: {} -> {}", old_name, new_name);

    // If a shortcut is already bound, register it again for the new distro
    if has_binding && manager::is_active() {
        let outcome =
            manager::request_register(&new_name, modifiers, key, Some(new_name.clone()));
        let mut state = lock_ui();
        state.rows[slot].registered = outcome == Outcome::Registered;
    }
    let state = lock_ui();
    refresh_ui(app, &state);
}

// ---------------------------------------------------------------------------
// External cleanup
// ---------------------------------------------------------------------------

// Clean up a distro's shortcut binding after it has been uninstalled.
//
// The OS-level registration must be removed **first**, then the binding in `shortcuts.toml`, so a
// leftover global shortcut cannot trigger an action for a distro that no longer exists. Can be called
// from any thread (does not touch the UI).
pub fn remove_distro_binding(name: &str) {
    let name = name.trim();
    if name.is_empty() {
        return;
    }
    manager::request_unregister(name);
    {
        let mut state = lock_ui();
        if let Some(pos) = state
            .rows
            .iter()
            .position(|row| row.distro.as_deref() == Some(name))
        {
            state.rows.remove(pos);
        }
        state.recording = None;
        state.conflict = None;
        state.conflict_text.clear();
        state.message.clear();
    }
    let mut config = load_config();
    if config.terminals.remove(name).is_some() {
        config::touch(&mut config);
        if let Err(e) = config::save(&config::shortcut_config_path(), &config) {
            error!("Failed to save shortcuts.toml: {}", e);
        }
        info!("Cleaned shortcut binding of deleted distro: {}", name);
    }
}

// ---------------------------------------------------------------------------
// Registration and config persistence
// ---------------------------------------------------------------------------

// Register all shortcuts according to the current UI state and record the registration results
fn register_all(app: &AppWindow) {
    if !manager::start() {
        warn!("Shortcut feature is unavailable, skip registering global shortcuts");
        return;
    }
    let mut conflict: Option<(usize, String)> = None;
    {
        let mut state = lock_ui();
        for slot in 0..state.rows.len() {
            let (action_key, modifiers, key, distro, shortcut_text) = {
                let row = &state.rows[slot];
                (
                    row.action_key(),
                    row.modifiers,
                    row.key,
                    row.distro.clone(),
                    row.shortcut.clone(),
                )
            };
            if key == 0 {
                continue;
            }
            let outcome = manager::request_register(&action_key, modifiers, key, distro);
            state.rows[slot].registered = outcome == Outcome::Registered;
            match outcome {
                Outcome::Registered => {
                    info!("Global shortcut activated: {} -> {}", shortcut_text, action_key);
                }
                Outcome::ConflictInApp => {
                    warn!("Global shortcut conflict inside app: {}", action_key);
                    let stroke = stroke_from_row(&state.rows[slot]);
                    conflict = Some((slot, in_app_conflict_text(&state, slot, &stroke)));
                }
                Outcome::ConflictOs | Outcome::Unavailable => {
                    warn!("Global shortcut occupied or unavailable: {}", action_key);
                    conflict = Some((
                        slot,
                        i18n::tr("settings.shortcut.occupied", &[shortcut_text.clone()]),
                    ));
                }
            }
        }
        match conflict {
            Some((slot, text)) => {
                state.conflict = Some(slot);
                state.conflict_text = text;
            }
            None => {
                state.conflict = None;
                state.conflict_text.clear();
            }
        }
    }
    let state = lock_ui();
    refresh_ui(app, &state);
}

fn stroke_from_row(row: &RowState) -> KeyStroke {
    KeyStroke {
        modifiers: row.modifiers,
        key: row.key,
        display: row.shortcut.clone(),
    }
}

fn load_config() -> ShortcutConfigFile {
    config::load(&config::shortcut_config_path()).config
}

// Write the UI state back to the config file
fn persist_rows(app: &AppWindow, state: &UiState) {
    let mut config = load_config();
    config.global.toggle_window = state
        .rows
        .iter()
        .find(|row| row.distro.is_none())
        .and_then(|row| row.entry());

    let mut terminals = HashMap::new();
    for row in state.rows.iter().filter(|row| row.distro.is_some()) {
        let Some(name) = row.distro.clone() else { continue };
        terminals.insert(name, row.entry().unwrap_or_default());
    }
    config.terminals = terminals;
    persist_config(app, &mut config);
}

fn persist_config(app: &AppWindow, config: &mut ShortcutConfigFile) {
    config::touch(config);
    if let Err(e) = config::save(&config::shortcut_config_path(), config) {
        error!("Failed to save shortcuts.toml: {}", e);
        show_toast(app, i18n::t("settings.shortcut.error.save"));
    }
}

// ---------------------------------------------------------------------------
// Toast
// ---------------------------------------------------------------------------

fn show_toast(app: &AppWindow, message: String) {
    set_toast_text(app, message);
}

fn set_toast_text(app: &AppWindow, message: String) {
    app.set_task_status_text(message.into());
    app.set_task_status_visible(true);
    let handle = app.as_weak();
    slint::Timer::single_shot(TOAST_DURATION, move || {
        if let Some(app) = handle.upgrade() {
            app.set_task_status_visible(false);
        }
    });
}
