use the_witness::config::WitnessConfig;
use the_witness::tui::app::{App, Screen};

#[test]
fn settings_backend_shortcuts_update_config() {
    let mut app = App::new(WitnessConfig::default());
    app.screen = Screen::Settings;
    app.apply_backend_choice('d').unwrap();
    assert_eq!(app.cfg.gemma.backend, "demo");
    assert!(app.cfg.setup.demo_mode);
    assert!(app.cfg.setup.model_test_passed);

    app.apply_backend_choice('m').unwrap();
    assert_eq!(app.cfg.gemma.backend, "manual");
    assert_eq!(app.cfg.gemma.url, "http://localhost:8000/v1");
}
