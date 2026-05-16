use crate::{
    config::WitnessConfig,
    setup::backends::{backend_choices, detect_backend_health},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

pub fn settings_lines(cfg: &WitnessConfig) -> Vec<String> {
    let health = detect_backend_health(&cfg.gemma);
    let auth = if cfg.gemma.auth_header.is_some() {
        "configured/redacted"
    } else {
        "not configured"
    };
    let mut lines = vec![
        "Settings".to_string(),
        format!("Current backend: {}", cfg.gemma.backend),
        format!("Current model: {}", cfg.gemma.model),
        format!("Judge URL: {}", cfg.gemma.url),
        format!("Auth: {auth}"),
        format!("Runtime detected for current backend: {}", health.installed),
        format!("Reachable/configured URL: {}", health.reachable),
        "".to_string(),
            "Settings shortcuts: o=Ollama, l=llama.cpp, t=LiteRT, u=Unsloth, m=Manual endpoint, d=Demo judge".to_string(),
    ];
    for choice in backend_choices() {
        lines.push(format!("- {} ({})", choice.name, choice.id));
        lines.push(format!("  {}", choice.description));
        lines.push(format!("  Install: {}", choice.install_hint));
        lines.push(format!("  Test: {}", choice.test_hint));
    }
    lines.push("".to_string());
    lines.push("CLI controls:".to_string());
    lines.push("  the-witness setup".to_string());
    lines.push("  the-witness model install --backend <backend> --model <model>".to_string());
    lines.push("  the-witness model test".to_string());
    lines.push("  the-witness doctor".to_string());
    lines
}

pub fn draw(f: &mut Frame, app: &crate::tui::app::App, area: Rect) {
    let rows = settings_lines(&app.cfg)
        .into_iter()
        .map(ListItem::new)
        .collect::<Vec<_>>();
    f.render_widget(
        List::new(rows).block(
            Block::default()
                .title("Settings — Backend Choices")
                .borders(Borders::ALL),
        ),
        area,
    );
}
