use super::*;
use crate::prelude::*;

async fn init_test(conn: &SqlitePool) -> Result<Vec<StoreUser>> {
    Ok(try_join_all(TEST_USERS.into_iter().map(async |user_name| {
        Ok::<StoreUser, Error>({
            let param = UserParams::new().user_name(user_name);
            let user = param.post(conn).await?;
            user
        })
    }))
    .await?
    .into_iter()
    .collect())
}

#[sqlx::test]
async fn test_add_users(conn: SqlitePool) -> Result<()> {
    let got = init_test(&conn).await?;
    got.into_iter().zip(TEST_USERS).for_each(|(got, want)| {
        assert_eq!(want, got.name, "Test adding users.")
    });

    Ok(())
}

#[sqlx::test]
async fn test_get_users(conn: SqlitePool) -> Result<()> {
    let unordered = init_test(&conn).await?;
    let ordered = get_store_users(&conn).await?;

    assert_eq!(
        ordered.len(),
        unordered.len(),
        "Test if returned users matches length of inputs"
    );

    assert_eq!(
        ordered.get(0).unwrap().name,
        "Jon".to_string(),
        "Test if returned users are ordered alphabetically"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_user(conn: SqlitePool) -> Result<()> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        anyhow::Ok::<()>({
            let got = UserParams::new().user_id(want.id).get(&conn).await?;
            assert_eq!(
                want.name, got.name,
                "Test getting user matches expected"
            );
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_user(conn: SqlitePool) -> Result<()> {
    let original = init_test(&conn).await?;
    let params = UserParams::new().user_id(original.get(0).unwrap().id);

    params.delete(&conn).await?;

    let finals = get_store_users(&conn)
        .await?
        .into_iter()
        .map(|user| user.name)
        .collect::<Vec<String>>();

    assert_ne!(original.len(), finals.len(), "Test if user was deleted.");
    assert!(
        !finals.contains(&"Thing".to_string()),
        "Test if expected user was deleted."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_user(conn: SqlitePool) -> Result<()> {
    let users = init_test(&conn).await?;
    let want = ["Doodle", "Schmoodle", "Floofus"];

    let params = users
        .iter()
        .zip(want)
        .map(|(user, name)| UserParams::new().user_id(user.id).user_name(name))
        .collect::<Vec<UserParams>>();

    let got = try_join_all(params.into_iter().map(async |param| {
        Ok::<StoreUser, Error>({
            sleep_until(Instant::now() + Duration::from_secs(1)).await;
            param.update(&conn).await?
        })
    }))
    .await?
    .into_iter()
    .map(|user| (user.name, user.updated_at))
    .collect::<Vec<(String, i64)>>();

    want.iter().zip(users).zip(got).for_each(
        |((want, original), (got, got_time))| {
            assert_eq!(
                want.to_string(),
                got,
                "Test if updated user matches expected."
            );
            assert_ne!(
                original.updated_at, got_time,
                "Test if updated_at is updated."
            )
        },
    );

    Ok(())
}

#[sqlx::test]
async fn test_invalid_params(conn: SqlitePool) -> Result<()> {
    let no_id = UserParams::new();
    let no_name = UserParams::new().user_id(0);

    match no_id.delete(&conn).await {
        Ok(_) => panic!("Test user delete suceeded, but expected an error."),
        Err(e) => {
            assert_eq!(
                RequestError::missing_param("id").to_string(),
                e.to_string(),
                "Test if expected error matches."
            );
        }
    }

    match no_name.update(&conn).await {
        Ok(_) => panic!("Test user update suceeded, but expected an error."),
        Err(e) => {
            assert_eq!(
                RequestError::missing_param("name").to_string(),
                e.to_string(),
                "Test if expected error matches."
            )
        }
    }

    Ok(())
}
