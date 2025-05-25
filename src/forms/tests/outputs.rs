use super::*;

#[test]
fn test_form_validation() {
    let key_events = [
        KeyCode::Tab,
        KeyCode::Char('1'),
        KeyCode::Char('.'),
        KeyCode::Char('9'),
        KeyCode::Char('9'),
    ];

    let mut form = test_full_form();

    key_events.iter().for_each(|key| {
        form.handle_events(Some(Event::Key(KeyEvent::from(*key))));
    });

    let form = FormTui::ItemForm(form);
    assert!(form.validate().is_ok());
}
