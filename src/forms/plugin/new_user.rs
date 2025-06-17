use super::*;

impl Form {
    pub fn new_user() -> Option<FormTui> {
        let mut form = Self::new();

        form.add_title("New User")
            .add_rect(Self::form_rect(Self::ONE_FIELD_H))
            .add_request_type(RequestType::Post);

        let mut form = FormTui::UserFrom(form);
        InputField::<String>::new("User Name").plugin(&mut form);

        Some(form.map(|f| f.init()))
    }
}
