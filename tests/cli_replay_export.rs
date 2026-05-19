use std::path::Path;
use the_witness::cli::{export_request_report, replay_request_summary};
use the_witness::judge::schema::{CorrectnessRisk, JudgeVerdict, VerdictKind};
use the_witness::types::{RequestEvent, RequestStatus};

fn sample_event() -> RequestEvent {
    let mut event = RequestEvent::new(
        "Demo".to_string(),
        "coding".to_string(),
        serde_json::json!({"model":"mock","messages":[{"role":"user","content":"Say hello"}]}),
    );
    event.status = RequestStatus::Approved;
    event.retry_attempt = 1;
    event.latency_ms = 42;
    event.candidate_response =
        Some(serde_json::json!({"choices":[{"message":{"content":"hello"}}]}));
    event.final_response = event.candidate_response.clone();
    event.judge_verdict = Some(JudgeVerdict {
        verdict: VerdictKind::APPROVED,
        confidence: 0.99,
        safety_score: 95,
        usefulness_score: 90,
        prompt_alignment_score: 92,
        correctness_risk: CorrectnessRisk::Low,
        rejection_reason: "".to_string(),
        suggested_fix: "".to_string(),
        improved_prompt_instruction: "".to_string(),
        requires_human_review: false,
    });
    event
}

#[test]
fn replay_missing_request_id_returns_clear_error() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join("logs");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::write(log_dir.join("witness.jsonl"), "").unwrap();
    let err = replay_request_summary(dir.path(), "missing-id").unwrap_err();
    assert!(err.to_string().contains("request id not found: missing-id"));
}

#[test]
fn replay_and_export_use_jsonl_audit_events() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join("logs");
    std::fs::create_dir_all(&log_dir).unwrap();
    let event = sample_event();
    std::fs::write(
        log_dir.join("witness.jsonl"),
        format!("{}\n", serde_json::to_string(&event).unwrap()),
    )
    .unwrap();

    let summary = replay_request_summary(dir.path(), &event.id.to_string()).unwrap();
    assert!(summary.contains("Request ID:"));
    assert!(summary.contains("Endpoint: Demo"));
    assert!(summary.contains("Status: Approved"));

    let markdown = export_request_report(dir.path(), &event.id.to_string(), "markdown").unwrap();
    assert!(markdown.contains("# The Witness Verification Report"));
    assert!(markdown.contains("Endpoint: Demo"));
    assert!(markdown.contains("Verdict: APPROVED"));

    let json = export_request_report(dir.path(), &event.id.to_string(), "json").unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(json["endpoint_name"], "Demo");

    let jsonl = export_request_report(dir.path(), &event.id.to_string(), "jsonl").unwrap();
    assert!(!jsonl.contains('\n'));
    let jsonl: serde_json::Value = serde_json::from_str(&jsonl).unwrap();
    assert_eq!(jsonl["endpoint_name"], "Demo");
}

#[test]
fn replay_and_export_use_latest_jsonl_audit_event_for_retry_chain() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join("logs");
    std::fs::create_dir_all(&log_dir).unwrap();
    let mut retrying = sample_event();
    retrying.status = RequestStatus::Retrying;
    retrying.retry_attempt = 0;
    let mut approved = retrying.clone();
    approved.status = RequestStatus::Approved;
    approved.retry_attempt = 1;
    std::fs::write(
        log_dir.join("witness.jsonl"),
        format!(
            "{}\n{}\n",
            serde_json::to_string(&retrying).unwrap(),
            serde_json::to_string(&approved).unwrap()
        ),
    )
    .unwrap();

    let summary = replay_request_summary(dir.path(), &approved.id.to_string()).unwrap();
    assert!(summary.contains("Status: Approved"));
    assert!(summary.contains("Retry attempt: 1"));
}

#[test]
fn unsupported_export_format_fails_clearly() {
    let err = export_request_report(
        Path::new("/tmp/definitely-missing-witness-root"),
        "id",
        "pdf",
    )
    .unwrap_err();
    assert!(err.to_string().contains("unsupported export format: pdf"));
}
