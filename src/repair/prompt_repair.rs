pub fn build_repaired_prompt(
    original_prompt: &str,
    rejected_response: &str,
    rejection_reason: &str,
    suggested_fix: &str,
    retry_number: u32,
    strictness: &str,
) -> String {
    let escalation = if retry_number > 1 || matches!(strictness, "high" | "critical") {
        "\nBecause previous attempts failed, apply a stricter standard: verify factual claims, state uncertainty, avoid unsafe advice, and do not invent facts."
    } else {
        ""
    };
    format!("Original user request:\n{original_prompt}\n\nThe previous answer was rejected by The Witness.\n\nRejected response excerpt:\n{rejected_response}\n\nRejection reason:\n{rejection_reason}\n\nRequired fix:\n{suggested_fix}\n\nFor the next answer:\n- Answer the original request directly.\n- Fix the issues listed above.\n- Do not repeat the rejected mistake.\n- Be accurate, safe, complete, and aligned with the prompt.\n- State uncertainty when needed.\n- Do not invent facts.\n- If the topic is high-risk, include appropriate cautions.\n- Keep the answer useful and practical.{escalation}\n\nNow generate a corrected answer.")
}
