// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// System message business handler
//
// Called by SyncMonitorTask, handles message data filtering and UI updates.

use tracing::debug;
use slint::Model;
use crate::api::models::MessageItem;
use crate::app::popup;
use crate::AppWindow;

// Message display structure
#[derive(Clone)]
struct DisplayMessage {
    id: String,
    title: String,
    url: String,
    msg_type: String,
    is_read: bool,
    create_time_text: String,
}

// Format millisecond timestamp to "yyyy-MM-dd"
fn format_timestamp(ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    let dt = Utc.timestamp_millis_opt(ms).single().unwrap_or_default();
    dt.format("%Y-%m-%d").to_string()
}

// Match title for current language from the title array
fn match_title_by_lang(
    titles: &[crate::api::models::MessageTitle],
    sys_lang: &str,
) -> String {
    let nl = popup::normalize_lang(sys_lang);

    for t in titles {
        if popup::normalize_lang(&t.system_language) == nl {
            return t.title.clone();
        }
    }
    for t in titles {
        if popup::normalize_lang(&t.system_language) == "enus" {
            return t.title.clone();
        }
    }
    titles.first().map(|t| t.title.clone()).unwrap_or_default()
}

// Handle system messages
pub async fn handle_messages(
    messages: &[MessageItem],
    app_handle: &slint::Weak<AppWindow>,
) {
    if messages.is_empty() {
        debug!("message: no messages from API");
        clear_message_ui(app_handle);
        return;
    }

    // ① Get system language and timezone
    let (sys_lang, sys_tz) = {
        let ah = app_handle.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = slint::invoke_from_event_loop(move || {
            let lang = if let Some(app) = ah.upgrade() {
                app.get_system_language().to_string()
            } else {
                "en".to_string()
            };
            let _ = tx.send((lang, String::new()));
        });
        rx.recv_timeout(std::time::Duration::from_millis(100))
            .unwrap_or(("en".to_string(), String::new()))
    };

    // ② Filter messages matching criteria
    let mut filtered: Vec<&MessageItem> = messages.iter().filter(|msg| {
        if !popup::is_in_time_range(msg.start_time, msg.end_time) {
            debug!("message: id={} not in time range", msg.id);
            return false;
        }
        if !popup::matches_locale(&msg.system_language, &msg.timezone, &sys_lang, &sys_tz) {
            debug!("message: id={} locale mismatch", msg.id);
            return false;
        }
        true
    }).collect();

    if filtered.is_empty() {
        debug!("message: no matched messages after filter");
        clear_message_ui(app_handle);
        return;
    }

    // ③ Sort by create_time descending (newest first)
    filtered.sort_by(|a, b| b.create_time.cmp(&a.create_time));

    // ④ Get read records, calculate unread count
    let read_ids = popup::get_read_message_ids();
    let unread_count = filtered.iter().filter(|m| !read_ids.contains(&m.id)).count();

    // ⑤ Match title language + format time for each message
    let display_items: Vec<DisplayMessage> = filtered.iter().map(|msg| {
        let title = match_title_by_lang(&msg.title, &sys_lang);
        DisplayMessage {
            id: msg.id.clone(),
            title,
            url: msg.url.clone(),
            msg_type: msg.msg_type.clone(),
            is_read: read_ids.contains(&msg.id),
            create_time_text: format_timestamp(msg.create_time),
        }
    }).collect();

    // ⑥ Pass to UI
    update_message_ui(&display_items, unread_count, app_handle);
}

// Update UI message list and unread count
fn update_message_ui(
    items: &[DisplayMessage],
    unread_count: usize,
    app_handle: &slint::Weak<AppWindow>,
) {
    let slint_items: Vec<crate::MessageData> = items.iter().map(|m| {
        crate::MessageData {
            id: m.id.clone().into(),
            title: m.title.clone().into(),
            url: m.url.clone().into(),
            msg_type: m.msg_type.clone().into(),
            is_read: m.is_read,
            create_time_text: m.create_time_text.clone().into(),
        }
    }).collect();

    let ah = app_handle.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah.upgrade() {
            let model = std::rc::Rc::new(slint::VecModel::from(slint_items));
            app.set_mail_messages(slint::ModelRc::from(model));
            app.set_mail_unread_count(unread_count as i32);
            let _ = tx.send(());
        }
    });
    let _ = rx.recv_timeout(std::time::Duration::from_millis(500));
}

// Clear UI message list
fn clear_message_ui(app_handle: &slint::Weak<AppWindow>) {
    let ah = app_handle.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(app) = ah.upgrade() {
            let empty: Vec<crate::MessageData> = Vec::new();
            let model = std::rc::Rc::new(slint::VecModel::from(empty));
            app.set_mail_messages(slint::ModelRc::from(model));
            app.set_mail_unread_count(0);
            let _ = tx.send(());
        }
    });
    let _ = rx.recv_timeout(std::time::Duration::from_millis(500));
}

// Collect all IDs from UI message list
pub fn collect_active_message_ids(app_handle: &slint::Weak<AppWindow>) -> Vec<String> {
    if let Some(app) = app_handle.upgrade() {
        let messages = app.get_mail_messages();
        (0..messages.row_count())
            .filter_map(|i| messages.row_data(i).map(|m| m.id.to_string()))
            .collect()
    } else {
        Vec::new()
    }
}
