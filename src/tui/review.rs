use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Human Review Queue","Approve, reject, edit, retry, export report, mark unsafe, add note. Queue fed by NEEDS_HUMAN_REVIEW and fallback_mode=human_review.".into()),area);
}
