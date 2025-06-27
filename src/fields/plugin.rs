use super::*;
use crate::forms::FormBuilder;

pub mod prelude {
    pub use super::items::*;
    pub use super::receipts::*;
    pub use super::users::*;
}

mod items;
mod receipts;
mod users;

fn try_get_params(
    app_arm: AppArm,
    parent: &mut FormBuilder,
) -> Result<(&mut DbPayloadBuilder, AppArm)> {
    let Some(form_type) = &parent.form_type else {
        let e = FormError::malformed("form type").into();
        return Err(e);
    };
    if !(app_arm == *form_type) {
        let e = AppError::Mapping(app_arm, *form_type).into();
        return Err(e);
    }
    let Some(payload_builder) = &mut parent.payload else {
        let e = FormError::malformed("payload").into();
        return Err(e);
    };

    Ok((payload_builder, *form_type))
}

fn try_get_item_params(
    parent: &mut FormBuilder,
) -> Result<&mut ItemParamsBuilder> {
    let (payload_builder, form_type) = try_get_params(AppArm::Items, parent)?;
    let DbPayloadBuilder::ItemParams(params) = payload_builder else {
        let e = AppError::Mapping(AppArm::Items, form_type).into();
        return Err(e);
    };

    Ok(params)
}

fn try_get_user_params(
    parent: &mut FormBuilder,
) -> Result<&mut UserParamsBuilder> {
    let (payload_builder, form_type) = try_get_params(AppArm::Users, parent)?;
    let DbPayloadBuilder::UserParams(params) = payload_builder else {
        let e = AppError::Mapping(AppArm::Users, form_type).into();
        return Err(e);
    };

    Ok(params)
}

fn try_get_receipt_params(
    parent: &mut FormBuilder,
) -> Result<&mut JoinParamsBuilder> {
    let (payload_builder, form_type) =
        try_get_params(AppArm::Receipts, parent)?;
    let DbPayloadBuilder::ReceiptParams(params) = payload_builder else {
        let e = AppError::Mapping(AppArm::Receipts, form_type).into();
        return Err(e);
    };

    Ok(params)
}
