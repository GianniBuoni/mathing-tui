use super::*;

impl Form {
    pub fn new_builder() -> FormBuilder {
        FormBuilder::default()
    }
}

impl<T> InputField<T>
where
    T: Debug + Default + FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn new(title: impl Into<Rc<str>>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }
}

impl FormBuilder {
    pub fn add_field(mut self, field: impl Field + 'static) -> Self {
        self.fields.push(Box::new(field) as Box<dyn Field>);
        self
    }

    pub fn add_title(mut self, title: impl Into<Rc<str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn add_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }
}

impl ComponentBuilder<Form> for FormBuilder {
    fn build(mut self) -> Form {
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
