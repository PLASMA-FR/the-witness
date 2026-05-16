use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Logs and Audit","JSONL audit contains request timelines, approved/rejected events, retry chains, prompt repairs, human overrides, endpoint/judge/setup errors. CLI supports export.".into()),area);
}
