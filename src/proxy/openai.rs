#[derive(Debug, Clone, Default)]
pub struct PromptParts {
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub user_prompt: Option<String>,
}
pub fn extract_prompt_parts(body: &serde_json::Value) -> PromptParts {
    let model = body
        .get("model")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let mut system_prompt = None;
    let mut user_prompt = None;
    if let Some(messages) = body.get("messages").and_then(|v| v.as_array()) {
        for msg in messages {
            let role = msg.get("role").and_then(|v| v.as_str()).unwrap_or_default();
            let content = msg.get("content").map(text_content).unwrap_or_default();
            match role {
                "system" => system_prompt = Some(content),
                "user" => user_prompt = Some(content),
                _ => {}
            }
        }
    }
    PromptParts {
        model,
        system_prompt,
        user_prompt,
    }
}
pub fn append_hidden_repair(
    mut body: serde_json::Value,
    repaired_prompt: String,
) -> serde_json::Value {
    if let Some(messages) = body.get_mut("messages").and_then(|v| v.as_array_mut()) {
        messages.push(serde_json::json!({"role":"user","content":repaired_prompt}));
    }
    body
}

fn text_content(value: &serde_json::Value) -> String {
    if let Some(text) = value.as_str() {
        return text.to_string();
    }
    if let Some(parts) = value.as_array() {
        return parts
            .iter()
            .filter_map(|part| {
                part.get("text")
                    .and_then(|text| text.as_str())
                    .or_else(|| part.as_str())
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    String::new()
}
