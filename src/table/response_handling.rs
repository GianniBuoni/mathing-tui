use super::*;

pub(super) fn match_post_get(
    item: (&AppArm, &RequestType, &DbPayload),
) -> bool {
    matches!(
        item,
        (
            AppArm::Items,
            RequestType::Get | RequestType::GetAll | RequestType::Post,
            DbPayload::Item(_) | DbPayload::Items(_),
        )
    ) || matches!(
        item,
        (
            AppArm::Receipts,
            RequestType::Get | RequestType::GetAll | RequestType::Post,
            DbPayload::Receipt(_) | DbPayload::Receipts(_),
        )
    ) || matches!(
        item,
        (
            AppArm::Users,
            RequestType::Get | RequestType::GetAll | RequestType::Post,
            DbPayload::User(_) | DbPayload::Users(_),
        ),
    )
}
pub(super) fn match_update(item: (&AppArm, &RequestType, &DbPayload)) -> bool {
    matches!(
        item,
        (AppArm::Items, RequestType::Update, DbPayload::Item(_))
    ) || matches!(
        item,
        (AppArm::Users, RequestType::Update, DbPayload::User(_))
    ) || matches!(
        item,
        (AppArm::Receipts, RequestType::Update, DbPayload::Receipt(_))
    )
}
pub(super) fn match_count(item: (&AppArm, &RequestType, &DbPayload)) -> bool {
    matches!(
        item,
        (
            AppArm::Items,
            RequestType::Count,
            DbPayload::Count(AppArm::Items, _)
        )
    ) || matches!(
        item,
        (
            AppArm::Receipts,
            RequestType::Count,
            DbPayload::Count(AppArm::Receipts, _)
        )
    ) || matches!(
        item,
        (
            AppArm::Users,
            RequestType::Count,
            DbPayload::Count(AppArm::Users, _)
        )
    )
}
pub(super) fn match_reset(item: (&AppArm, &RequestType, &DbPayload)) -> bool {
    matches!(
        item,
        (
            AppArm::Receipts,
            RequestType::Reset,
            DbPayload::AffectedRows(_)
        )
    )
}
pub(super) fn try_add_store_total(
    (_, req_type, res_payload): (&AppArm, &RequestType, &DbPayload),
) -> Result<()> {
    // return early if the original req was not a Post
    if !(matches!(req_type, RequestType::Post | RequestType::Update)) {
        return Ok(());
    }
    let DbPayload::Receipt(receipt) = res_payload else {
        return Ok(());
    };
    let store_total = StoreTotal::try_get()?;

    store_total
        .lock()
        .map_err(|_| AppError::StoreTotalMutex)?
        .add(receipt.try_calc()?);

    Ok(())
}
