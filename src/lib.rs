use std::collections::HashMap;

use crate::prelude::*;

pub mod prelude {
    pub use super::App;
    pub(crate) use crate::items::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::receipt::prelude::*;
    pub(crate) use crate::views::prelude::*;
    pub use ratatui::{crossterm::event, prelude::*, widgets::*};
    pub use std::io;
}

mod app;
mod items;
mod model;
mod receipt;
mod ui;
mod views;

pub struct App {
    models: HashMap<CurrentModel, Box<dyn Model>>,
    current_model: CurrentModel,
    should_exit: bool,
}
