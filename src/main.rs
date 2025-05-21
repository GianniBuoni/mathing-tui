use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    config_check()?;

    let tui = Tui::new().start();
    let mut app = App::default();

    let app_result = app.run(tui).await;

    ratatui::restore();
    app_result
}
