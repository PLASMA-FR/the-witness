use crate::{
    config::{EndpointConfig, FallbackMode, Strictness, WitnessConfig},
    control::{self, DashboardOptions},
    endpoints::manager,
    judge::gemma::{DemoJudge, OpenAiCompatibleJudge},
    service, setup,
    storage::jsonl::JsonlLogger,
    tui::app::App,
};
use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};
#[derive(Parser, Debug)]
#[command(
    name = "the-witness",
    version,
    about = "Gemma 4 reliability firewall TUI and OpenAI-compatible proxy"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(long)]
    pub config: Option<PathBuf>,
}

impl Cli {
    pub fn resolved_config_path(&self) -> PathBuf {
        self.config
            .clone()
            .unwrap_or_else(WitnessConfig::default_path)
    }
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a starter witness.toml in the chosen project directory.
    Init {
        path: PathBuf,
    },
    /// Re-run the first-run setup wizard and readiness checks.
    Setup,
    /// Check config, local models, proxy ports, logs, and endpoint prerequisites.
    Doctor,
    /// Start the terminal UI, or start only the proxy when --proxy-addr is provided.
    Start {
        #[arg(long)]
        proxy_addr: Option<SocketAddr>,
    },
    /// Start the local Web UI and control API. It binds to localhost by default.
    Dashboard {
        /// Open the dashboard in the default browser once. The app service itself still runs either way.
        #[arg(long)]
        open: bool,
        /// Legacy alias. Browser auto-open is disabled by default.
        #[arg(long)]
        no_open: bool,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 8790)]
        port: u16,
    },
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },
    Model {
        #[command(subcommand)]
        command: ModelCommands,
    },
    Endpoint {
        #[command(subcommand)]
        command: EndpointCommands,
    },
    Logs,
    Replay {
        request_id: String,
    },
    Export {
        request_id: String,
        #[arg(long, default_value = "markdown")]
        format: String,
    },
}
#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    Install,
    Uninstall,
    Start,
    Stop,
    Status,
    Logs,
}

#[derive(Subcommand, Debug)]
pub enum ModelCommands {
    List,
    Install(ModelInstall),
    Download(ModelDownload),
    /// Register an editable custom Ollama model in models/models.toml.
    AddOllama(ModelAddOllama),
    Test(ModelTest),
}
#[derive(Args, Debug)]
pub struct ModelInstall {
    #[arg(long)]
    pub backend: Option<String>,
    #[arg(long)]
    pub model: Option<String>,
}
#[derive(Args, Debug)]
pub struct ModelAddOllama {
    /// Ollama model tag/name, for example gemma4:e2b or your-local-model:latest.
    #[arg(long)]
    pub model: String,
    /// Optional friendly label shown in the Web UI model manager.
    #[arg(long)]
    pub display_name: Option<String>,
    /// Also set this model as the current judge model.
    #[arg(long)]
    pub set_default: bool,
    /// Pull the model through Ollama after registering it.
    #[arg(long)]
    pub pull: bool,
}
#[derive(Args, Debug)]
pub struct ModelDownload {
    #[arg(long, default_value = "huggingface")]
    pub source: String,
    #[arg(long)]
    pub model: String,
}
#[derive(Args, Debug)]
pub struct ModelTest {
    #[arg(long)]
    pub backend: Option<String>,
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub model_path: Option<String>,
}
#[derive(Subcommand, Debug)]
pub enum EndpointCommands {
    Add(EndpointAdd),
    AddBlackbox,
    List,
    Test { name: String },
    Disable { name: String },
    Enable { name: String },
}
#[derive(Args, Debug)]
pub struct EndpointAdd {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub upstream: Option<String>,
    #[arg(long)]
    pub local: Option<String>,
    #[arg(long, default_value = "coding")]
    pub profile: String,
    #[arg(long, default_value_t = 3)]
    pub retry_limit: u32,
    #[arg(long, default_value = "medium")]
    pub strictness: String,
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub auth_header: Option<String>,
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.resolved_config_path();
    match cli.command.unwrap_or(Commands::Start { proxy_addr: None }) {
        Commands::Init { path: dir } => init(&dir),
        Commands::Setup => {
            setup::wizard::run_setup_wizard(&path).await?;
            Ok(())
        }
        Commands::Doctor => doctor(&path).await,
        Commands::Start { proxy_addr } => start(&path, proxy_addr).await,
        Commands::Dashboard {
            open,
            no_open,
            host,
            port,
        } => {
            control::serve_dashboard(
                path,
                DashboardOptions {
                    host,
                    port,
                    no_open: no_open || !open,
                },
            )
            .await
        }
        Commands::Service { command } => service_command(command),
        Commands::Model { command } => model(&path, command).await,
        Commands::Endpoint { command } => endpoint(&path, command).await,
        Commands::Logs => {
            println!(
                "Logs: {}",
                path.parent()
                    .unwrap_or(Path::new("."))
                    .join("logs/witness.jsonl")
                    .display()
            );
            Ok(())
        }
        Commands::Replay { request_id } => {
            let root = path.parent().unwrap_or(Path::new("."));
            println!("{}", replay_request_summary(root, &request_id)?);
            Ok(())
        }
        Commands::Export { request_id, format } => {
            let root = path.parent().unwrap_or(Path::new("."));
            println!("{}", export_request_report(root, &request_id, &format)?);
            Ok(())
        }
    }
}

