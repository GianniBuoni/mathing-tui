use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

mod builder;
mod component;
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
    from_params: Option<DbPayloadBuilder>,
    keymap: HashMap<KeyEvent, Action>,
    components: Vec<Box<dyn Component>>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder {
    keymap: HashMap<KeyEvent, Action>,
    components: Vec<Box<dyn Component>>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
}

impl Home {
    pub fn new_builder() -> HomeBuilder {
        HomeBuilder::default()
    }
    fn cycle_active(&mut self, add: i32) {
        if self.components.is_empty() {
            return;
        }

        let max = self.components.len() - 1;
        let mut current = self.component_tracker.borrow_mut();

        match *current as i32 + add {
            int if int > max as i32 => *current = 0,
            int if int < 0 => *current = max,
            _ => *current = (*current as i32 + add) as usize,
        }
    }
    fn handle_submit(&mut self) {
        if let Some(form) = self.form.as_mut() {
            if let Err(e) = form.submit() {
                form.map_err(Some(e));
                return;
            }

            if let Some(params) = self.from_params.as_ref() {
                let payload = params.build();
                let req = DbRequest::new()
                    .req_type(form.get_req_type())
                    .payload(payload);

                if let Some(tx) = self.req_tx.clone() {
                    if let Err(err) = tx.send(req) {
                        let err = anyhow::Error::msg(err.to_string());
                        form.map_err(Some(err));
                        return;
                    }
                }
            };

            self.form = None;
            self.from_params = None;
            self.mode = Mode::Normal;
        }
    }
}
