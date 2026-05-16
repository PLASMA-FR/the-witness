use crate::{
    config::GemmaConfig,
    judge::gemma::{GemmaJudge, JudgeInput, JudgeOutput, OpenAiCompatibleJudge},
};
use anyhow::Result;
#[derive(Clone)]
pub struct OllamaJudge(pub OpenAiCompatibleJudge);
impl OllamaJudge {
    pub fn new(mut cfg: GemmaConfig) -> Self {
        cfg.backend = "ollama".into();
        if cfg.url.is_empty() {
            cfg.url = "http://localhost:11434".into();
        }
        Self(OpenAiCompatibleJudge::new(cfg))
    }
}
#[async_trait::async_trait]
impl GemmaJudge for OllamaJudge {
    async fn judge(&self, input: &JudgeInput) -> Result<JudgeOutput> {
        self.0.judge(input).await
    }
}
pub async fn list_models(url: &str) -> Result<Vec<String>> {
    let v: serde_json::Value = reqwest::get(format!("{}/api/tags", url.trim_end_matches('/')))
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(v.get("models")
        .and_then(|m| m.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.get("name").and_then(|n| n.as_str()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default())
}
