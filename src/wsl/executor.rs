use std::process::Stdio;
use tokio::io::AsyncReadExt;
// use tokio::task; // Removed
use tracing::{debug, error, info, warn};

use crate::wsl::models::WslCommandResult;

use crate::wsl::decoder::{decode_output, WslOutputDecoder};

// WSL command executor, responsible for executing various WSL commands
#[derive(Clone)]
pub struct WslCommandExecutor {
    // Limit concurrent WSL commands to prevent resource exhaustion
    semaphore: std::sync::Arc<tokio::sync::Semaphore>,
    // Semaphore to limit concurrent background heavy operations (like launcher cleanup)
    background_semaphore: std::sync::Arc<tokio::sync::Semaphore>,
}

impl Default for WslCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl WslCommandExecutor {
    // Create a new WSL command executor instance
    pub fn new() -> Self {
        Self {
            // Limit to 16 concurrent operations. Higher than before to buffer hangs.
            semaphore: std::sync::Arc::new(tokio::sync::Semaphore::new(16)),
            // Limit to 4 concurrent background heavy operations
            background_semaphore: std::sync::Arc::new(tokio::sync::Semaphore::new(4)),
        }
    }

    pub fn background_semaphore(&self) -> &std::sync::Arc<tokio::sync::Semaphore> {
        &self.background_semaphore
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
        
        // Use case-insensitive check for write operations
        let is_write_op = args_owned.iter().any(|arg| {
            let lower = arg.to_lowercase();
            write_ops.contains(&lower.as_str())
        });

        // Log the executed command
        if is_write_op {
            info!("Executing WSL command: {}", command_str);
        } else {
            debug!("Executing WSL command: {}", command_str);
        }
        
        if is_write_op {
            debug!("Starting async WSL command: {}", command_str);
        }

        let future = async {
            let mut cmd = tokio::process::Command::new("wsl.exe");
            cmd.args(&args_owned);
            cmd.env("WSL_UTF8", "1");
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            
            #[cfg(windows)]
            {
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            // Ensure process is killed if the future is dropped (timeout/cancellation)
            cmd.kill_on_drop(true);

            debug!("Spawning wsl process for: {}", command_str);
            let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn wsl process: {}", e))?;
            debug!("Wsl process spawned (pid: {:?})", child.id());
            
            let mut stdout = child.stdout.take().ok_or_else(|| "Failed to capture stdout".to_string())?;
            let mut stderr = child.stderr.take().ok_or_else(|| "Failed to capture stderr".to_string())?;

            let mut stdout_data = Vec::new();
            let mut stderr_data = Vec::new();
            
            const MAX_OUTPUT_SIZE: usize = 1024 * 1024; // 1MB limit - more than enough for text output

            let read_stdout = async {
                debug!("Reading stdout for: {}", command_str);
                let mut buf = [0u8; 8192];
                loop {
                    let n = stdout.read(&mut buf).await.map_err(|e| format!("Stdout read error: {}", e))?;
                    if n == 0 { break; }
                    if stdout_data.len() + n > MAX_OUTPUT_SIZE {
                        stdout_data.extend_from_slice(b"\n... [TRUNCATED DUE TO SIZE LIMIT]");
                        break;
                    }
                    stdout_data.extend_from_slice(&buf[..n]);
                }
                debug!("Stdout reading complete for: {}", command_str);
                Ok::<(), String>(())
            };

            let read_stderr = async {
                debug!("Reading stderr for: {}", command_str);
                let mut buf = [0u8; 8192];
                loop {
                    let n = stderr.read(&mut buf).await.map_err(|e| format!("Stderr read error: {}", e))?;
                    if n == 0 { break; }
                    if stderr_data.len() + n > MAX_OUTPUT_SIZE {
                        stderr_data.extend_from_slice(b"\n... [TRUNCATED DUE TO SIZE LIMIT]");
                        break;
                    }
                    stderr_data.extend_from_slice(&buf[..n]);
                }
                debug!("Stderr reading complete for: {}", command_str);
                Ok::<(), String>(())
            };

            debug!("Waiting for output streams and process exit for: {}", command_str);
            let (res_out, res_err) = tokio::join!(read_stdout, read_stderr);
            
            // Wait for process to exit
            let status = child.wait().await.map_err(|e| format!("Failed to wait for child: {}", e))?;
            debug!("Wsl process exited with status: {} for: {}", status, command_str);
            
            if let Err(e) = res_out { error!("Stdout error: {}", e); }
            if let Err(e) = res_err { error!("Stderr error: {}", e); }

            Ok::<(Vec<u8>, Vec<u8>, std::process::ExitStatus), String>((stdout_data, stderr_data, status))
        };

        // Detect heavy operations that need much longer timeouts (e.g., multi-GiB disk transfers)
        let is_heavy_op = args_owned.iter().any(|arg| {
            let lower = arg.to_lowercase();
            lower == "--import" || lower == "--export" || lower == "--install"
        });

        let timeout_duration = if is_heavy_op {
            std::time::Duration::from_secs(1800) // 30 minutes for large disk operations
        } else if is_write_op {
            std::time::Duration::from_secs(45)   // 45 seconds for normal write operations
        } else {
            std::time::Duration::from_secs(10)   // 10 seconds for read operations
        };

        if is_heavy_op {
            info!("Executing heavy WSL operation with 30m timeout: {}", command_str);
        }

        // Acquire semaphore permit with its own timeout to avoid deadlock if slots are stuck
        let permit_timeout = std::time::Duration::from_secs(20);
        debug!("Acquiring WSL semaphore permit (Available: {}/16) for: {}", self.semaphore.available_permits(), command_str);
        let _permit = match tokio::time::timeout(permit_timeout, self.semaphore.acquire()).await {
            Ok(Ok(p)) => p,
            Ok(Err(_)) => {
                let err = "Failed to acquire semaphore permit (closed)".to_string();
                error!("{}", err);
                return WslCommandResult::error(String::new(), err);
            }
            Err(_) => {
                let err = format!("WSL command pending timeout after {}s (Queue full): {}", permit_timeout.as_secs(), command_str);
                warn!("{}", err);
                return WslCommandResult::error(String::new(), err);
            }
        };
        debug!("WSL semaphore permit acquired for: {}", command_str);

        let result = match tokio::time::timeout(timeout_duration, future).await {
            Ok(Ok((stdout_bytes, stderr_bytes, status))) => {
                let stdout = decode_output(&stdout_bytes);
                let stderr = decode_output(&stderr_bytes);

                fn truncate_log(s: &str, max_len: usize) -> String {
                    if s.len() > max_len {
                        format!("{}... (truncated, total {} chars)", &s[..max_len], s.len())
                    } else {
                        s.to_string()
                    }
                }

                if is_write_op {
                    info!("WSL command stdout: {}", truncate_log(&stdout, 1000));
                    if !stderr.is_empty() {
                        info!("WSL command stderr: {}", truncate_log(&stderr, 1000));
                    }
                    info!("WSL command exit status: {}", status);
                } else {
                    debug!("WSL command stdout: {}", truncate_log(&stdout, 1000));
                    debug!("WSL command stderr: {}", truncate_log(&stderr, 1000));
                    debug!("WSL command exit status: {}", status);
                }

                if status.success() {
                    WslCommandResult::success(stdout, None)
                } else {
                    let final_error = if stderr.trim().is_empty() && !stdout.trim().is_empty() {
                        stdout.clone()
                    } else {
                        stderr
                    };
                    WslCommandResult::error(stdout, final_error)
                }
            }
            Ok(Err(e)) => {
                let error = format!("Command execution failed: {}", e);
                error!("WSL command error: {}", error);
                WslCommandResult::error(String::new(), error)
            }
            Err(_) => {
                let error = format!("WSL command timed out after {}s: {}", timeout_duration.as_secs(), command_str);
                error!("{}", error);
                // Child is killed automatically due to kill_on_drop(true)
                WslCommandResult::error(String::new(), error)
            }
        };

        drop(_permit);
        result
    }
 
    // Execute WSL commands asynchronously and callback output in real-time
    pub async fn execute_command_streaming<F>(&self, args: &[&str], mut callback: F) -> WslCommandResult<String>
    where
        F: FnMut(String) + Send + 'static,
    {
        let args_owned: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
        let command_str = format!("wsl {}", args_owned.join(" "));
        info!("Executing Streaming WSL command: {}", command_str);

        let future = async {
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
            
            // Ensure process is killed on drop
            cmd.kill_on_drop(true);

            let mut child = match cmd.spawn()
            {
                Ok(child) => {
                    info!("Process spawned successfully, PID: {:?}", child.id());
                    child
                },
                Err(e) => return Err(format!("Failed to spawn wsl: {}", e)),
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

            // Wait for both process exit AND EOF on streams
            while !stdout_done || !stderr_done {
                tokio::select! {
                    result = stdout.read(&mut out_buf), if !stdout_done => {
                        match result {
                            Ok(0) => {
                                debug!("Streaming STDOUT reached EOF for: {}", command_str);
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
                                error!("Streaming STDOUT read error: {}", e);
                                stdout_done = true;
                            }
                        }
                    }
                    result = stderr.read(&mut err_buf), if !stderr_done => {
                        match result {
                            Ok(0) => {
                                debug!("Streaming STDERR reached EOF for: {}", command_str);
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
                                error!("Streaming STDERR read error: {}", e);
                                stderr_done = true;
                            }
                        }
                    }
                    // We don't exit early on process exit, we wait for EOF to get all data
                }
            }

            let status = child.wait().await.map_err(|e| format!("Wait failed: {}", e))?;
            info!("Streaming process exited with status: {} for: {}", status, command_str);
            
            Ok((full_output, status))
        };

        // Streaming commands usually used for install/import, so 30m timeout
        let timeout_duration = std::time::Duration::from_secs(1800);
        
        // Also respect semaphore for consistency
        let permit_timeout = std::time::Duration::from_secs(10);
        let _permit = match tokio::time::timeout(permit_timeout, self.semaphore.acquire()).await {
            Ok(Ok(p)) => p,
            _ => {
                warn!("Streaming WSL command started without semaphore slot (too busy): {}", command_str);
                // We proceed anyway but with a warning, or we could return error. 
                // For install, it's better to fail early if WSL is totally unresponsive.
                return WslCommandResult::error(String::new(), "WSL service busy, please try again later".to_string());
            }
        };

        match tokio::time::timeout(timeout_duration, future).await {
            Ok(Ok((full_output, status))) => {
                if status.success() {
                    WslCommandResult::success(full_output, None)
                } else {
                    let err_msg = format!("Process exited with error: {}", status);
                    WslCommandResult::error(full_output, err_msg)
                }
            }
            Ok(Err(e)) => {
                error!("Streaming command failed: {}", e);
                WslCommandResult::error(String::new(), e)
            }
            Err(_) => {
                let error = format!("Streaming WSL command timed out after {}s: {}", timeout_duration.as_secs(), command_str);
                error!("{}", error);
                WslCommandResult::error(String::new(), error)
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
}
