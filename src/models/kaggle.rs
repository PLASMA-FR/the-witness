use anyhow::{bail, Context, Result};
use std::{path::Path, process::Command};

pub fn kaggle_cli_available() -> bool {
    let home = std::env::var("HOME").unwrap_or_default();
    Command::new("sh")
        .arg("-lc")
        .arg(format!(
            "PATH=\"{home}/.local/bin:$PATH\"; command -v kaggle >/dev/null 2>&1"
        ))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
pub fn kaggle_credentials_available() -> bool {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    Path::new(&home).join(".kaggle/kaggle.json").exists()
        || (std::env::var("KAGGLE_USERNAME").is_ok() && std::env::var("KAGGLE_KEY").is_ok())
}
pub fn verify_model_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("model directory does not exist: {}", path.display());
    }
    let names = [
        "adapter_config.json",
        "config.json",
        "tokenizer.json",
        "tokenizer.model",
        "model.safetensors",
        "README.md",
    ];
    if names.iter().any(|n| path.join(n).exists()) {
        Ok(())
    } else {
        bail!(
            "{} exists but has no recognizable model/tokenizer/adapter files",
            path.display()
        )
    }
}
pub fn download_model(slug: &str, dest: &Path) -> Result<String> {
    if !kaggle_cli_available() {
        bail!("kaggle CLI missing. Install with `python -m pip install kaggle` and configure credentials.");
    }
    if !kaggle_credentials_available() {
        bail!("Kaggle credentials missing. Put kaggle.json at ~/.kaggle/kaggle.json with chmod 600, or set KAGGLE_USERNAME/KAGGLE_KEY.");
    }
    std::fs::create_dir_all(dest)?;
    let home = std::env::var("HOME").unwrap_or_default();
    let cmd = format!(
        "PATH=\"{home}/.local/bin:$PATH\"; kaggle models instances versions download {} --untar -p {} || kaggle datasets download -d {} --unzip -p {}",
        shell_escape(slug),
        shell_escape(&dest.display().to_string()),
        shell_escape(slug),
        shell_escape(&dest.display().to_string())
    );
    let status = Command::new("sh")
        .arg("-lc")
        .arg(cmd)
        .status()
        .context("run kaggle model download")?;
    if !status.success() {
        bail!("Kaggle download failed for slug {slug}");
    }
    verify_model_dir(dest)?;
    Ok(format!("Downloaded {slug} to {}", dest.display()))
}
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
