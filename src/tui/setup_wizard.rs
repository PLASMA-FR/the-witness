use crate::tui::{app::App, dashboard::*};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Gauge, List, ListItem, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(area);
    let steps = [
        ("Welcome", "what The Witness does", true),
        (
            "Backend",
            "Ollama / llama.cpp / LiteRT / Unsloth / manual",
            true,
        ),
        ("Hardware", "OS, RAM, disk, GPU, ports", true),
        ("Model", "Gemma 4 E2B/E4B/custom", true),
        (
            "Install",
            "pull/download/configure selected judge",
            app.cfg.setup.model_test_passed,
        ),
        (
            "Judge test",
            "bad 2+2 rejects; good 2+2 approves",
            app.cfg.setup.judge_schema_test_passed,
        ),
        (
            "Proxy test",
            "receive, forward, judge, retry, log",
            app.cfg.setup.proxy_test_passed,
        ),
        (
            "Endpoint",
            "real endpoint or demo mode",
            app.cfg.setup.demo_mode || !app.cfg.endpoints.is_empty(),
        ),
    ];
    let items = steps.iter().map(|(name, desc, pass)| {
        let state = if *pass {
            badge("PASS", GREEN)
        } else {
            badge("TODO", AMBER)
        };
        ListItem::new(Line::from(vec![
            state,
            Span::raw(format!(" {name:<12} {desc}")),
        ]))
    });
    f.render_widget(
        List::new(items).block(panel("First-run setup wizard", AMBER)),
        chunks[0],
    );
    let readiness = if app.cfg.setup_ready() { 100 } else { 62 };
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(10)])
        .split(chunks[1]);
    f.render_widget(
        Gauge::default()
            .block(panel("Readiness gate", TEAL))
            .gauge_style(
                Style::default()
                    .fg(if app.cfg.setup_ready() { GREEN } else { AMBER })
                    .bg(PANEL_2),
            )
            .percent(readiness),
        right[0],
    );
    let text = vec![
        Line::from(Span::styled(
            "The dashboard opens only after setup passes or demo mode is selected.",
            Style::default().fg(WHITE),
        )),
        Line::from(""),
        Line::from(vec![
            badge("o", BLUE),
            Span::raw(" Ollama  "),
            badge("l", PURPLE),
            Span::raw(" llama.cpp"),
        ]),
        Line::from(vec![
            badge("t", AMBER),
            Span::raw(" LiteRT  "),
            badge("u", GREEN),
            Span::raw(" Unsloth"),
        ]),
        Line::from(vec![
            badge("m", TEAL),
            Span::raw(" Manual  "),
            badge("d", GREEN),
            Span::raw(" Demo mode"),
        ]),
        Line::from(""),
        Line::from("Run `the-witness doctor` for non-interactive health checks."),
    ];
    f.render_widget(
        Paragraph::new(text)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(panel("Setup controls", TEAL)),
        right[1],
    );
}
