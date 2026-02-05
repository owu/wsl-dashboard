use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::task;
use tracing::{debug, error, info};

use crate::wsl::models::WslCommandResult;

// Helper structure for stateful decoding of WSL output streams
#[derive(Default)]
pub(crate) struct WslOutputDecoder {
    pub(crate) is_utf16: Option<bool>,
    pub(crate) buffer: Vec<u8>,
}

impl WslOutputDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode(&mut self, new_bytes: &[u8]) -> String {
        if new_bytes.is_empty() && self.buffer.is_empty() {
            return String::new();
        }
        
        self.buffer.extend_from_slice(new_bytes);

        // Attempt to detect encoding (if not yet determined)
        if self.is_utf16.is_none() {
            // Check BOM
            if self.buffer.len() >= 2 && self.buffer[0] == 0xFF && self.buffer[1] == 0xFE {
                self.is_utf16 = Some(true);
                self.buffer.drain(0..2);
            } else if self.buffer.len() >= 4 {
                // Heuristic detection: count proportion of 0 bytes at even positions (0-indexed 1, 3, 5...)
                let mut null_count = 0;
                let pair_count = self.buffer.len() / 2;
                for i in 0..pair_count {
                    if self.buffer[i * 2 + 1] == 0 {
                        null_count += 1;
                    }
                }
                
                if null_count >= pair_count * 60 / 100 {
                    self.is_utf16 = Some(true);
                } else {
                    // Default fallback to UTF-8
                    self.is_utf16 = Some(false);
                }
            } else {
                // Too little data, cannot determine yet unless already contains 0-byte characteristics
                if self.buffer.iter().any(|&b| b == 0) {
                    // If already saw 0 and length less than 4, might be small packet UTF-16
                    if self.buffer.len() >= 2 && self.buffer[1] == 0 {
                        self.is_utf16 = Some(true);
                    }
                }
                
                // If still not determined, don't decode yet (or fallback if it's simple ASCII without 0)
                if self.buffer.len() < 2 { return String::new(); }
            }
        }

        // Decode according to the determined encoding
        match self.is_utf16 {
            Some(true) => {
                // UTF-16 LE: must be double-byte aligned
                let data_len = self.buffer.len() & !1;
                if data_len == 0 { return String::new(); }
                
                let u16_chars: Vec<u16> = self.buffer[..data_len]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                
                self.buffer.drain(0..data_len);
                String::from_utf16_lossy(&u16_chars)
            }
            Some(false) => {
                self.decode_utf8()
            }
            None => {
                if self.buffer.is_empty() { return String::new(); }
                
                let b0 = self.buffer[0];
                
                // Detect UTF-16 LE: second byte is usually 0 (for ASCII)
                if self.buffer.len() >= 2 && self.buffer[1] == 0 {
                    self.is_utf16 = Some(true);
                    self.decode(&[])
                } 
                // Common ASCII or control characters -> UTF-8
                else if (b0 >= 0x20 && b0 <= 0x7E) || b0 == b'\r' || b0 == b'\n' || b0 == b'\t' {
                    self.is_utf16 = Some(false);
                    self.decode_utf8()
                }
                // BOM detection (UTF-8 or UTF-16)
                else if b0 == 0xEF || b0 == 0xFF || b0 == 0xFE {
                    if self.buffer.len() >= 3 && self.buffer[0] == 0xEF && self.buffer[1] == 0xBB && self.buffer[2] == 0xBF {
                        // UTF-8 BOM
                        self.is_utf16 = Some(false);
                        self.buffer.drain(..3);
                        self.decode(&[])
                    } else if self.buffer.len() >= 2 {
                        if self.buffer[0] == 0xFF && self.buffer[1] == 0xFE {
                            self.is_utf16 = Some(true);
                            self.buffer.drain(..2);
                            self.decode(&[])
                        } else {
                            self.is_utf16 = Some(false);
                            self.decode_utf8()
                        }
                    } else {
                        String::new()
                    }
                }
                // Non-ASCII and non-0 -> likely UTF-8 multi-byte sequence
                else if self.buffer.len() >= 3 {
                    self.is_utf16 = Some(false);
                    self.decode_utf8()
                }
                else {
                    String::new()
                }
            }
        }
    }

    fn decode_utf8(&mut self) -> String {
        // Try to parse as UTF-8. 
        // Note: On Chinese Windows, WSL output might be GBK (CP936) even with WSL_UTF8=1 for some system messages.
        match std::str::from_utf8(&self.buffer) {
            Ok(_) => {
                let s = String::from_utf8_lossy(&self.buffer).to_string();
                self.buffer.clear();
                s
            }
            Err(e) => {
                let valid_len = e.valid_up_to();
                if valid_len > 0 {
                    let s = String::from_utf8_lossy(&self.buffer[..valid_len]).to_string();
                    self.buffer.drain(..valid_len);
                    s
                } else {
                    // If buffer is full of unrecognized characters (> 4 bytes), most likely it's another encoding (like GBK)
                    // We use from_utf8_lossy anyway but we don't clear the buffer unless it's getting too large.
                    // This prevents getting stuck on a single multi-byte sequence.
                    if self.buffer.len() > 8 {
                        let s = String::from_utf8_lossy(&self.buffer).to_string();
                        self.buffer.clear();
                        s
                    } else {
                        String::new()
                    }
                }
            }
        }
    }
}

