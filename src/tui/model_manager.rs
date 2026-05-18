use crate::{
    models::registry::ModelRegistry,
    tui::{app::App, dashboard::*},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};
use std::path::Path;

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let root = app
        .config_path
        .as_ref()
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("/home/admin/Gemma/witness"));
    let registry =
        ModelRegistry::load_or_default(root).unwrap_or_else(|_| ModelRegistry::default_models());

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(72), Constraint::Percentage(28)])
        .split(area);

    let header = Row::new(vec!["Model", "Backend", "Source", "Status"])
        .style(Style::default().fg(WHITE).add_modifier(Modifier::BOLD));
    let rows = registry.models.into_iter().map(|m| {
        let status = if m.installed {
            "installed"
        } else if m.status.is_empty() {
            "not tested"
        } else {
            &m.status
        };
        Row::new(vec![
            Cell::from(m.display_name),
            Cell::from(m.backend),
            Cell::from(m.source),
            Cell::from(status.to_string()),
        ])
        .style(Style::default().fg(MUTED))
    });
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(44),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .column_spacing(1)
    .block(panel("Configured models", BLUE));
    f.render_widget(table, cols[0]);

    let links = vec![
        Line::from(Span::styled(
            "Hugging Face",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "ahmadalfakeh/witness-gemma4-e2b-judge",
            Style::default().fg(TEAL),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Colab notebook",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "drive/17-CgEQL...",
            Style::default().fg(AMBER),
        )),
        Line::from(""),
        Line::from(vec![badge("Ollama", BLUE), Span::raw(" local judge")]),
        Line::from(vec![badge("llama.cpp", PURPLE), Span::raw(" GGUF")]),
        Line::from(vec![badge("LiteRT", AMBER), Span::raw(" edge")]),
        Line::from(vec![badge("Manual", TEAL), Span::raw(" endpoint")]),
    ];
    f.render_widget(
        Paragraph::new(links)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Model links", PURPLE)),
        cols[1],
    );
}
