use crate::{models::registry::ModelRegistry, tui::app::App};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use std::path::Path;

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let root = app
        .config_path
        .as_ref()
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("/home/admin/Gemma/witness"));
    let registry =
        ModelRegistry::load_or_default(root).unwrap_or_else(|_| ModelRegistry::default_models());
    let mut lines = vec![
        "Model Manager — install/pull/download/test/set default/delete/open card/copy path/refresh".to_string(),
        "Columns: display name | backend | base model | source | installed | local path | status".to_string(),
        "Actions via CLI: model list, model install, model test; optional Kaggle artifact download with model download --source kaggle".to_string(),
        "".to_string(),
    ];
    for m in registry.models {
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {}",
            m.display_name,
            m.backend,
            if m.base_model.is_empty() {
                "-"
            } else {
                &m.base_model
            },
            m.source,
            m.installed,
            if m.local_path.is_empty() {
                "-"
            } else {
                &m.local_path
            },
            if m.status.is_empty() {
                "not tested"
            } else {
                &m.status
            }
        ));
    }
    f.render_widget(
        List::new(lines.into_iter().map(ListItem::new).collect::<Vec<_>>()).block(
            Block::default()
                .title("Model Manager")
                .borders(Borders::ALL),
        ),
        area,
    );
}
