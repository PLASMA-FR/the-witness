use crate::tui::{app::App, dashboard::paragraph};
use ratatui::prelude::*;
pub fn draw(f: &mut Frame, _: &App, area: Rect) {
    f.render_widget(paragraph("Live Request Stream","Rows: request id, endpoint, model, profile, pending/forwarded/judging/approved/disapproved/retrying/human_review/failed, retry, latency, timestamp. JSONL logs update as proxy traffic flows.".into()),area);
}
