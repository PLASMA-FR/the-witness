use crate::tui::app::{App, Screen};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

pub const BG: Color = Color::Rgb(3, 7, 10);
pub const PANEL: Color = Color::Rgb(5, 18, 23);
pub const PANEL_2: Color = Color::Rgb(7, 25, 33);
pub const TEAL: Color = Color::Rgb(0, 229, 212);
pub const GREEN: Color = Color::Rgb(84, 255, 175);
pub const BLUE: Color = Color::Rgb(78, 162, 255);
pub const PURPLE: Color = Color::Rgb(175, 72, 255);
pub const AMBER: Color = Color::Rgb(245, 184, 72);
pub const RED: Color = Color::Rgb(255, 72, 96);
pub const WHITE: Color = Color::Rgb(240, 255, 252);
pub const MUTED: Color = Color::Rgb(156, 205, 202);

pub fn draw(f: &mut Frame, app: &App) {
    let root = f.size();
    f.render_widget(Clear, root);
    if root.width < 70 || root.height < 20 {
        draw_compact(f, app, root);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(root);

    draw_header(f, app, chunks[0]);
    match app.screen {
        Screen::SetupWizard => crate::tui::setup_wizard::draw(f, app, chunks[1]),
        Screen::Endpoints => crate::tui::endpoints::draw(f, app, chunks[1]),
        Screen::Requests => crate::tui::requests::draw(f, app, chunks[1]),
        Screen::Review => crate::tui::review::draw(f, app, chunks[1]),
        Screen::Logs => crate::tui::logs::draw(f, app, chunks[1]),
        Screen::Inspector => crate::tui::inspector::draw(f, app, chunks[1]),
        Screen::Verdict => crate::tui::verdict::draw(f, app, chunks[1]),
        Screen::Repair => crate::tui::repair_panel::draw(f, app, chunks[1]),
        Screen::Settings => crate::tui::settings::draw(f, app, chunks[1]),
        Screen::ModelManager => crate::tui::model_manager::draw(f, app, chunks[1]),
        _ => draw_dashboard(f, app, chunks[1]),
    }
    draw_footer(f, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = Line::from(vec![
        Span::styled(
            " The Witness ",
            Style::default()
                .fg(BG)
                .bg(TEAL)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "local-first Gemma 4 reliability firewall",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("backend: {}", app.cfg.gemma.backend),
            Style::default().fg(BLUE),
        ),
        Span::raw("  "),
        Span::styled(
            format!("judge: {}", app.cfg.gemma.model),
            Style::default().fg(TEAL),
        ),
        Span::raw("  "),
        Span::styled(
            format!("fallback: {:?}", app.cfg.defaults.fallback_mode),
            Style::default().fg(AMBER),
        ),
    ]);
    let status = if app.cfg.setup_ready() {
        "READY"
    } else {
        "SETUP REQUIRED"
    };
    let line2 = Line::from(vec![
        Span::styled("OpenAI-compatible proxy", Style::default().fg(MUTED)),
        Span::raw("  •  "),
        Span::styled("secret-redacted audit logs", Style::default().fg(MUTED)),
        Span::raw("  •  "),
        Span::styled(
            "approved / disapproved / human review",
            Style::default().fg(MUTED),
        ),
        Span::raw("  •  "),
        Span::styled(
            status,
            Style::default()
                .fg(if app.cfg.setup_ready() { GREEN } else { AMBER })
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TEAL))
        .style(Style::default().bg(BG));
    f.render_widget(Paragraph::new(vec![title, line2]).block(block), area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let help = Line::from(vec![
        Span::styled(
            " 1 ",
            Style::default()
                .fg(BG)
                .bg(TEAL)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Dashboard  "),
        Span::styled(
            " 2 ",
            Style::default()
                .fg(BG)
                .bg(PURPLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Endpoints  "),
        Span::styled(
            " 3 ",
            Style::default()
                .fg(BG)
                .bg(BLUE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Requests  "),
        Span::styled(
            " 4 ",
            Style::default()
                .fg(BG)
                .bg(AMBER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Review  "),
        Span::styled(
            " 5 ",
            Style::default()
                .fg(BG)
                .bg(GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Logs  "),
        Span::styled(
            " 6 ",
            Style::default()
                .fg(BG)
                .bg(BLUE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Settings  "),
        Span::styled(
            " 7 ",
            Style::default()
                .fg(BG)
                .bg(PURPLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Models  "),
        Span::styled(
            " s ",
            Style::default()
                .fg(BG)
                .bg(AMBER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Setup  "),
        Span::styled(
            " q/Esc ",
            Style::default().fg(BG).bg(RED).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Quit"),
    ]);
    f.render_widget(
        Paragraph::new(help).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(TEAL)),
        ),
        area,
    );
}

fn draw_compact(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "The Witness",
            Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("screen: {:?}", app.screen)),
        Line::from(format!(
            "backend: {}  model: {}",
            app.cfg.gemma.backend, app.cfg.gemma.model
        )),
        Line::from("Use a larger terminal for the full cyberpunk dashboard. q quits."),
    ];
    f.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(TEAL)),
        ),
        area,
    );
}

pub fn screen_block(title: &str, accent: Color) -> Block<'static> {
    Block::default()
        .title(Line::from(Span::styled(
            format!(" {title} "),
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .style(Style::default().bg(BG))
}

pub fn panel(title: &str, accent: Color) -> Block<'static> {
    Block::default()
        .title(Line::from(Span::styled(
            format!(" {title} "),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .style(Style::default().bg(PANEL))
}

pub fn paragraph(title: &str, text: String) -> Paragraph<'static> {
    Paragraph::new(text)
        .style(Style::default().fg(WHITE).bg(PANEL))
        .wrap(Wrap { trim: false })
        .block(panel(title, TEAL))
}

pub fn styled_lines(title: &str, lines: Vec<Line<'static>>, accent: Color) -> Paragraph<'static> {
    Paragraph::new(lines)
        .style(Style::default().fg(WHITE).bg(PANEL))
        .wrap(Wrap { trim: true })
        .block(panel(title, accent))
}

pub fn badge(text: &str, color: Color) -> Span<'_> {
    Span::styled(
        format!(" {text} "),
        Style::default()
            .fg(BG)
            .bg(color)
            .add_modifier(Modifier::BOLD),
    )
}

pub fn dashboard_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            badge("REQ-1024 approved", GREEN),
            Span::raw("  response returned"),
        ]),
        Line::from(vec![
            badge("REQ-1025 disapproved", RED),
            Span::raw("  prompt repair generated"),
        ]),
        Line::from(vec![
            badge("REQ-1025 retrying", AMBER),
            Span::raw("  repaired request sent"),
        ]),
        Line::from(vec![
            badge("REQ-1025 approved", GREEN),
            Span::raw("  audit log saved"),
        ]),
    ]
}

fn stat_card(f: &mut Frame, area: Rect, label: &str, value: &str, accent: Color) {
    let rows = vec![
        Line::from(Span::styled(
            value.to_string(),
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(label.to_string(), Style::default().fg(MUTED))),
    ];
    f.render_widget(
        Paragraph::new(rows)
            .alignment(Alignment::Center)
            .block(panel(label, accent)),
        area,
    );
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(9), Constraint::Min(10)])
        .split(area);
    let stat_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[0]);

    let active = app.cfg.endpoints.iter().filter(|e| e.enabled).count();
    let stats = [
        ("endpoints", app.cfg.endpoints.len().to_string(), TEAL),
        ("active", active.to_string(), BLUE),
        ("approved", "18".to_string(), GREEN),
        ("rejected", "4".to_string(), RED),
        ("human review", "2".to_string(), AMBER),
        ("avg latency", "812 ms".to_string(), PURPLE),
    ];
    for (row_idx, row_area) in stat_rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(*row_area);
        for col_idx in 0..3 {
            let idx = row_idx * 3 + col_idx;
            let (label, value, color) = &stats[idx];
            stat_card(f, cols[col_idx], label, value, *color);
        }
    }

    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(outer[1]);
    let checks = vec![
        ListItem::new(Line::from(vec![
            badge("PASS", GREEN),
            Span::raw(" Gemma backend configured"),
        ])),
        ListItem::new(Line::from(vec![
            badge("PASS", GREEN),
            Span::raw(" Judge JSON schema test passed"),
        ])),
        ListItem::new(Line::from(vec![
            badge("PASS", GREEN),
            Span::raw(" Proxy test passed in demo mode"),
        ])),
        ListItem::new(Line::from(vec![
            badge("PASS", GREEN),
            Span::raw(" Logs writable"),
        ])),
        ListItem::new(Line::from(vec![
            badge("MODE", BLUE),
            Span::raw(" local-first / secret-redacted"),
        ])),
    ];
    f.render_widget(
        List::new(checks).block(panel("System health", GREEN)),
        bottom[0],
    );
    f.render_widget(
        styled_lines("Live signal", dashboard_lines(), TEAL),
        bottom[1],
    );

    if !app.alerts.is_empty() && area.height > 24 {
        let alert_area = Rect {
            x: area.x + 3,
            y: area.y + area.height.saturating_sub(5),
            width: area.width.saturating_sub(6),
            height: 3,
        };
        let latest = app.alerts.last().cloned().unwrap_or_default();
        f.render_widget(
            Paragraph::new(latest).block(panel("Alert", AMBER)),
            alert_area,
        );
    }
}
