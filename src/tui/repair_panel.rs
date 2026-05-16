use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Prompt Repair Panel","Original prompt, rejected response, rejection reason, suggested fix, repaired prompt, retry number; edit/regenerate/accept/cancel/send-to-review planned beyond CLI MVP.".into()),area);
}
