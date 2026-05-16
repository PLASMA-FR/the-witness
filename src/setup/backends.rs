use crate::config::GemmaConfig;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fmt, path::Path, process::Command as StdCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendKind {
    Ollama,
    LlamaCpp,
    LiteRt,
    Unsloth,
    Manual,
    Demo,
}

impl BackendKind {
    pub fn id(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::LlamaCpp => "llama.cpp",
            Self::LiteRt => "litert",
            Self::Unsloth => "unsloth",
            Self::Manual => "manual",
            Self::Demo => "demo",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Ollama => "Ollama",
            Self::LlamaCpp => "llama.cpp",
            Self::LiteRt => "LiteRT",
            Self::Unsloth => "Unsloth fine-tuned Gemma",
            Self::Manual => "Manual OpenAI-compatible",
            Self::Demo => "Demo judge",
        }
    }

    pub fn from_config(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "ollama" => Some(Self::Ollama),
            "llama.cpp" | "llamacpp" | "llama-cpp" => Some(Self::LlamaCpp),
            "litert" | "lite-rt" => Some(Self::LiteRt),
            "unsloth" => Some(Self::Unsloth),
            "manual" | "openai" | "openai-compatible" => Some(Self::Manual),
            "demo" => Some(Self::Demo),
            _ => None,
        }
    }

    pub fn install_plan(self, model: &str, url: Option<&str>) -> InstallPlan {
        match self {
            Self::Ollama => InstallPlan::Command {
                summary: format!("Install Ollama if missing, then run: ollama pull {model}"),
                command: format!("ollama pull {model}"),
            },
            Self::LlamaCpp => InstallPlan::Command {
                summary: format!(
                    "Install llama.cpp and run an OpenAI-compatible server, e.g. llama-server -m {model} --host 127.0.0.1 --port 8080"
                ),
                command: format!("llama-server -m {model} --host 127.0.0.1 --port 8080"),
            },
            Self::LiteRt => InstallPlan::Manual {
                summary: format!(
                    "Install LiteRT runtime, provide a LiteRT model path ({model}), and run the small JSON classification smoke test."
                ),
            },
            Self::Unsloth => InstallPlan::Manual {
                summary: format!(
                    "Install Unsloth in Python, load/fine-tune {model}, serve it behind an OpenAI-compatible endpoint at {}, then run model test.",
                    url.unwrap_or("http://localhost:8000/v1")
                ),
            },
            Self::Manual => InstallPlan::Manual {
                summary: format!(
                    "Point The Witness at an existing OpenAI-compatible judge endpoint {} using model {model}.",
                    url.unwrap_or("http://localhost:8000/v1")
                ),
            },
            Self::Demo => InstallPlan::BuiltIn {
                summary: "No install needed. Uses deterministic local demo judge for proxy/retry testing.".into(),
            },
        }
    }
}

impl fmt::Display for BackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

#[derive(Debug, Clone)]
pub struct BackendChoice {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub install_hint: &'static str,
    pub test_hint: &'static str,
}

