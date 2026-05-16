use crate::{
    config::GemmaConfig,
    judge::gemma::{GemmaJudge, JudgeInput, JudgeOutput, OpenAiCompatibleJudge},
};
use anyhow::Result;
#[derive(Clone)]
pub struct LlamaCppJudge(pub OpenAiCompatibleJudge);
impl LlamaCppJudge {
    pub fn new(mut cfg: GemmaConfig) -> Self {
        cfg.backend = "llama.cpp".into();
        if cfg.url.is_empty() {
            cfg.url = "http://localhost:8080/v1".into();
        }
        Self(OpenAiCompatibleJudge::new(cfg))
    }
}
#[async_trait::async_trait]
impl GemmaJudge for LlamaCppJudge {
    async fn judge(&self, input: &JudgeInput) -> Result<JudgeOutput> {
        self.0.judge(input).await
    }
}
pub async fn test_server(url: &str) -> Result<()> {
    let base = url.trim_end_matches('/').trim_end_matches("/v1");
    reqwest::get(format!("{base}/v1/models"))
        .await?
        .error_for_status()?;
    Ok(())
}
