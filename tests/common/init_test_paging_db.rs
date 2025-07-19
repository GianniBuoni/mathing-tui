use std::rc::Rc;

use super::*;

const PAGING_ITEMS: [&str; 40] = [
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew",
    "Kiwi",
    "Lemon",
    "Mango",
    "Nectarine",
    "Orange",
    "Papaya",
    "Pear",
    "Pineapple",
    "Plum",
    "Pomegranate",
    "Raspberry",
    "Strawberry",
    "Tangerine",
    "Watermelon",
    "Apricot",
    "Blueberry",
    "Cantaloupe",
    "Guava",
    "Lychee",
    "Dragonfruit",
    "Coconut",
    "Persimmon",
    "Passionfruit",
    "Cranberry",
    "Jackfruit",
    "Starfruit",
    "Soursop",
    "Mulberry",
    "Açaí",
    "Chayote",
    "Clementine",
    "Gooseberry",
];

const PAGING_PRICES: [f64; 40] = [
    1.99, 2.49, 3.00, 4.75, 5.99, 6.49, 7.25, 8.99, 9.99, 10.50, 11.75, 12.00,
    13.99, 14.25, 15.50, 16.00, 17.95, 18.25, 19.99, 20.00, 21.50, 22.99,
    23.75, 24.99, 25.50, 26.00, 27.99, 28.25, 29.99, 30.00, 31.50, 32.75,
    33.99, 34.00, 35.99, 36.50, 37.75, 38.99, 39.00, 40.00,
];

const PAGING_USERS: [&str; 2] = ["Blue", "Noodle"];

/// (recipt_id, item_id, item_qty)
const PAGING_RECEIPT: [(i64, i64, i64); 3] =
    [(1, 15, 2), (2, 4, 1), (3, 40, 1)];

/// (receipt_id, user_id)
const PAGING_RU: [(i64, i64); 4] = [(1, 1), (1, 2), (2, 1), (3, 2)];

pub async fn try_init_paging_db(conn: &SqlitePool) -> Result<()> {
    let now = AppConfig::try_get_time()?;
    let items = PAGING_ITEMS
        .into_iter()
        .zip(PAGING_PRICES)
        .collect::<Rc<[(&str, f64)]>>();

    // add items
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO items (name, price, created_at, updated_at) ",
    );
    q.push_values(items.iter(), |mut q, (name, price)| {
        q.push_bind(name)
            .push_bind(price)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add users
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO users (name, created_at, updated_at) ",
    );
    q.push_values(PAGING_USERS, |mut q, name| {
        q.push_bind(name).push_bind(now).push_bind(now);
    });
    q.build().execute(conn).await?;

    // add receipts
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO receipts (id, item_id, item_qty, created_at, updated_at) ",
    );
    q.push_values(PAGING_RECEIPT, |mut q, (r_id, item_id, item_qty)| {
        q.push_bind(r_id)
            .push_bind(item_id)
            .push_bind(item_qty)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add receipts_users
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO receipts_users (
            receipt_id, user_id, created_at, updated_at
        ) ",
    );
    q.push_values(PAGING_RU, |mut q, (r_id, u_id)| {
        q.push_bind(r_id)
            .push_bind(u_id)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    Ok(())
}
