use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::task;
use tracing::{debug, error, info};

use crate::wsl::models::WslCommandResult;

use crate::wsl::decoder::{decode_output, WslOutputDecoder};

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
}
