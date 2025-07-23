use std::{fmt::Debug, rc::Rc};

use crate::prelude::*;

// public methods
mod builder;
mod component;
mod form_building;
mod getters;
mod plugin;
mod table_req;

// private methods
mod render;
mod response_handling;
mod state_management;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::TableData;
}

#[derive(Debug, Default)]
pub struct TableData {
    title: Rc<str>,
    items: Vec<DbTable>,
    headings: Rc<[Rc<str>]>,
    last_search: Option<Rc<str>>,
    table_index: usize,
    app_index: usize,
    tracker: ComponentTracker,
    limit: i64,
    pub count: i64,
    pub current_page: i64,
    pub next_page: i64,
    pub table_type: Option<AppArm>,
}

#[derive(Debug, Default)]
pub struct TableBuilder {
    title: Rc<str>,
    headings: Vec<Rc<str>>,
    limit: Option<i64>,
    table_type: Option<AppArm>,
}
