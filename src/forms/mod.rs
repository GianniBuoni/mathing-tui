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
    T: Default + Debug,
{
    inputs: Vec<InputWidget<'a>>,
    title: Cow<'a, str>,
    layout: Rc<[Constraint]>,
    active_feild: i32,
    data: T,
}

impl<'a, T> FormWidget<'a, T>
where
    T: Default + Debug,
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
    pub fn register_component(mut self, component: InputWidget<'a>) -> Self {
        self.inputs.push(component);
        self
    }
}
