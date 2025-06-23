// ISSUE: dyn fields require static lifetimes
// which is causing problems in form plugins
// consider introducing a 'form lifetime

use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::prelude::*;

mod builder;
mod component;
mod errors;
mod form_data;
mod plugin;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::errors::FormErrors;
    #[allow(unused_imports)]
    pub use super::{Form, FormBuilder};
}

impl Form {
    const HEIGHT: u16 = 6;
    const WIDTH: u16 = 100;
}

#[derive(Default, Debug)]
pub struct Form {
    payload: Option<DbPayloadBuilder>,
    error: Option<String>,
    fields: Vec<Box<dyn Field>>,
    title: Rc<str>,
    active_field: ComponentTracker,
    rect: Rect,
    request_type: RequestType,
}

#[derive(Default, Debug)]
pub struct FormBuilder {
    pub payload: Option<DbPayloadBuilder>,
    fields: Vec<Box<dyn Field>>,
    title: Rc<str>,
    active_field: ComponentTracker,
    request_type: RequestType,
    pub form_type: Option<AppArm>,
}
