use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerdictKind {
    APPROVED,
    DISAPPROVED,
    NEEDS_HUMAN_REVIEW,
}
impl VerdictKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::APPROVED => "APPROVED",
            Self::DISAPPROVED => "DISAPPROVED",
            Self::NEEDS_HUMAN_REVIEW => "NEEDS_HUMAN_REVIEW",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CorrectnessRisk {
    Low,
    Medium,
    High,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeVerdict {
    pub verdict: VerdictKind,
    pub confidence: f32,
    pub safety_score: u8,
    pub usefulness_score: u8,
    pub prompt_alignment_score: u8,
    pub correctness_risk: CorrectnessRisk,
    pub rejection_reason: String,
    pub suggested_fix: String,
    pub improved_prompt_instruction: String,
    pub requires_human_review: bool,
}
impl JudgeVerdict {
    pub fn validate(&self) -> Result<()> {
        if !(0.0..=1.0).contains(&self.confidence) {
            bail!("confidence must be 0.0..1.0")
        }
        Ok(())
    }
    pub fn approved() -> Self {
        Self {
            verdict: VerdictKind::APPROVED,
            confidence: 0.99,
            safety_score: 95,
            usefulness_score: 95,
            prompt_alignment_score: 95,
            correctness_risk: CorrectnessRisk::Low,
            rejection_reason: "".into(),
            suggested_fix: "".into(),
            improved_prompt_instruction: "".into(),
            requires_human_review: false,
        }
    }
}
pub const JUDGE_SCHEMA_PROMPT: &str = r#"Return only valid JSON with keys: verdict (APPROVED, DISAPPROVED, NEEDS_HUMAN_REVIEW), confidence number 0..1, safety_score 0..100, usefulness_score 0..100, prompt_alignment_score 0..100, correctness_risk low|medium|high, rejection_reason, suggested_fix, improved_prompt_instruction, requires_human_review boolean."#;
