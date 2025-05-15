use std::marker::PhantomData;

use super::*;

#[derive(Default, Debug)]
pub struct FormWidgetBuilder<'a, T>
where
    T: Debug + Default,
{
    inputs: Vec<InputWidget<'a>>,
    title: Cow<'a, str>,
    layout: Rc<[Constraint]>,
    area: Rect,
    action: FormAction,
    data: PhantomData<T>,
}

impl<'a, T> FormWidget<'a, T>
where
    T: Debug + Default,
{
    pub fn new_builder() -> FormWidgetBuilder<'a, T> {
        FormWidgetBuilder::default()
    }
}

impl<'a, T> FormWidgetBuilder<'a, T>
where
    T: Debug + Default,
{
    pub fn title(mut self, line: &'a str) -> Self {
        self.title = Cow::Borrowed(line);
        self
    }
    pub fn layout<U>(mut self, layout: U) -> Self
    where
        U: Into<Rc<[Constraint]>>,
    {
        self.layout = layout.into();
        self
    }
    pub fn add_component(mut self, component: InputWidget<'a>) -> Self {
        self.inputs.push(component);
        self
    }
    pub fn area(mut self, size: Rect) -> Self {
        self.area = size;
        self
    }
    pub fn build(self) -> FormWidget<'a, T> {
        let mut form = FormWidget {
            inputs: self.inputs,
            title: self.title,
            layout: self.layout,
            area: self.area,
            action: self.action,
            ..Default::default()
        };
        if let Some(field) = form.inputs.get_mut(form.active_field) {
            field.toggle();
        }
        form
    }
}
