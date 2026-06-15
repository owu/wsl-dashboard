// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::collections::HashMap;
use tokio::sync::{Mutex, Notify};
use tokio::time::Duration;
use tracing::trace;

use crate::wsl::command::WslCommandExecutor;
use crate::wsl::models::{WslDistro, WslCommandResult, WslStatus};

// WSL state manager, responsible for managing and monitoring the status of WSL subsystems
#[derive(Clone)]
pub struct WslDashboard {
    // WSL command executor, responsible for executing actual WSL commands
    executor: WslCommandExecutor,
    // List of currently installed WSL subsystems, using Mutex for concurrent safety
    pub distros: Arc<Mutex<Vec<WslDistro>>>,
    // Status refresh interval, default 5 seconds
    refresh_interval: Duration,
    // Status change notifier, notifies listeners when WSL status changes
    state_changed: Arc<Notify>,
    // Manual operation flag
    manual_operation: Arc<std::sync::atomic::AtomicI32>,
    // Heavy operation lock
    heavy_op_lock: Arc<Mutex<()>>,
    // Active operations per distro (DistroName -> OpName)
    active_ops: Arc<Mutex<HashMap<String, String>>>,
    // Whether the last wsl -l -v command succeeded
    last_command_ok: Arc<std::sync::atomic::AtomicBool>,
}

impl WslDashboard {
    pub fn new(initial_distros: Vec<WslDistro>) -> Self {
        Self {
            executor: WslCommandExecutor::new(),
            distros: Arc::new(Mutex::new(initial_distros)),
            refresh_interval: Duration::from_secs(5),
            state_changed: Arc::new(Notify::new()),
            manual_operation: Arc::new(std::sync::atomic::AtomicI32::new(0)),
            heavy_op_lock: Arc::new(Mutex::new(())),
            active_ops: Arc::new(Mutex::new(HashMap::new())),
            last_command_ok: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    pub fn executor(&self) -> &WslCommandExecutor {
        &self.executor
    }

    pub fn heavy_op_lock(&self) -> &Mutex<()> {
        &*self.heavy_op_lock
    }

    #[allow(dead_code)]
    pub fn set_refresh_interval(&mut self, interval: Duration) {
        self.refresh_interval = interval;
    }

    pub fn state_changed(&self) -> &Arc<Notify> {
        &self.state_changed
    }

    pub fn is_manual_operation(&self) -> bool {
        self.manual_operation.load(Ordering::SeqCst) > 0
    }

    pub fn last_command_ok(&self) -> bool {
        self.last_command_ok.load(Ordering::SeqCst)
    }

    pub fn set_manual_operation(&self, value: bool) {
        if value {
            self.increment_manual_operation();
        } else {
            self.decrement_manual_operation();
        }
    }

    pub fn increment_manual_operation(&self) {
        self.manual_operation.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_manual_operation(&self) {
        let old = self.manual_operation.fetch_sub(1, Ordering::SeqCst);
        if old == 1 {
            // Just finished the last manual operation
            let self_clone = self.clone();
            tokio::spawn(async move {
                trace!("All manual operations complete, performing final sync...");
                let _ = self_clone.refresh_distros().await;
            });
        }
        if old <= 0 {
            self.manual_operation.store(0, Ordering::SeqCst);
        }
    }

    pub async fn refresh_distros(&self) -> WslCommandResult<Vec<WslDistro>> {
        // Intercept: Skip wsl -l -v if heavy operation is in progress
        // if self.heavy_op_lock.try_lock().is_err() {
        //     debug!("Heavy operation in progress, skipping wsl -l -v to avoid conflicts");
        //     let cached_distros = {
        //         let distros_lock = self.distros.lock().await;
        //         distros_lock.clone()
        //     };
        //     return WslCommandResult::success(
        //         "Skipped refresh due to heavy operation".to_string(),
        //         Some(cached_distros)
        //     );
        // }

        let result = self.executor.list_distros().await;
        self.last_command_ok.store(result.success, Ordering::SeqCst);
        if result.success {
            if let Some(distros) = result.data.clone() {
                let mut distros_lock = self.distros.lock().await;
                let old_distros = distros_lock.clone();
                *distros_lock = distros.clone();
                
                let mut has_changes = false;
                if old_distros.len() != distros.len() {
                    has_changes = true;
                } else {
                    for (old, new) in old_distros.iter().zip(distros.iter()) {
                        if !old.business_equals(new) {
                            has_changes = true;
                            break;
                        }
                    }
                }
                
                if has_changes {
                    trace!("WSL distribution list changed, notifying UI update");
                    self.state_changed.notify_one();
                }
            }
        }
        result
    }

    pub async fn get_distros(&self) -> Vec<WslDistro> {
        let distros_lock = self.distros.lock().await;
        distros_lock.clone()
    }

    #[allow(dead_code)]
    pub async fn start_monitoring(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.refresh_interval);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                interval.tick().await;
                if !manager.is_manual_operation() {
                    let _ = manager.refresh_distros().await;
                }
            }
        });
    }

    #[allow(dead_code)]
    pub async fn get_distro(&self, name: &str) -> Option<WslDistro> {
        let distros_lock = self.distros.lock().await;
        distros_lock.iter().find(|d| d.name == name).cloned()
    }

    #[allow(dead_code)]
    pub async fn is_distro_running(&self, name: &str) -> bool {
        if let Some(distro) = self.get_distro(name).await {
            return matches!(distro.status, WslStatus::Running);
        }
        false
    }

    pub async fn register_operation(&self, distro_name: String, op_name: String) {
        let mut ops = self.active_ops.lock().await;
        ops.insert(distro_name.clone(), op_name.clone());
        trace!("Operation registered for '{}': {}", distro_name, op_name);
    }

    pub async fn unregister_operation(&self, distro_name: &str) {
        let mut ops = self.active_ops.lock().await;
        if let Some(op) = ops.remove(distro_name) {
            trace!("Operation unregistered for '{}': {}", distro_name, op);
        }
    }

    pub async fn get_active_op(&self, distro_name: &str) -> Option<String> {
        let ops = self.active_ops.lock().await;
        ops.get(distro_name).cloned()
    }

    pub async fn mark_distro_stopped(&self, name: &str) {
        let mut distros_lock = self.distros.lock().await;
        let mut changed = false;
        for distro in distros_lock.iter_mut() {
            if distro.name == name && !matches!(distro.status, WslStatus::Stopped) {
                distro.status = WslStatus::Stopped;
                changed = true;
                break;
            }
        }
        if changed {
            tracing::trace!("Artificially marked distro '{}' as Stopped", name);
            self.state_changed.notify_one();
        }
    }

    pub async fn mark_all_distros_stopped(&self) {
        let mut distros_lock = self.distros.lock().await;
        let mut changed = false;
        for distro in distros_lock.iter_mut() {
            if !matches!(distro.status, WslStatus::Stopped) {
                distro.status = WslStatus::Stopped;
                changed = true;
            }
        }
        if changed {
            tracing::trace!("Artificially marked all distros as Stopped");
            self.state_changed.notify_one();
        }
    }
}

impl Default for WslDashboard {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

mod ops;
pub mod operation_guard;
