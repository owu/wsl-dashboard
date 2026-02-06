use tokio::time::Duration;
use tracing::{info, warn};
use crate::wsl::models::WslCommandResult;
use super::WslDashboard;

impl WslDashboard {
    pub async fn start_distro(&self, name: &str) -> WslCommandResult<String> {
        self.increment_manual_operation();
        let result = self.executor.start_distro(name).await;
        if result.success {
            info!("WSL distro '{}' startup command executed, waiting for status update", name);
            let _ = self.refresh_distros().await;
            
            let manager_clone = self.clone();
            let name_clone = name.to_string();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(3)).await;
                info!("Delayed refresh of WSL distro '{}' status after startup", name_clone);
                let _ = manager_clone.refresh_distros().await;
                manager_clone.decrement_manual_operation();
            });
        } else {
            self.decrement_manual_operation();
        }
        result
    }

    pub async fn stop_distro(&self, name: &str) -> WslCommandResult<String> {
        self.increment_manual_operation();
        let result = self.executor.stop_distro(name).await;
        if result.success {
            info!("WSL distro '{}' termination command executed, waiting for status update", name);
            let _ = self.refresh_distros().await;
            
            let manager_clone = self.clone();
            let name_clone = name.to_string();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(3)).await;
                info!("Delayed refresh of WSL distro '{}' status after termination", name_clone);
                let _ = manager_clone.refresh_distros().await;
                manager_clone.decrement_manual_operation();
            });
        } else {
            self.decrement_manual_operation();
        }
        result
    }

    pub async fn restart_distro(&self, name: &str) -> WslCommandResult<String> {
        info!("WSL distro '{}' restart initiated", name);
        let stop_result = self.stop_distro(name).await;
        if !stop_result.success {
            return stop_result;
        }
        tokio::time::sleep(Duration::from_secs(4)).await;
        self.start_distro(name).await
    }

    pub async fn shutdown_wsl(&self) -> WslCommandResult<String> {
        self.increment_manual_operation();
        info!("Initiating WSL system shutdown");
        let result = self.executor.shutdown_wsl().await;
        if result.success {
            let _ = self.refresh_distros().await;
        }
        self.decrement_manual_operation();
        result
    }

    pub async fn delete_distro(&self, config_manager: &crate::config::ConfigManager, name: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        self.increment_manual_operation();

        warn!("Initiating deletion of WSL distro '{}' (irreversible operation)", name);
        let result = self.executor.delete_distro(config_manager, name).await;
        
        if result.success {
            // Update cache immediately so subsequent UI refreshes see the change
            let _ = self.refresh_distros().await;

            let manager = self.clone();
            tokio::spawn(async move {
                // Secondary check after 1s to ensure WSL state is fully settled
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    manager.refresh_distros()
                ).await;
                manager.decrement_manual_operation();
            });
        } else {
            self.decrement_manual_operation();
        }
        result
    }

    pub async fn export_distro(&self, name: &str, file_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        self.increment_manual_operation();
        let result = self.executor.export_distro(name, file_path).await;
        self.decrement_manual_operation();
        result
    }

    pub async fn import_distro(&self, name: &str, install_location: &str, file_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        self.increment_manual_operation();
        let result = self.executor.import_distro(name, install_location, file_path).await;
        if result.success {
            let _ = self.refresh_distros().await;
        }
        self.decrement_manual_operation();
        result
    }

    pub async fn move_distro(&self, name: &str, new_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        self.increment_manual_operation();
        let result = self.executor.move_distro(name, new_path).await;
        if result.success {
            let _ = self.refresh_distros().await;
        }
        self.decrement_manual_operation();
        result
    }

    pub async fn open_distro_bashrc(&self, name: &str) -> WslCommandResult<String> {
        self.executor.open_distro_folder_path(name, "~").await
    }

    #[allow(dead_code)]
    pub async fn open_distro_folder(&self, distro_name: &str) -> WslCommandResult<String> {
        self.executor.open_distro_folder(distro_name).await
    }
}
