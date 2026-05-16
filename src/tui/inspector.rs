use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Request / Response Inspector","Shows endpoint, upstream, local proxy, method, path, redacted headers, body, prompts, model, token estimate, candidate/final/rejected history and diffs.".into()),area);
}
