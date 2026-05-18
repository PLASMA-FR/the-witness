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
    let req = vec![
        Line::from(vec![
            Span::styled("endpoint      ", Style::default().fg(MUTED)),
            Span::styled("Blackbox Grok Code", Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("upstream      ", Style::default().fg(MUTED)),
            Span::styled("https://api.blackbox.ai/v1", Style::default().fg(BLUE)),
        ]),
        Line::from(vec![
            Span::styled("local proxy   ", Style::default().fg(MUTED)),
            Span::styled("http://localhost:8787/v1", Style::default().fg(TEAL)),
        ]),
        Line::from(vec![
            Span::styled("method/path   ", Style::default().fg(MUTED)),
            Span::styled("POST /chat/completions", Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("headers       ", Style::default().fg(MUTED)),
            Span::styled("Authorization: Bearer ********", Style::default().fg(AMBER)),
        ]),
        Line::from(vec![
            Span::styled("model         ", Style::default().fg(MUTED)),
            Span::styled(
                "blackboxai/x-ai/grok-code-fast-1:free",
                Style::default().fg(WHITE),
            ),
        ]),
        Line::from(""),
        Line::from("User prompt: Write a Python script that prints Hello World"),
    ];
    f.render_widget(
        Paragraph::new(req)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Request Inspector", BLUE)),
        chunks[0],
    );
    let resp = vec![
        Line::from(Span::styled(
            "Candidate response",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled("print(Hello World)", Style::default().fg(RED))),
        Line::from(""),
        Line::from(Span::styled(
            "Final approved response",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "print(\"Hello World\")",
            Style::default().fg(GREEN),
        )),
        Line::from(""),
        Line::from(vec![
            badge("attempts: 2", AMBER),
            Span::raw("  "),
            badge("latency: 921ms", TEAL),
        ]),
    ];
    f.render_widget(
        Paragraph::new(resp)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Response Inspector", GREEN)),
        chunks[1],
    );
}
