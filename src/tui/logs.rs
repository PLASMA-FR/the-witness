use crate::tui::{app::App, dashboard::*};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::List,
};

pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);
    let events = vec![
        Line::from(vec![
            Span::styled("12:03:01 ", Style::default().fg(MUTED)),
            badge("received", BLUE),
            Span::raw(" REQ-1024 request captured"),
        ]),
        Line::from(vec![
            Span::styled("12:03:01 ", Style::default().fg(MUTED)),
            badge("redacted", TEAL),
            Span::raw(" Authorization hidden"),
        ]),
        Line::from(vec![
            Span::styled("12:03:02 ", Style::default().fg(MUTED)),
            badge("candidate", PURPLE),
            Span::raw(" response captured"),
        ]),
        Line::from(vec![
            Span::styled("12:03:02 ", Style::default().fg(MUTED)),
            badge("DISAPPROVED", RED),
            Span::raw(" syntax error"),
        ]),
        Line::from(vec![
            Span::styled("12:03:02 ", Style::default().fg(MUTED)),
            badge("repair", AMBER),
            Span::raw(" prompt repair generated"),
        ]),
        Line::from(vec![
            Span::styled("12:03:03 ", Style::default().fg(MUTED)),
            badge("retry #1", AMBER),
            Span::raw(" forwarded"),
        ]),
        Line::from(vec![
            Span::styled("12:03:04 ", Style::default().fg(MUTED)),
            badge("APPROVED", GREEN),
            Span::raw(" final response returned"),
        ]),
        Line::from(vec![
            Span::styled("12:03:04 ", Style::default().fg(MUTED)),
            badge("saved", GREEN),
            Span::raw(" JSONL audit log written"),
        ]),
    ];
    f.render_widget(
        List::new(events).block(panel("Audit timeline", GREEN)),
        chunks[0],
    );
    let report = vec![
        Line::from(Span::styled(
            "Audit Report",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Request ID: REQ-1024"),
        Line::from("Endpoint: Blackbox Grok Code"),
        Line::from("Profile: coding"),
        Line::from("Attempts: 2"),
        Line::from("Attempt 1: DISAPPROVED"),
        Line::from("Repair: require valid quoted string"),
        Line::from("Attempt 2: APPROVED"),
        Line::from(""),
        Line::from("the-witness export REQ-1024 --format markdown"),
    ];
    f.render_widget(styled_lines("Markdown export", report, TEAL), chunks[1]);
}
