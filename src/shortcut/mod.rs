// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Global shortcut business logic.
//
// - [`config`]: `shortcuts.toml` data model and I/O
// - [`keymap`]: Slint key text <-> Win32 virtual key code / modifier mapping
// - [`manager`]: global shortcut registration, unregistration and action dispatch (based on Windows `RegisterHotKey`)

pub mod config;
pub mod keymap;
pub mod manager;
