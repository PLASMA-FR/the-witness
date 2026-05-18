use crate::{
    config::WitnessConfig,
    setup::backends::{backend_choices, detect_backend_health},
    tui::dashboard::*,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{List, ListItem, Paragraph},
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
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(area);
    let health = detect_backend_health(&app.cfg.gemma);
    let summary = vec![
        Line::from(vec![
            Span::styled("backend          ", Style::default().fg(MUTED)),
            Span::styled(app.cfg.gemma.backend.clone(), Style::default().fg(TEAL)),
        ]),
        Line::from(vec![
            Span::styled("model            ", Style::default().fg(MUTED)),
            Span::styled(app.cfg.gemma.model.clone(), Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("judge_url        ", Style::default().fg(MUTED)),
            Span::styled(app.cfg.gemma.url.clone(), Style::default().fg(BLUE)),
        ]),
        Line::from(vec![
            Span::styled("runtime          ", Style::default().fg(MUTED)),
            badge(
                if health.installed {
                    "detected"
                } else {
                    "missing"
                },
                if health.installed { GREEN } else { AMBER },
            ),
        ]),
        Line::from(vec![
            Span::styled("auth             ", Style::default().fg(MUTED)),
            Span::styled(
                "configured/redacted or env-only",
                Style::default().fg(AMBER),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            badge("o", BLUE),
            Span::raw(" Ollama   "),
            badge("l", PURPLE),
            Span::raw(" llama.cpp"),
        ]),
        Line::from(vec![
            badge("t", AMBER),
            Span::raw(" LiteRT   "),
            badge("u", GREEN),
            Span::raw(" Unsloth"),
        ]),
        Line::from(vec![
            badge("m", TEAL),
            Span::raw(" Manual   "),
            badge("d", GREEN),
            Span::raw(" Demo judge"),
        ]),
    ];
    f.render_widget(
        Paragraph::new(summary)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Settings", BLUE)),
        chunks[0],
    );

    let rows = settings_lines(&app.cfg)
        .into_iter()
        .map(|line| ListItem::new(line).style(Style::default().fg(MUTED)))
        .collect::<Vec<_>>();
    f.render_widget(
        List::new(rows).block(panel("Backend Choices", TEAL)),
        chunks[1],
    );
}
