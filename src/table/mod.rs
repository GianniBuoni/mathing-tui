use std::{fmt::Debug, rc::Rc};

use crate::prelude::*;

// public methods
mod builder;
mod component;
mod form_building;
mod getters;
mod plugin;

// private methods
mod render;
mod response_handling;
mod state_management;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::TableData;
}

pub trait TableDisplay: Debug {
    fn ref_array(&self) -> Row;
}

#[derive(Debug, Default)]
pub struct TableData {
    title: Rc<str>,
    items: Vec<DbTable>,
    headings: Rc<[Rc<str>]>,
    table_index: usize,
    app_index: usize,
    tracker: ComponentTracker,
    pub table_type: Option<AppArm>,
}

#[derive(Debug, Default)]
pub struct TableBuilder {
    title: Rc<str>,
    headings: Vec<Rc<str>>,
    table_type: Option<AppArm>,
}
