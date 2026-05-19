use std::path::Path;
use the_witness::config::WitnessConfig;

#[test]
fn documented_example_configs_load_as_full_witness_configs() {
    for path in [
        "examples/blackbox.toml",
        "examples/ollama.toml",
        "examples/llamacpp.toml",
    ] {
        WitnessConfig::load(Path::new(path))
            .unwrap_or_else(|err| panic!("{path} should load as WitnessConfig: {err}"));
    }
}
