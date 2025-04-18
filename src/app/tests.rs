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
