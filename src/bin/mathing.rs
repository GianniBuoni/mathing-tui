use clap::Parser;
use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    AppConfig::try_init(AppConfig::try_get_config_dir()?).await?;
    Args::parse();

    let app_result = App::new()?.run().await;
    ratatui::restore();
    app_result
}