fn service_command(command: ServiceCommands) -> Result<()> {
    match command {
        ServiceCommands::Install => {
            service::install()?;
            println!("The Witness service installed for this user.");
        }
        ServiceCommands::Uninstall => {
            service::uninstall()?;
            println!("The Witness service removed.");
        }
        ServiceCommands::Start => {
            service::start()?;
            println!("The Witness service start requested.");
        }
        ServiceCommands::Stop => {
            service::stop()?;
            println!("The Witness service stop requested.");
        }
        ServiceCommands::Status => {
            let status = service::status()?;
            println!("{}", serde_json::to_string_pretty(&status)?);
        }
        ServiceCommands::Logs => println!("{}", service::logs()?),
    }
    Ok(())
}

pub fn replay_request_summary(root: &Path, request_id: &str) -> Result<String> {
    let event = find_request_event(root, request_id)?;
    Ok(format!(
        "Request ID: {}\nEndpoint: {}\nModel: {}\nProfile: {}\nStatus: {:?}\nRetry attempt: {}\nLatency: {}ms\nTimestamp: {}",
        event.id,
        event.endpoint_name,
        event.model.as_deref().unwrap_or("<unknown>"),
        event.profile,
        event.status,
        event.retry_attempt,
        event.latency_ms,
        event.timestamp
    ))
}

pub fn export_request_report(root: &Path, request_id: &str, format: &str) -> Result<String> {
    match format {
        "markdown" | "md" => {
            let event = find_request_event(root, request_id)?;
            let verdict = event
                .judge_verdict
                .as_ref()
                .map(|v| v.verdict.as_str())
                .unwrap_or("<none>");
            Ok(format!(
                "# The Witness Verification Report\n\n- Request ID: {}\n- Endpoint: {}\n- Model: {}\n- Profile: {}\n- Status: {:?}\n- Retry attempt: {}\n- Latency: {}ms\n- Timestamp: {}\n- Verdict: {}\n\n## Request Body\n\n```json\n{}\n```\n\n## Candidate Response\n\n```json\n{}\n```\n\n## Final Response\n\n```json\n{}\n```",
                event.id,
                event.endpoint_name,
                event.model.as_deref().unwrap_or("<unknown>"),
                event.profile,
                event.status,
                event.retry_attempt,
                event.latency_ms,
                event.timestamp,
                verdict,
                pretty_json(&event.request_body),
                pretty_json_opt(&event.candidate_response),
                pretty_json_opt(&event.final_response)
            ))
        }
        "json" => Ok(serde_json::to_string_pretty(&find_request_event(
            root, request_id,
        )?)?),
        "jsonl" => Ok(serde_json::to_string(&find_request_event(
            root, request_id,
        )?)?),
        other =>            anyhow::bail!("unsupported export format: {other}; expected markdown, json, or jsonl. Why it matters: export reports should be readable and safe to share. Fix: use `--format markdown`, `--format json`, or `--format jsonl`.")
    }
}

