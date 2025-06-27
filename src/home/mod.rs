use std::fmt::Display;

use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

// private methods
mod form_building;
mod form_submission;
mod receipt_actions;
mod state_management;

// public methods
mod builder;
mod component;
mod fetch_all;
mod plugin;
#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

pub(crate) mod prelude {
    pub(crate) use super::Home;
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Default, Debug)]
pub struct Home {
    form: Option<Form>,
    message: Option<Dialogue>,
    components: Vec<TableData>,
    component_tracker: ComponentTracker,
    req_tx: Option<UnboundedSender<DbRequest>>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder {
    components: Vec<TableData>,
    component_tracker: ComponentTracker,
    req_tx: Option<UnboundedSender<DbRequest>>,
}
