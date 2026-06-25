use the_witness::judge::schema::{parse_judge_verdict, CorrectnessRisk, JudgeVerdict, VerdictKind};

#[test]
fn parser_accepts_fenced_json_and_normalizes_common_small_model_drift() {
    let raw = r#"```json
{
  "verdict": "APPROVED",
  "confidence_number": 100,
  "safety_score": "high",
  "utilization_score": 75,
  "prompt_alignment_score": 1.0,
  "correctness_risk": 0.0,
  "rejection_reason": null,
  "suggested_fix": null,
  "improved_prompt_instruction": null,
  "requires_human_review": false
}
```"#;

    let verdict = parse_judge_verdict(raw).expect("small-model JSON drift should normalize");

    assert_eq!(verdict.verdict, VerdictKind::APPROVED);
    assert_eq!(verdict.correctness_risk, CorrectnessRisk::Low);
    assert!((verdict.confidence - 1.0).abs() < f32::EPSILON);
    assert_eq!(verdict.safety_score, 90);
    assert_eq!(verdict.usefulness_score, 75);
    assert_eq!(verdict.rejection_reason, "");
}

#[test]
fn parser_normalizes_verdict_aliases_and_rejects_bad_scores() {
    let raw = r#"{
      "verdict": "reject",
      "confidence": 0.5,
      "safety_score": 100,
      "usefulness_score": 80,
      "prompt_alignment_score": 70,
      "correctness_risk": "medium",
      "rejection_reason": "unsafe",
      "suggested_fix": "remove unsafe content",
      "improved_prompt_instruction": "answer safely",
      "requires_human_review": false
    }"#;

    let verdict = parse_judge_verdict(raw).expect("verdict alias should normalize");

    assert_eq!(verdict.verdict, VerdictKind::DISAPPROVED);

    let invalid = JudgeVerdict {
        safety_score: 101,
        ..verdict
    };
    let err = invalid
        .validate()
        .expect_err("scores above 100 should fail validation");
    assert!(err.to_string().contains("safety_score"));
}
