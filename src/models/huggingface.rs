use anyhow::{bail, Context, Result};
use std::{path::Path, process::Command};

pub fn hf_cli_available() -> bool {
    Command::new("sh")
        .arg("-lc")
        .arg("command -v hf >/dev/null 2>&1 || command -v huggingface-cli >/dev/null 2>&1")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn download_model(repo_id: &str, dest: &Path) -> Result<String> {
    if !hf_cli_available() {
        bail!("Hugging Face CLI missing. Install with `python -m pip install -U huggingface_hub`, then run `hf download {repo_id} --local-dir {}`.", dest.display());
    }
    std::fs::create_dir_all(dest)?;
    let command = if Command::new("sh")
        .arg("-lc")
        .arg("command -v hf >/dev/null 2>&1")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        format!(
            "hf download {} --local-dir {} --local-dir-use-symlinks False",
            shell_quote(repo_id),
            shell_quote(&dest.display().to_string())
        )
    } else {
        format!(
            "huggingface-cli download {} --local-dir {} --local-dir-use-symlinks False",
            shell_quote(repo_id),
            shell_quote(&dest.display().to_string())
        )
    };
    let status = Command::new("sh")
        .arg("-lc")
        .arg(&command)
        .status()
        .context("run Hugging Face model download")?;
    if !status.success() {
        bail!("Hugging Face download failed for repo {repo_id}");
    }
    Ok(format!(
        "Downloaded Hugging Face LoRA adapter repo {repo_id} into {}. Load it with base model google/gemma-4-e2b (or the configured Gemma 4 E2B base) plus this adapter.",
        dest.display()
    ))
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
