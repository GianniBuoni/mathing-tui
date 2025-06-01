use super::*;

async fn join_init_test(conn: &SqlitePool) -> Result<Vec<StoreJoinRow>> {
    let users = init_users(conn).await?;
    let items = intit_items(conn).await?;

    Ok(
        try_join_all(items.into_iter().enumerate().map(async |(index, r)| {
            anyhow::Ok::<StoreJoinRow>({
                let mut param = JoinedReceiptParams::new()
                    .item_id(r.0.id)
                    .item_qty(r.1)
                    .offset(0);

                match index {
                    0 => {
                        param = param.add_user(users.get(2).unwrap().id);
                    }
                    1 => {
                        param = param.add_user(users.get(1).unwrap().id);
                    }
                    2 => {
                        param = param
                            .add_user(users.get(1).unwrap().id)
                            .add_user(users.get(2).unwrap().id);
                    }
                    _ => {}
                }
                param.post(conn).await?
            })
        }))
        .await?,
    )
}

#[sqlx::test]
async fn test_join_post(conn: SqlitePool) -> Result<()> {
    join_init_test(&conn).await?;
    Ok(())
}
