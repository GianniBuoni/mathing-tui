use crate::test_cases::*;

use super::*;

fn test_component_cycling() {
    let mut test_home = test_home();
    let key_event = KeyEvent::from(KeyCode::Tab);

    assert_eq!(
        test_home.current_model, 0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 0 } else { 1 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
        test_home.update(action);
        assert_eq!(
            test_home.current_model, want,
            "Test if current view changes with repeated input"
        );
    }
}
