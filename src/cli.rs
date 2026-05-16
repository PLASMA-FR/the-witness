use crate::{
    config::{EndpointConfig, FallbackMode, Strictness, WitnessConfig},
    endpoints::manager,
    judge::gemma::{DemoJudge, OpenAiCompatibleJudge},
    setup,
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
    #[arg(long, default_value = "/home/admin/Gemma/witness/witness.toml")]
    pub config: PathBuf,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        path: PathBuf,
    },
    Setup,
    Doctor,
    Start {
        #[arg(long)]
        proxy_addr: Option<SocketAddr>,
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
pub enum ModelCommands {
    List,
    Install(ModelInstall),
    Download(ModelDownload),
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
pub struct ModelDownload {
    #[arg(long, default_value = "kaggle")]
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
    let path = cli.config;
    match cli.command.unwrap_or(Commands::Start { proxy_addr: None }) {
        Commands::Init { path: dir } => init(&dir),
        Commands::Setup => {
            setup::wizard::run_setup_wizard(&path).await?;
            Ok(())
        }
        Commands::Doctor => doctor(&path).await,
        Commands::Start { proxy_addr } => start(&path, proxy_addr).await,
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
            println!("Replay requested for {request_id}. MVP stores enough audit data for replay; full interactive replay is next.");
            Ok(())
        }
        Commands::Export { request_id, format } => {
            println!("Export {request_id} as {format}. Use JSONL log until rich exporter lands.");
            Ok(())
        }
    }
}
fn init(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir.join("src"))?;
    let cfg = WitnessConfig::default();
    cfg.save(&dir.join("witness.toml"))?;
    println!(
        "Initialized The Witness config at {}",
        dir.join("witness.toml").display()
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
    let report = setup::doctor::run_doctor(&cfg).await?;
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
        println!("Setup incomplete; opening setup wizard first.");
        let cfg = setup::wizard::run_setup_wizard(path).await?;
        return App::new(cfg).run();
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
    let cfg = load_or_default(path)?;
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
                .ok_or_else(|| anyhow::anyhow!("unknown backend: {backend}"))?;
            let out =
                setup::installer::install_backend(kind, &model, Some(&cfg.gemma.url), true).await?;
            println!("{out}");
            Ok(())
        }
        ModelCommands::Download(dl) => {
            if dl.source != "kaggle" {
                anyhow::bail!("only --source kaggle is supported for model download in the MVP");
            }
            let registry = crate::models::registry::ModelRegistry::load_or_default(root)?;
            let entry = registry
                .find(&dl.model)
                .context("model not found in models/models.toml")?
                .clone();
            let out = crate::models::installer::install_model(&entry, root).await?;
            let mut registry = registry;
            registry.mark_installed(&dl.model, true, "installed/downloaded");
            let registry_path = crate::models::registry::registry_path(root);
            registry.save(&registry_path)?;
            println!("{out}");
            Ok(())
        }
        ModelCommands::Test(t) => {
            let mut gemma = cfg.gemma;
            if let Some(backend) = t.backend {
                gemma.backend = backend;
            }
            if let Some(model) = t.model.or(t.model_path) {
                gemma.model = model;
            }
            if let Some(url) = t.url {
                gemma.url = url;
            }
            crate::models::health::run_model_sanity_test(gemma).await
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
            println!("Endpoint saved.");
            Ok(())
        }
        EndpointCommands::AddBlackbox => {
            if std::env::var("BLACKBOX_API_KEY").is_err() {
                anyhow::bail!("BLACKBOX_API_KEY is not set. Run: export BLACKBOX_API_KEY=\"...\" (do not put API keys in config files or logs)");
            }
            manager::add_endpoint(&mut cfg, WitnessConfig::blackbox_endpoint())?;
            cfg.save(path)?;
            println!("Blackbox Grok Code endpoint saved using auth env BLACKBOX_API_KEY.");
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
                .context("endpoint not found")?;
            crate::endpoints::health::test_endpoint(ep).await?;
            println!("Endpoint {name} reachable.");
            Ok(())
        }
        EndpointCommands::Disable { name } => {
            manager::set_enabled(&mut cfg, &name, false)?;
            cfg.save(path)?;
            println!("Endpoint {name} disabled.");
            Ok(())
        }
        EndpointCommands::Enable { name } => {
            manager::set_enabled(&mut cfg, &name, true)?;
            cfg.save(path)?;
            println!("Endpoint {name} enabled.");
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
