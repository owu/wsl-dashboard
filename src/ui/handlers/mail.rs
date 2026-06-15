// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// System message UI callback handler

use crate::AppWindow;
use crate::app::popup;
use crate::app::tasks::message_handler;
use slint::Model;
use tracing::debug;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>) {
    // Mail icon click: toggle popup visibility
    {
        let ah = app_handle.clone();
        app.on_mail_clicked(move || {
            if let Some(app) = ah.upgrade() {
                let current = app.get_show_mail_popup();
                app.set_show_mail_popup(!current);
            }
        });
    }

    // Message click: open link + mark as read
    {
        let ah = app_handle.clone();
        app.on_mail_message_clicked(move |id, url| {
            debug!("mail: message clicked, id={}, url={}", id, url);
            if !url.is_empty() {
                let _ = open::that(url.as_str());
            }
            // Collect all IDs from current message list
            let active_ids = message_handler::collect_active_message_ids(&ah);
            popup::mark_message_read(id.as_str(), &active_ids);
            // Update UI: unread count + message read status
            if let Some(app) = ah.upgrade() {
                let current = app.get_mail_unread_count();
                if current > 0 {
                    app.set_mail_unread_count(current - 1);
                }
                // Update read status of the corresponding message in the list
                let messages = app.get_mail_messages();
                let updated: Vec<crate::MessageData> = (0..messages.row_count())
                    .filter_map(|i| messages.row_data(i).map(|mut m| {
                        if m.id.to_string() == id.as_str() {
                            m.is_read = true;
                        }
                        m
                    }))
                    .collect();
                let model = std::rc::Rc::new(slint::VecModel::from(updated));
                app.set_mail_messages(slint::ModelRc::from(model));
            }
        });
    }

    // Mark all as read
    {
        let ah = app_handle.clone();
        app.on_mail_mark_all_read(move || {
            debug!("mail: mark all read");
            if let Some(app) = ah.upgrade() {
                let messages = app.get_mail_messages();
                let ids: Vec<String> = (0..messages.row_count())
                    .filter_map(|i| messages.row_data(i).map(|m| m.id.to_string()))
                    .collect();
                popup::mark_all_messages_read(&ids);
                app.set_mail_unread_count(0);
                // Reset message list, marking all messages as read
                let updated: Vec<crate::MessageData> = (0..messages.row_count())
                    .filter_map(|i| messages.row_data(i).map(|mut m| {
                        m.is_read = true;
                        m
                    }))
                    .collect();
                let model = std::rc::Rc::new(slint::VecModel::from(updated));
                app.set_mail_messages(slint::ModelRc::from(model));
            }
        });
    }

    // Close popup
    {
        let ah = app_handle.clone();
        app.on_mail_close(move || {
            if let Some(app) = ah.upgrade() {
                app.set_show_mail_popup(false);
            }
        });
    }
}
