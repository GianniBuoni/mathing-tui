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

#[test]
fn test_malformed_form_error() {
    let mut test_case = Form::builder();
    test_case.with_request_type(RequestType::Get);

    let mut test_case_1 = Form::builder();
    test_case_1
        .with_request_type(RequestType::Post)
        .with_form_type(AppArm::Items);

    let test_cases = [
        (Form::builder(), FormErrors::malformed("request type")),
        (test_case, FormErrors::malformed("form type")),
        (test_case_1, FormErrors::malformed("fields")),
    ];

    test_cases.into_iter().for_each(|(form, want)| {
        let res = form.build();

        if let Ok(unexpected) = &res {
            dbg!(unexpected);
            panic!("Expected an error");
        }

        if let Err(got) = &res {
            let got = got.to_string();
            let want = want.to_string();
            assert_eq!(want, got, "Test malformed form");
        }
    });
}
