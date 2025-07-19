use common::prelude::*;

mod common;

#[sqlx::test]
fn test_post_paging(conn: SqlitePool) -> Result<()> {
    try_init_paging_db(&conn).await?;

    Ok(())
}
