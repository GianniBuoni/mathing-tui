use sqlx::query_as;

use super::*;

pub struct StoreItem {
    name: String,
    id: i64,
    price: f64,
}

impl StoreItem {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn price(&self) -> f64 {
        self.price
    }
}

pub async fn get_items() -> Result<Vec<StoreItem>, Box<dyn Error>> {
    let db = get_db().await?;

    let rows = query_as!(StoreItem, "SELECT * FROM items ORDER BY name")
        .fetch_all(db)
        .await?;

    Ok(rows)
}

pub async fn add_items(
    name: &str,
    price: f64,
) -> Result<StoreItem, Box<dyn Error>> {
    let db = get_db().await?;

    let new_item = query_as!(
        StoreItem,
        "INSERT INTO items (name, price) VALUES (?1, ?2) RETURNING *",
        name,
        price
    )
    .fetch_one(db)
    .await?;

    Ok(new_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO write a better way of connecting to the DB?
    // TODO right now tests arent running against a test database.
    #[sqlx::test]
    async fn test_add_and_get_items() -> Result<(), Box<dyn Error>> {
        let db = get_db().await?;
        sqlx::query!("DELETE from items").execute(db).await?;

        let test_items = [
            ("PB Pretzel", 4.99),
            ("Slamin' Salmon", 9.49),
            ("Chips and Dip", 5.55),
        ];

        for item in test_items {
            let test_item = add_items(item.0, item.1).await?;
            assert_eq!(item.0, test_item.name(), "test new item's name match");
            assert_eq!(
                item.1,
                test_item.price(),
                "test new item's price match"
            );
        }

        let test_fetch = get_items().await?;
        assert_eq!(
            test_items.len(),
            test_fetch.len(),
            "test row count and amount items added match"
        );
        assert_eq!(
            "Chips and Dip",
            test_fetch[0].name(),
            "test db returning in alphabetical order"
        );

        Ok(())
    }
}
