//! Test module focuses on testing table's state management in
//! response to making requests (via cascading requests)
//! and from handling DBResponses.
//! Main variables that are tested are `table.current page`
//! and `table.count`.

use common::prelude::*;

mod common;

#[sqlx::test]
fn test_basic(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    let message = "Couldn't get a set of paging requests.";

    table.handle_action(Some(Action::NavigateRight));
    let req = table.get_req().ok_or(Error::msg(message))?;
    try_process_req(&conn, &mut table, req).await?;
    assert_eq!(2, table.current_page, "Test basic paging.");

    table.handle_action(Some(Action::NavigateRight));
    let req = table.get_req().ok_or(Error::msg(message))?;
    try_process_req(&conn, &mut table, req).await?;
    assert_eq!(1, table.current_page, "Test paging back.");

    Ok(())
}

#[sqlx::test]
fn test_post(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    try_process_req(&conn, &mut table, test_req(RequestType::Post)).await?;

    assert_eq!(2, table.current_page, "Post should go to last page.");
    assert_eq!(41, table.count, "Post should update counts.");

    Ok(())
}

#[sqlx::test]
fn test_update(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::Update)).await?;

    assert_eq!(2, table.current_page, "Update should stay on same page.");
    assert_eq!(40, table.count, "Update should't change counts.");

    Ok(())
}

#[sqlx::test]
fn test_delete(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::Delete)).await?;

    assert_eq!(2, table.current_page, "Delete should stay on same page.");
    assert_eq!(39, table.count, "Delete should update counts.");

    Ok(())
}

#[sqlx::test]
fn test_refresh(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::None)).await?;

    assert_eq!(1, table.current_page, "Refresh pages back to 1.");
    assert_eq!(40, table.count, "Refresh should't change counts.");

    Ok(())
}

#[sqlx::test]
fn test_reset(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    table.next_page = 2;
    table.current_page = 2;

    try_process_req(&conn, &mut table, test_req(RequestType::Reset)).await?;

    assert_eq!(2, table.current_page, "Reset should stay on same page.");
    assert_eq!(40, table.count, "Reset should't change counts.");

    Ok(())
}

#[sqlx::test]
fn test_filtered(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;

    assert_eq!(1, table.current_page, "Filtering should go to first page.");
    assert_eq!(25, table.count, "Filtering should update count.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_post(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;
    try_process_req(&conn, &mut table, test_req(RequestType::Post)).await?;

    assert_eq!(2, table.current_page, "Posting should go to last pages.");
    assert_eq!(26, table.count, "Posting should update counts.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_update(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::Update)).await?;

    assert_eq!(2, table.current_page, "Updating should stay on same page.");
    assert_eq!(25, table.count, "Updating shouldn't change counts.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_deletion(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::Delete)).await?;

    assert_eq!(2, table.current_page, "Deleting should stay on same page.");
    assert_eq!(24, table.count, "Deleting should change counts.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_refresh(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::None)).await?;

    assert_eq!(1, table.current_page, "Refresh should go to frist page.");
    assert_eq!(40, table.count, "Refresh should give unfiltered counts.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_reset(conn: SqlitePool) -> Result<()> {
    let mut table = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut table, test_req(RequestType::GetAll)).await?;
    table.next_page = 2;
    table.current_page = 2;
    try_process_req(&conn, &mut table, test_req(RequestType::Reset)).await?;

    assert_eq!(2, table.current_page, "Resets should stay on same page.");
    assert_eq!(25, table.count, "Reset should keep filtered counts.");

    Ok(())
}
