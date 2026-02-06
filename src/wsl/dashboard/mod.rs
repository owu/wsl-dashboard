use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::{Mutex, Notify};
use tokio::time::Duration;

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
}

impl WslDashboard {
    pub fn new() -> Self {
        Self {
            executor: WslCommandExecutor::new(),
            distros: Arc::new(Mutex::new(Vec::new())),
            refresh_interval: Duration::from_secs(5),
            state_changed: Arc::new(Notify::new()),
            manual_operation: Arc::new(std::sync::atomic::AtomicI32::new(0)),
            heavy_op_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn executor(&self) -> &WslCommandExecutor {
        &self.executor
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
        if old <= 0 {
            self.manual_operation.store(0, Ordering::SeqCst);
        }
    }

    pub async fn refresh_distros(&self) -> WslCommandResult<Vec<WslDistro>> {
        let result = self.executor.list_distros().await;
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
                    tracing::debug!("WSL distribution list changed, notifying UI update");
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

    pub async fn start_monitoring(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            loop {
                if !manager.is_manual_operation() {
                    let _ = manager.refresh_distros().await;
                }
                tokio::time::sleep(manager.refresh_interval).await;
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
}

impl Default for WslDashboard {
    fn default() -> Self {
        Self::new()
    }
}

mod ops;