// Decode byte data to string, automatically detecting UTF-16 LE or UTF-8 encoding
pub fn decode_output(bytes: &[u8]) -> String {
    let mut decoder = WslOutputDecoder::new();
    decoder.decode(bytes)
}

// WSL command executor, responsible for executing various WSL commands
#[derive(Clone, Default)]
pub struct WslCommandExecutor;

impl WslCommandExecutor {
    // Create a new WSL command executor instance
    pub fn new() -> Self {
        Self
    }

    // Execute WSL commands asynchronously
    pub async fn execute_command(&self, args: &[&str]) -> WslCommandResult<String> {
        // Convert args to owned string vector for use in closure
        let args_owned: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
        let command_str = format!("wsl {}", args_owned.join(" "));
        
        // Identify if the command is a write operation (state changing)
        let write_ops = [
            "--import", "--export", "--unregister", "--install", 
            "--set-version", "--set-default-version", "--set-default", "-s",
            "--shutdown", "--terminate", "-t", "--mount", "--unmount", "--update"
        ];
        
        let is_write_op = args_owned.iter().any(|arg| write_ops.contains(&arg.to_lowercase().as_str()));

        // Log the executed command
        if is_write_op {
            info!("Executing WSL command: {}", command_str);
        } else {
            debug!("Executing WSL command: {}", command_str);
        }
        
        let command_str_clone = command_str.clone();
        let join_handle = task::spawn_blocking(move || {
            let mut command = std::process::Command::new("wsl.exe");
            command.args(&args_owned);
            command.env("WSL_UTF8", "1");
            
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                command.creation_flags(CREATE_NO_WINDOW);
            }
            
            // Inner log also respecting the op type
            if is_write_op {
                 info!("WSL process starting: {}", command_str_clone);
            } else {
                 debug!("WSL process starting: {}", command_str_clone);
            }

            let output = command.output();

            match output {
                Ok(output) => {
                    // Use auto-detect encoding decoding function
                    let stdout = decode_output(&output.stdout);
                    let stderr = decode_output(&output.stderr);
                    
                    if is_write_op {
                        info!("WSL command stdout: {}", stdout);
                        if !stderr.is_empty() {
                            info!("WSL command stderr: {}", stderr);
                        }
                        info!("WSL command exit status: {}", output.status);
                    } else {
                        debug!("WSL command stdout: {}", stdout);
                        debug!("WSL command stderr: {}", stderr);
                        debug!("WSL command exit status: {}", output.status);
                    }

                    if output.status.success() {
                        WslCommandResult::success(stdout, None)
                    } else {
                        // FIX: If stderr is empty, use stdout as the error message. 
                        let final_error = if stderr.trim().is_empty() && !stdout.trim().is_empty() {
                            stdout.clone()
                        } else {
                            stderr
                        };
                        WslCommandResult::error(stdout, final_error)
                    }
                }
                Err(e) => {
                    let error = format!("Failed to execute command: {}", e);
                    error!("WSL command execution error: {}", error);
                    WslCommandResult::error(String::new(), error)
                }
            }
        });

        let timeout_duration = if is_write_op {
            std::time::Duration::from_secs(600) // 10 minutes for write operations
        } else {
            std::time::Duration::from_secs(15)  // 15 seconds for read operations
        };

