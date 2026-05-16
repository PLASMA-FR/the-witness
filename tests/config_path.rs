use clap::Parser;
use std::path::PathBuf;
use the_witness::{cli::Cli, config::WitnessConfig};

#[test]
fn default_config_path_uses_user_config_dir_not_build_machine_path() {
    let cli = Cli::parse_from(["the-witness", "doctor"]);
    let path = cli.resolved_config_path();
    assert!(
        path.ends_with("the-witness/witness.toml"),
        "unexpected config path: {}",
        path.display()
    );
    assert!(
        !path.starts_with("/home/admin/Gemma/witness"),
        "installed CLI must not write to source checkout path: {}",
        path.display()
    );
}

#[test]
fn witness_config_dir_env_overrides_default_config_path() {
    let base = PathBuf::from("/tmp/witness-config-test");
    std::env::set_var("WITNESS_CONFIG_DIR", &base);
    let path = WitnessConfig::default_path();
    std::env::remove_var("WITNESS_CONFIG_DIR");
    assert_eq!(path, base.join("witness.toml"));
}

#[test]
fn explicit_config_path_still_wins() {
    let cli = Cli::parse_from([
        "the-witness",
        "--config",
        "/tmp/custom-witness.toml",
        "doctor",
    ]);
    assert_eq!(
        cli.resolved_config_path(),
        PathBuf::from("/tmp/custom-witness.toml")
    );
}
