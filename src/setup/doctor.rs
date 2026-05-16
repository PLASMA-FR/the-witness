use crate::{
    config::WitnessConfig,
    models::{huggingface, kaggle, registry::ModelRegistry},
    setup::{backends, hardware},
};
use anyhow::{Context, Result};
use chrono::Utc;
use std::{net::TcpListener, path::Path, process::Command, time::Duration};

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub lines: Vec<String>,
    pub passed: bool,
}
fn pass(label: &str) -> String {
    format!("[PASS] {label}")
}
fn warn(label: &str, fix: &str) -> String {
    format!("[WARN] {label}\nFix: {fix}")
}
fn fail(label: &str, fix: &str) -> String {
    format!("[FAIL] {label}\nFix: {fix}")
}

pub async fn run_doctor(cfg: &WitnessConfig, root: &Path) -> Result<DoctorReport> {
    let hw = hardware::detect();
    let backend = backends::validate_backend_config(&cfg.gemma)?;
    let health = backends::detect_backend_health(&cfg.gemma);
    let registry =
        ModelRegistry::load_or_default(root).unwrap_or_else(|_| ModelRegistry::default_models());
    let ollama_running = url_ok("http://localhost:11434/api/tags").await;
    let e2b_installed = ollama_model_installed("gemma4:e2b").await;
    let e4b_installed = ollama_model_installed("gemma4:e4b").await;
    let blackbox = cfg
        .endpoints
        .iter()
        .find(|e| e.name == "Blackbox Grok Code" && e.enabled);
    let mut lines = vec![
        "Default backend: Ollama".into(),
        "Default model: gemma4:e2b".into(),
        "Strong model: gemma4:e4b".into(),
        "Fine-tuning runtime: one-cell Google Colab T4 GPU with Unsloth 4-bit LoRA/QLoRA".into(),
        "Custom fine-tuned LoRA adapter: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge".into(),
        "Fallback: human_review".into(),
        pass(&format!("OS detected: {} {}", hw.os, hw.arch)),
        pass(&format!(
            "Hardware snapshot: RAM={} Disk free={} GPU={}",
            hw.ram, hw.disk, hw.gpu
        )),
        pass(&format!(
            "Backend selectable: {} ({})",
            backend.id(),
            backend.display_name()
        )),
        if backends::command_exists("ollama") {
            pass("Ollama installed")
        } else {
            fail(
                "Ollama not installed",
                "Install Ollama, then run `ollama pull gemma4:e2b`.",
            )
        },
        if ollama_running {
            pass("Ollama running")
        } else {
            fail(
                "Ollama not running",
                "Start Ollama before testing Ollama judge models.",
            )
        },
        if e2b_installed {
            pass("gemma4:e2b installed")
        } else {
            fail("gemma4:e2b missing", "Run `ollama pull gemma4:e2b`.")
        },
        if e4b_installed {
            pass("gemma4:e4b installed")
        } else {
            warn("gemma4:e4b missing/optional", "Coding and high-risk profiles prefer it. Run `ollama pull gemma4:e4b` when hardware allows.")
        },
        if health.installed {
            pass("Runtime detected")
        } else {
            fail(
                "Selected runtime/tool missing",
                "Install the selected backend tool, or choose Demo only for demo mode.",
            )
        },
        if health.reachable {
            pass("Selected backend URL/path configured")
        } else {
            fail(
                "Selected backend URL/path missing",
                "Set the judge URL or local model path in Settings/witness.toml.",
            )
        },
        if cfg.setup.judge_schema_test_passed {
            pass("Judge JSON schema test flag passed")
        } else {
            fail(
                "Judge JSON schema test flag not passed",
                "Run `the-witness model test` after configuring a working judge.",
            )
        },
        if cfg.setup.proxy_test_passed {
            pass("Proxy test flag passed")
        } else {
            fail(
                "Proxy test flag not passed",
                "Run setup/proxy demo; proxy test is required before production use.",
            )
        },
        if cfg.setup.model_test_passed {
            pass("Model test flag passed")
        } else {
            fail(
                "Model test flag not passed",
                "Run `the-witness model test`.",
            )
        },
        if std::env::var("BLACKBOX_API_KEY").is_ok() {
            pass("BLACKBOX_API_KEY set")
        } else if blackbox.is_some() {
            fail("BLACKBOX_API_KEY missing for enabled Blackbox endpoint", "Run `export BLACKBOX_API_KEY=\"...\"` in your shell; never store it in repo files.")
        } else {
            warn(
                "BLACKBOX_API_KEY not set",
                "Only required when Blackbox endpoint is enabled.",
            )
        },
        if port_8787_free() {
            pass("Local proxy port 8787 is free")
        } else {
            warn("Local proxy port 8787 is in use", "This is OK only if it is already The Witness; otherwise stop the process or choose another port.")
        },
        if huggingface::hf_cli_available() {
            pass("Hugging Face CLI installed")
        } else {
            warn(
                "Hugging Face CLI missing/optional",
                "Install with `python -m pip install -U huggingface_hub` if you want `the-witness model download` to fetch the LoRA adapter from the Hub.",
            )
        },
        if kaggle::kaggle_cli_available() {
            warn("Kaggle CLI installed but not used for the current custom model", "The published E2B LoRA adapter is on Hugging Face, not Kaggle.")
        } else {
            warn(
                "Kaggle CLI not configured",
                "OK: the current custom E2B LoRA adapter is distributed via Hugging Face, not Kaggle.",
            )
        },
        if registry
            .find("witness-gemma4-e2b-judge")
            .map(|m| (m.source.as_str(), m.slug.as_str()))
            == Some(("huggingface", "ahmadalfakeh/witness-gemma4-e2b-judge"))
        {
            pass("Hugging Face LoRA adapter repo configured")
        } else {
            fail(
                "Hugging Face LoRA adapter repo not configured",
                "Set source=huggingface and slug=ahmadalfakeh/witness-gemma4-e2b-judge in models/models.toml.",
            )
        },
        if root.join("models/witness-gemma4-e2b-judge").exists() {
            pass("Fine-tuned Witness judge downloaded")
        } else if cfg.gemma.backend == "unsloth" || cfg.gemma.model == "witness-gemma4-e2b-judge" {
            fail("Fine-tuned Witness judge not downloaded", "Run `the-witness model download --source huggingface --model witness-gemma4-e2b-judge`.")
        } else {
            warn(
                "Fine-tuned Witness judge not downloaded",
                "Only required when selected.",
            )
        },
        if root
            .join("training/notebooks/finetune_gemma4_e2b_unsloth.ipynb")
            .exists()
        {
            pass("Fine-tuning notebook found")
        } else {
            warn(
                "Fine-tuning notebook missing",
                "Only required when developing or training the Colab T4 GPU fine-tuned judge from source.",
            )
        },
        if root.join("models/models.toml").exists() {
            pass("Model registry found")
        } else {
            warn(
                "Model registry file missing",
                "Using built-in defaults; create models/models.toml only for custom registries.",
            )
        },
        if root.join("logs").exists() || std::fs::create_dir_all(root.join("logs")).is_ok() {
            pass("Logs writable")
        } else {
            fail(
                "Logs not writable",
                &format!("Check permissions for {}.", root.join("logs").display()),
            )
        },
        format!(
            "Gemma backend: {} model: {} url: {}",
            cfg.gemma.backend, cfg.gemma.model, cfg.gemma.url
        ),
        format!("Checked at: {}", Utc::now()),
    ];
    if let Some(ep) = blackbox {
        match test_blackbox_reachability(ep).await {
            Ok(()) => lines.push(pass("Blackbox upstream reachable")),
            Err(e) => lines.push(fail(
                "Blackbox upstream not reachable",
                &format!("Check BLACKBOX_API_KEY/network/upstream URL. Details: {e}"),
            )),
        }
    }
    for detail in health.details {
        lines.push(format!("Backend detail: {detail}"));
    }
    let passed = cfg.setup_ready() || cfg.setup.demo_mode;
    if !passed {
        lines.push(fail(
            "Readiness gate not passed",
            "Complete setup, pass judge/model/proxy tests, or choose demo mode.",
        ));
    }
    Ok(DoctorReport { lines, passed })
}

async fn url_ok(url: &str) -> bool {
    reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}
async fn ollama_model_installed(model: &str) -> bool {
    let Ok(resp) = reqwest::Client::new()
        .get("http://localhost:11434/api/tags")
        .timeout(Duration::from_secs(2))
        .send()
        .await
    else {
        return false;
    };
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return false;
    };
    json.get("models")
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .any(|m| m.get("name").and_then(|n| n.as_str()) == Some(model))
        })
        .unwrap_or(false)
}
fn port_8787_free() -> bool {
    TcpListener::bind("127.0.0.1:8787").is_ok()
}
async fn test_blackbox_reachability(ep: &crate::config::EndpointConfig) -> Result<()> {
    let auth = ep
        .resolved_auth_header()?
        .context("BLACKBOX_API_KEY is required")?;
    let url = format!("{}/models", ep.upstream_url.trim_end_matches('/'));
    reqwest::Client::new()
        .get(url)
        .header("Authorization", auth)
        .timeout(Duration::from_secs(8))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
#[allow(dead_code)]
fn _command_ok(command: &str) -> bool {
    Command::new("sh")
        .arg("-lc")
        .arg(command)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
