use the_witness::judge::gemma::{DemoJudge, GemmaJudge, JudgeInput};
use the_witness::judge::schema::VerdictKind;

#[tokio::test]
async fn demo_judge_disapproves_banana_boomerang_nonsense() {
    let judge = DemoJudge;
    let out = judge
        .judge(&JudgeInput {
            original_request: serde_json::json!({"messages":[{"role":"user","content":"What are bananas? Answer factually in one sentence."}]}),
            candidate_response: serde_json::json!({"choices":[{"message":{"role":"assistant","content":"Bananas are just boomerangs that gave up on returning."}}]}),
            profile: "education".into(),
            strictness: "high".into(),
        })
        .await
        .unwrap();
    assert_eq!(out.verdict.verdict, VerdictKind::DISAPPROVED);
}
