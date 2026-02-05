use serde::{Deserialize, Serialize};

// WSL subsystem version
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WslVersion {
    V1,
    V2,
}

// WSL subsystem status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WslStatus {
    Running,
    Stopped,
}

// WSL subsystem information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WslDistro {
    pub name: String,
    pub status: WslStatus,
    pub version: WslVersion,
    pub is_default: bool,
    pub last_start_time: Option<std::time::SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WslInformation {
    pub distro_name: String,
    pub wsl_version: String,
    pub status: String,
    pub ip_address: String,
    pub install_location: String,
    pub vhdx_path: String,
    pub vhdx_size: String,
    pub actual_used: String,
    pub docker_containers: Vec<String>,
    pub package_family_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WslConf {
    pub systemd: bool,
    pub generate_hosts: bool,
    pub generate_resolv_conf: bool,
    pub interop_enabled: bool,
    pub append_windows_path: bool,
}

impl WslDistro {
    // Check if two WSL subsystems are logically equal (ignore startup time)
    pub fn business_equals(&self, other: &WslDistro) -> bool {
        self.name == other.name 
            && self.status == other.status
            && self.version == other.version
            && self.is_default == other.is_default
    }
}

// WSL command execution result
#[derive(Debug, Clone)]
pub struct WslCommandResult<T> {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub data: Option<T>,
}

impl<T> WslCommandResult<T> {
    #[allow(dead_code)]
    pub fn new(success: bool, output: String, error: Option<String>, data: Option<T>) -> Self {
        Self {
            success,
            output,
            error,
            data,
        }
    }

    pub fn success(output: String, data: Option<T>) -> Self {
        Self {
            success: true,
            output,
            error: None,
            data,
        }
    }

    pub fn error(output: String, error: String) -> Self {
        Self {
            success: false,
            output,
            error: Some(error),
            data: None,
        }
    }
}