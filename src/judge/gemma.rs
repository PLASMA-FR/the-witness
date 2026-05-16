use crate::{
    config::GemmaConfig,
    judge::schema::{JudgeVerdict, JUDGE_SCHEMA_PROMPT},
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde_json::json;
use std::time::Instant;
#[derive(Debug, Clone)]
pub struct JudgeInput {
    pub original_request: serde_json::Value,
    pub candidate_response: serde_json::Value,
    pub profile: String,
    pub strictness: String,
}
#[derive(Debug, Clone)]
pub struct JudgeOutput {
    pub verdict: JudgeVerdict,
    pub latency_ms: u128,
    pub raw: String,
}
#[async_trait]
pub trait GemmaJudge: Send + Sync {
    async fn judge(&self, input: &JudgeInput) -> Result<JudgeOutput>;
}
#[derive(Clone)]
pub struct OpenAiCompatibleJudge {
    pub config: GemmaConfig,
    client: reqwest::Client,
}
#[derive(Clone)]
pub struct DemoJudge;
#[async_trait]
impl GemmaJudge for DemoJudge {
    async fn judge(&self, input: &JudgeInput) -> Result<JudgeOutput> {
        let text = input.candidate_response.to_string().to_lowercase();
        let bad = text.contains("2 + 2 equals 5")
            || text.contains("unsafe")
            || text.contains("bad answer")
            || text.contains("bananas are just boomerangs that gave up on returning")
            || (text.contains("banana") && text.contains("boomerang"));
        let verdict = if bad {
            JudgeVerdict {
                verdict: crate::judge::schema::VerdictKind::DISAPPROVED,
                confidence: 0.97,
                safety_score: 40,
                usefulness_score: 35,
                prompt_alignment_score: 30,
                correctness_risk: crate::judge::schema::CorrectnessRisk::High,
                rejection_reason: "Demo judge detected an obviously wrong or unsafe response"
                    .into(),
                suggested_fix: "Return a correct, safe, directly aligned answer".into(),
                improved_prompt_instruction: "Correct the previous mistake and answer safely"
                    .into(),
                requires_human_review: false,
            }
        } else {
            JudgeVerdict::approved()
        };
        Ok(JudgeOutput {
            verdict,
            latency_ms: 1,
            raw: "demo".into(),
        })
    }
}

impl OpenAiCompatibleJudge {
    pub fn new(config: GemmaConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
    fn chat_url(&self) -> String {
        if self.config.backend == "ollama" {
            format!("{}/api/chat", self.config.url.trim_end_matches('/'))
        } else {
            format!("{}/chat/completions", self.config.url.trim_end_matches('/'))
        }
    }
}
#[async_trait]
impl GemmaJudge for OpenAiCompatibleJudge {
    async fn judge(&self, input: &JudgeInput) -> Result<JudgeOutput> {
        let now = Instant::now();
        let prompt=format!("{JUDGE_SCHEMA_PROMPT}\nProfile: {}\nStrictness: {}\nOriginal request JSON: {}\nCandidate response JSON: {}\nDecide if candidate is safe, useful, correct, and aligned.",input.profile,input.strictness,input.original_request,input.candidate_response);
        let url = self.chat_url();
        let mut req = if self.config.backend == "ollama" {
            self.client.post(&url).json(&json!({"model":self.config.model,"stream":false,"messages":[{"role":"system","content":"You are Gemma 4 acting as The Witness judge. Return only JSON."},{"role":"user","content":prompt}]}))
        } else {
            self.client.post(&url).json(&json!({"model":self.config.model,"messages":[{"role":"system","content":"You are Gemma 4 acting as The Witness judge. Return only JSON."},{"role":"user","content":prompt}],"temperature":0.0}))
        };
        if let Some(h) = &self.config.auth_header {
            req = req.header("Authorization", h);
        }
        let value: serde_json::Value = req
            .send()
            .await
            .with_context(|| format!("judge POST {url}"))?
            .error_for_status()?
            .json()
            .await?;
        let raw = value
            .pointer("/message/content")
            .or_else(|| value.pointer("/choices/0/message/content"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("judge response did not contain message content: {value}"))?
            .to_string();
        let verdict: JudgeVerdict = serde_json::from_str(raw.trim())
            .with_context(|| format!("judge returned invalid JSON: {raw}"))?;
        verdict.validate()?;
        Ok(JudgeOutput {
            verdict,
            latency_ms: now.elapsed().as_millis(),
            raw,
        })
    }
}
