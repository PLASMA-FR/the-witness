use crate::tui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};
pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let rows = app.cfg.endpoints.iter().map(|e| {
        Row::new(vec![
            e.name.clone(),
            e.upstream_url.clone(),
            e.local_proxy_url.clone(),
            e.profile.clone(),
            if e.enabled { "enabled" } else { "disabled" }.into(),
            e.retry_limit.to_string(),
            format!("{:?}", e.strictness),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(16),
            Constraint::Length(28),
            Constraint::Length(28),
            Constraint::Length(14),
            Constraint::Length(9),
            Constraint::Length(5),
            Constraint::Length(9),
        ],
    )
    .header(Row::new(vec![
        "name",
        "upstream",
        "local proxy",
        "profile",
        "status",
        "retry",
        "strict",
    ]))
    .block(
        Block::default()
            .title("Endpoint Watchlist — add/edit/test via CLI MVP")
            .borders(Borders::ALL),
    );
    f.render_widget(table, area);
}
