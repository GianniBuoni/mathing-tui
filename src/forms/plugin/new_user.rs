use super::*;

impl Form {
    pub fn new_user() -> (Option<FormTui>, Option<DbPayloadBuilder>) {
        let payload_builder = UserParamsBuilder::default().user_name("");
        let mut form = Self::builder();

        form.add_title("New User")
            .add_rect(Self::form_rect(Self::ONE_FIELD_H))
            .add_request_type(RequestType::Post);

        if let Err(err) =
            form.try_map_input::<String>(&payload_builder.name, "User Name")
        {
            return (Some(FormTui::UserFrom(form.build_with_error(err))), None);
        }

        (
            Some(FormTui::UserFrom(form.build())),
            Some(DbPayloadBuilder::UserParams(payload_builder)),
        )
    }
}
