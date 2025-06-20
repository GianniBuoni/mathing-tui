use core::panic;

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

    let mut form = Form::test_valid();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));
    form.submit()?;

    Ok(())
}

#[test]
fn test_form_submit() -> Result<()> {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let want = ("a", 1.99 as f64);

    let mut form = Form::test_valid();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));
    form.submit()?;

    // check if from values changed
    let Some(DbPayloadBuilder::ItemParams(params)) = form.payload else {
        let panic_msg = "Test is not testing the expected kind of form.";
        panic!("{panic_msg}")
    };

    let desc = "Test if submitting with input values produces the correct resulting value.";
    let got_name = params.item_name.unwrap().unwrap();
    let got_price = params.item_price.unwrap().unwrap();

    assert_eq!(want.0, got_name, "{desc}");
    assert_eq!(want.1, got_price, "{desc}");

    Ok(())
}

// TODO test every step of the builder process.
#[test]
fn test_malformed_form_error() {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let mut form = Form::test_no_fields();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));

    let want = "Malformed: form has no fields.".to_string();
    let got = match form.submit() {
        Ok(_) => panic!("Expected an error!"),
        Err(e) => e.to_string(),
    };

    assert_eq!(want, got, "Test malformed form");
}
