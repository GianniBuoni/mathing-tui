#![allow(dead_code)]

use std::{collections::HashMap, error::Error, fmt::Debug, rc::Rc};

use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Form;
}

#[cfg(test)]
mod tests;

pub trait Form: WidgetRef + Debug {
    fn submit(&self);
}

pub trait FormComponent: WidgetRef + Debug {
    fn output(&self);
}

#[derive(Default, Debug)]
pub struct FormWidget<'a, T> {
    components: HashMap<i32, Box<dyn FormComponent>>,
    context_menu: Line<'a>,
    layout: Rc<[Constraint]>,
    data: T,
}

impl<T> WidgetRef for FormWidget<'_, T> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let popup_block =
            Block::default().title_bottom(self.context_menu.clone().centered());

        let form_block = Block::bordered().border_type(BorderType::Rounded);

        let chunks = Layout::vertical(self.layout.iter())
            .spacing(1)
            .split(form_block.inner(area));

        chunks.iter().zip(self.components.values()).for_each(
            |(chunk, widget)| {
                widget.render_ref(*chunk, buf);
            },
        );

        form_block.render(popup_block.inner(area), buf);
        popup_block.render(area, buf);
    }
}

impl<'a, T> FormWidget<'a, T> {
    pub fn menu<U>(mut self, line: U) -> Self
    where
        U: Into<Line<'a>>,
    {
        self.context_menu = line.into();
        self
    }
    pub fn layout<U>(mut self, layout: U) -> Self
    where
        U: Into<Rc<[Constraint]>>,
    {
        self.layout = layout.into();
        self
    }
    pub fn register_component<U>(
        mut self,
        key: i32,
        component: Box<dyn FormComponent>,
    ) -> Result<Self, Box<dyn Error>> {
        match self.components.contains_key(&key) {
            true => return Err("Key already is registered".into()),
            false => {
                self.components.insert(key, component);
                Ok(self)
            }
        }
    }
}
