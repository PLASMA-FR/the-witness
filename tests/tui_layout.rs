use ratatui::{backend::TestBackend, Terminal};
use the_witness::{
    config::WitnessConfig,
    tui::{
        app::{App, Screen},
        dashboard,
    },
};

fn demo_app(screen: Screen) -> App {
    let mut cfg = WitnessConfig::default();
    cfg.gemma.backend = "demo".into();
    cfg.gemma.model = "demo-judge".into();
    cfg.gemma.setup_completed = true;
    cfg.setup.demo_mode = true;
    cfg.setup.judge_schema_test_passed = true;
    cfg.setup.proxy_test_passed = true;
    cfg.setup.model_test_passed = true;
    cfg.endpoints.push(WitnessConfig::blackbox_endpoint());
    let mut app = App::new(cfg);
    app.screen = screen;
    app
}

fn render_text(screen: Screen, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("test backend");
    let app = demo_app(screen);
    terminal
        .draw(|frame| dashboard::draw(frame, &app))
        .expect("draw TUI");
    terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn screenshot_style_dashboard_renders_core_markers() {
    let text = render_text(Screen::Dashboard, 120, 36);
    for marker in [
        "The Witness",
        "local-first Gemma 4 reliability firewall",
        "backend: demo",
        "System health",
        "Live signal",
        "REQ-1025 disapproved",
    ] {
        assert!(text.contains(marker), "missing marker {marker}\n{text}");
    }
}

#[test]
fn primary_screens_render_without_layout_panics_at_common_sizes() {
    let screens = [
        Screen::SetupWizard,
        Screen::Dashboard,
        Screen::Endpoints,
        Screen::Requests,
        Screen::Inspector,
        Screen::Verdict,
        Screen::Repair,
        Screen::Review,
        Screen::Logs,
        Screen::Settings,
        Screen::ModelManager,
    ];
    for screen in screens {
        for (w, h) in [(80, 24), (100, 30), (120, 36), (160, 44)] {
            let text = render_text(screen, w, h);
            assert!(
                text.contains("The Witness"),
                "screen {screen:?} missing shell at {w}x{h}"
            );
        }
    }
}

#[test]
fn endpoint_request_verdict_and_model_screens_keep_required_usage_text_visible() {
    let cases = [
        (
            Screen::Endpoints,
            [
                "Endpoint configuration",
                "local_proxy_url",
                "BLACKBOX_API_KEY",
            ],
        ),
        (
            Screen::Requests,
            ["Requests through localhost", "disapproved", "approved"],
        ),
        (
            Screen::Verdict,
            ["Gemma 4 Verdict", "DISAPPROVED", "Prompt Repair"],
        ),
        (
            Screen::ModelManager,
            ["Configured models", "Hugging Face", "witness-gemma4"],
        ),
    ];
    for (screen, markers) in cases {
        let text = render_text(screen, 140, 38);
        for marker in markers {
            assert!(
                text.contains(marker),
                "screen {screen:?} missing {marker}\n{text}"
            );
        }
    }
}

#[test]
fn compact_layout_has_explicit_small_terminal_fallback() {
    let text = render_text(Screen::Dashboard, 60, 16);
    assert!(text.contains("Use a larger terminal"));
    assert!(text.contains("The Witness"));
}
