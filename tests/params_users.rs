use common::prelude::*;

mod common;

#[sqlx::test]
async fn test_add_users(conn: SqlitePool) -> Result<()> {
    let got = try_join_all({
        MOCK_USERS.into_iter().map(async |(_, name)| {
            UserParams::builder()
                .with_user_name(ParamOption::new().map_value(name).to_owned())
                .build()
                .post(&conn)
                .await
        })
    })
    .await?;

    got.into_iter().zip(MOCK_USERS).for_each(|(got, want)| {
        assert_eq!(want.1, got.name, "Test adding users.")
    });

    Ok(())
}

#[sqlx::test]
async fn test_get_users(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let got = UserParams::default().get_all(&conn).await?;
    let got_first = got
        .first()
        .ok_or(Error::msg("UserParams returned no users"))?;

    assert_eq!(
        3,
        got.len(),
        "Test if returned users matches length of inputs"
    );

    assert_eq!(
        "Jon".to_string(),
        got_first.name,
        "Test if returned users are ordered alphabetically"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_user(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    try_join_all(MOCK_USERS.into_iter().map(async |(id, name)| {
        let got = UserParams::builder()
            .with_user_id(ParamOption::new().map_value(id).to_owned())
            .build()
            .get(&conn)
            .await?;
        assert_eq!(name, got.name, "Test getting user matches expected");
        Aok(())
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_user(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    UserParams::builder()
        .with_user_id(ParamOption::new().map_value(1).to_owned())
        .build()
        .delete(&conn)
        .await?;

    let finals = UserParams::builder()
        .build()
        .get_all(&conn)
        .await?
        .into_iter()
        .map(|user| user.name)
        .collect::<Vec<String>>();

    assert_eq!(2, finals.len(), "Test if user was deleted.");
    assert!(
        !finals.contains(&"Thing".to_string()),
        "Test if expected user was deleted."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_user(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let want = ["Doodle", "Schmoodle", "Floofus"];
    let test_cases = MOCK_USERS
        .into_iter()
        .zip(want)
        .map(|((id, name), want)| (id, name, want))
        .collect::<Vec<(i64, &str, &str)>>();

    try_join_all(test_cases.into_iter().map(async |(id, name, want)| {
        let got = UserParams::builder()
            .with_user_id(ParamOption::new().map_value(id).to_owned())
            .with_user_name(ParamOption::new().map_value(want).to_owned())
            .build()
            .update(&conn)
            .await?;

        assert_ne!(name, got.name, "Test if {name} was renamed to {want}.");
        Aok(())
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_user_count(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let got = UserParams::default().count(&conn).await?;
    assert_eq!(3, got, "Test if item count matches expected.");

    Ok(())
}
