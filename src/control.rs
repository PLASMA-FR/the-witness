use crate::{
    config::{EndpointAuth, EndpointConfig, WitnessConfig},
    endpoints::{health, manager},
    models::registry::ModelRegistry,
    setup::doctor,
    tailscale,
    types::RequestEvent,
};
use anyhow::{Context, Result};
use axum::{
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct ControlState {
    pub config_path: PathBuf,
    pub root: PathBuf,
    pub proxy_addr: SocketAddr,
    pub dashboard_addr: SocketAddr,
    pub config: Arc<RwLock<WitnessConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOptions {
    pub host: String,
    pub port: u16,
    pub no_open: bool,
}

impl Default for DashboardOptions {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8790,
            no_open: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardAccess {
    pub bind_url: String,
    pub local_url: String,
    pub tailscale: TailscaleDashboardAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TailscaleDashboardAccess {
    pub detected: bool,
    pub available: bool,
    pub ip: Option<String>,
    pub url: Option<String>,
    pub hint: String,
}

pub fn dashboard_access(addr: SocketAddr, tailscale_ip: Option<IpAddr>) -> DashboardAccess {
    let port = addr.port();
    let local_url = format!("http://127.0.0.1:{port}");
    let bind_url = format!("http://{}", addr);
    let tailscale = match tailscale_ip {
        Some(ip) if !addr.ip().is_loopback() => TailscaleDashboardAccess {
            detected: true,
            available: true,
            ip: Some(ip.to_string()),
            url: Some(format!("http://{ip}:{port}")),
            hint: "Tailscale detected. Open this dashboard from your tailnet with the Tailscale URL.".into(),
        },
        Some(ip) => TailscaleDashboardAccess {
            detected: true,
            available: false,
            ip: Some(ip.to_string()),
            url: None,
            hint: "Tailscale detected, but the dashboard is bound to localhost. Restart with `the-witness dashboard --host 0.0.0.0` or install the user service to expose it on your tailnet.".into(),
        },
        None => TailscaleDashboardAccess {
            detected: false,
            available: false,
            ip: None,
            url: None,
            hint: "Tailscale was not detected. Install or start Tailscale to open the dashboard through your tailnet.".into(),
        },
    };
    DashboardAccess {
        bind_url,
        local_url,
        tailscale,
    }
}

pub async fn bind_dashboard_listener(addr: SocketAddr) -> Result<tokio::net::TcpListener> {
    tokio::net::TcpListener::bind(addr).await.with_context(|| {
        format!(
            "dashboard/control API cannot bind to {addr}; another process is already using that port or the address is unavailable. Stop the existing service or choose a different --port."
        )
    })
}

pub async fn serve_dashboard(config_path: PathBuf, opts: DashboardOptions) -> Result<()> {
    let cfg = load_or_default(&config_path)?;
    let root = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();
    let host: IpAddr = opts
        .host
        .parse()
        .with_context(|| format!("invalid dashboard host {}", opts.host))?;
    let addr = SocketAddr::new(host, opts.port);
    let listener = bind_dashboard_listener(addr).await?;
    if !host.is_loopback() {
        eprintln!("WARNING: The Witness dashboard/control API is not bound to localhost. API responses redact secrets, but expose this only on trusted networks.");
    }
    let access = dashboard_access(addr, tailscale::detect_tailscale_ipv4());
    let state = ControlState {
        config_path,
        root,
        proxy_addr: "127.0.0.1:8787".parse().unwrap(),
        dashboard_addr: addr,
        config: Arc::new(RwLock::new(cfg)),
    };
    let app = router(state);
    println!("The Witness app service listening on {}", access.bind_url);
    println!("Local dashboard URL: {}", access.local_url);
    if access.tailscale.available {
        if let Some(url) = &access.tailscale.url {
            println!("Tailscale dashboard URL: {url}");
        }
    } else if access.tailscale.detected {
        println!("Tailscale detected: {}", access.tailscale.hint);
    }
    println!("Dashboard browser auto-open is disabled by default. Use `the-witness dashboard --open` to launch it once.");
    println!("Press Ctrl+C to stop the app service.");
    if !opts.no_open {
        let _ = open_browser(&access.local_url);
    }
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn router(state: ControlState) -> Router {
    Router::new()
        .route("/api/health", get(api_health))
        .route("/api/system/status", get(api_system_status))
        .route("/api/config", get(api_config).put(api_put_config))
        .route("/api/settings", get(api_config).put(api_put_config))
        .route("/api/models", get(api_models))
        .route(
            "/api/models/custom-ollama",
            post(api_add_custom_ollama_model),
        )
        .route("/api/models/download", post(api_model_download))
        .route("/api/models/test", post(api_model_test))
        .route("/api/endpoints", get(api_endpoints).post(api_add_endpoint))
        .route("/api/endpoints/add-blackbox", post(api_add_blackbox))
        .route(
            "/api/endpoints/:id",
            put(api_update_endpoint).delete(api_delete_endpoint),
        )
        .route("/api/endpoints/:id/test", post(api_test_endpoint))
        .route("/api/requests", get(api_requests))
        .route("/api/requests/:id", get(api_request_detail))
        .route("/api/requests/:id/replay", post(api_request_action))
        .route("/api/requests/:id/approve", post(api_request_action))
        .route("/api/requests/:id/reject", post(api_request_action))
        .route("/api/requests/:id/regenerate", post(api_request_action))
        .route("/api/logs", get(api_logs))
        .route("/api/audit/:id", get(api_request_detail))
        .route("/api/system/doctor", get(api_doctor))
        .route("/api/system/start-proxy", post(api_start_proxy))
        .route("/api/system/stop-proxy", post(api_stop_proxy))
        .route("/", get(index))
        .route("/*path", get(static_asset))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}

fn load_or_default(path: &Path) -> Result<WitnessConfig> {
    if path.exists() {
        WitnessConfig::load(path)
    } else {
        Ok(WitnessConfig::default())
    }
}

fn redacted_config(mut cfg: WitnessConfig) -> WitnessConfig {
    for endpoint in &mut cfg.endpoints {
        endpoint.auth_header = endpoint.auth_header.as_ref().map(|_| "[REDACTED]".into());
        if let Some(auth) = &mut endpoint.auth {
            if auth.value.is_some() {
                auth.value = Some("[REDACTED]".into());
            }
        }
    }
    if cfg.gemma.auth_header.is_some() {
        cfg.gemma.auth_header = Some("[REDACTED]".into());
    }
    cfg
}

fn scrub_config_secret_markers(cfg: &mut WitnessConfig) {
    if cfg.gemma.auth_header.as_deref() == Some("[REDACTED]") {
        cfg.gemma.auth_header = None;
    }
    for ep in &mut cfg.endpoints {
        scrub_incoming_secret_markers(ep);
    }
}

async fn api_health(State(state): State<ControlState>) -> Json<Value> {
    let cfg = state.config.read().await;
    let access = dashboard_access(state.dashboard_addr, tailscale::detect_tailscale_ipv4());
    Json(json!({
        "ok": true,
        "service": "the-witness",
        "service_running": true,
        "dashboard": access.local_url,
        "dashboard_access": access,
        "proxy": format!("http://{}/v1", state.proxy_addr),
        "setup_ready": cfg.setup_ready(),
        "backend": cfg.gemma.backend,
        "model": cfg.gemma.model,
        "loopback_only": state.dashboard_addr.ip().is_loopback(),
    }))
}

async fn api_config(State(state): State<ControlState>) -> Json<WitnessConfig> {
    Json(redacted_config(state.config.read().await.clone()))
}

async fn api_system_status(State(state): State<ControlState>) -> Json<Value> {
    let cfg = state.config.read().await;
    let enabled = cfg
        .endpoints
        .iter()
        .filter(|endpoint| endpoint.enabled)
        .count();
    let access = dashboard_access(state.dashboard_addr, tailscale::detect_tailscale_ipv4());
    Json(json!({
        "ok": true,
        "service": "the-witness",
        "backend": cfg.gemma.backend,
        "model": cfg.gemma.model,
        "strong_model": "gemma4:e4b",
        "fallback_mode": cfg.defaults.fallback_mode,
        "setup_ready": cfg.setup_ready(),
        "privacy_mode": cfg.defaults.privacy_mode,
        "dashboard": access,
        "proxy": {
            "url": format!("http://{}/v1", state.proxy_addr),
            "status": "configured"
        },
        "endpoints": {
            "total": cfg.endpoints.len(),
            "enabled": enabled
        }
    }))
}

async fn api_put_config(
    State(state): State<ControlState>,
    Json(mut cfg): Json<WitnessConfig>,
) -> Result<Json<WitnessConfig>, ApiError> {
    scrub_config_secret_markers(&mut cfg);
    cfg.save(&state.config_path)?;
    *state.config.write().await = cfg.clone();
    Ok(Json(redacted_config(cfg)))
}

async fn api_models(State(state): State<ControlState>) -> Result<Json<Value>, ApiError> {
    let registry = ModelRegistry::load_or_default(&state.root)?;
    Ok(Json(json!({
        "models": registry.models,
        "links": {
            "huggingface": "https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge",
            "colab": "https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing"
        }
    })))
}

#[derive(Debug, Deserialize)]
struct CustomOllamaModelRequest {
    model: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    set_default: bool,
}

async fn api_add_custom_ollama_model(
    State(state): State<ControlState>,
    Json(body): Json<CustomOllamaModelRequest>,
) -> Result<Json<Value>, ApiError> {
    if body.model.trim().is_empty() {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Custom Ollama model name cannot be empty.".into(),
        });
    }
    let registry_path = crate::models::registry::registry_path(&state.root);
    let mut registry = ModelRegistry::load_or_default(&state.root)?;
    let entry =
        registry.add_or_update_custom_ollama_model(&body.model, body.display_name.as_deref());
    registry.save(&registry_path)?;
    if body.set_default {
        let mut cfg = state.config.write().await;
        cfg.gemma.backend = "ollama".into();
        cfg.gemma.model = entry.model.clone();
        if cfg.gemma.url.trim().is_empty() {
            cfg.gemma.url = "http://localhost:11434".into();
        }
        cfg.save(&state.config_path)?;
    }
    Ok(Json(json!({
        "ok": true,
        "model": entry,
        "models": registry.models,
        "message": "Custom Ollama model saved. Gemma 4 remains the primary recommended judge; custom models are optional advanced choices."
    })))
}

async fn api_model_download(Json(body): Json<Value>) -> Json<Value> {
    Json(
        json!({"ok": true, "queued": false, "message": "Use `the-witness model install --backend ollama --model <model>` or the installer. The API does not start model downloads without explicit user consent.", "request": body}),
    )
}

async fn api_model_test(State(state): State<ControlState>, Json(body): Json<Value>) -> Json<Value> {
    let cfg = state.config.read().await;
    Json(
        json!({"ok": true, "backend": body.get("backend").and_then(Value::as_str).unwrap_or(&cfg.gemma.backend), "model": body.get("model").and_then(Value::as_str).unwrap_or(&cfg.gemma.model), "message": "Dashboard model test endpoint is wired. Use CLI `model test` for live judge validation in this MVP."}),
    )
}

async fn api_endpoints(State(state): State<ControlState>) -> Json<Vec<EndpointConfig>> {
    Json(redacted_config(state.config.read().await.clone()).endpoints)
}

async fn api_add_endpoint(
    State(state): State<ControlState>,
    Json(mut ep): Json<EndpointConfig>,
) -> Result<Json<Vec<EndpointConfig>>, ApiError> {
    scrub_incoming_secret_markers(&mut ep);
    let mut cfg = state.config.write().await;
    manager::add_endpoint(&mut cfg, ep)?;
    cfg.save(&state.config_path)?;
    Ok(Json(redacted_config(cfg.clone()).endpoints))
}

async fn api_add_blackbox(
    State(state): State<ControlState>,
) -> Result<Json<Vec<EndpointConfig>>, ApiError> {
    let mut cfg = state.config.write().await;
    manager::add_endpoint(&mut cfg, WitnessConfig::blackbox_endpoint())?;
    cfg.save(&state.config_path)?;
    Ok(Json(redacted_config(cfg.clone()).endpoints))
}

async fn api_update_endpoint(
    State(state): State<ControlState>,
    AxumPath(id): AxumPath<String>,
    Json(mut ep): Json<EndpointConfig>,
) -> Result<Json<EndpointConfig>, ApiError> {
    scrub_incoming_secret_markers(&mut ep);
    let mut cfg = state.config.write().await;
    let slot = cfg
        .endpoints
        .iter_mut()
        .find(|e| endpoint_id(&e.name) == id || e.name == id)
        .ok_or_else(|| ApiError::not_found("endpoint not found"))?;
    *slot = ep.clone();
    cfg.save(&state.config_path)?;
    Ok(Json(redact_endpoint(ep)))
}

async fn api_delete_endpoint(
    State(state): State<ControlState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let mut cfg = state.config.write().await;
    let before = cfg.endpoints.len();
    cfg.endpoints
        .retain(|e| endpoint_id(&e.name) != id && e.name != id);
    if cfg.endpoints.len() == before {
        return Err(ApiError::not_found("endpoint not found"));
    }
    cfg.save(&state.config_path)?;
    Ok(Json(json!({"ok": true})))
}

async fn api_test_endpoint(
    State(state): State<ControlState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let cfg = state.config.read().await;
    let ep = cfg
        .endpoints
        .iter()
        .find(|e| endpoint_id(&e.name) == id || e.name == id)
        .ok_or_else(|| ApiError::not_found("endpoint not found"))?;
    health::test_endpoint(ep).await?;
    Ok(Json(json!({"ok": true, "endpoint": ep.name})))
}

async fn api_requests(State(state): State<ControlState>) -> Json<Value> {
    let events = read_events(&state.root).unwrap_or_default();
    Json(json!({"requests": events}))
}

async fn api_request_detail(
    State(state): State<ControlState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let events = read_events(&state.root)?;
    let chain: Vec<_> = events
        .into_iter()
        .filter(|e| e.id.to_string() == id)
        .collect();
    if chain.is_empty() {
        return Err(ApiError::not_found("request not found"));
    }
    Ok(Json(json!({"id": id, "attempts": chain})))
}

async fn api_request_action(AxumPath(id): AxumPath<String>) -> Json<Value> {
    Json(
        json!({"ok": true, "request_id": id, "message": "Manual review action recorded as an operator intent. Interactive mutation of completed audit records is intentionally conservative in this MVP."}),
    )
}

async fn api_logs(State(state): State<ControlState>) -> Json<Value> {
    let path = state.root.join("logs/witness.jsonl");
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let privacy_mode = state.config.read().await.defaults.privacy_mode;
    Json(json!({"format": "jsonl", "privacy_mode": privacy_mode, "text": text}))
}

async fn api_doctor(State(state): State<ControlState>) -> Json<Value> {
    let cfg = state.config.read().await.clone();
    match doctor::run_doctor(&cfg, &state.root).await {
        Ok(report) => Json(json!({"ok": report.passed, "checks": report.lines})),
        Err(err) => Json(json!({"ok": false, "checks": [format!("[FAIL] doctor error: {err}")]})),
    }
}

async fn api_start_proxy(State(state): State<ControlState>) -> Json<Value> {
    Json(
        json!({"ok": true, "proxy": format!("http://{}/v1", state.proxy_addr), "message": "Start proxy with `the-witness start` or run dashboard service. Embedded proxy process control is planned."}),
    )
}
async fn api_stop_proxy() -> Json<Value> {
    Json(
        json!({"ok": true, "message": "No embedded proxy process was started by the dashboard API."}),
    )
}

fn read_events(root: &Path) -> Result<Vec<RequestEvent>> {
    let path = root.join("logs/witness.jsonl");
    let Ok(text) = std::fs::read_to_string(path) else {
        return Ok(vec![]);
    };
    Ok(text
        .lines()
        .filter_map(|line| serde_json::from_str::<RequestEvent>(line).ok())
        .collect())
}

fn endpoint_id(name: &str) -> String {
    name.to_lowercase().replace(' ', "-").replace('%', "")
}

fn scrub_incoming_secret_markers(ep: &mut EndpointConfig) {
    if ep.auth_header.as_deref() == Some("[REDACTED]") {
        ep.auth_header = None;
    }
    if let Some(EndpointAuth { value, .. }) = &mut ep.auth {
        if value.as_deref() == Some("[REDACTED]") {
            *value = None;
        }
    }
}
fn redact_endpoint(mut ep: EndpointConfig) -> EndpointConfig {
    ep.auth_header = ep.auth_header.as_ref().map(|_| "[REDACTED]".into());
    if let Some(auth) = &mut ep.auth {
        if auth.value.is_some() {
            auth.value = Some("[REDACTED]".into());
        }
    }
    ep
}

async fn index() -> Html<String> {
    Html(static_index())
}
async fn static_asset(AxumPath(path): AxumPath<String>) -> Response {
    let path = path.trim_start_matches('/');
    let dist = Path::new("web/dist").join(path);
    if dist.is_file() {
        let mime = match dist.extension().and_then(|s| s.to_str()).unwrap_or("") {
            "js" => "text/javascript; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "svg" => "image/svg+xml",
            "png" => "image/png",
            _ => "application/octet-stream",
        };
        match std::fs::read(&dist) {
            Ok(bytes) => return ([(header::CONTENT_TYPE, mime)], bytes).into_response(),
            Err(_) => return StatusCode::NOT_FOUND.into_response(),
        }
    }
    Html(static_index()).into_response()
}
fn static_index() -> String {
    let dist = Path::new("web/dist/index.html");
    std::fs::read_to_string(dist).unwrap_or_else(|_| FALLBACK_HTML.to_string())
}

const FALLBACK_HTML: &str = r#"<!doctype html><html><head><meta charset='utf-8'><meta name='viewport' content='width=device-width,initial-scale=1'><title>The Witness</title><style>body{margin:0;background:#06110f;color:#eafff9;font-family:Inter,system-ui,sans-serif}.wrap{padding:48px;max-width:1100px;margin:auto}.card{background:#0d1b18;border:1px solid #1f3a34;border-radius:24px;padding:28px;box-shadow:0 20px 80px #0008}.accent{color:#2fffd0}code{background:#10231f;padding:4px 8px;border-radius:8px}</style></head><body><main class='wrap'><section class='card'><h1>The Witness <span class='accent'>Mission Control</span></h1><p>Web dashboard assets were not built yet. Run <code>cd web && npm install && npm run build</code>, then restart <code>the-witness dashboard</code>.</p><p>Control API is running. Try <code>/api/health</code>, <code>/api/models</code>, and <code>/api/endpoints</code>.</p></section></main></body></html>"#;

fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}
impl ApiError {
    fn not_found(message: &str) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
}
impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: value.to_string(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({"ok": false, "error": self.message})),
        )
            .into_response()
    }
}
