use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use std::{env, path::PathBuf};

use mathing_tui::prelude::*;

/// Sets up the testing CONFIG static struct.
/// Assures that the test is not varaible based on
/// whether or not certain app env variables are set.
/// For keymap testing only; use [`try_init_test_db`]
/// for database integration tests.
pub async fn try_init_test_config() -> Result<()> {
    let config_dir =
        PathBuf::from_iter([env::var("PWD")?.as_str(), ".config", "mathing"]);
    AppConfig::try_init(config_dir).await?;

    Ok(())
}

pub async fn try_init_test_db(conn: &SqlitePool) -> Result<()> {
    let now = AppConfig::try_get_time()?;

    let mock_items = [
        (1, "PB Prezel", 4.99),
        (2, "Slamin' Salmon", 9.49),
        (3, "Chips and Dip", 5.55),
    ];

    let mock_users = [(1, "Thing"), (2, "Noodle"), (3, "Jon")];
    // (r_id, item_id, item_qty)
    let mock_r = [(1, 1, 2), (2, 2, 1), (3, 3, 3)];
    // (r_id, u_id)
    // add Jon to PB Pretzel
    // add Noodle to Salmon
    // add Noodle and Jon to Chips and Dip
    let mock_ru = [(1, 3), (2, 2), (3, 2), (3, 3)];

    // add in items
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO items (id, name, price, created_at, updated_at) ",
    );
    q.push_values(mock_items, |mut q, (id, name, price)| {
        q.push_bind(id)
            .push_bind(name)
            .push_bind(price)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add users
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO users (id, name, created_at, updated_at) ",
    );
    q.push_values(mock_users, |mut q, (id, name)| {
        q.push_bind(id)
            .push_bind(name)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add receits
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO receipts (id, item_id, item_qty, created_at, updated_at) ",
    );
    q.push_values(mock_r, |mut q, (r_id, item_id, item_qty)| {
        q.push_bind(r_id)
            .push_bind(item_id)
            .push_bind(item_qty)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add receipts users
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO receipts_users (
            receipt_id, user_id, created_at, updated_at
        ) ",
    );
    q.push_values(mock_ru, |mut q, (r_id, u_id)| {
        q.push_bind(r_id)
            .push_bind(u_id)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    Ok(())
}
