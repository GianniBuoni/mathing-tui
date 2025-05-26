use super::*;

#[test]
fn test_form_validation() -> Result<()> {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let mut form = test_full_form();
    key_events.iter().for_each(|key| form.update(Some(*key)));

    let form = FormTui::ItemForm(form);
    form.validate()?;
    Ok(())
}

#[test]
fn test_invalid_data() {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
    ];

    let mut form = test_full_form();
    key_events.iter().for_each(|key| form.update(Some(*key)));

    let form = FormTui::ItemForm(form);
    match form.validate() {
        Ok(_) => panic!("expected an Error"),
        Err(got) => {
            let want = "Item price is not a float.";
            assert_eq!(want, AsRef::<str>::as_ref(&got.to_string()))
        }
    }
}

#[test]
fn test_unset_data() {
    let key_events = [
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
    ];

    let mut form = test_full_form();
    key_events.iter().for_each(|key| form.update(Some(*key)));
    let form = FormTui::ItemForm(form);

    match form.validate() {
        Ok(_) => panic!("expected an Error"),
        Err(got) => {
            let want = "Item Name is unset.";
            assert_eq!(want, AsRef::<str>::as_ref(&got.to_string()))
        }
    }
}
