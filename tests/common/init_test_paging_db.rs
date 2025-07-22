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




    Ok(())
}
