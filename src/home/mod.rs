use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

mod builder;
mod component;
mod methods;
#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

pub(crate) mod prelude {
    #[cfg(test)]
    pub(crate) use super::test_cases::test_home;
    pub(crate) use super::{Home, HomeBuilder};
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Default, Debug)]
pub struct Home {
    form: Option<FormTui>,
    form_params: Option<DbPayloadBuilder>,
    keymap: HashMap<KeyEvent, Action>,
    components: Vec<TableTui>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder {
    keymap: HashMap<KeyEvent, Action>,
    components: Vec<TableTui>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
}
