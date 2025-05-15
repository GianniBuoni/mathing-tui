#![allow(dead_code)]
use std::{borrow::Cow, fmt::Debug, rc::Rc};

use crate::prelude::*;
use text_input::*;

pub(crate) mod prelude {
    pub(crate) use super::FormAction;
    pub(crate) use super::from_trait::Form;
}

mod from_render;
mod from_trait;
#[cfg(test)]
mod tests;
mod text_input;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FormAction {
    #[default]
    Create,
    Update,
    Delete,
}

#[derive(Default, Debug)]
pub struct FormWidget<'a, T>
where
    T: Debug + Default,
{
    inputs: Vec<InputWidget<'a>>,
    title: Cow<'a, str>,
    layout: Rc<[Constraint]>,
    active_field: usize,
    area: Rect,
    action: FormAction,
    params: T,
}

impl<'a, T> FormWidget<'a, T>
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
    pub fn final_feild(&self) -> usize {
        self.inputs.len() - 1
    }
    pub fn next_feild(&mut self) {
        if self.active_field < self.final_feild() {
            self.active_field += 1;
        } else {
            self.active_field = 0;
        }
    }
    pub fn prev_feild(&mut self) {
        if self.active_field > 0 {
            self.active_field -= 1;
        } else {
            self.active_field = self.final_feild();
        }
    }
}
