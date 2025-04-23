use super::*;

const TEST_ITEMS: [&str; 3] = ["Thing", "Noodle", "Jon"];

async fn init_test(conn: &SqlitePool) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut ids = vec![];
    for name in TEST_ITEMS {
        let new_user = add_store_user(conn, name).await?;
        ids.push(new_user.id);
    }
    Ok(ids)
}

#[sqlx::test]
async fn test_add_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let users = init_test(&conn).await;
    assert!(users.is_ok(), "Test if items were added to db");
    assert_eq!(
        users.unwrap().len(),
        TEST_ITEMS.len(),
        "Test if items added matches length of inputs"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_user_single(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let rows = init_test(&conn).await?;

    for (id, name) in rows.iter().zip(TEST_ITEMS) {
        let desc = "Test if added user matches inputs";
        let user = get_store_user_single(&conn, *id).await?;
        assert_eq!(user.name, name, "{desc}");
    }

    Ok(())
}

#[sqlx::test]
async fn test_get_users(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let users = get_store_users(&conn).await?;

    assert_eq!(
        users.len(),
        TEST_ITEMS.len(),
        "Test if returned users matches length of inputs"
    );

    assert_eq!(
        users[0].name, "Jon",
        "Test if returned users are alphabetical"
    );

    Ok(())
}

#[sqlx::test]
async fn test_delete_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let ids = init_test(&conn).await?;
    delete_store_user(&conn, ids[0]).await?;

    let users = get_store_users(&conn).await?;
    assert_ne!(users.len(), TEST_ITEMS.len(), "Test if user was deleted");

    Ok(())
}

#[sqlx::test]
async fn test_update_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let ids = init_test(&conn).await?;
    let update_params = [Some("Doodle"), None, None];

    for (id, name) in ids.iter().zip(update_params) {
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        update_store_user(&conn, *id, name).await?;
        let updated_user = get_store_user_single(&conn, *id).await?;
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
    }

    let updated_users = get_store_users(&conn).await?;
    assert_eq!(
        updated_users[0].name, "Doodle",
        "Test order of returned users updated"
    );

    Ok(())
}
