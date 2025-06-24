use std::io::Stdout;

use ratatui::{TerminalOptions, Viewport};

use super::*;

impl Dialogue {
    const TEST_WIDTH: u16 = 48;

    fn mock_error() -> Self {
        Self::error("This is an error")
    }
    fn mock() -> Self {
        let mut log = Self::builder();
        log.with_message("This is a message");
        log.build().unwrap()
    }
    fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 9)
    }
    fn test_term() -> Terminal<CrosstermBackend<Stdout>> {
        let viewport = Viewport::Fixed(Dialogue::test_rect());
        let backend = CrosstermBackend::new(std::io::stdout());
        Terminal::with_options(backend, TerminalOptions { viewport }).unwrap()
    }
}

#[test]
fn test_error_rendering() {
    let mut test_case = Dialogue::mock_error();
    test_case.rect.width = Dialogue::TEST_WIDTH;

    let mut frame = Dialogue::test_term();
    let mut frame = frame.get_frame();

    test_case.draw(&mut frame, Dialogue::test_rect());
    let got = frame.buffer_mut();

    let mut want = Buffer::with_lines([
        "                                                  ",
        "                                                  ",
        " ╭──────────────────────────────────────────────╮ ",
        " │                                              │ ",
        " │  This is an error                            │ ",
        " │                                              │ ",
        " ╰──────────────────────────────────────────────╯ ",
        "                                                  ",
        "                                                  ",
    ]);
    want.set_style(Rect::new(4, 4, 42, 1), Style::default().red());

    assert_eq!(want, *got, "Test error message rendering");
}

#[test]
fn test_rendering() {
    let mut test_case = Dialogue::mock();
    test_case.rect.width = Dialogue::TEST_WIDTH;

    let mut frame = Dialogue::test_term();
    let mut frame = frame.get_frame();

    test_case.draw(&mut frame, Dialogue::test_rect());
    let got = frame.buffer_mut();

    let want = Buffer::with_lines([
        "                                                  ",
        "                                                  ",
        " ╭──────────────────────────────────────────────╮ ",
        " │                                              │ ",
        " │  This is a message                           │ ",
        " │                                              │ ",
        " ╰──────────────────────────────────────────────╯ ",
        "                                                  ",
        "                                                  ",
    ]);

    assert_eq!(want, *got, "Test dialogue message rendering");
}
