use crate::{
    config::{EndpointConfig, FallbackMode, WitnessConfig},
    judge::gemma::{GemmaJudge, JudgeInput},
    judge::schema::VerdictKind,
    proxy::openai::{append_hidden_repair, extract_prompt_parts},
    repair::prompt_repair::build_repaired_prompt,
    storage::jsonl::JsonlLogger,
    types::{RequestEvent, RequestStatus},
};
use anyhow::{anyhow, Result};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use std::{net::SocketAddr, sync::Arc, time::Instant};
#[derive(Clone)]
pub struct ProxyState {
    pub config: WitnessConfig,
    pub judge: Arc<dyn GemmaJudge>,
    pub logger: JsonlLogger,
    pub client: reqwest::Client,
}
pub fn app(state: ProxyState) -> Router {
    Router::new()
        .route("/:endpoint/*path", post(handle_chat))
        .with_state(state)
}
pub async fn serve(addr: SocketAddr, state: ProxyState) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app(state)).await?;
    Ok(())
}
async fn handle_chat(
    State(state): State<ProxyState>,
    Path((endpoint_name, path)): Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    match process(endpoint_name, path, headers, body, state).await {
        Ok(v) => v.into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    }
}
async fn process(
    endpoint_name: String,
    path: String,
    headers: HeaderMap,
    body: Bytes,
    state: ProxyState,
) -> Result<(StatusCode, axum::Json<serde_json::Value>)> {
    let endpoint = state
        .config
        .endpoints
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case(&endpoint_name) && e.enabled)
        .cloned()
        .ok_or_else(|| anyhow!("unknown or disabled endpoint {endpoint_name}"))?;
    let original: serde_json::Value = serde_json::from_slice(&body)?;
    let parts = extract_prompt_parts(&original);
    let mut event = RequestEvent::new(
        endpoint.name.clone(),
        endpoint.profile.clone(),
        if state.config.defaults.privacy_mode {
            serde_json::json!({"redacted":true})
        } else {
            original.clone()
        },
    );
    event.model = parts.model.clone();
    let mut current = original.clone();
    let mut rejected = "".to_string();
    for attempt in 0..=endpoint.retry_limit {
        event.retry_attempt = attempt;
        let started = Instant::now();
        event.status = RequestStatus::Forwarded;
        let candidate = forward(&state.client, &endpoint, &path, &headers, &current).await?;
        event.candidate_response = Some(candidate.clone());
        event.status = RequestStatus::Judging;
        let judge = state
            .judge
            .judge(&JudgeInput {
                original_request: current.clone(),
                candidate_response: candidate.clone(),
                profile: endpoint.profile.clone(),
                strictness: format!("{:?}", endpoint.strictness),
            })
            .await?;
        event.judge_verdict = Some(judge.verdict.clone());
        event.latency_ms = started.elapsed().as_millis();
        match judge.verdict.verdict {
            VerdictKind::APPROVED => {
                event.status = RequestStatus::Approved;
                event.final_response = Some(candidate.clone());
                state.logger.append(&event).await.ok();
                return Ok((StatusCode::OK, axum::Json(candidate)));
            }
            VerdictKind::NEEDS_HUMAN_REVIEW => {
                event.status = RequestStatus::HumanReview;
                state.logger.append(&event).await.ok();
                return fallback(&endpoint, &judge.verdict.rejection_reason);
            }
            VerdictKind::DISAPPROVED => {
                rejected = candidate.to_string();
                if attempt >= endpoint.retry_limit {
                    event.status = RequestStatus::Failed;
                    state.logger.append(&event).await.ok();
                    return fallback(&endpoint, &judge.verdict.rejection_reason);
                }
                event.status = RequestStatus::Retrying;
                state.logger.append(&event).await.ok();
                let repaired = build_repaired_prompt(
                    parts.user_prompt.as_deref().unwrap_or(""),
                    &rejected,
                    &judge.verdict.rejection_reason,
                    &judge.verdict.suggested_fix,
                    attempt + 1,
                    &format!("{:?}", endpoint.strictness).to_lowercase(),
                );
                current = append_hidden_repair(original.clone(), repaired);
            }
        }
    }
    fallback(&endpoint, &rejected)
}
async fn forward(
    client: &reqwest::Client,
    endpoint: &EndpointConfig,
    path: &str,
    headers: &HeaderMap,
    body: &serde_json::Value,
) -> Result<serde_json::Value> {
    let url = format!(
        "{}/{}",
        endpoint.upstream_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    let mut req = client.post(url).json(body);
    if let Some(auth) = endpoint.resolved_auth_header()? {
        req = req.header("Authorization", auth);
    } else if let Some(h) = headers.get("authorization").and_then(|h| h.to_str().ok()) {
        req = req.header("Authorization", h);
    }
    Ok(req.send().await?.error_for_status()?.json().await?)
}
fn fallback(
    endpoint: &EndpointConfig,
    reason: &str,
) -> Result<(StatusCode, axum::Json<serde_json::Value>)> {
    let msg = match endpoint.fallback_mode {
        FallbackMode::SafeResponse => "The Witness blocked this response after verification. Please retry or request human review.",
        FallbackMode::HumanReview => "The Witness paused this response for human review.",
        FallbackMode::DemoJudge => "The Witness blocked this response; demo judge fallback is only for explicitly selected demo workflows.",
        FallbackMode::Error => "The Witness rejected this response.",
    };
    Ok((
        StatusCode::OK,
        axum::Json(
            serde_json::json!({"choices":[{"message":{"role":"assistant","content":msg}}],"witness":{"blocked":true,"reason":reason}}),
        ),
    ))
}
