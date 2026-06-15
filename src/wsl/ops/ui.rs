// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use tokio::task;
use tracing::warn;
use crate::wsl::executor::WslCommandExecutor;
use crate::wsl::models::WslCommandResult;

pub async fn open_distro_folder(_executor: &WslCommandExecutor, distro_name: &str) -> WslCommandResult<String> {
    let path = format!(r"\\wsl$\{}", distro_name);
    task::spawn_blocking(move || {
        let mut command = std::process::Command::new("explorer.exe");
        command.arg(path);
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let output = command.status();
        match output {
            Ok(_) => WslCommandResult::success(String::new(), None),
            Err(e) => WslCommandResult::error(String::new(), e.to_string()),
        }
    }).await.unwrap()
}

pub async fn open_distro_vscode(_executor: &WslCommandExecutor, distro_name: &str, working_dir: &str) -> WslCommandResult<String> {
    let remote_arg = format!("wsl+{}", distro_name);
    let dir = working_dir.to_string();
    task::spawn_blocking(move || {
        let mut command = std::process::Command::new("powershell");
        // Using -Command with formatted string ensures it's executed correctly in PS
        let ps_command = format!("code --remote {} '{}'", remote_arg, dir);
        command.args(&["-NoProfile", "-Command", &ps_command]);
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let output = command.status();
        match output {
            Ok(_) => WslCommandResult::success(String::new(), None),
            Err(e) => WslCommandResult::error(String::new(), e.to_string()),
        }
    }).await.unwrap()
}

pub async fn check_vscode_extension(_executor: &WslCommandExecutor) -> WslCommandResult<String> {
    task::spawn_blocking(move || {
        let mut command = std::process::Command::new("powershell");
        command.args(&["-NoProfile", "-Command", "code --list-extensions"]);
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        
        let output = command.output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                if out.status.success() {
                    WslCommandResult::success(stdout, None)
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    WslCommandResult::error(stdout, stderr)
                }
            },
            Err(e) => WslCommandResult::error(String::new(), e.to_string()),
        }
    }).await.unwrap()
}

fn is_wt_available() -> bool {
    // Use pure-Rust PATH search instead of spawning where.exe.
    // Spawning where.exe (a console app) without CREATE_NO_WINDOW causes a
    // brief console flash every time the terminal button is clicked.
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths)
                .any(|dir| dir.join("wt.exe").is_file())
        })
        .unwrap_or(false)
}

fn open_with_wt(name: &str, cd_path: &str, proxy_exports: &Option<Vec<(String, String)>>) -> Result<(), std::io::Error> {
    let mut command = std::process::Command::new("wt");
    let mut args = vec![
        "--title".to_string(), format!("WSL: {}", name),
        "wsl.exe".to_string(), "-d".to_string(), name.to_string(),
    ];

    if !cd_path.is_empty() {
        args.extend(vec!["--cd".to_string(), cd_path.to_string()]);
    }

    if let Some(exports) = proxy_exports {
        let mut wslenv = String::new();
        for (k, v) in exports {
            wslenv.push_str(&format!("{}/u:", k));
            command.env(k, v);
        }
        if !wslenv.is_empty() {
            wslenv.pop();
            command.env("WSLENV", wslenv);
        }
    }

    command.args(&args);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // Use DETACHED_PROCESS instead of CREATE_NO_WINDOW to prevent console
        // allocation entirely. wt.exe is a broker that briefly spawns, delegates
        // to the real Windows Terminal via COM, then exits. CREATE_NO_WINDOW still
        // allocates (then hides) a console, which can cause a visible flash.
        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW);
    }

    // Use spawn() instead of status() — wt.exe is a broker process that exits
    // immediately after launching the actual Windows Terminal window. There is
    // no meaningful exit code to wait for, and blocking here is unnecessary.
    command.spawn().map(|_| ())
}

fn open_with_cmd(name: &str, cd_path: &str, proxy_exports: &Option<Vec<(String, String)>>) -> Result<(), std::io::Error> {
    let mut command = std::process::Command::new("cmd");

    if let Some(exports) = proxy_exports {
        let mut export_cmds = String::new();
        for (k, v) in exports {
            export_cmds.push_str(&format!("export {}={}; ", k, v));
        }
        let bash_cmd = format!("{}exec bash -l", export_cmds);
        command.args(&["/c", "start", &format!("WSL: {}", name), "wsl.exe", "-d", name, "--cd", cd_path, "--", "bash", "-c", &bash_cmd]);
    } else {
        command.args(&["/c", "start", &format!("WSL: {}", name), "wsl.exe", "-d", name, "--cd", cd_path]);
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.status().map(|_| ())
}

pub async fn open_distro_terminal(_executor: &WslCommandExecutor, distro_name: &str, working_dir: &str, proxy_exports: Option<Vec<(String, String)>>) -> WslCommandResult<String> {
    let name = distro_name.to_string();
    let cd_path = working_dir.to_string();

    task::spawn_blocking(move || {
        if is_wt_available() {
            match open_with_wt(&name, &cd_path, &proxy_exports) {
                Ok(_) => return WslCommandResult::success(String::new(), None),
                Err(e) => {
                    warn!("wt.exe failed: {}, falling back to cmd.exe", e);
                }
            }
        }
        match open_with_cmd(&name, &cd_path, &proxy_exports) {
            Ok(_) => WslCommandResult::success(String::new(), None),
            Err(e) => WslCommandResult::error(String::new(), e.to_string()),
        }
    }).await.unwrap()
}

pub async fn open_distro_folder_path(_executor: &WslCommandExecutor, distro_name: &str, sub_path: &str) -> WslCommandResult<String> {
    let path = if sub_path == "~" {
        format!(r"\\wsl$\{}", distro_name)
    } else {
        format!(r"\\wsl$\{}\{}", distro_name, sub_path.replace("/", "\\"))
    };

    task::spawn_blocking(move || {
        let mut command = std::process::Command::new("explorer.exe");
        command.arg(path);
        
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let output = command.status();
        match output {
            Ok(_) => WslCommandResult::success(String::new(), None),
            Err(e) => WslCommandResult::error(String::new(), e.to_string()),
        }
    }).await.unwrap()
}
