use super::*;

impl Form {
    pub fn new() -> Self {
        Form::default()
    }
    pub fn add_title(&mut self, title: impl Display) -> &mut Self {
        let title = format!(" {} ", title);
        self.title = title.into();
        self
    }

    pub fn add_rect(&mut self, rect: Rect) -> &mut Self {
        self.rect = rect;
        self
    }
    pub fn add_request_type(&mut self, req_type: RequestType) -> &mut Self {
        self.request_type = req_type;
        self
    }
    pub fn add_field(&mut self, field: impl Field + 'static) -> &mut Self {
        self.fields.push(Box::new(field) as Box<dyn Field>);
        self
    }
}
