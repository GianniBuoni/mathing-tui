// ISSUE: dyn fields require static lifetimes which required form plugins
// to be 'static as well. The root of this issue stems from Home struct
// owning the Form struct.
// 1. Is there a way we can make the form exist independant of Home?
// 2. Consider introducing a 'form lifetime.

use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::prelude::*;

mod builder;
mod component;
mod form_data;
mod plugin;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::Form;
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
