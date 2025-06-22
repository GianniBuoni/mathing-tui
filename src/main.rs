use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Config::get_config();
    let mut app = App::new()?;
    let app_result = app.run().await;

    ratatui::restore();
    app_result
}
