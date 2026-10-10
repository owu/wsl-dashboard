// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tracing::info;
use super::{Config, SETTINGS_VERSION};

pub fn migrate_config(config: &mut Config) {
    let old_version = config.application.setting_version as u32;
    
    if old_version >= SETTINGS_VERSION {
        return;
    }

    info!("Detected configuration version v{}, upgrading to v{}...", old_version, SETTINGS_VERSION);

    // v0 -> v1 logic
    if old_version < 1 {
        info!("Upgrading to v1: migrating version position, ensuring base fields exist");
    }

    // v1 -> v2 logic
    if old_version < 2 {
        info!("Upgrading to v2: adding [settings] check-time,check-update");
        config.settings.check_time = "0".to_string();
        config.settings.check_update = 7;
    }

    // v2 -> v3 logic
    if old_version < 3 {
        info!("Upgrading to v3: adding [tray] close-to-tray");
        config.tray.close_to_tray = true;
    }

    // v3 -> v4 logic
    if old_version < 4 {
        info!("Upgrading to v4: adding [settings] sidebar-collapsed");
        config.settings.sidebar_collapsed = false;
    }

    // v4 -> v5 logic
    if old_version < 5 {
        info!("Upgrading to v5: adding [settings] system-color");
        config.settings.system_color = false;
    }
    
    // v5 -> v6 logic
    if old_version < 6 {
        info!("Upgrading to v6: adding [application] project_website,[settings] colorful-icons,[sidebar] toggle");
        config.application.project_website = crate::app::PROJECT_WEBSITE.to_string();
        config.settings.colorful_icons = true;
        config.sidebar.toggle = true;
    }

    // v6 -> v7 logic
    if old_version < 7 {
        info!("Upgrading to v7: adding [settings] mail (default true),hide-pin (default false)");
        config.settings.mail = true;
        config.settings.hide_pin = false;
    }

    // v7 -> v8 logic
    if old_version < 8 {
        info!("Upgrading to v8: adding [settings] show-drag (default true)");
        config.settings.show_drag = true;
    }

    // v8 -> v9 logic
    if old_version < 9 {
        info!("Upgrading to v9: adding [sidebar] scheduler, mount (default true)");
        config.sidebar.scheduler = true;
        config.sidebar.mount = true;
    }

    // v9 -> v10 logic
    if old_version < 10 {
        info!("Upgrading to v10: adding [application] device-id, [settings] backup-location");
        config.application.device_id = String::new();
        config.settings.backup_location = crate::config::default_backup_location();
    }

    config.application.setting_version = SETTINGS_VERSION as u8;
    info!("Configuration migration complete, current version: v{}", SETTINGS_VERSION);
}

pub fn migrate_instances_config(container: &mut super::InstancesContainer) {
    let old_version = container.common.setting_version;
    
    if old_version >= super::INSTANCES_VERSION {
        return;
    }

    info!("Detected instances configuration version v{}, upgrading to v{}...", old_version, super::INSTANCES_VERSION);

    if old_version < 2 {
        info!("Upgrading instances config to v2: adding terminal-proxy field (default enabled), distro-notes (default empty)");
        for instance in container.instances.values_mut() {
            instance.distro_notes = String::new();
        }
    }
    
    container.common.setting_version = super::INSTANCES_VERSION;
    info!("Instances configuration migration complete, current version: v{}", super::INSTANCES_VERSION);
}
