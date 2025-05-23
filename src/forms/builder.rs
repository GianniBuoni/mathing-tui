use super::*;

impl<'a> Form<'a> {
    pub fn new_builder() -> FormBuilder<'a> {
        FormBuilder::default()
    }
}

impl<'a> FormField<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title: Cow::Borrowed(title),
            ..Default::default()
        }
    }
}

impl<'a> FormBuilder<'a> {
    pub fn add_field(mut self, field: FormField<'a>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn add_title(mut self, title: &'a str) -> Self {
        self.title = Cow::Borrowed(title);
        self
    }

    pub fn add_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }
}

impl<'a> ComponentBuilder<Form<'a>> for FormBuilder<'a> {
    fn build(mut self) -> Form<'a> {
        self.fields.iter_mut().for_each(|field| field.init());
        Form {
            title: self.title,
            fields: self.fields,
            rect: self.rect,
            ..Default::default()
        }
    }
}
