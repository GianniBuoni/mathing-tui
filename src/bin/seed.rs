use mathing_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let items = [
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

    let prices = [
        1.99, 2.49, 3.00, 4.75, 5.99, 6.49, 7.25, 8.99, 9.99, 10.50, 11.75,
        12.00, 13.99, 14.25, 15.50, 16.00, 17.95, 18.25, 19.99, 20.00, 21.50,
        22.99, 23.75, 24.99, 25.50, 26.00, 27.99, 28.25, 29.99, 30.00, 31.50,
        32.75, 33.99, 34.00, 35.99, 36.50, 37.75, 38.99, 39.00, 40.00,
    ];

    let names = ["Blue", "Noodle"];

    AppConfig::try_init(AppConfig::try_get_config_dir()?).await?;
    let conn = DbConn::try_get()?;

    for (name, price) in items.into_iter().zip(prices.into_iter()) {
        let mut params = ItemParams::builder();
        params
            .with_item_name(ParamOption::new().map_value(name).to_owned())
            .with_item_price(ParamOption::new().map_value(price).to_owned());
        params.build().post(conn).await?;
    }

    for name in names {
        let mut params = UserParams::builder();
        params.with_user_name(
            ParamOption::new().map_value(name.to_owned()).to_owned(),
        );
        params.build().post(conn).await?;
    }

    Ok(())
}
