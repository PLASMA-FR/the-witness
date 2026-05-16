use crate::tui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        "First Run Setup Wizard",
        "1 Welcome: The Witness is a local-first Gemma 4 reliability firewall for AI endpoints.",
        "2 Technology path: Quick Ollama, low-resource llama.cpp, edge LiteRT, Colab T4 GPU fine-tuned judge, or manual.",
        "3 Backend selection: Ollama local judge / llama.cpp local judge / LiteRT edge judge / Unsloth fine-tuned judge / Manual endpoint.",
        "4 Hardware check: OS, CPU arch, system RAM, GPU VRAM hints, disk, Ollama, llama.cpp, optional Kaggle CLI, free ports.",
        "5 Model picker: Gemma 4 E2B, Gemma 4 E4B, larger Gemma 4, fine-tuned Witness E2B/E4B, custom editable name/path.",
        "6 Install/pull/download: ollama pull, llama.cpp server URL, LiteRT path, Colab T4 GPU fine-tuned model path, manual URL/auth.",
        "7 Judge capability test: bad 2+2 must DISAPPROVE; good 2+2 must APPROVE; valid JSON schema required.",
        "8 Proxy test: temp route receives, forwards, captures, judges, retries/blocks, and writes JSONL logs.",
        "9 Endpoint test: upstream connectivity/auth/local proxy/real request/retry loop or demo endpoint.",
        "10 Final checklist: backend, model, schema, proxy, logs, endpoint or demo mode.",
        "",
        "Settings shortcuts: o Ollama | l llama.cpp | t LiteRT | u Unsloth | m Manual | d Demo. Model list: press 7.",
        if app.cfg.setup_ready() { "READY: dashboard may open" } else { "NOT READY: run `the-witness setup`, `the-witness model test`, or choose demo mode" },
    ];
    f.render_widget(
        List::new(lines.into_iter().map(ListItem::new).collect::<Vec<_>>())
            .block(Block::default().title("Setup Wizard").borders(Borders::ALL)),
        area,
    );
}
