use crate::config::WitnessConfig;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, path::PathBuf, time::Duration};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    SetupWizard,
    Dashboard,
    Endpoints,
    Requests,
    Inspector,
    Verdict,
    Repair,
    Review,
    Profiles,
    Logs,
    Settings,
    ModelManager,
}
pub struct App {
    pub cfg: WitnessConfig,
    pub screen: Screen,
    pub alerts: Vec<String>,
    pub config_path: Option<PathBuf>,
}
impl App {
    pub fn new(cfg: WitnessConfig) -> Self {
        let screen = if cfg.setup_ready() {
            Screen::Dashboard
        } else {
            Screen::SetupWizard
        };
        Self {
            cfg,
            screen,
            alerts: vec![],
            config_path: None,
        }
    }
    pub fn new_with_path(cfg: WitnessConfig, path: PathBuf) -> Self {
        let mut app = Self::new(cfg);
        app.config_path = Some(path);
        app
    }
    pub fn apply_backend_choice(&mut self, key: char) -> Result<()> {
        let (backend, model, url) = match key {
            'o' => ("ollama", "gemma4:e2b", "http://localhost:11434"),
            'l' => ("llama.cpp", "gemma4.gguf", "http://localhost:8080/v1"),
            't' => ("litert", "/path/to/judge.tflite", ""),
            'u' => (
                "unsloth",
                "witness-gemma4-e2b-judge",
                "http://localhost:8000/v1",
            ),
            'm' => ("manual", "local-gemma-judge", "http://localhost:8000/v1"),
            'd' => ("demo", "demo-judge", "http://localhost:11434"),
            _ => return Ok(()),
        };
        self.cfg.gemma.backend = backend.into();
        self.cfg.gemma.model = model.into();
        self.cfg.gemma.url = url.into();
        let demo = backend == "demo";
        self.cfg.setup.demo_mode = demo;
        self.cfg.gemma.setup_completed = demo;
        if demo {
            self.cfg.setup.judge_schema_test_passed = true;
            self.cfg.setup.proxy_test_passed = true;
            self.cfg.setup.model_test_passed = true;
        } else {
            self.cfg.setup.judge_schema_test_passed = false;
            self.cfg.setup.proxy_test_passed = false;
            self.cfg.setup.model_test_passed = false;
            self.cfg.gemma.setup_completed = false;
        }
        self.alerts.push(format!(
            "Selected backend: {backend}. Run setup/model test to verify."
        ));
        if let Some(path) = &self.config_path {
            self.cfg.save(path)?;
        }
        Ok(())
    }
    pub fn run(&mut self) -> Result<()> {
        if !std::io::stdout().is_terminal() {
            println!("The Witness TUI requires a TTY. Use `the-witness doctor` for non-interactive health checks.");
            return Ok(());
        }
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let res = self.loop_ui(&mut terminal);
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        res
    }
    fn loop_ui<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| crate::tui::dashboard::draw(f, self))?;
            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(k) = event::read()? {
                    match k.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('1') => self.screen = Screen::Dashboard,
                        KeyCode::Char('2') => self.screen = Screen::Endpoints,
                        KeyCode::Char('3') => self.screen = Screen::Requests,
                        KeyCode::Char('4') => self.screen = Screen::Review,
                        KeyCode::Char('5') => self.screen = Screen::Logs,
                        KeyCode::Char('6') => self.screen = Screen::Settings,
                        KeyCode::Char('7') => self.screen = Screen::ModelManager,
                        KeyCode::Char(c @ ('o' | 'l' | 't' | 'u' | 'm' | 'd'))
                            if self.screen == Screen::Settings =>
                        {
                            self.apply_backend_choice(c)?;
                        }
                        KeyCode::Char('s') => self.screen = Screen::SetupWizard,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
use std::io::IsTerminal;
