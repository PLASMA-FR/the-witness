use crate::tui::{app::App, dashboard::*};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let endpoint = app
        .cfg
        .endpoints
        .first()
        .cloned()
        .unwrap_or_else(crate::config::WitnessConfig::blackbox_endpoint);

    let lines = vec![
        Line::from(Span::styled(
            endpoint.name,
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("upstream_url     ", Style::default().fg(MUTED)),
            Span::styled(endpoint.upstream_url, Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("local_proxy_url  ", Style::default().fg(MUTED)),
            Span::styled(endpoint.local_proxy_url, Style::default().fg(TEAL)),
        ]),
        Line::from(vec![
            Span::styled("auth             ", Style::default().fg(MUTED)),
            Span::styled("bearer_env BLACKBOX_API_KEY", Style::default().fg(AMBER)),
        ]),
        Line::from(vec![
            Span::styled("model            ", Style::default().fg(MUTED)),
            Span::styled(endpoint.model, Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("profile          ", Style::default().fg(MUTED)),
            Span::styled(endpoint.profile, Style::default().fg(BLUE)),
        ]),
        Line::from(vec![
            Span::styled("strictness       ", Style::default().fg(MUTED)),
            Span::styled(
                format!("{:?}", endpoint.strictness),
                Style::default().fg(RED),
            ),
        ]),
        Line::from(vec![
            Span::styled("retry_limit      ", Style::default().fg(MUTED)),
            Span::styled(endpoint.retry_limit.to_string(), Style::default().fg(AMBER)),
        ]),
        Line::from(vec![
            Span::styled("status           ", Style::default().fg(MUTED)),
            badge(
                if endpoint.enabled {
                    "watching"
                } else {
                    "disabled"
                },
                if endpoint.enabled { GREEN } else { RED },
            ),
        ]),
    ];
    f.render_widget(
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Endpoint configuration", PURPLE)),
        chunks[0],
    );

    let rows = app.cfg.endpoints.iter().map(|e| {
        Row::new(vec![
            Cell::from(e.name.clone()),
            Cell::from(e.profile.clone()),
            Cell::from(if e.enabled { "watching" } else { "disabled" }),
            Cell::from(e.retry_limit.to_string()),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(36),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
            Constraint::Percentage(16),
        ],
    )
    .header(
        Row::new(vec!["name", "profile", "status", "retry"])
            .style(Style::default().fg(WHITE).add_modifier(Modifier::BOLD)),
    )
    .block(panel("Watched endpoints", TEAL));

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(66), Constraint::Percentage(34)])
        .split(chunks[1]);
    f.render_widget(table, right[0]);
    let controls = vec![
        Line::from(vec![
            badge("Copy proxy URL", TEAL),
            Span::raw("  use in your AI app"),
        ]),
        Line::from(vec![
            badge("Test endpoint", BLUE),
            Span::raw("  upstream + auth + proxy"),
        ]),
        Line::from(vec![
            badge("Assign profile", PURPLE),
            Span::raw("  coding / health / education"),
        ]),
        Line::from(vec![
            badge("Fallback", AMBER),
            Span::raw("  human_review / safe_response"),
        ]),
    ];
    f.render_widget(
        Paragraph::new(controls).block(panel("Controls", TEAL)),
        right[1],
    );
}
