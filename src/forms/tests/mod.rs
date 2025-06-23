use super::*;

mod outputs;
mod rendering;

impl Form {
    pub fn test_rect_buffer() -> Rect {
        Rect::new(0, 0, 56, 12)
    }
    pub fn test_rect() -> Rect {
        Rect::new(0, 0, 52, 12)
    }
    pub fn test_no_fields() -> Form {
        Self {
            title: "Add New Item".into(),
            rect: Self::test_rect(),
            ..Default::default()
        }
    }
    pub fn test_valid() -> Form {
        let mut form = Self::builder();
        form.with_title("Add New Item")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Items)
            .add_plugins(test_text_inputs)
            .unwrap();

        let mut form = form.build().unwrap();
        form.rect = Self::test_rect();
        form
    }
}
