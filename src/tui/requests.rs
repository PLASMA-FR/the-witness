use crate::tui::{app::App, dashboard::*};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Cell, Row, Table},
};

pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(10)])
        .split(area);
    let rows = [
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "received",
            "0",
            "--",
        ),
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "forwarded",
            "0",
            "118ms",
        ),
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "judging",
            "0",
            "601ms",
        ),
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "disapproved",
            "0",
            "733ms",
        ),
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "retrying",
            "1",
            "--",
        ),
        (
            "REQ-1024",
            "Blackbox",
            "grok-code",
            "coding",
            "approved",
            "1",
            "921ms",
        ),
    ];
    let table_rows = rows.into_iter().map(|r| {
        let color = match r.4 {
            "approved" => GREEN,
            "disapproved" => RED,
            "retrying" | "judging" => AMBER,
            _ => BLUE,
        };
        Row::new(vec![
            Cell::from(r.0),
            Cell::from(r.1),
            Cell::from(r.2),
            Cell::from(r.3),
            Cell::from(Line::from(badge(r.4, color))),
            Cell::from(r.5),
            Cell::from(r.6),
        ])
        .style(Style::default().fg(MUTED))
    });
    let table = Table::new(
        table_rows,
        [
            Constraint::Percentage(16),
            Constraint::Percentage(17),
            Constraint::Percentage(17),
            Constraint::Percentage(14),
            Constraint::Percentage(18),
            Constraint::Percentage(8),
            Constraint::Percentage(10),
        ],
    )
    .header(
        Row::new(vec![
            "request_id",
            "endpoint",
            "model",
            "profile",
            "status",
            "retry",
            "latency",
        ])
        .style(Style::default().fg(WHITE).add_modifier(Modifier::BOLD)),
    )
    .column_spacing(1)
    .block(panel("Requests through localhost:8787/v1", TEAL));
    f.render_widget(table, root[0]);
}
