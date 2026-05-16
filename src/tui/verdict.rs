use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Gemma 4 Verdict Panel","verdict, confidence, safety, usefulness, prompt alignment, correctness risk, rejection reason, suggested fix, improved prompt instruction, human review flag.".into()),area);
}
