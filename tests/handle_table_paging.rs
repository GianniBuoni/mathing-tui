//! Test module focuses on testing table's state management in
//! response to making requests (via cascading requests)
//! and from handling DBResponses.
//! Main variables that are tested are `table.current page`
//! and `table.count`.

use common::prelude::*;

mod common;

#[sqlx::test]
async fn test_basic(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    page_items_to(&mut tables, &conn, 2).await?;

    {
        let [i, r] = &tables;
        assert_eq!(2, i.current_page, "Test i paging to 2.");
        assert_eq!(20, i.count, "Item count should be unaffected.");
        assert_eq!(1, r.current_page, "Receipt page should be unaffected.");
        assert_eq!(8, r.count, "Receipt count should be unaffected.");
    }

    page_items_to(&mut tables, &conn, 1).await?;

    let [i, r] = &tables;
    assert_eq!(1, i.current_page, "Test i paging back to 1.");
    assert_eq!(20, i.count, "Item count should be unaffected.");
    assert_eq!(1, r.current_page, "Receipt page should be unaffected.");
    assert_eq!(8, r.count, "Receipt count should be unaffected.");

    Ok(())
}

#[sqlx::test]
async fn test_receipt_reset(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_r_req(RequestType::Reset)).await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Reset r shouldn't update page.");
    assert_eq!(20, i.count, "Reset r shouldn't update counts.");
    assert_eq!(1, r.current_page, "Reset r should go to first page.");
    assert_eq!(0, r.count, "Reset r should update count.");
    assert_eq!(0, r.get_items().len(), "Reset r should empty table.");

    Ok(())
}

#[sqlx::test]
async fn test_post_to_empty(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_r_req(RequestType::Reset)).await?;

    try_process_req(&conn, &mut tables, test_r_req(RequestType::Post)).await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Reset r shouldn't update page.");
    assert_eq!(20, i.count, "Reset r shouldn't update counts.");
    assert_eq!(1, r.current_page, "Reset r should go to first page.");
    assert_eq!(1, r.count, "Reset r should update count.");
    assert_eq!(1, r.get_items().len(), "Reset r should empty table.");

    Ok(())
}

#[sqlx::test]
async fn test_post_item_new_page(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    try_process_req(&conn, &mut tables, test_item_req(RequestType::Post))
        .await?;

    let [i, r] = &tables;
    assert_eq!(6, i.current_page, "Post item should add new page.");
    assert_eq!(21, i.count, "Post item should update counts.");
    assert_eq!(1, r.current_page, "Post item shouldn't affect r page.");
    assert_eq!(8, r.count, "Post item shouldn't affect r count.");
    assert_eq!(1, i.get_items().len(), "Affected table should have 1 item.");

    Ok(())
}

#[sqlx::test]
async fn test_post_r_new_page(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    try_process_req(&conn, &mut tables, test_r_req(RequestType::Post)).await?;

    let [i, r] = &tables;
    assert_eq!(1, i.current_page, "Post r shouldn't affect item page.");
    assert_eq!(20, i.count, "Post r shouldn't update item counts.");
    assert_eq!(3, r.current_page, "Post r should add new page.");
    assert_eq!(9, r.count, "Post r should affect r count.");
    assert_eq!(1, r.get_items().len(), "Affected table should have 1 item.");

    Ok(())
}

#[sqlx::test]
async fn test_update_item(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::Update))
        .await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Update should stay on same page.");
    assert_eq!(20, i.count, "Update should't change counts.");
    assert_eq!(2, r.current_page, "Update should stay on same page.");
    assert_eq!(8, r.count, "Update should't change counts.");
    assert_eq!(4, i.get_items().len(), "Update shouldn't change item len.");

    Ok(())
}

#[sqlx::test]
async fn test_update_r(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_r_req(RequestType::Update))
        .await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Update should stay on same page.");
    assert_eq!(20, i.count, "Update should't change counts.");
    assert_eq!(2, r.current_page, "Update should stay on same page.");
    assert_eq!(8, r.count, "Update should't change counts.");
    assert_eq!(4, i.get_items().len(), "Update shouldn't change item len.");

    Ok(())
}

#[sqlx::test]
async fn test_item_delete(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::Delete))
        .await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Delete item should stay on same page.");
    assert_eq!(19, i.count, "Delete item should change counts.");
    assert_eq!(3, i.get_items().len(), "Delete should change items length.");
    assert_eq!(2, r.current_page, "Delete item shouldn't affect r page.");
    assert_eq!(7, r.count, "Delete item should change r counts.");
    assert_eq!(3, r.get_items().len(), "Delete should change r length.");

    Ok(())
}

#[sqlx::test]
async fn test_r_delete(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_r_req(RequestType::Delete))
        .await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Delete r should stay on same page.");
    assert_eq!(20, i.count, "Delete r should change counts.");
    assert_eq!(4, i.get_items().len(), "Delete r shouldn't affect i len.");
    assert_eq!(2, r.current_page, "Delete r shouldn't affect r page.");
    assert_eq!(7, r.count, "Delete r should change r counts.");
    assert_eq!(3, r.get_items().len(), "Delete r should change r length.");

    Ok(())
}

