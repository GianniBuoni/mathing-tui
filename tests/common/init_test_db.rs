use super::*;

pub const MOCK_ITEMS: [(i64, &str, f64); 3] = [
    (1, "PB Prezel", 4.99),
    (2, "Slamin' Salmon", 9.49),
    (3, "Chips and Dip", 5.55),
];

pub const MOCK_USERS: [(i64, &str); 3] =
    [(1, "Thing"), (2, "Noodle"), (3, "Jon")];

/// (r_id, item_id, item_qty)
pub const MOCK_RECEIPTS: [(i64, i64, i64); 3] =
    [(1, 1, 2), (2, 2, 1), (3, 3, 3)];

/// (r_id, u_id)
/// add Jon to PB Pretzel
/// add Noodle to Salmon
/// add Noodle and Jon to Chips and Dip
pub const MOCK_RU: [(i64, i64); 4] = [(1, 3), (2, 2), (3, 2), (3, 3)];

/// Sets up a testing Db for request and response testing.
pub async fn try_init_test_db(conn: &SqlitePool) -> Result<()> {
    let now = AppConfig::try_get_time()?;

    // add in items
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO items (id, name, price, created_at, updated_at) ",
    );
    q.push_values(MOCK_ITEMS, |mut q, (id, name, price)| {
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
    q.push_values(MOCK_USERS, |mut q, (id, name)| {
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
    q.push_values(MOCK_RECEIPTS, |mut q, (r_id, item_id, item_qty)| {
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
    q.push_values(MOCK_RU, |mut q, (r_id, u_id)| {
        q.push_bind(r_id)
            .push_bind(u_id)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    Ok(())
}
