use std::{net::IpAddr, process::Command};

pub fn detect_tailscale_ipv4() -> Option<IpAddr> {
    let output = Command::new("tailscale").args(["ip", "-4"]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_tailscale_ipv4(&stdout)
}

pub fn parse_tailscale_ipv4(output: &str) -> Option<IpAddr> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<IpAddr>().ok())
        .find(|ip| ip.is_ipv4())
}
