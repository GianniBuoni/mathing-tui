use super::*;

impl<'a> Form<'a> {
    pub fn new_builder() -> FormBuilder<'a> {
        FormBuilder::default()
    }
}

impl<'a, T> InputField<'a, T>
where
    T: Debug + Default + FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn new(title: &'a str) -> Self {
        Self {
            title: Cow::Borrowed(title),
            ..Default::default()
        }
    }
}

impl<'a> FormBuilder<'a> {
    pub fn add_field(mut self, field: impl Field + 'static) -> Self {
        self.fields.push(Box::new(field) as Box<dyn Field>);
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
        self.fields
            .iter_mut()
            .enumerate()
            .for_each(|(index, field)| {
                field.init(index, self.active_field.clone());
            });
        Form {
            title: self.title,
            fields: self.fields,
            rect: self.rect,
            active_field: self.active_field,
            ..Default::default()
        }
    }
}
