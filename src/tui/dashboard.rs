use crate::tui::app::{App, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.size());
    let title = Paragraph::new("The Witness — local-first Gemma 4 reliability firewall")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
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
    let help = Paragraph::new(
        "1 Dashboard  2 Endpoints  3 Live Requests  4 Review  5 Logs  6 Settings  7 Models  s Setup  q Quit",
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let active = app.cfg.endpoints.iter().filter(|e| e.enabled).count();
    let items = vec![
        format!("Total watched endpoints: {}", app.cfg.endpoints.len()),
        format!("Active endpoints: {active}"),
        "Total requests today: 0".into(),
        "Approved responses: 0".into(),
        "Rejected responses: 0".into(),
        "Retry count: 0".into(),
        "Human review queue size: 0".into(),
        "Average latency: n/a".into(),
        format!("Gemma backend: {}", app.cfg.gemma.backend),
        format!("Gemma model: {}", app.cfg.gemma.model),
        format!(
            "Status: {}",
            if app.cfg.setup_ready() {
                "online/ready"
            } else {
                "setup required"
            }
        ),
    ];
    let list = List::new(items.into_iter().map(ListItem::new).collect::<Vec<_>>())
        .block(Block::default().title("Dashboard").borders(Borders::ALL));
    f.render_widget(list, area);
}
pub fn paragraph(title: &str, text: String) -> Paragraph<'static> {
    Paragraph::new(text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title(title.to_string())
            .borders(Borders::ALL),
    )
}
