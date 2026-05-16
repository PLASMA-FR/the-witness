use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessConfig {
    pub gemma: GemmaConfig,
    pub setup: SetupConfig,
    pub defaults: DefaultConfig,
    #[serde(default = "default_profiles")]
    pub profiles: BTreeMap<String, ProfileDefaults>,
    #[serde(default)]
    pub endpoints: Vec<EndpointConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemmaConfig {
    pub backend: String,
    pub model: String,
    pub url: String,
    pub setup_completed: bool,
    #[serde(default)]
    pub auth_header: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupConfig {
    pub last_doctor_check: String,
    pub judge_schema_test_passed: bool,
    pub proxy_test_passed: bool,
    pub model_test_passed: bool,
    #[serde(default)]
    pub demo_mode: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub retry_limit: u32,
    pub strictness: Strictness,
    pub fallback_mode: FallbackMode,
    pub log_format: String,
    pub privacy_mode: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Strictness {
    Relaxed,
    Medium,
    High,
    Critical,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackMode {
    HumanReview,
    DemoJudge,
    SafeResponse,
    Error,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileDefaults {
    pub preferred_backend: String,
    pub preferred_model: String,
    pub fallback_model: String,
    pub strictness: Strictness,
    pub fallback_mode: FallbackMode,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct EndpointAuth {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub env: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}
impl fmt::Debug for EndpointAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EndpointAuth")
            .field("kind", &self.kind)
            .field("env", &self.env)
            .field("value", &self.value.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}
impl EndpointAuth {
    pub fn resolve_header(&self) -> Result<Option<String>> {
        match self.kind.as_str() {
            "bearer_env" => {
                let env = self.env.as_deref().context("auth env name missing")?;
                let value = std::env::var(env)
                    .with_context(|| format!("required auth env {env} is not set"))?;
                if value.to_ascii_lowercase().starts_with("bearer ") {
                    Ok(Some(value))
                } else {
                    Ok(Some(format!("Bearer {value}")))
                }
            }
            "header_env" => {
                let env = self.env.as_deref().context("auth env name missing")?;
                Ok(Some(std::env::var(env).with_context(|| {
                    format!("required auth env {env} is not set")
                })?))
            }
            "static" => Ok(self.value.clone()),
            "none" => Ok(None),
            other => anyhow::bail!("unsupported auth type {other}"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub name: String,
    pub enabled: bool,
    pub upstream_url: String,
    pub local_proxy_url: String,
    pub model: String,
    pub profile: String,
    pub retry_limit: u32,
    pub strictness: Strictness,
    pub fallback_mode: FallbackMode,
    #[serde(default)]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub auth: Option<EndpointAuth>,
    #[serde(default)]
    pub judge_backend: Option<String>,
    #[serde(default)]
    pub judge_model: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}
fn default_timeout() -> u64 {
    60
}

impl EndpointConfig {
    pub fn resolved_auth_header(&self) -> Result<Option<String>> {
        if let Some(auth) = &self.auth {
            auth.resolve_header()
        } else {
            Ok(self.auth_header.clone())
        }
    }
}

pub fn default_profiles() -> BTreeMap<String, ProfileDefaults> {
    let mut profiles = BTreeMap::new();
    profiles.insert(
        "coding".into(),
        ProfileDefaults {
            preferred_backend: "ollama".into(),
            preferred_model: "gemma4:e4b".into(),
            fallback_model: "gemma4:e2b".into(),
            strictness: Strictness::High,
            fallback_mode: FallbackMode::HumanReview,
        },
    );
    profiles.insert(
        "education".into(),
        ProfileDefaults {
            preferred_backend: "ollama".into(),
            preferred_model: "gemma4:e2b".into(),
            fallback_model: "gemma4:e2b".into(),
            strictness: Strictness::Medium,
            fallback_mode: FallbackMode::SafeResponse,
        },
    );
    profiles.insert(
        "high_risk".into(),
        ProfileDefaults {
            preferred_backend: "ollama".into(),
            preferred_model: "gemma4:e4b".into(),
            fallback_model: "gemma4:e2b".into(),
            strictness: Strictness::Critical,
            fallback_mode: FallbackMode::HumanReview,
        },
    );
    profiles
}

impl Default for WitnessConfig {
    fn default() -> Self {
        Self {
            gemma: GemmaConfig {
                backend: "ollama".into(),
                model: "gemma4:e2b".into(),
                url: "http://localhost:11434".into(),
                setup_completed: false,
                auth_header: None,
            },
            setup: SetupConfig::default(),
            defaults: DefaultConfig {
                retry_limit: 3,
                strictness: Strictness::Medium,
                fallback_mode: FallbackMode::HumanReview,
                log_format: "jsonl".into(),
                privacy_mode: false,
            },
            profiles: default_profiles(),
            endpoints: vec![],
        }
    }
}
impl WitnessConfig {
    pub fn path_in(root: &Path) -> PathBuf {
        root.join("witness.toml")
    }
    pub fn load(path: &Path) -> Result<Self> {
        let data = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        Ok(toml::from_str(&data)?)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?
        }
        fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    pub fn setup_ready(&self) -> bool {
        (self.gemma.setup_completed || self.setup.demo_mode)
            && self.setup.judge_schema_test_passed
            && self.setup.proxy_test_passed
            && self.setup.model_test_passed
    }
    pub fn add_or_replace_endpoint(&mut self, ep: EndpointConfig) {
        if let Some(existing) = self.endpoints.iter_mut().find(|e| e.name == ep.name) {
            *existing = ep
        } else {
            self.endpoints.push(ep)
        }
    }
    pub fn blackbox_endpoint() -> EndpointConfig {
        EndpointConfig {
            name: "Blackbox Grok Code".into(),
            enabled: true,
            upstream_url: "https://api.blackbox.ai/v1".into(),
            local_proxy_url: "http://localhost:8787/v1".into(),
            model: "blackboxai/x-ai/grok-code-fast-1:free".into(),
            profile: "coding".into(),
            retry_limit: 4,
            strictness: Strictness::High,
            fallback_mode: FallbackMode::HumanReview,
            auth_header: None,
            auth: Some(EndpointAuth {
                kind: "bearer_env".into(),
                env: Some("BLACKBOX_API_KEY".into()),
                value: None,
            }),
            judge_backend: Some("ollama".into()),
            judge_model: Some("gemma4:e4b".into()),
            timeout_seconds: 60,
        }
    }
}
