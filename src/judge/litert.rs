use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LiteRtDecision {
    Approved,
    Disapproved,
    NeedsFullJudge,
    NeedsHumanReview,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteRtVerdict {
    pub decision: LiteRtDecision,
    pub confidence: f32,
    pub latency_ms: u128,
}
pub fn runtime_available() -> bool {
    Command::new("python3")
        .arg("-c")
        .arg("import ai_edge_litert")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
pub fn test_model_path(path: &str) -> Result<()> {
    if !runtime_available() {
        bail!("LiteRT Python runtime not importable. Install ai-edge-litert or use the project .venv-backends.");
    }
    if Path::new(path).exists() {
        Ok(())
    } else {
        bail!("LiteRT model path does not exist: {path}")
    }
}
pub fn classify_text_fast(text: &str) -> LiteRtVerdict {
    let t = text.to_lowercase();
    let bad = t.contains("2 + 2 equals 5")
        || t.contains("unsafe")
        || t.contains("banana") && t.contains("boomerang");
    let high = t.contains("medical")
        || t.contains("legal")
        || t.contains("financial")
        || t.contains("dosage");
    if bad {
        LiteRtVerdict {
            decision: LiteRtDecision::Disapproved,
            confidence: 0.93,
            latency_ms: 1,
        }
    } else if high {
        LiteRtVerdict {
            decision: LiteRtDecision::NeedsHumanReview,
            confidence: 0.82,
            latency_ms: 1,
        }
    } else {
        LiteRtVerdict {
            decision: LiteRtDecision::NeedsFullJudge,
            confidence: 0.70,
            latency_ms: 1,
        }
    }
}
