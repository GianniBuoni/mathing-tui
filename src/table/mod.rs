use std::{fmt::Debug, rc::Rc};

use crate::prelude::*;

mod builder;
mod data;
mod plugin;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{TableData, TableDisplay};
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
    table_type: Option<AppArm>,
}

#[derive(Debug, Default)]
pub struct TableBuilder {
    title: Rc<str>,
    headings: Vec<Rc<str>>,
    table_type: Option<AppArm>,
}
