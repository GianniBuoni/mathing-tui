use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let tui = Tui::new().build();
    let mut app = App::default();

    let app_result = app.run(tui).await;

    ratatui::restore();
    app_result
}
