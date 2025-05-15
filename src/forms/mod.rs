#![allow(dead_code)]
use std::{borrow::Cow, fmt::Debug, rc::Rc};

use crate::prelude::*;
use text_input::*;

pub(crate) mod prelude {
    pub(crate) use super::FormAction;
    pub(crate) use super::from_trait::Form;
}

mod builder;
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

impl<T> FormWidget<'_, T>
where
    T: Debug + Default,
{
    pub fn final_feild(&self) -> usize {
        self.inputs.len() - 1
    }
    pub fn next_feild(&mut self) {
        if let Some(feild) = self.inputs.get_mut(self.active_field) {
            feild.toggle();
        }
        if self.active_field < self.final_feild() {
            self.active_field += 1;
        } else {
            self.active_field = 0;
        }
        if let Some(feild) = self.inputs.get_mut(self.active_field) {
            feild.toggle();
        }
    }
    pub fn prev_feild(&mut self) {
        if let Some(feild) = self.inputs.get_mut(self.active_field) {
            feild.toggle();
        }
        if self.active_field > 0 {
            self.active_field -= 1;
        } else {
            self.active_field = self.final_feild();
        }
        if let Some(feild) = self.inputs.get_mut(self.active_field) {
            feild.toggle();
        }
    }
}