#[sqlx::test]
async fn test_refresh(conn: SqlitePool) -> Result<()> {
    try_init_test_config().await?;
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::None))
        .await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::None))
        .await?;

    let [i, r] = &tables;
    assert_eq!(1, i.current_page, "Refresh pages i back to 1.");
    assert_eq!(20, i.count, "Refresh should't change i counts.");
    assert_eq!(4, i.get_items().len(), "Refreshed len should keep i limit.");
    assert_eq!(1, r.current_page, "Refresh pages r back to 1.");
    assert_eq!(8, r.count, "Refresh should't change r counts.");
    assert_eq!(4, r.get_items().len(), "Refresh len should keep r limit.");

    Ok(())
}

#[sqlx::test]
fn test_filtered(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;

    let [i, r] = &tables;
    assert_eq!(1, i.current_page, "Filtering should go to first page.");
    assert_eq!(14, i.count, "Filtering should update count.");
    assert_eq!(4, i.get_items().len(), "Filtering i len should be updated.");
    assert_eq!(2, r.current_page, "Filtering i shouldn't affect r page.");
    assert_eq!(8, r.count, "Filtering i shouldn't update r count.");
    assert_eq!(4, r.get_items().len(), "Filting i shouldn't change r len.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_post(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::Post))
        .await?;

    let [i, r] = &tables;
    assert_eq!(4, i.current_page, "Filtered i post should go to last page.");
    assert_eq!(15, i.count, "Filtering i post should update count.");
    assert_eq!(3, i.get_items().len(), "Filtering i len should be updated.");
    assert_eq!(
        1, r.current_page,
        "Filtering i post shouldn't affect r page."
    );
    assert_eq!(8, r.count, "Filtering i post shouldn't update r count.");
    assert_eq!(4, r.get_items().len(), "Filting i shouldn't change r len.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_update(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    // set filter
    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;

    page_to(&mut tables, &conn, 4).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::Update))
        .await?;

    let [i, r] = &tables;
    assert_eq!(4, i.current_page, "Updating i should stay on same i page.");
    assert_eq!(14, i.count, "Updating i shouldn't change i counts.");
    assert_eq!(2, i.get_items().len(), "Updating i shouldn't change i len.");
    assert_eq!(4, r.current_page, "Updating i should stay on same r page.");
    assert_eq!(8, r.count, "Updating i shouldn't change r counts.");
    assert_eq!(0, r.get_items().len(), "Updating i shouldn't change r len.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_deletion(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    // set filter
    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;

    page_to(&mut tables, &conn, 2).await?;

    try_process_req(&conn, &mut tables, test_item_req(RequestType::Delete))
        .await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Deleting should stay on same page.");
    assert_eq!(13, i.count, "Deleting should change counts.");
    assert_eq!(3, i.get_items().len(), "Deleting item should affect i len.");
    assert_eq!(2, r.current_page, "Deleting i should stay on same r page.");
    assert_eq!(7, r.count, "Deleting i should change r counts.");
    assert_eq!(3, r.get_items().len(), "Deleting i should change r len.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_refresh(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;
    page_to(&mut tables, &conn, 2).await?;
    try_process_req(&conn, &mut tables, test_item_req(RequestType::None))
        .await?;

    let [i, r] = &tables;
    assert_eq!(1, i.current_page, "Refresh i page table to first.");
    assert_eq!(20, i.count, "Refresh i should give unfiltered counts.");
    assert_eq!(4, i.get_items().len(), "Refresh should replace table.");
    assert_eq!(1, r.current_page, "Refresh r should page table to first.");
    assert_eq!(8, r.count, "Refresh r shouldn't affect counts.");
    assert_eq!(4, r.get_items().len(), "Refresh r should replace table.");

    Ok(())
}

#[sqlx::test]
fn test_filtered_reset(conn: SqlitePool) -> Result<()> {
    let mut tables = try_init_paging_test(&conn).await?;
    try_process_req(&conn, &mut tables, test_item_req(RequestType::GetAll))
        .await?;
    page_to(&mut tables, &conn, 2).await?;
    try_process_req(&conn, &mut tables, test_r_req(RequestType::Reset)).await?;

    let [i, r] = &tables;
    assert_eq!(2, i.current_page, "Reset r shouldn't affect i page.");
    assert_eq!(14, i.count, "Reset r shouldn't affect i counts.");
    assert_eq!(4, i.get_items().len(), "Reset r shouldn't affect i len.");
    assert_eq!(1, r.current_page, "Reset r should page table to first.");
    assert_eq!(0, r.count, "Reset r should affect counts.");
    assert_eq!(0, r.get_items().len(), "Reset r should empty table.");

    Ok(())
}
