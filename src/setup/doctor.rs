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
fn warn(label: &str, why: &str, fix: &str) -> String {
    format!("[WARN] {label}\nWhy it matters: {why}\nFix: {fix}")
}
fn fail(label: &str, why: &str, fix: &str) -> String {
    format!("[FAIL] {label}\nWhy it matters: {why}\nFix: {fix}")
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
                "Ollama is not installed or not available in PATH",
                "The default Gemma judge path uses Ollama for local verification.",
                "Install Ollama, then run `ollama pull gemma4:e2b`.",
            )
        },
        if ollama_running {
            pass("Ollama running")
        } else {
            fail(
                "Ollama is installed but not running",
                "The judge cannot answer local verdict requests until the Ollama service is reachable.",
                "Start Ollama, then run `the-witness model test --backend ollama --model gemma4:e2b`.",
            )
        },
        if e2b_installed {
            pass("gemma4:e2b installed")
        } else {
            fail("gemma4:e2b is missing", "This is the default local judge model.", "Run `ollama pull gemma4:e2b`.")
        },
        if e4b_installed {
            pass("gemma4:e4b installed")
        } else {
            warn("gemma4:e4b is missing", "This stronger model is useful for high-risk and stricter profiles.", "Run `ollama pull gemma4:e4b` when hardware allows.")
        },
        if health.installed {
            pass("Runtime detected")
        } else {
            fail(
                "Selected runtime/tool is missing",
                "The configured judge backend must be available before live endpoint protection.",
                "Install the selected backend tool, or choose demo mode for a local walkthrough.",
            )
        },
        if health.reachable {
            pass("Selected backend URL/path configured")
        } else {
            fail(
                "Selected backend URL/path is not reachable",
                "The Witness cannot ask the configured judge for verdicts until the endpoint or model path works.",
                "Set the judge URL or local model path in Settings or witness.toml, then rerun doctor.",
            )
        },
        if cfg.setup.judge_schema_test_passed {
            pass("Judge JSON schema test flag passed")
        } else {
            fail(
                "Judge JSON schema test has not passed yet",
                "The proxy only trusts judge responses that match the verdict schema.",
                "Run `the-witness model test` after configuring a working judge.",
            )
        },
        if cfg.setup.proxy_test_passed {
            pass("Proxy test flag passed")
        } else {
            fail(
                "Proxy test has not passed yet",
                "Endpoint traffic should not be trusted until the local proxy has been checked.",
                "Run setup again or use demo mode to complete the proxy health check.",
            )
        },
        if cfg.setup.model_test_passed {
            pass("Model test flag passed")
        } else {
            fail(
                "Model test has not passed yet",
                "The configured judge should reject a clearly wrong answer and approve a correct one before live use.",
                "Run `the-witness model test --backend ollama --model gemma4:e2b` or test your selected backend.",
            )
        },
        if std::env::var("BLACKBOX_API_KEY").is_ok() {
            pass("BLACKBOX_API_KEY set")
        } else if blackbox.is_some() {
            fail("BLACKBOX_API_KEY is not set for the enabled Blackbox endpoint", "The Blackbox example reads this environment variable instead of storing your key.", "Run `export BLACKBOX_API_KEY=\"YOUR_KEY_HERE\"` in your shell; never store it in repo files.")
        } else {
            warn(
                "BLACKBOX_API_KEY is not set",
                "The Blackbox example endpoint cannot be tested without it.",
                "Set it only when you want to use Blackbox: `export BLACKBOX_API_KEY=\"YOUR_KEY_HERE\"`.",
            )
        },
        if port_8787_free() {
            pass("Local proxy port 8787 is free")
        } else {
            warn("Local proxy port 8787 is already in use", "The Witness cannot start the default local proxy if another process owns the port.", "Stop the other process, confirm it is already The Witness, or choose another local_proxy_url.")
        },
        if huggingface::hf_cli_available() {
            pass("Hugging Face CLI installed")
        } else {
            warn(
                "Hugging Face CLI is not installed",
                "It is only needed when downloading the fine-tuned Witness judge from the Hub through the CLI.",
                "Install with `python -m pip install -U huggingface_hub` if you want `the-witness model download` to fetch the LoRA adapter.",
            )
        },
        if kaggle::kaggle_cli_available() {
            warn("Kaggle CLI installed but not used for the current custom model", "The published Witness E2B LoRA adapter is distributed through Hugging Face.", "Use Kaggle for submission workflow only; use Hugging Face for the model adapter.")
        } else {
            warn(
                "Kaggle CLI is not configured",
                "This does not block local verification or the Hugging Face model path.",
                "No action needed unless you are using Kaggle submission tooling.",
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
                "Hugging Face LoRA adapter repo is not configured",
                "The model manager expects the Witness judge adapter to point at the public Hub repo.",
                "Set source=huggingface and slug=ahmadalfakeh/witness-gemma4-e2b-judge in models/models.toml.",
            )
        },
        if root.join("models/witness-gemma4-e2b-judge").exists() {
            pass("Fine-tuned Witness judge downloaded")
        } else if cfg.gemma.backend == "unsloth" || cfg.gemma.model == "witness-gemma4-e2b-judge" {
            fail("Fine-tuned Witness judge is selected but not downloaded", "The Unsloth/Hugging Face path needs the adapter files on disk before testing.", "Run `the-witness model download --source huggingface --model witness-gemma4-e2b-judge`.")
        } else {
            warn(
                "Fine-tuned Witness judge is not downloaded",
                "This is optional unless you select the Unsloth/Hugging Face judge path.",
                "Download it when needed with `the-witness model download --source huggingface --model witness-gemma4-e2b-judge`.",
            )
        },
        if root
            .join("training/notebooks/finetune_gemma4_e2b_unsloth.ipynb")
            .exists()
        {
            pass("Fine-tuning notebook found")
        } else {
            warn(
                "Fine-tuning notebook is not present",
                "This only matters if you plan to retrain the Witness judge from source.",
                "Use the public Colab notebook link in the README or restore training/notebooks/finetune_gemma4_e2b_unsloth.ipynb.",
            )
        },
        if root.join("models/models.toml").exists() {
            pass("Model registry found")
        } else {
            warn(
                "Model registry file is missing",
                "The built-in registry still covers the default demo and Gemma paths.",
                "Create models/models.toml only when you need custom model entries.",
            )
        },
        if root.join("logs").exists() || std::fs::create_dir_all(root.join("logs")).is_ok() {
            pass("Logs writable")
        } else {
            fail(
                "Logs are not writable",
                "The Witness needs an audit trail for verdicts, retries, repairs, and manual decisions.",
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
                "Blackbox upstream is not reachable",
                "The endpoint test could not confirm upstream auth and connectivity.",
                &format!("Check BLACKBOX_API_KEY, network access, and upstream URL. Details: {e}"),
            )),
        }
    }
    for detail in health.details {
        lines.push(format!("Backend detail: {detail}"));
    }
    let passed = cfg.setup_ready() || cfg.setup.demo_mode;
    if !passed {
        lines.push(fail(
            "Readiness gate has not passed",
            "The main dashboard should only open for live traffic after setup, model, judge, and proxy checks are complete or demo mode is selected.",
            "Run `the-witness setup`, pass the checks, or choose demo mode.",
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
