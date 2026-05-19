use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{fs, path::PathBuf, process::Command};

const SERVICE_NAME: &str = "the-witness";
#[cfg(target_os = "macos")]
const MACOS_LABEL: &str = "com.thewitness.dashboard";
#[cfg(target_os = "windows")]
const WINDOWS_TASK: &str = "TheWitness";

#[derive(Debug, Clone, Serialize)]
pub struct ServiceStatus {
    pub platform: String,
    pub installed: bool,
    pub running: bool,
    pub detail: String,
}

pub fn install() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        return install_linux();
    }
    #[cfg(target_os = "macos")]
    {
        return install_macos();
    }
    #[cfg(target_os = "windows")]
    {
        return install_windows();
    }
    #[allow(unreachable_code)]
    Err(anyhow!(
        "service install is not implemented for this platform"
    ))
}
pub fn uninstall() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("systemctl")
            .args(["--user", "disable", "--now", SERVICE_NAME])
            .status();
        let p = linux_unit_path()?;
        if p.exists() {
            fs::remove_file(p)?;
        }
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        let p = macos_plist_path()?;
        let _ = Command::new("launchctl")
            .args(["unload", p.to_string_lossy().as_ref()])
            .status();
        if p.exists() {
            fs::remove_file(p)?;
        }
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("schtasks")
            .args(["/Delete", "/TN", WINDOWS_TASK, "/F"])
            .status();
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(anyhow!(
        "service uninstall is not implemented for this platform"
    ))
}
pub fn start() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("systemctl")
            .args(["--user", "start", SERVICE_NAME])
            .status()?;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("launchctl")
            .args(["load", macos_plist_path()?.to_string_lossy().as_ref()])
            .status()?;
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("schtasks")
            .args(["/Run", "/TN", WINDOWS_TASK])
            .status()?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(anyhow!(
        "service start is not implemented for this platform"
    ))
}
pub fn stop() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("systemctl")
            .args(["--user", "stop", SERVICE_NAME])
            .status()?;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("launchctl")
            .args(["unload", macos_plist_path()?.to_string_lossy().as_ref()])
            .status()?;
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("schtasks")
            .args(["/End", "/TN", WINDOWS_TASK])
            .status()?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(anyhow!("service stop is not implemented for this platform"))
}
pub fn logs() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        return Ok("Run: journalctl --user -u the-witness -f".into());
    }
    #[cfg(target_os = "macos")]
    {
        return Ok("Run: log stream --predicate 'process == \"the-witness\"' or inspect ~/Library/Logs/TheWitness".into());
    }
    #[cfg(target_os = "windows")]
    {
        return Ok("Run: Get-ScheduledTaskInfo -TaskName TheWitness; dashboard logs are in %APPDATA%\\TheWitness\\logs".into());
    }
    #[allow(unreachable_code)]
    Ok("No platform log command available".into())
}
pub fn status() -> Result<ServiceStatus> {
    #[cfg(target_os = "linux")]
    {
        let installed = linux_unit_path()?.exists();
        let running = Command::new("systemctl")
            .args(["--user", "is-active", "--quiet", SERVICE_NAME])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        return Ok(ServiceStatus {
            platform: "linux".into(),
            installed,
            running,
            detail: "systemd user service".into(),
        });
    }
    #[cfg(target_os = "macos")]
    {
        let installed = macos_plist_path()?.exists();
        return Ok(ServiceStatus {
            platform: "macos".into(),
            installed,
            running: false,
            detail: "launchd user agent; running check is best-effort via launchctl list".into(),
        });
    }
    #[cfg(target_os = "windows")]
    {
        let out = Command::new("schtasks")
            .args(["/Query", "/TN", WINDOWS_TASK])
            .output();
        let installed = out.map(|o| o.status.success()).unwrap_or(false);
        return Ok(ServiceStatus {
            platform: "windows".into(),
            installed,
            running: false,
            detail: "Scheduled Task fallback, no admin required".into(),
        });
    }
    #[allow(unreachable_code)]
    Ok(ServiceStatus {
        platform: "unknown".into(),
        installed: false,
        running: false,
        detail: "unsupported platform".into(),
    })
}

#[cfg(target_os = "linux")]
fn install_linux() -> Result<()> {
    let unit = linux_unit_path()?;
    if let Some(parent) = unit.parent() {
        fs::create_dir_all(parent)?;
    }
    let exe = std::env::current_exe()?;
    fs::write(&unit, format!("[Unit]\nDescription=The Witness dashboard and control API\nAfter=network.target\n\n[Service]\nExecStart={} dashboard --no-open\nRestart=on-failure\nRestartSec=5\n\n[Install]\nWantedBy=default.target\n", exe.display()))?;
    let _ = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status();
    let _ = Command::new("systemctl")
        .args(["--user", "enable", SERVICE_NAME])
        .status();
    Ok(())
}
#[cfg(target_os = "linux")]
fn linux_unit_path() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("systemd/user/the-witness.service"))
}

#[cfg(target_os = "macos")]
fn install_macos() -> Result<()> {
    let plist = macos_plist_path()?;
    if let Some(parent) = plist.parent() {
        fs::create_dir_all(parent)?;
    }
    let exe = std::env::current_exe()?;
    fs::write(
        &plist,
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict><key>Label</key><string>{}</string><key>ProgramArguments</key><array><string>{}</string><string>dashboard</string><string>--no-open</string></array><key>RunAtLoad</key><true/><key>KeepAlive</key><true/></dict></plist>"#,
            MACOS_LABEL,
            exe.display()
        ),
    )?;
    Ok(())
}
#[cfg(target_os = "macos")]
fn macos_plist_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Library/LaunchAgents/com.thewitness.dashboard.plist"))
}

#[cfg(target_os = "windows")]
fn install_windows() -> Result<()> {
    let exe = std::env::current_exe()?;
    Command::new("schtasks")
        .args([
            "/Create",
            "/TN",
            WINDOWS_TASK,
            "/SC",
            "ONLOGON",
            "/TR",
            &format!("\"{}\" dashboard --no-open", exe.display()),
            "/F",
        ])
        .status()?;
    Ok(())
}
