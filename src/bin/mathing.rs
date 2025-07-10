use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    AppConfig::try_init(AppConfig::try_get_config_dir()?).await?;

    let mut app = App::new()?;
    let app_result = app.run().await;

    ratatui::restore();
    app_result
}
