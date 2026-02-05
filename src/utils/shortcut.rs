use std::path::PathBuf;
use tracing::{info, error};

pub fn create_distro_shortcut(distro_name: &str) -> Result<(), String> {
    let desktop_dir = dirs::desktop_dir().ok_or("Failed to find desktop directory")?;
    let shortcut_path = desktop_dir.join(format!("{}.lnk", distro_name));
    
    // We use a powershell script to create the shortcut since Rust doesn't have a native light-weight crate for .lnk files
    let ps_script = format!(
        "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $s.TargetPath = 'wsl.exe'; $s.Arguments = '-d {}'; $s.IconLocation = 'wsl.exe,0'; $s.Save()",
        shortcut_path.display(),
        distro_name
    );

    let mut cmd = std::process::Command::new("powershell");
    cmd.args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    if output.status.success() {
        info!("Shortcut created for {} at {}", distro_name, shortcut_path.display());
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to create shortcut: {}", err);
        Err(err.to_string())
    }
}
