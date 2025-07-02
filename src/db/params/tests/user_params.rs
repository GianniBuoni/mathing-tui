use super::*;

async fn init_test(conn: &SqlitePool) -> Result<Vec<StoreUser>> {
    Ok(try_join_all(TEST_USERS.into_iter().map(async |user_name| {
        Ok::<StoreUser, Error>({
            UserParams::builder()
                .with_user_name(ParamOption::new().map_value(user_name).clone())
                .build()
                .post(conn)
                .await?
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
    let ordered = UserParams::builder().build().get_all(&conn).await?;

    assert_eq!(
        ordered.len(),
        unordered.len(),
        "Test if returned users matches length of inputs"
    );

    assert_eq!(
        "Jon".to_string(),
        ordered.get(0).unwrap().name,
        "Test if returned users are ordered alphabetically"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_user(conn: SqlitePool) -> Result<()> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        anyhow::Ok::<()>({
            let got = UserParams::builder()
                .with_user_id(ParamOption::new().map_value(want.id).clone())
                .build()
                .get(&conn)
                .await?;
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
    let params = UserParams::builder()
        .with_user_id(
            ParamOption::new()
                .map_value(original.get(0).unwrap().id)
                .clone(),
        )
        .build();

    params.delete(&conn).await?;

    let finals = UserParams::builder()
        .build()
        .get_all(&conn)
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
        .map(|(user, name)| {
            UserParams::builder()
                .with_user_id(ParamOption::new().map_value(user.id).clone())
                .with_user_name(ParamOption::new().map_value(name).clone())
                .build()
        })
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
async fn test_user_count(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;
    let got = UserParams::default().count(&conn).await?;
    assert_eq!(3, got, "Test if item count matches expected.");

    Ok(())
}

#[sqlx::test]
async fn test_invalid_params(conn: SqlitePool) -> Result<()> {
    let no_id = UserParams::builder().build();
    let no_name = UserParams::builder()
        .with_user_id(ParamOption::new().map_value(0).clone())
        .build();

    match no_id.delete(&conn).await {
        Ok(_) => panic!("Test user delete suceeded, but expected an error."),
        Err(e) => {
            assert_eq!(
                RequestError::missing_param(RequestType::Delete, "user", "id")
                    .to_string(),
                e.to_string(),
                "Test if expected error matches."
            );
        }
    }

    match no_name.update(&conn).await {
        Ok(_) => panic!("Test user update suceeded, but expected an error."),
        Err(e) => {
            assert_eq!(
                RequestError::missing_param(
                    RequestType::Update,
                    "user",
                    "name"
                )
                .to_string(),
                e.to_string(),
                "Test if expected error matches."
            )
        }
    }

    Ok(())
}
