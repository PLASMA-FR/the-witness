use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareReport {
    pub os: String,
    pub arch: String,
    pub ram: String,
    pub disk: String,
    pub gpu: String,
    pub ollama_installed: bool,
    pub llamacpp_available: bool,
}
pub fn detect() -> HardwareReport {
    HardwareReport{ os:std::env::consts::OS.into(), arch:std::env::consts::ARCH.into(), ram:cmd("sh", &["-lc","free -h 2>/dev/null | awk '/Mem:/ {print $2}' || true"]), disk:cmd("sh", &["-lc","df -h . | awk 'NR==2 {print $4}'"]), gpu:cmd("sh", &["-lc","(command -v nvidia-smi >/dev/null && nvidia-smi --query-gpu=name --format=csv,noheader | head -1) || echo none"]), ollama_installed:Command::new("sh").arg("-lc").arg("command -v ollama >/dev/null").status().map(|s|s.success()).unwrap_or(false), llamacpp_available:false }
}
fn cmd(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}