pub fn backend_choices() -> Vec<BackendChoice> {
    vec![
        BackendChoice { id: "ollama", name: "Ollama", description: "Recommended local Gemma backend; easiest default path.", install_hint: "Install Ollama and run `ollama pull <model>`.", test_hint: "Calls /api/chat and requires valid JSON verdict content." },
        BackendChoice { id: "llama.cpp", name: "llama.cpp", description: "Local GGUF inference for CPU/GPU constrained systems.", install_hint: "Install llama.cpp and start `llama-server` with an OpenAI-compatible /v1 API.", test_hint: "Checks /v1/models then /v1/chat/completions." },
        BackendChoice { id: "litert", name: "LiteRT", description: "Lightweight edge verifier path for LiteRT model artifacts.", install_hint: "Install LiteRT Python/runtime and provide a model path.", test_hint: "Validates model path and optional sidecar endpoint/classifier." },
        BackendChoice { id: "unsloth", name: "Unsloth fine-tuned Gemma", description: "Fine-tuned Gemma judge served locally.", install_hint: "Install Unsloth in a Python environment; serve the model through an OpenAI-compatible endpoint.", test_hint: "Requires required JSON schema verdicts." },
        BackendChoice { id: "manual", name: "Manual OpenAI-compatible", description: "Any local OpenAI-compatible judge endpoint.", install_hint: "Configure URL, model, and optional auth header.", test_hint: "Checks /v1/chat/completions JSON verdict output." },
        BackendChoice { id: "demo", name: "Demo judge", description: "Deterministic local judge for hackathon demos and tests.", install_hint: "Built in; no external install.", test_hint: "Approves/disapproves known fixtures without network." },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallPlan {
    Command { summary: String, command: String },
    Manual { summary: String },
    BuiltIn { summary: String },
}

impl InstallPlan {
    pub fn summary(&self) -> &str {
        match self {
            Self::Command { summary, .. }
            | Self::Manual { summary }
            | Self::BuiltIn { summary } => summary,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackendHealth {
    pub backend: BackendKind,
    pub installed: bool,
    pub reachable: bool,
    pub details: Vec<String>,
}

pub fn detect_backend_health(cfg: &GemmaConfig) -> BackendHealth {
    let backend = BackendKind::from_config(&cfg.backend).unwrap_or(BackendKind::Manual);
    let mut details = Vec::new();
    let installed = match backend {
        BackendKind::Ollama => command_exists("ollama"),
        BackendKind::LlamaCpp => command_exists("llama-server") || command_exists("llama-cli"),
        BackendKind::LiteRt => {
            python_import_exists("ai_edge_litert") || python_import_exists("litert")
        }
        BackendKind::Unsloth => python_import_exists("unsloth"),
        BackendKind::Manual => true,
        BackendKind::Demo => true,
    };
    details.push(format!("installed/runtime detected: {installed}"));
    if matches!(backend, BackendKind::LiteRt | BackendKind::Unsloth) && !cfg.model.is_empty() {
        details.push(format!(
            "model path exists: {}",
            Path::new(&cfg.model).exists()
        ));
    }
    let reachable =
        matches!(backend, BackendKind::Manual | BackendKind::Demo) || !cfg.url.trim().is_empty();
    details.push(format!("configured URL: {}", cfg.url));
    BackendHealth {
        backend,
        installed,
        reachable,
        details,
    }
}

pub fn command_exists(name: &str) -> bool {
    let home = std::env::var("HOME").unwrap_or_default();
    let cmd = format!("PATH=\"{home}/.local/bin:$PATH\"; command -v {name} >/dev/null 2>&1");
    StdCommand::new("sh")
        .arg("-lc")
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn python_import_exists(module: &str) -> bool {
    let project_venv = "/home/admin/Gemma/witness/.venv-backends/bin/python";
    let mut candidates = vec!["python3".to_string()];
    if Path::new(project_venv).exists() {
        candidates.push(project_venv.to_string());
    }
    candidates.into_iter().any(|python| {
        StdCommand::new(python)
            .arg("-c")
            .arg(format!("import {module}"))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}

pub fn validate_backend_config(cfg: &GemmaConfig) -> Result<BackendKind> {
    let backend = BackendKind::from_config(&cfg.backend)
        .ok_or_else(|| anyhow::anyhow!("unknown Gemma backend: {}", cfg.backend))?;
    if !matches!(backend, BackendKind::Demo) && cfg.model.trim().is_empty() {
        bail!("model name/path is required for backend {backend}");
    }
    if matches!(
        backend,
        BackendKind::Manual | BackendKind::Unsloth | BackendKind::LlamaCpp
    ) && cfg.url.trim().is_empty()
    {
        bail!("URL is required for backend {backend}");
    }
    Ok(backend)
}
