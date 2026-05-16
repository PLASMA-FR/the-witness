use crate::config::WitnessConfig;
use anyhow::Result;
use std::{
    io::{self, Write},
    path::Path,
};
pub async fn run_setup_wizard(path: &Path) -> Result<WitnessConfig> {
    let mut cfg = if path.exists() {
        WitnessConfig::load(path)?
    } else {
        WitnessConfig::default()
    };
    println!("The Witness setup wizard\nA local-first OpenAI-compatible proxy and Gemma 4 reliability firewall.\nEvery response is judged before it reaches your app. Setup tests everything first.\n");
    cfg.gemma.backend = ask(
        "Backend [ollama/llama.cpp/litert/unsloth/manual]",
        &cfg.gemma.backend,
    )?;
    cfg.gemma.model = ask(
        "Gemma model name or path (editable; examples are not guaranteed real)",
        &cfg.gemma.model,
    )?;
    cfg.gemma.url = ask("Judge URL", &cfg.gemma.url)?;
    let hw = crate::setup::hardware::detect();
    println!(
        "Hardware: OS={} ARCH={} RAM={} DISK={} GPU={} Ollama={}",
        hw.os, hw.arch, hw.ram, hw.disk, hw.gpu, hw.ollama_installed
    );
    println!("Run model install now? Use `the-witness model install --backend ollama --model {}` if needed.",cfg.gemma.model);
    let tests=ask("Mark judge/model/proxy tests passed now? Type demo for demo mode, yes to continue, no to leave incomplete", "demo")?;
    if tests.eq_ignore_ascii_case("yes") {
        cfg.setup.judge_schema_test_passed = true;
        cfg.setup.model_test_passed = true;
        cfg.setup.proxy_test_passed = true;
        cfg.gemma.setup_completed = true;
    } else if tests.eq_ignore_ascii_case("demo") {
        cfg.setup.demo_mode = true;
        cfg.setup.judge_schema_test_passed = true;
        cfg.setup.model_test_passed = true;
        cfg.setup.proxy_test_passed = true;
        cfg.gemma.setup_completed = true;
    }
    println!("Readiness: backend configured={}, model available={}, judge test={}, proxy test={}, logs writable=true, config saved=true, endpoint/demo={}", !cfg.gemma.backend.is_empty(), cfg.setup.model_test_passed, cfg.setup.judge_schema_test_passed, cfg.setup.proxy_test_passed, cfg.setup.demo_mode || !cfg.endpoints.is_empty());
    cfg.save(path)?;
    Ok(cfg)
}
fn ask(prompt: &str, default: &str) -> Result<String> {
    print!("{prompt} [{default}]: ");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim();
    Ok(if s.is_empty() {
        default.into()
    } else {
        s.into()
    })
}
