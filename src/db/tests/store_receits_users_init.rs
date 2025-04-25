use super::*;

pub async fn init_test(conn: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let mut r_ids = vec![];

    for ((name, price, qty), user) in TEST_ITEMS.into_iter().zip(TEST_USERS) {
        add_store_user(conn, user).await?;
        let item_row = add_store_item(conn, name, price).await?;
        let receipt_row = add_store_receipt(conn, item_row.id, qty).await?;
        r_ids.push(receipt_row.id);
    }

    let err = "Wrong id/user match; check how you add users into db.";
    let noodle = get_store_user_single(conn, 2).await?;
    if noodle.name != TEST_USERS[1] {
        return Err(err.into());
    }
    let jon = get_store_user_single(conn, 3).await?;
    if jon.name != TEST_USERS[2] {
        return Err(err.into());
    }

    try_join_all(r_ids.into_iter().map(async |r_id| {
        Ok::<(), Box<dyn Error>>({
            let uids = match r_id {
                1 => vec![jon.id],
                2 => vec![noodle.id],
                3 => vec![noodle.id, jon.id],
                _ => vec![],
            };
            try_join_all(uids.into_iter().map(async |u_id| {
                Ok::<(), Box<dyn Error>>({
                    add_store_receipts_users(conn, r_id, u_id).await?
                })
            }))
            .await?;
        })
    }))
    .await?;

    Ok(())
}
