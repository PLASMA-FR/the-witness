use std::net::SocketAddr;

use clap::Parser;
use the_witness::{
    cli::{Cli, Commands},
    control::{bind_dashboard_listener, dashboard_access, DashboardOptions},
    tailscale::parse_tailscale_ipv4,
};

#[test]
fn dashboard_command_does_not_open_browser_unless_requested() {
    let cli = Cli::try_parse_from(["the-witness", "dashboard"]).unwrap();
    match cli.command.unwrap() {
        Commands::Dashboard { open, no_open, .. } => {
            assert!(!open, "dashboard should not auto-open a browser by default");
            assert!(
                !no_open,
                "legacy --no-open flag should be optional because no-open is now default"
            );
        }
        other => panic!("expected dashboard command, got {other:?}"),
    }

    let cli = Cli::try_parse_from(["the-witness", "dashboard", "--open"]).unwrap();
    match cli.command.unwrap() {
        Commands::Dashboard { open, .. } => {
            assert!(open, "--open should opt in to launching the browser")
        }
        other => panic!("expected dashboard command, got {other:?}"),
    }
}

#[test]
fn dashboard_options_default_does_not_open_browser() {
    assert!(DashboardOptions::default().no_open);
}

#[test]
fn parses_tailscale_ipv4_from_cli_output() {
    assert_eq!(
        parse_tailscale_ipv4("100.101.102.103\nfd7a:115c:a1e0::abcd\n"),
        Some("100.101.102.103".parse().unwrap())
    );
    assert_eq!(parse_tailscale_ipv4("fd7a:115c:a1e0::abcd\n"), None);
}

#[test]
fn dashboard_access_enables_tailscale_url_when_bound_to_all_interfaces() {
    let addr: SocketAddr = "0.0.0.0:8790".parse().unwrap();
    let access = dashboard_access(addr, Some("100.101.102.103".parse().unwrap()));

    assert_eq!(access.local_url, "http://127.0.0.1:8790");
    assert_eq!(access.bind_url, "http://0.0.0.0:8790");
    assert!(access.tailscale.available);
    assert_eq!(
        access.tailscale.url.as_deref(),
        Some("http://100.101.102.103:8790")
    );
    assert!(access.tailscale.hint.contains("Tailscale"));
}

#[test]
fn dashboard_access_detects_tailscale_but_requires_non_loopback_bind() {
    let addr: SocketAddr = "127.0.0.1:8790".parse().unwrap();
    let access = dashboard_access(addr, Some("100.101.102.103".parse().unwrap()));

    assert_eq!(access.local_url, "http://127.0.0.1:8790");
    assert!(!access.tailscale.available);
    assert_eq!(access.tailscale.ip.as_deref(), Some("100.101.102.103"));
    assert!(access.tailscale.hint.contains("--host 0.0.0.0"));
}

#[tokio::test]
async fn dashboard_bind_failure_explains_port_conflict_before_startup_banner() {
    let occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = occupied.local_addr().unwrap();

    let err = bind_dashboard_listener(addr).await.unwrap_err().to_string();

    assert!(err.contains("dashboard/control API cannot bind"));
    assert!(err.contains(&addr.to_string()));
    assert!(err.contains("choose a different --port"));
}
