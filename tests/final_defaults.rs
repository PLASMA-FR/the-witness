use std::fs;
use std::path::Path;

use the_witness::config::{EndpointAuth, FallbackMode, Strictness, WitnessConfig};
use the_witness::models::registry::ModelRegistry;

#[test]
fn final_default_config_uses_confirmed_ollama_models_and_profile_preferences() {
    let cfg = WitnessConfig::default();
    assert_eq!(cfg.gemma.backend, "ollama");
    assert_eq!(cfg.gemma.model, "gemma4:e2b");
    assert_eq!(cfg.defaults.fallback_mode, FallbackMode::HumanReview);

    let coding = cfg.profiles.get("coding").expect("coding profile");
    assert_eq!(coding.preferred_backend, "ollama");
    assert_eq!(coding.preferred_model, "gemma4:e4b");
    assert_eq!(coding.fallback_model, "gemma4:e2b");
    assert_eq!(coding.strictness, Strictness::High);
    assert_eq!(coding.fallback_mode, FallbackMode::HumanReview);

    let high_risk = cfg.profiles.get("high_risk").expect("high-risk profile");
    assert_eq!(high_risk.preferred_model, "gemma4:e4b");
    assert_eq!(high_risk.fallback_model, "gemma4:e2b");
    assert_eq!(high_risk.strictness, Strictness::Critical);
}

#[test]
fn model_registry_uses_confirmed_tags_and_kaggle_slug() {
    let registry = ModelRegistry::default_models();
    let e2b = registry.find("gemma4-e2b-ollama").unwrap();
    assert_eq!(e2b.model, "gemma4:e2b");
    let e4b = registry.find("gemma4-e4b-ollama").unwrap();
    assert_eq!(e4b.model, "gemma4:e4b");
    let ft = registry.find("witness-gemma4-e2b-judge").unwrap();
    assert_eq!(ft.base_model, "gemma4:e2b");
    assert_eq!(ft.slug, "plasmafr/witness-gemma4-e2b-judge");
}

#[test]
fn blackbox_example_uses_env_auth_not_literal_secret() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/blackbox.toml");
    let text = fs::read_to_string(path).expect("blackbox example exists");
    assert!(text.contains("Blackbox Grok Code"));
    assert!(text.contains("bearer_env"));
    assert!(text.contains("BLACKBOX_API_KEY"));
    assert!(!text.contains("sk-"));
    assert!(!text.contains("CfDJ"));
}

#[test]
fn endpoint_auth_env_resolves_without_storing_secret() {
    unsafe { std::env::set_var("WITNESS_TEST_AUTH", "Bearer value-from-env") };
    let auth = EndpointAuth {
        kind: "bearer_env".into(),
        env: Some("WITNESS_TEST_AUTH".into()),
        value: None,
    };
    assert_eq!(
        auth.resolve_header().unwrap().as_deref(),
        Some("Bearer value-from-env")
    );
    assert!(!format!("{:?}", auth).contains("value-from-env"));
}

#[test]
fn fallback_mode_includes_demo_judge_only_as_explicit_choice() {
    #[derive(serde::Deserialize)]
    struct Wrap {
        fallback_mode: FallbackMode,
    }
    let parsed: Wrap = toml::from_str("fallback_mode = \"demo_judge\"").unwrap();
    assert_eq!(parsed.fallback_mode, FallbackMode::DemoJudge);
    assert_eq!(
        WitnessConfig::default().defaults.fallback_mode,
        FallbackMode::HumanReview
    );
}
