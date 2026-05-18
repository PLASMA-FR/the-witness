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
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(area);
    let verdict = vec![
        Line::from(vec![
            Span::styled("verdict:          ", Style::default().fg(MUTED)),
            badge("DISAPPROVED", RED),
        ]),
        Line::from(vec![
            Span::styled("confidence:       ", Style::default().fg(MUTED)),
            Span::styled("0.94", Style::default().fg(WHITE)),
        ]),
        Line::from(vec![
            Span::styled("safety_score:     ", Style::default().fg(MUTED)),
            Span::styled("92", Style::default().fg(GREEN)),
        ]),
        Line::from(vec![
            Span::styled("alignment_score:  ", Style::default().fg(MUTED)),
            Span::styled("71", Style::default().fg(AMBER)),
        ]),
        Line::from(vec![
            Span::styled("correctness_risk: ", Style::default().fg(MUTED)),
            Span::styled("high", Style::default().fg(RED)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "rejection_reason",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Python string is not quoted."),
        Line::from(""),
        Line::from(Span::styled(
            "suggested_fix",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Return valid Python syntax."),
    ];
    f.render_widget(
        Paragraph::new(verdict)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Gemma 4 Verdict", RED)),
        chunks[0],
    );
    let repair = vec![
        Line::from(Span::styled(
            "Prompt Repair Preview",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from("Original: Write a Python script that prints Hello World"),
        Line::from(Span::styled(
            "Rejected: print(Hello World)",
            Style::default().fg(RED),
        )),
        Line::from(""),
        Line::from("For the next answer:"),
        Line::from("- answer the original request directly"),
        Line::from("- fix the syntax error"),
        Line::from("- do not repeat the rejected mistake"),
        Line::from("- be accurate, safe, complete, and aligned"),
        Line::from(""),
        Line::from(vec![
            badge("retry #1", AMBER),
            Span::raw("  auto-generated repair  "),
            badge("approved after repair", GREEN),
        ]),
    ];
    f.render_widget(
        Paragraph::new(repair)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Prompt Repair", AMBER)),
        chunks[1],
    );
}
