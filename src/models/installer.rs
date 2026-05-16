use crate::{
    models::{huggingface, kaggle, registry::ModelEntry},
    setup::{backends::BackendKind, installer::install_backend},
};
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub async fn install_model(entry: &ModelEntry, root: &Path) -> Result<String> {
    let kind = BackendKind::from_config(&entry.backend)
        .ok_or_else(|| anyhow::anyhow!("unknown backend {}", entry.backend))?;
    match entry.source.as_str() {
        "ollama" => install_backend(kind, &entry.model, None, true).await,
        "kaggle" => {
            let local = if entry.local_path.is_empty() {
                root.join("models").join(&entry.id)
            } else {
                resolve(root, &entry.local_path)
            };
            kaggle::download_model(&entry.slug, &local)
        }
        "huggingface" | "hf" => {
            let local = if entry.local_path.is_empty() {
                root.join("models").join(&entry.id)
            } else {
                resolve(root, &entry.local_path)
            };
            huggingface::download_model(&entry.slug, &local)
        }
        "manual" => Ok("Manual model: configure URL/path in Settings, then run model test.".into()),
        _ if matches!(
            kind,
            BackendKind::LiteRt | BackendKind::LlamaCpp | BackendKind::Unsloth
        ) =>
        {
            install_backend(kind, &entry.model, None, false).await
        }
        _ => bail!("unsupported model source {}", entry.source),
    }
}
fn resolve(root: &Path, p: &str) -> PathBuf {
    let path = Path::new(p);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}
