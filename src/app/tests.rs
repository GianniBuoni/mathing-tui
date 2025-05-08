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

fn base_buffer() -> Buffer {
    Buffer::with_lines(vec![
        "╭ [0] Grocery Items ─────────────────────────────╮",
        "│                                                │",
        "│   Items                  Price                 │",
        "│   Slamon                 9.49                  │",
        "│   Pretzels               5.59                  │",
        "│   Blueberries            4.59                  │",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ])
}

fn test_styles() -> (Style, Style, Style) {
    (
        Style::new()
            .fg(Color::Reset)
            .bg(Color::Reset)
            .underline_color(Color::Reset),
        Style::new()
            .fg(Color::Black)
            .bg(Color::Magenta)
            .underline_color(Color::Reset)
            .add_modifier(Modifier::BOLD),
        Style::new()
            .fg(Color::Red)
            .bg(Color::Reset)
            .underline_color(Color::Reset),
    )
}

#[test]
fn test_down_navigation_input() {
    let down_events = [
        (event::KeyEvent::from(KeyCode::Char('j')), "j"),
        (event::KeyEvent::from(KeyCode::Down), "Down"),
    ];

    down_events.iter().for_each(|(event, key)| {
        let mut app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 8));
        let mut want = base_buffer();

        want.set_style(Rect::new(0, 0, 50, 1), test_styles().0);
        want.set_style(Rect::new(3, 2, 44, 1), test_styles().1);
        want.set_style(Rect::new(3, 4, 44, 1), test_styles().2);

        app.handle_key_events(*event);
        app.models
            .get(&app.current_model)
            .unwrap()
            .render_ref(buf.area, &mut buf);

        assert_eq!(want, buf, "Test key code: {:?} changes app", key,);
    });
}

#[test]
fn test_up_navigation_input() {
    let up_events = [
        (event::KeyEvent::from(KeyCode::Char('k')), "k"),
        (event::KeyEvent::from(KeyCode::Up), "Up"),
    ];

    up_events.iter().for_each(|(event, key)| {
        let mut app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 8));
        let mut want = base_buffer();

        want.set_style(Rect::new(0, 0, 50, 1), test_styles().0);
        want.set_style(Rect::new(3, 2, 44, 1), test_styles().1);
        want.set_style(Rect::new(3, 5, 44, 1), test_styles().2);

        app.handle_key_events(*event);
        app.models
            .get(&app.current_model)
            .unwrap()
            .render_ref(buf.area, &mut buf);

        assert_eq!(want, buf, "Test key code: {:?} changes app", key,);
    });
}
