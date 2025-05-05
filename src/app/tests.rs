use ratatui::crossterm::event::KeyCode;

use super::*;

#[test]
fn test_state_cycling() {
    let mut app = App::default();
    let key_event = event::KeyEvent::from(KeyCode::Tab);

    assert_eq!(
        app.current_model,
        CurrentModel::Items,
        "Test if current_view is properly initialized"
    );

    for i in 0..100 {
        let want = if i % 2 == 0 {
            CurrentModel::Receipt
        } else {
            CurrentModel::Items
        };

        app.handle_key_events(key_event);
        assert_eq!(
            app.current_model, want,
            "Test if current view changes with repeated input"
        );
    }
}

#[test]
fn test_view_data() {
    let mut app = App::default();
    let key_event = event::KeyEvent::from(KeyCode::Tab);

    assert!(
        app.models.get(&app.current_model).unwrap().is_active(),
        "Test if models are properly initialized, Items should be active"
    );

    assert!(
        !app.models.get(&CurrentModel::Receipt).unwrap().is_active(),
        "Expecting Receipt model to be inactive"
    );

    for i in 0..100 {
        app.handle_key_events(key_event);

        assert!(
            app.models.get(&app.current_model).unwrap().is_active(),
            "Repeat input: {i}. assert current_model is in sync with model's is_active method"
        );
    }
}

#[test]
fn test_model_order() {
    // Note this test does not 100% catch that it's sorted.
    // The underlying hashmap may or may not get collected
    // in the correct order the amount of times its iterated.
    // More iterations do not seem to make the test more likely
    // to catch issues

    let app = App::default();
    let desc = "Test models are displayed in the correct order";

    for _ in 0..100 {
        let models = app.list_models();
        assert_eq!(models[0].index(), 0, "{desc}");
        assert_eq!(models[1].index(), 1, "{desc}");
    }
}
