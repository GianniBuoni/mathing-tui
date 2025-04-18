pub mod prelude {
    pub use crate::app::prelude::*;
    pub(crate) use crate::items::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::receipt::prelude::*;
    pub(crate) use ratatui::{crossterm::event, prelude::*, widgets::*};
    pub(crate) use std::io;
}

mod app;
mod items;
mod model;
mod receipt;
mod ui;
