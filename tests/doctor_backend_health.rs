use the_witness::config::WitnessConfig;
use the_witness::setup::doctor::run_doctor;

#[tokio::test]
async fn doctor_reports_backend_health_lines() {
    let mut cfg = WitnessConfig::default();
    cfg.gemma.backend = "demo".into();
    cfg.gemma.model = "demo-judge".into();
    cfg.gemma.setup_completed = true;
    cfg.setup.demo_mode = true;
    cfg.setup.judge_schema_test_passed = true;
    cfg.setup.proxy_test_passed = true;
    cfg.setup.model_test_passed = true;

    let report = run_doctor(&cfg).await.unwrap();
    let text = report.lines.join("\n");
    assert!(report.passed);
    assert!(text.contains("Backend selectable: demo"));
    assert!(text.contains("Runtime detected"));
}
