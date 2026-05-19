use the_witness::config::{GemmaConfig, WitnessConfig};
use the_witness::setup::backends::{backend_choices, BackendKind, InstallPlan};
use the_witness::tui::settings::settings_lines;

#[test]
fn all_required_backends_are_selectable() {
    let ids: Vec<_> = backend_choices().iter().map(|b| b.id).collect();
    for required in ["ollama", "llama.cpp", "litert", "unsloth", "manual", "demo"] {
        assert!(ids.contains(&required), "missing backend choice {required}");
    }
}

#[test]
fn backend_install_plans_are_actionable_and_honest() {
    let ollama = BackendKind::Ollama.install_plan("gemma4", None);
    assert!(matches!(ollama, InstallPlan::Command { .. }));
    assert!(ollama.summary().contains("ollama pull"));

    let llama =
        BackendKind::LlamaCpp.install_plan("/models/gemma.gguf", Some("http://localhost:8080/v1"));
    assert!(llama.summary().contains("llama-server"));

    let litert = BackendKind::LiteRt.install_plan("/models/judge.tflite", None);
    assert!(litert.summary().contains("LiteRT"));

    let unsloth =
        BackendKind::Unsloth.install_plan("/models/unsloth", Some("http://localhost:8000/v1"));
    assert!(unsloth.summary().contains("OpenAI-compatible"));
}

#[test]
fn backend_kind_round_trips_from_config_strings() {
    assert_eq!(
        BackendKind::from_config("ollama"),
        Some(BackendKind::Ollama)
    );
    assert_eq!(
        BackendKind::from_config("llama.cpp"),
        Some(BackendKind::LlamaCpp)
    );
    assert_eq!(
        BackendKind::from_config("litert"),
        Some(BackendKind::LiteRt)
    );
    assert_eq!(
        BackendKind::from_config("unsloth"),
        Some(BackendKind::Unsloth)
    );
    assert_eq!(
        BackendKind::from_config("manual"),
        Some(BackendKind::Manual)
    );
    assert_eq!(BackendKind::from_config("demo"), Some(BackendKind::Demo));
}

#[test]
fn settings_screen_lists_backend_choices_and_current_config() {
    let cfg = WitnessConfig {
        gemma: GemmaConfig {
            backend: "manual".into(),
            model: "local-gemma-judge".into(),
            url: "http://localhost:8000/v1".into(),
            setup_completed: true,
            auth_header: Some("Bearer secret".into()),
        },
        ..WitnessConfig::default()
    };
    let text = settings_lines(&cfg).join("\n");
    assert!(text.contains("Current backend: manual"));
    assert!(text.contains("Ollama"));
    assert!(text.contains("llama.cpp"));
    assert!(text.contains("LiteRT"));
    assert!(text.contains("Unsloth"));
    assert!(text.contains("Manual OpenAI-compatible"));
    assert!(text.contains("Demo judge"));
    assert!(text.contains("Auth: configured/redacted"));
}
