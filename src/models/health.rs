use crate::{config::GemmaConfig, setup::tests::run_judge_capability_test};
use anyhow::Result;
pub async fn run_model_sanity_test(cfg: GemmaConfig) -> Result<()> {
    run_judge_capability_test(cfg).await
}
