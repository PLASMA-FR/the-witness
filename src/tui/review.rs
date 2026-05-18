use crate::tui::{app::App, dashboard::*};
use ratatui::{prelude::*, widgets::Paragraph};

pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    let lines = vec![
        Line::from(vec![
            badge("REQ-2011", AMBER),
            Span::raw("  Profile: Health & Sciences"),
        ]),
        Line::from(vec![
            Span::styled("Risk: ", Style::default().fg(MUTED)),
            Span::styled("high", Style::default().fg(RED)),
        ]),
        Line::from("Reason: medical-style answer requires uncertainty and source caution."),
        Line::from(""),
        Line::from(vec![
            badge("Approve", GREEN),
            Span::raw("  "),
            badge("Reject", RED),
            Span::raw("  "),
            badge("Edit", BLUE),
            Span::raw("  "),
            badge("Regenerate", AMBER),
        ]),
        Line::from(""),
        Line::from("Human review pauses risky responses before they reach users."),
    ];
    f.render_widget(
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Human Review Queue", AMBER)),
        area,
    );
}
