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
