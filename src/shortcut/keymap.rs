// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Mapping between Slint key text and Win32 virtual key codes (VK) / modifier keys (MOD_*).
//
// Slint provides the **character text** of a key via `KeyEvent.text`, while Windows
// `RegisterHotKey` needs virtual key codes, so a bidirectional conversion is required.
// This module also builds the text shown in the UI.
//
// Note: Shift combinations give the **shifted character** in Slint (e.g. Shift+1 yields `!`),
// whereas Win32 registers the physical key (`0x31`), so the mapping must map shifted characters
// back to the base key.

// ---- Win32 modifiers ----
pub const MOD_ALT: u32 = 0x0001;
pub const MOD_CONTROL: u32 = 0x0002;
pub const MOD_SHIFT: u32 = 0x0004;
pub const MOD_WIN: u32 = 0x0008;
// Do not repeat while held (supported on Windows 7 and later)
pub const MOD_NOREPEAT: u32 = 0x4000;

// ---- Common virtual key codes ----
pub const VK_BACK: u32 = 0x08;
pub const VK_TAB: u32 = 0x09;
pub const VK_RETURN: u32 = 0x0D;
pub const VK_PAUSE: u32 = 0x13;
pub const VK_CAPITAL: u32 = 0x14;
pub const VK_ESCAPE: u32 = 0x1B;
pub const VK_SPACE: u32 = 0x20;
pub const VK_PRIOR: u32 = 0x21;
pub const VK_NEXT: u32 = 0x22;
pub const VK_END: u32 = 0x23;
pub const VK_HOME: u32 = 0x24;
pub const VK_LEFT: u32 = 0x25;
pub const VK_UP: u32 = 0x26;
pub const VK_RIGHT: u32 = 0x27;
pub const VK_DOWN: u32 = 0x28;
pub const VK_SNAPSHOT: u32 = 0x2C;
pub const VK_INSERT: u32 = 0x2D;
pub const VK_DELETE: u32 = 0x2E;
pub const VK_NUMLOCK: u32 = 0x90;
pub const VK_SCROLL: u32 = 0x91;
pub const VK_APPS: u32 = 0x5D;

// Parse failure reasons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    // Only modifier keys were pressed (Ctrl/Shift/Alt/Win)
    ModifierOnly,
    // No modifier key was included (using a single key as a global shortcut would interfere with normal typing)
    NeedsModifier,
    // This key cannot be bound yet
    Unsupported,
}

// Result of a single key recording
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyStroke {
    pub modifiers: u32,
    pub key: u32,
    // Display text, e.g. `Ctrl+Alt+W`
    pub display: String,
}

// Whether the text corresponds to a modifier key
pub fn is_modifier_text(text: &str) -> bool {
    matches!(
        text,
        "\u{0010}" | "\u{0011}" | "\u{0012}" | "\u{0013}" | "\u{0014}" | "\u{0015}" | "\u{0016}"
            | "\u{0017}" | "\u{0018}"
    )
}

// Parse a Slint key event into a virtual key code and modifiers.
pub fn parse(text: &str, ctrl: bool, alt: bool, shift: bool, meta: bool) -> Result<KeyStroke, ParseError> {
    if text.is_empty() || is_modifier_text(text) {
        return Err(ParseError::ModifierOnly);
    }
    let key = vk_from_text(text).ok_or(ParseError::Unsupported)?;
    let mut modifiers = 0u32;
    if ctrl {
        modifiers |= MOD_CONTROL;
    }
    if alt {
        modifiers |= MOD_ALT;
    }
    if shift {
        modifiers |= MOD_SHIFT;
    }
    if meta {
        modifiers |= MOD_WIN;
    }
    if modifiers == 0 {
        return Err(ParseError::NeedsModifier);
    }
    Ok(KeyStroke {
        modifiers,
        key,
        display: display_text(modifiers, key),
    })
}

// Key text -> virtual key code
fn vk_from_text(text: &str) -> Option<u32> {
    let ch = text.chars().next()?;

    // Control characters and special keys
    let special = match ch {
        '\u{0008}' => Some(VK_BACK),
        '\u{0009}' => Some(VK_TAB),
        '\u{000A}' | '\u{000D}' => Some(VK_RETURN),
        '\u{001B}' => Some(VK_ESCAPE),
        '\u{007F}' => Some(VK_DELETE),
        '\u{0020}' => Some(VK_SPACE),
        '\u{F700}' => Some(VK_UP),
        '\u{F701}' => Some(VK_DOWN),
        '\u{F702}' => Some(VK_LEFT),
        '\u{F703}' => Some(VK_RIGHT),
        '\u{F727}' => Some(VK_INSERT),
        '\u{F729}' => Some(VK_HOME),
        '\u{F72B}' => Some(VK_END),
        '\u{F72C}' => Some(VK_PRIOR),
        '\u{F72D}' => Some(VK_NEXT),
        '\u{F72F}' => Some(VK_SCROLL),
        '\u{F730}' => Some(VK_PAUSE),
        '\u{F731}' => Some(VK_SNAPSHOT),
        '\u{F735}' => Some(VK_APPS),
        _ => None,
    };
    if let Some(vk) = special {
        return Some(vk);
    }

    // Function keys F1 ~ F24 (Slint: \u{F704} ~ \u{F71B}; Win32: 0x70 ~ 0x87)
    if ('\u{F704}'..='\u{F71B}').contains(&ch) {
        return Some(0x70 + (ch as u32 - 0xF704));
    }

    match ch {
        'a'..='z' => Some(0x41 + (ch as u32 - 'a' as u32)),
        'A'..='Z' => Some(0x41 + (ch as u32 - 'A' as u32)),
        '0'..='9' => Some(0x30 + (ch as u32 - '0' as u32)),
        _ => punctuation_vk(ch),
    }
}