fn find_request_event(root: &Path, request_id: &str) -> Result<crate::types::RequestEvent> {
    let path = root.join("logs/witness.jsonl");
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            anyhow::bail!("request id not found: {request_id}; audit log is empty or has not been created yet. Why it matters: replay/export needs a saved request event. Fix: send traffic through a watched endpoint, then run `the-witness logs` to confirm events are being written.")
        }
        Err(err) => {
            return Err(err).with_context(|| format!("could not read audit log {}", path.display()))
        }
    };
    let mut parse_errors = Vec::new();
    let mut found = None;
    for (idx, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<crate::types::RequestEvent>(line) {
            Ok(event) if event.id.to_string() == request_id => found = Some(event),
            Ok(_) => {}
            Err(err) => parse_errors.push(format!("line {}: {err}", idx + 1)),
        }
    }
    if let Some(event) = found {
        Ok(event)
    } else if parse_errors.is_empty() {
        anyhow::bail!("request id not found: {request_id}. Fix: run `the-witness logs`, copy an existing request ID, and try again.")
    } else {
        anyhow::bail!(
            "request id not found: {request_id}; some audit log lines could not be parsed and were ignored: {}",
            parse_errors.join("; ")
        )
    }
}

fn pretty_json(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

fn pretty_json_opt(value: &Option<serde_json::Value>) -> String {
    value
        .as_ref()
        .map(pretty_json)
        .unwrap_or_else(|| "null".to_string())
}
fn init(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir.join("src"))?;
    let cfg = WitnessConfig::default();
    cfg.save(&dir.join("witness.toml"))?;
    println!(
        "The Witness project config is ready at {}. Next: run `the-witness --config {}/witness.toml setup`, then `the-witness doctor`.",
        dir.join("witness.toml").display(),
        dir.display()
    );
    Ok(())
}
fn load_or_default(path: &Path) -> Result<WitnessConfig> {
    if path.exists() {
        WitnessConfig::load(path)
    } else {
        Ok(WitnessConfig::default())
    }
}
async fn doctor(path: &Path) -> Result<()> {
    let mut cfg = load_or_default(path)?;
    let root = path.parent().unwrap_or(Path::new("."));
    let report = setup::doctor::run_doctor(&cfg, root).await?;
    for l in report.lines {
        println!("{l}");
    }
    cfg.setup.last_doctor_check = chrono::Utc::now().to_rfc3339();
    cfg.save(path)?;
    if report.passed {
        Ok(())
    } else {
        anyhow::bail!("doctor found readiness failures")
    }
}
async fn start(path: &Path, addr: Option<SocketAddr>) -> Result<()> {
    let cfg = load_or_default(path)?;
    if !cfg.setup_ready() {
        println!("Setup needs attention before live endpoint watching. Opening the setup wizard now; choose demo mode if you want a local walkthrough first.");
        let cfg = setup::wizard::run_setup_wizard(path).await?;
        return App::new_with_path(cfg, path.to_path_buf()).run();
    }
    if let Some(addr) = addr {
        let judge: Arc<dyn crate::judge::gemma::GemmaJudge> = if cfg.gemma.backend == "demo" {
            Arc::new(DemoJudge)
        } else {
            Arc::new(OpenAiCompatibleJudge::new(cfg.gemma.clone()))
        };
        let logger = JsonlLogger::new(
            path.parent()
                .unwrap_or(Path::new("."))
                .join("logs/witness.jsonl"),
        );
        let state = crate::proxy::server::ProxyState {
            config: cfg,
            judge,
            logger,
            client: reqwest::Client::new(),
        };
        println!("Starting proxy at http://{addr}. Routes look like /<endpoint-name>/v1/chat/completions");
        crate::proxy::server::serve(addr, state).await
    } else {
        App::new_with_path(cfg, path.to_path_buf()).run()
    }
}
async fn model(path: &Path, cmd: ModelCommands) -> Result<()> {
    let mut cfg = load_or_default(path)?;
    let root = path.parent().unwrap_or(Path::new("."));
    match cmd {
        ModelCommands::List => {
            println!(
                "Configured model: backend={} model={} url={}",
                cfg.gemma.backend, cfg.gemma.model, cfg.gemma.url
            );
            println!("Selectable backends:");
            for choice in setup::backends::backend_choices() {
                println!("- {} ({}) — {}", choice.name, choice.id, choice.description);
            }
            println!("Configured model registry:");
            let registry = crate::models::registry::ModelRegistry::load_or_default(root)?;
            for m in registry.models {
                println!(
                    "- {}\t{}\t{}\t{}\tinstalled={}\t{}",
                    m.id, m.display_name, m.backend, m.source, m.installed, m.local_path
                );
            }
            Ok(())
        }
        ModelCommands::Install(mi) => {
            let backend = mi.backend.unwrap_or(cfg.gemma.backend);
            let model = mi.model.unwrap_or(cfg.gemma.model);
            let kind = setup::backends::BackendKind::from_config(&backend)
                .ok_or_else(|| anyhow::anyhow!("Unknown judge backend `{backend}`. Fix: choose one of ollama, llamacpp, litert, unsloth, manual, or demo."))?;
            let out =
                setup::installer::install_backend(kind, &model, Some(&cfg.gemma.url), true).await?;
            println!("{out}");
            Ok(())
        }
        ModelCommands::Download(dl) => {
            let requested_source = dl.source;
            let registry = crate::models::registry::ModelRegistry::load_or_default(root)?;
            let entry = registry
                .find(&dl.model)
                .context("model not found in models/models.toml")?
                .clone();
            if requested_source != entry.source
                && !(requested_source == "hf" && entry.source == "huggingface")
            {
                anyhow::bail!(
                    "Model `{}` is registered with source `{}`, not `{}`. Fix: run `the-witness model list`, then use the source shown for that model.",
                    dl.model,
                    entry.source,
                    requested_source
                );
            }
            let out = crate::models::installer::install_model(&entry, root).await?;
            let mut registry = registry;
            registry.mark_installed(&dl.model, true, "installed/downloaded");
            let registry_path = crate::models::registry::registry_path(root);
            registry.save(&registry_path)?;
            println!("{out}");
            Ok(())
        }
        ModelCommands::AddOllama(add) => {
            if add.model.trim().is_empty() {
                anyhow::bail!(
                    "Custom Ollama model name cannot be empty. Fix: pass `--model <ollama-tag>`."
                );
            }
            let registry_path = crate::models::registry::registry_path(root);
            let mut registry = crate::models::registry::ModelRegistry::load_or_default(root)?;
            let entry =
                registry.add_or_update_custom_ollama_model(&add.model, add.display_name.as_deref());
            registry.save(&registry_path)?;
            if add.set_default {
                cfg.gemma.backend = "ollama".into();
                cfg.gemma.model = entry.model.clone();
                cfg.gemma.url = if cfg.gemma.url.trim().is_empty() {
                    "http://localhost:11434".into()
                } else {
                    cfg.gemma.url.clone()
                };
                cfg.save(path)?;
            }
            if add.pull {
                let out = setup::installer::install_backend(
                    setup::backends::BackendKind::Ollama,
                    &entry.model,
                    Some(&cfg.gemma.url),
                    true,
                )
                .await?;
                println!("{out}");
            }
            println!(
                "Registered custom Ollama model `{}` in {}. Gemma 4 remains the primary recommended judge; use this custom model only when you intentionally select it.",
                entry.model,
                registry_path.display()
            );
            Ok(())
        }
        ModelCommands::Test(t) => {
            let mut gemma = cfg.gemma.clone();
            if let Some(backend) = t.backend {
                gemma.backend = backend;
            }
            if let Some(model) = t.model.or(t.model_path) {
                gemma.model = model;
            }
            if let Some(url) = t.url {
                gemma.url = url;
            }
            crate::models::health::run_model_sanity_test(gemma.clone()).await?;
            cfg.gemma = gemma;
            cfg.setup.model_test_passed = true;
            cfg.setup.judge_schema_test_passed = true;
            cfg.save(path)?;
            println!(
                "Model sanity test passed. Saved judge backend={} model={} url={}.",
                cfg.gemma.backend, cfg.gemma.model, cfg.gemma.url
            );
            Ok(())
        }
    }
}
async fn endpoint(path: &Path, cmd: EndpointCommands) -> Result<()> {
    let mut cfg = load_or_default(path)?;
    match cmd {
        EndpointCommands::Add(a) => {
            let name = a.name.unwrap_or_else(|| prompt("Endpoint name", "Codex"));
            let upstream = a
                .upstream
                .unwrap_or_else(|| prompt("Upstream URL", "https://api.openai.com/v1"));
            let local = a
                .local
                .unwrap_or_else(|| prompt("Local proxy URL", "http://localhost:8787/v1"));
            let strictness = parse_strictness(&a.strictness);
            let model = a.model.unwrap_or_else(|| "gpt-5.5".into());
            manager::add_endpoint(
                &mut cfg,
                EndpointConfig {
                    name,
                    enabled: true,
                    upstream_url: upstream,
                    local_proxy_url: local,
                    model,
                    profile: a.profile,
                    retry_limit: a.retry_limit,
                    strictness,
                    fallback_mode: FallbackMode::HumanReview,
                    auth_header: a.auth_header,
                    auth: None,
                    judge_backend: None,
                    judge_model: None,
                    timeout_seconds: 60,
                },
            )?;
            cfg.save(path)?;
            println!("Endpoint is now being watched. Send traffic to the local proxy URL to begin verification.");
            Ok(())
        }
        EndpointCommands::AddBlackbox => {
            if std::env::var("BLACKBOX_API_KEY").is_err() {
                anyhow::bail!("BLACKBOX_API_KEY is not set. The Blackbox endpoint uses this environment variable instead of storing your key. Fix: run `export BLACKBOX_API_KEY=\"YOUR_KEY_HERE\"`, then retry `the-witness endpoint add-blackbox`.");
            }
            manager::add_endpoint(&mut cfg, WitnessConfig::blackbox_endpoint())?;
            cfg.save(path)?;
            println!("Blackbox Grok Code is now being watched. The key stays in BLACKBOX_API_KEY; The Witness stores only the environment variable name.");
            Ok(())
        }
        EndpointCommands::List => {
            for e in &cfg.endpoints {
                println!(
                    "{}\t{}\t{}\t{}\t{}",
                    e.name,
                    e.upstream_url,
                    e.local_proxy_url,
                    e.profile,
                    if e.enabled { "enabled" } else { "disabled" }
                );
            }
            Ok(())
        }
        EndpointCommands::Test { name } => {
            let ep = cfg
                .endpoints
                .iter()
                .find(|e| e.name == name)
                .with_context(|| format!("Endpoint `{name}` was not found. Fix: run `the-witness endpoint list`, copy the endpoint name exactly, then retry."))?;
            crate::endpoints::health::test_endpoint(ep).await?;
            println!("Endpoint `{name}` is reachable. The next step is to send a test chat completion through its local proxy URL.");
            Ok(())
        }
        EndpointCommands::Disable { name } => {
            manager::set_enabled(&mut cfg, &name, false)?;
            cfg.save(path)?;
            println!("Endpoint `{name}` is disabled. Existing config is kept, but new traffic will not be watched until you enable it again.");
            Ok(())
        }
        EndpointCommands::Enable { name } => {
            manager::set_enabled(&mut cfg, &name, true)?;
            cfg.save(path)?;
            println!("Endpoint `{name}` is enabled. Send traffic to its local proxy URL to start verification.");
            Ok(())
        }
    }
}
fn prompt(label: &str, default: &str) -> String {
    use std::io::{self, Write};
    print!("{label} [{default}]: ");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    let s = s.trim();
    if s.is_empty() {
        default.into()
    } else {
        s.into()
    }
}
fn parse_strictness(s: &str) -> Strictness {
    match s {
        "relaxed" => Strictness::Relaxed,
        "high" => Strictness::High,
        "critical" => Strictness::Critical,
        _ => Strictness::Medium,
    }
}
