use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelRegistry {
    #[serde(default)]
    pub models: Vec<ModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelEntry {
    pub id: String,
    pub display_name: String,
    pub backend: String,
    #[serde(default)]
    pub base_model: String,
    pub model: String,
    pub source: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub local_path: String,
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub last_tested: String,
    #[serde(default)]
    pub status: String,
}

impl ModelRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        let s = fs::read_to_string(path)
            .with_context(|| format!("read model registry {}", path.display()))?;
        Ok(toml::from_str(&s)?)
    }
    pub fn load_or_default(root: &Path) -> Result<Self> {
        let path = root.join("models/models.toml");
        if path.exists() {
            Self::load(&path)
        } else {
            Ok(Self::default_models())
        }
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    pub fn default_models() -> Self {
        Self {
            models: vec![
                ModelEntry {
                    id: "gemma4-e2b-ollama".into(),
                    display_name: "Gemma 4 E2B via Ollama".into(),
                    backend: "ollama".into(),
                    base_model: "gemma4:e2b".into(),
                    model: "gemma4:e2b".into(),
                    source: "ollama".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "gemma4-e4b-ollama".into(),
                    display_name: "Gemma 4 E4B via Ollama".into(),
                    backend: "ollama".into(),
                    base_model: "gemma4:e4b".into(),
                    model: "gemma4:e4b".into(),
                    source: "ollama".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "witness-gemma4-e2b-judge".into(),
                    display_name: "Custom Fine-tuned Witness Gemma 4 E2B Judge LoRA Adapter".into(),
                    backend: "unsloth".into(),
                    base_model: "google/gemma-4-e2b".into(),
                    model: "witness-gemma4-e2b-judge".into(),
                    source: "huggingface".into(),
                    slug: "ahmadalfakeh/witness-gemma4-e2b-judge".into(),
                    local_path: "./models/witness-gemma4-e2b-judge".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "custom-ollama-model".into(),
                    display_name: "Custom Ollama model name".into(),
                    backend: "ollama".into(),
                    model: "custom".into(),
                    source: "ollama".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "llamacpp-custom".into(),
                    display_name: "llama.cpp backend".into(),
                    backend: "llama.cpp".into(),
                    model: "./models/gemma4.gguf".into(),
                    source: "manual".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "litert-custom".into(),
                    display_name: "LiteRT backend".into(),
                    backend: "litert".into(),
                    model: "./models/witness-gemma4-e2b-litert".into(),
                    source: "manual".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "unsloth-local-path".into(),
                    display_name: "Unsloth local fine-tuned model path".into(),
                    backend: "unsloth".into(),
                    base_model: "gemma4:e2b".into(),
                    model: "./models/witness-gemma4-e2b-judge".into(),
                    source: "manual".into(),
                    local_path: "./models/witness-gemma4-e2b-judge".into(),
                    ..Default::default()
                },
                ModelEntry {
                    id: "manual-openai-compatible".into(),
                    display_name: "Manual OpenAI-compatible judge endpoint".into(),
                    backend: "manual".into(),
                    model: "local-gemma-judge".into(),
                    source: "manual".into(),
                    ..Default::default()
                },
            ],
        }
    }
    pub fn find(&self, id: &str) -> Option<&ModelEntry> {
        self.models.iter().find(|m| m.id == id || m.model == id)
    }
    pub fn mark_installed(&mut self, id: &str, installed: bool, status: &str) {
        if let Some(m) = self.models.iter_mut().find(|m| m.id == id || m.model == id) {
            m.installed = installed;
            m.status = status.into();
        }
    }
}

pub fn registry_path(root: &Path) -> PathBuf {
    root.join("models/models.toml")
}
