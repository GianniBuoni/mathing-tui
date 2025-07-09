use common::*;
use mathing_tui::prelude::*;

mod common;

#[tokio::test]
async fn test_handle_key_events() -> Result<()> {
    try_init_test_config().await?;

    Ok(())
}