        match tokio::time::timeout(timeout_duration, join_handle).await {
            Ok(spawn_result) => {
                spawn_result.unwrap_or_else(|e| {
                    let error = format!("Command execution panicked: {}", e);
                    error!("WSL command panic: {}", error);
                    WslCommandResult::error(String::new(), error)
                })
            }
            Err(_) => {
                let error = format!("WSL command timed out after {}s: {}", timeout_duration.as_secs(), command_str);
                error!("{}", error);
                WslCommandResult::error(String::new(), error)
            }
        }
    }
 
    // Execute WSL commands asynchronously and callback output in real-time
    pub async fn execute_command_streaming<F>(&self, args: &[&str], mut callback: F) -> WslCommandResult<String>
    where
        F: FnMut(String) + Send + 'static,
    {
        let args_owned: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
        let command_str = format!("wsl {}", args_owned.join(" "));
        info!("Executing Streaming WSL command: {}", command_str);
 
        let mut cmd = tokio::process::Command::new("wsl.exe");
        cmd.args(&args_owned)
           .env("WSL_UTF8", "1")
           .stdin(Stdio::null())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        #[cfg(windows)]
        {
            #[allow(unused_imports)]
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = match cmd.spawn()
        {
            Ok(child) => {
                info!("Process spawned successfully, PID: {:?}", child.id());
                child
            },
            Err(e) => return WslCommandResult::error(String::new(), format!("Failed to spawn wsl: {}", e)),
        };
 
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        
        let mut full_output = String::new();
        let mut out_buf = [0u8; 1024];
        let mut err_buf = [0u8; 1024];
        
        let mut out_decoder = WslOutputDecoder::new();
        let mut err_decoder = WslOutputDecoder::new();
        
        let mut stdout_done = false;
        let mut stderr_done = false;
 
        let mut exit_status = None;

        while (!stdout_done || !stderr_done) && exit_status.is_none() {
            tokio::select! {
                result = stdout.read(&mut out_buf), if !stdout_done => {
                    match result {
                        Ok(0) => {
                            info!("STDOUT reached EOF");
                            stdout_done = true;
                        }
                        Ok(n) => {
                            let text = out_decoder.decode(&out_buf[..n]);
                            if !text.is_empty() {
                                full_output.push_str(&text);
                                callback(text);
                            }
                        }
                        Err(e) => {
                            error!("STDOUT read error: {}", e);
                            stdout_done = true;
                        }
                    }
                }
                result = stderr.read(&mut err_buf), if !stderr_done => {
                    match result {
                        Ok(0) => {
                            stderr_done = true;
                        }
                        Ok(n) => {
                            let text = err_decoder.decode(&err_buf[..n]);
                            if !text.is_empty() {
                                full_output.push_str(&text);
                                callback(text);
                            }
                        }
                        Err(e) => {
                            error!("STDERR read error: {}", e);
                            stderr_done = true;
                        }
                    }
                }
                status = child.wait() => {
                    exit_status = Some(status);
                }
            }
        }

        let status = match exit_status {
            Some(s) => s.map_err(|e| e.to_string()),
            None => child.wait().await.map_err(|e| e.to_string()),
        };
        match status {
            Ok(s) => {
                info!("Process exited with status: {}", s);
                if s.success() {
                    WslCommandResult::success(full_output.clone(), None)
                } else {
                    // FIX: Also handle streaming failure by checking if full_output contains error details
                    let err_msg = format!("Process exited with error: {}", s);
                    WslCommandResult::error(full_output, err_msg)
                }
            }
            Err(e) => {
                error!("Failed to wait for process: {}", e);
                WslCommandResult::error(full_output, e)
            }
        }
    }

    pub async fn check_path_exists(&self, distro_name: &str, path: &str) -> bool {
        if path == "~" {
            return true;
        }
        // wsl -d distro -e test -d path
        let result = self.execute_command(&["-d", distro_name, "-e", "test", "-d", path]).await;
        result.success
    }

    pub async fn check_file_executable(&self, distro_name: &str, path: &str) -> (bool, bool) {
        // Execute [ -f path ] to check if it's a file
        let exists_res = self.execute_command(&["-d", distro_name, "-u", "root", "-e", "test", "-f", path]).await;
        // Execute [ -x path ] to check if it's executable
        let exec_res = self.execute_command(&["-d", distro_name, "-u", "root", "-e", "test", "-x", path]).await;
        (exists_res.success, exec_res.success)
    }

    pub async fn get_distro_default_uid(&self, distro_name: &str) -> WslCommandResult<u32> {
        crate::wsl::ops::info::get_distro_default_uid(self, distro_name).await
    }

    pub async fn set_distro_default_uid(&self, distro_name: &str, uid: u32) -> WslCommandResult<String> {
        crate::wsl::ops::lifecycle::set_distro_default_uid(self, distro_name, uid).await
    }

    pub async fn get_systemd_status(&self, distro_name: &str) -> bool {
        crate::wsl::ops::distro_config::get_systemd_status(self, distro_name).await
    }

    pub async fn set_systemd_status(&self, distro_name: &str, enabled: bool) -> WslCommandResult<String> {
        crate::wsl::ops::distro_config::set_systemd_status(self, distro_name, enabled).await
    }

    pub async fn get_wsl_conf(&self, distro_name: &str) -> crate::wsl::models::WslConf {
        crate::wsl::ops::distro_config::get_wsl_conf(self, distro_name).await
    }

    pub async fn check_windows_features(&self) -> WslCommandResult<Vec<(String, bool)>> {
        crate::wsl::ops::info::check_windows_features(self).await
    }

    pub async fn set_wsl_conf(&self, distro_name: &str, conf: crate::wsl::models::WslConf) -> WslCommandResult<String> {
        crate::wsl::ops::distro_config::set_wsl_conf(self, distro_name, conf).await
    }

    pub async fn reset_wsl_network(&self) -> WslCommandResult<String> {
        crate::wsl::ops::lifecycle::reset_wsl_network(self).await
    }

    pub async fn compact_distro_disk(&self, distro_name: &str, vhdx_path: &str) -> WslCommandResult<String> {
        crate::wsl::ops::lifecycle::compact_distro_disk(self, distro_name, vhdx_path).await
    }
}
