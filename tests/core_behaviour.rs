use the_witness::{
    config::{Strictness, WitnessConfig},
    judge::schema::JudgeVerdict,
    proxy::openai::extract_prompt_parts,
    repair::prompt_repair::build_repaired_prompt,
    types::SecretString,
};

#[test]
fn default_config_contains_safe_defaults() {
    let cfg = WitnessConfig::default();
    assert_eq!(cfg.gemma.backend, "ollama");
    assert_eq!(cfg.defaults.retry_limit, 3);
    assert_eq!(cfg.defaults.strictness, Strictness::Medium);
    assert!(!cfg.gemma.setup_completed);
}

#[test]
fn judge_verdict_accepts_required_schema() {
    let json = r#"{
      "verdict":"DISAPPROVED",
      "confidence":0.91,
      "safety_score":72,
      "usefulness_score":12,
      "prompt_alignment_score":20,
      "correctness_risk":"high",
      "rejection_reason":"Math is wrong",
      "suggested_fix":"Say 2 + 2 = 4",
      "improved_prompt_instruction":"Correct arithmetic",
      "requires_human_review":false
    }"#;
    let verdict: JudgeVerdict = serde_json::from_str(json).unwrap();
    verdict.validate().unwrap();
    assert_eq!(verdict.verdict.as_str(), "DISAPPROVED");
}

#[test]
fn secret_display_is_redacted() {
    assert_eq!(SecretString::new("sk-test-secret").redacted(), "sk-t…cret");
    assert!(format!("{:?}", SecretString::new("abc")).contains("<redacted>"));
}

#[test]
fn repair_prompt_preserves_user_intent_and_adds_fix() {
    let repaired = build_repaired_prompt(
        "Explain rust lifetimes",
        "Unsafe advice",
        "Missing caveat",
        "Add soundness warning",
        2,
        "high",
    );
    assert!(repaired.contains("Explain rust lifetimes"));
    assert!(repaired.contains("Missing caveat"));
    assert!(repaired.contains("Add soundness warning"));
    assert!(repaired.contains("stricter"));
    assert!(!repaired.contains("Gemma internal"));
}

#[test]
fn openai_prompt_extraction_finds_system_user_and_model() {
    let body = serde_json::json!({
        "model":"gpt-test",
        "messages":[
            {"role":"system", "content":"Be safe"},
            {"role":"user", "content":"Hello"}
        ]
    });
    let parts = extract_prompt_parts(&body);
    assert_eq!(parts.model.as_deref(), Some("gpt-test"));
    assert_eq!(parts.system_prompt.as_deref(), Some("Be safe"));
    assert_eq!(parts.user_prompt.as_deref(), Some("Hello"));
}