// Symbol (including Shift-shifted characters) -> virtual key code (mapped back to the physical key)
fn punctuation_vk(ch: char) -> Option<u32> {
    let vk = match ch {
        ' ' => VK_SPACE,
        // Shifted characters on the number row
        '!' => 0x31, // Shift+1
        '@' => 0x32, // Shift+2
        '#' => 0x33, // Shift+3
        '$' => 0x34, // Shift+4
        '%' => 0x35, // Shift+5
        '^' => 0x36, // Shift+6
        '&' => 0x37, // Shift+7
        '*' => 0x38, // Shift+8
        '(' => 0x39, // Shift+9
        ')' => 0x30, // Shift+0
        // Symbol keys and their shifted characters
        '-' | '_' => 0xBD,
        '=' | '+' => 0xBB,
        '[' | '{' => 0xDB,
        ']' | '}' => 0xDD,
        '\\' | '|' => 0xDC,
        ';' | ':' => 0xBA,
        '\'' | '"' => 0xDE,
        ',' | '<' => 0xBC,
        '.' | '>' => 0xBE,
        '/' | '?' => 0xBF,
        '`' | '~' => 0xC0,
        _ => return None,
    };
    Some(vk)
}

// Virtual key code -> display key text
pub fn key_text(key: u32) -> String {
    match key {
        0x30..=0x39 => ((b'0' + (key - 0x30) as u8) as char).to_string(),
        0x41..=0x5A => ((b'A' + (key - 0x41) as u8) as char).to_string(),
        0x70..=0x87 => format!("F{}", key - 0x70 + 1),
        0xBD => "-".to_string(),
        0xBB => "=".to_string(),
        0xDB => "[".to_string(),
        0xDD => "]".to_string(),
        0xDC => "\\".to_string(),
        0xBA => ";".to_string(),
        0xDE => "'".to_string(),
        0xBC => ",".to_string(),
        0xBE => ".".to_string(),
        0xBF => "/".to_string(),
        0xC0 => "`".to_string(),
        VK_BACK => "Backspace".to_string(),
        VK_TAB => "Tab".to_string(),
        VK_RETURN => "Enter".to_string(),
        VK_PAUSE => "Pause".to_string(),
        VK_CAPITAL => "CapsLock".to_string(),
        VK_ESCAPE => "Esc".to_string(),
        VK_SPACE => "Space".to_string(),
        VK_PRIOR => "PageUp".to_string(),
        VK_NEXT => "PageDown".to_string(),
        VK_END => "End".to_string(),
        VK_HOME => "Home".to_string(),
        VK_LEFT => "Left".to_string(),
        VK_UP => "Up".to_string(),
        VK_RIGHT => "Right".to_string(),
        VK_DOWN => "Down".to_string(),
        VK_SNAPSHOT => "PrtScn".to_string(),
        VK_INSERT => "Insert".to_string(),
        VK_DELETE => "Delete".to_string(),
        VK_NUMLOCK => "NumLock".to_string(),
        VK_SCROLL => "ScrollLock".to_string(),
        VK_APPS => "Menu".to_string(),
        other => format!("0x{:02X}", other),
    }
}

// Modifier bitmask -> display text (in Ctrl, Alt, Shift, Win order)
pub fn modifier_text(modifiers: u32) -> String {
    let mut parts = Vec::new();
    if modifiers & MOD_CONTROL != 0 {
        parts.push("Ctrl");
    }
    if modifiers & MOD_ALT != 0 {
        parts.push("Alt");
    }
    if modifiers & MOD_SHIFT != 0 {
        parts.push("Shift");
    }
    if modifiers & MOD_WIN != 0 {
        parts.push("Win");
    }
    parts.join("+")
}

// Compose the full display text
pub fn display_text(modifiers: u32, key: u32) -> String {
    let key_text = key_text(key);
    let modifier_text = modifier_text(modifiers);
    if modifier_text.is_empty() {
        key_text
    } else {
        format!("{}+{}", modifier_text, key_text)
    }
}
