pub mod prelude {
    pub use crate::app::prelude::*;
    pub(crate) use crate::component::prelude::*;
    pub use crate::db::prelude::*;
    pub(crate) use crate::home::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::styles::prelude::*;
    pub use crate::table::prelude::*;
    pub use crate::tui::prelude::*;
    pub(crate) use crossterm::event::{
        Event as CrosstermEvent, KeyCode, KeyEvent,
    };
    pub(crate) use futures::channel::mpsc::UnboundedSender;
    pub(crate) use ratatui::{prelude::*, widgets::*};
    pub(crate) use std::collections::HashMap;
}

mod app;
mod component;
mod db;
mod home;
mod model;
mod styles;
mod table;
#[cfg(test)]
mod test_cases;
mod tui;
