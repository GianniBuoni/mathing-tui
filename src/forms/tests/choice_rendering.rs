use super::*;

impl Choice<i64> {
    pub fn test_choice_1() -> Self {
        Self::new("Jon").with_value(1)
    }
    pub fn test_choice_2() -> Self {
        Self::new("Noodle").with_value(1)
    }
}

impl Choice<bool> {
    pub fn test_choice_yes() -> Self {
        Self::new("YES").with_value(true)
    }
    pub fn test_choice_no() -> Self {
        Self::new("CANCEL").with_value(false)
    }
}

#[test]
fn test_choices() {
    let mut test_choice = Choice::<i64>::test_choice_1();

    let test_cases = [
        (
            Line::from("  ◉︎ Jon").dark_gray().bold(),
            false,
            true,
            "Test inactive selected choice",
        ),
        (
            Line::from("  ○︎ Jon").dark_gray().bold(),
            false,
            false,
            "Test inactive unselected choice.",
        ),
        (
            Line::from("> ◉︎ Jon").green().bold(),
            true,
            true,
            "Test active selected choice",
        ),
        (
            Line::from("> ○︎ Jon").green().bold(),
            true,
            false,
            "Test active unselected choice.",
        ),
    ];

    test_cases
        .into_iter()
        .for_each(|(want, active, selected, desc)| {
            test_choice.active = active;
            test_choice.selected = selected;

            let mut got_buffer = Buffer::empty(test_input_rect());
            let got = test_choice.get_display();
            got.render(got_buffer.area, &mut got_buffer);

            let mut want_buffer = Buffer::empty(test_input_rect());
            want.render(want_buffer.area, &mut want_buffer);

            assert_eq!(want_buffer, got_buffer, "{desc}");
        });
}
