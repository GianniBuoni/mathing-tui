use super::*;

async fn init_test(
    conn: &SqlitePool,
) -> Result<Vec<StoreUser>, Box<dyn Error>> {
    let users = try_join_all(TEST_USERS.into_iter().map(async |name| {
        Ok::<StoreUser, Box<dyn Error>>(add_store_user(conn, name).await?)
    }))
    .await?;

    Ok(users)
}

#[sqlx::test]
async fn test_add_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    assert!(
        init_test(&conn).await.is_ok(),
        "Test if users were added to db"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_user_single(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        Ok::<(), Box<dyn Error>>({
            let got = get_store_user_single(&conn, want.id).await?;
            assert_eq!(want, got, "Test if id returns expected user.");
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_get_users(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let unordered = init_test(&conn).await?;
    let ordered = get_store_users(&conn).await?;

    assert_eq!(
        ordered.len(),
        unordered.len(),
        "Test if returned users matches length of inputs"
    );

    assert_eq!(
        ordered[0].name, "Jon",
        "Test if returned users are ordered alphabetically"
    );

    Ok(())
}

#[sqlx::test]
async fn test_delete_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let added_users = init_test(&conn).await?;
    delete_store_user(&conn, added_users[0].id).await?;

    let users = get_store_users(&conn)
        .await?
        .into_iter()
        .map(|user| user.name)
        .collect::<Vec<String>>();

    assert_ne!(users.len(), added_users.len(), "Test if user was deleted.");
    assert!(
        !users.contains(&"Thing".to_string()),
        "Test if expected user was deleted."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let update_params = [Some("Doodle"), None, None];

    try_join_all(init_test(&conn).await?.into_iter().zip(update_params).map(
        async |(user, name): (StoreUser, Option<&str>)| {
            Ok::<(), Box<dyn Error>>({
                sleep_until(Instant::now() + Duration::from_secs(1)).await;
                update_store_user(&conn, user.id, name).await?;

                let updated_user =
                    get_store_user_single(&conn, user.id).await?;
                let desc = "Test if item was updated.";

                match name {
                    Some(_) => {
                        assert_eq!(updated_user.name, name.unwrap(), "{desc}");
                        assert_ne!(
                            updated_user.created_at, updated_user.updated_at,
                            "{desc}",
                        );
                    }
                    None => {
                        assert_eq!(
                            updated_user.created_at, updated_user.updated_at,
                            "{desc}"
                        );
                    }
                }
            })
        },
    ))
    .await?;

    let updated_users = get_store_users(&conn).await?;
    assert_eq!(
        updated_users[0].name, "Doodle",
        "Test order of returned users updated"
    );

    Ok(())
}
