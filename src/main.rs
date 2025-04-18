use std::io;

use mathing_tui::prelude::*;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let mut app = App::default();
    let app_result = app.run(terminal);

    ratatui::restore();
    app_result
}
