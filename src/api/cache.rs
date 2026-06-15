// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::time::{SystemTime, UNIX_EPOCH};

// --- Cache TTL constants (seconds) ---

// Medium (5 minutes) — for occasionally changing business data
pub const CACHE_TTL_MEDIUM: u64 = 300;

// Long (10 minutes) — for rarely changing static configuration
pub const CACHE_TTL_LONG: u64 = 600;

// --- Cache entry ---

#[derive(Debug)]
pub(crate) struct CacheEntry<T: Clone + std::fmt::Debug> {
    pub data: T,
    pub cached_at_ms: u64,
    pub ttl_secs: u64,
}

impl<T: Clone + std::fmt::Debug> CacheEntry<T> {
    pub fn is_expired(&self, now_ms: u64) -> bool {
        let elapsed_secs = (now_ms - self.cached_at_ms) / 1000;
        elapsed_secs >= self.ttl_secs
    }
}

// --- Cache helpers ---

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub(crate) fn try_get_cache<T: Clone + std::fmt::Debug>(cache: &std::sync::Mutex<Option<CacheEntry<T>>>) -> Option<T> {
    let current_ms = now_ms();
    if let Ok(guard) = cache.lock() {
        if let Some(ref entry) = *guard {
            if !entry.is_expired(current_ms) {
                return Some(entry.data.clone());
            }
        }
    }
    None
}

// Get stale cache data (even if expired), for degraded fallback
pub(crate) fn try_get_stale_cache<T: Clone + std::fmt::Debug>(cache: &std::sync::Mutex<Option<CacheEntry<T>>>) -> Option<T> {
    if let Ok(guard) = cache.lock() {
        if let Some(ref entry) = *guard {
            return Some(entry.data.clone());
        }
    }
    None
}

pub(crate) fn set_cache<T: Clone + std::fmt::Debug>(cache: &std::sync::Mutex<Option<CacheEntry<T>>>, data: T, ttl_secs: u64) {
    if let Ok(mut guard) = cache.lock() {
        *guard = Some(CacheEntry { data, cached_at_ms: now_ms(), ttl_secs });
    }
}
