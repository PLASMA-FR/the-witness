use crate::tui::{app::App, dashboard::*};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::Paragraph,
};

pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let original = vec![
        Line::from(Span::styled(
            "Original prompt",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Write a Python script that prints Hello World"),
        Line::from(""),
        Line::from(Span::styled(
            "Rejected response",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled("print(Hello World)", Style::default().fg(RED))),
        Line::from(""),
        Line::from(Span::styled(
            "Reason",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Python string is not quoted."),
    ];
    f.render_widget(
        Paragraph::new(original)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Rejected attempt", RED)),
        chunks[0],
    );
    let repaired = vec![
        Line::from(Span::styled(
            "Repaired prompt",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Original user request: Write a Python script that prints Hello World"),
        Line::from(""),
        Line::from("The previous answer was rejected by The Witness."),
        Line::from("Required fix: quote the Python string literal."),
        Line::from(""),
        Line::from("Now generate a corrected answer."),
        Line::from(""),
        Line::from(vec![
            badge("auto-generated", AMBER),
            Span::raw("  "),
            badge("retry 1", BLUE),
        ]),
    ];
    f.render_widget(
        Paragraph::new(repaired)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Prompt Repair", AMBER)),
        chunks[1],
    );
}
