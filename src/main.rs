use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::builder().build();
    let app_result = app.run().await;

    ratatui::restore();
    app_result
}
