// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tracing::info;

// Generate device ID — using a random UUID v7 as a globally unique identifier.
pub async fn generate_device_id() -> String {
    // v7 output is 36 lowercase characters (including hyphens); removing hyphens yields a compact 32-character format
    let id = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext))
        .to_string()
        .replace('-', "");
    info!("Device ID: generated random uuid (v7) as device-id");
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compact_v7(id: &str) -> bool {
        id.len() == 32 && id.chars().all(|c| c.is_ascii_hexdigit())
    }

    #[tokio::test]
    async fn test_generate_device_id_format() {
        let id = generate_device_id().await;
        assert!(compact_v7(&id), "device-id should be 32 hex characters, actual: {}", id);
        assert!(id.chars().all(|c| !c.is_ascii_uppercase()), "should be all lowercase: {}", id);
    }

    #[tokio::test]
    async fn test_generate_device_id_unique() {
        let a = generate_device_id().await;
        let b = generate_device_id().await;
        assert_ne!(a, b, "device-id generated twice should not be identical");
    }
}