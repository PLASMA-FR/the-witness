use crate::setup::backends::BackendKind;
use anyhow::{bail, Result};
use tokio::process::Command;

pub async fn install_backend(
    backend: BackendKind,
    model: &str,
    url: Option<&str>,
    execute: bool,
) -> Result<String> {
    let plan = backend.install_plan(model, url);
    if !execute {
        return Ok(plan.summary().to_string());
    }
    match backend {
        BackendKind::Ollama => ollama_pull(model).await,
        BackendKind::LlamaCpp => Ok(format!(
            "llama.cpp is managed as an external server. Install command guidance: {}",
            plan.summary()
        )),
        BackendKind::LiteRt => Ok(format!(
            "LiteRT requires a Python/runtime environment and a model artifact. Guidance: {}",
            plan.summary()
        )),
        BackendKind::Unsloth => Ok(format!(
            "Unsloth requires a Python environment, model weights, and an OpenAI-compatible serving layer. Guidance: {}",
            plan.summary()
        )),
        BackendKind::Manual => Ok(plan.summary().to_string()),
        BackendKind::Demo => Ok(plan.summary().to_string()),
    }
}

pub async fn ollama_pull(model: &str) -> Result<String> {
    let exists = Command::new("sh")
        .arg("-lc")
        .arg("command -v ollama >/dev/null")
        .status()
        .await?
        .success();
    if !exists {
        bail!("Ollama is not installed. Install from https://ollama.com/download then retry.")
    }
    let out = Command::new("ollama")
        .arg("pull")
        .arg(model)
        .output()
        .await?;
    if !out.status.success() {
        bail!(
            "ollama pull failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn install_plan_text(backend: &str, model: &str, url: Option<&str>) -> Result<String> {
    let kind = BackendKind::from_config(backend)
        .ok_or_else(|| anyhow::anyhow!("unknown backend: {backend}"))?;
    Ok(kind.install_plan(model, url).summary().to_string())
}
