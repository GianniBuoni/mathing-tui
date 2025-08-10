pub mod prelude {
    pub use crate::actions::Action;
    pub use crate::app::prelude::*;
    pub use crate::center_widget::center_widget;
    pub use crate::cli::prelude::*;
    pub use crate::component::prelude::*;
    pub use crate::config::prelude::*;
    pub use crate::db::prelude::*;
    pub(crate) use crate::dialogue::prelude::*;
    pub use crate::errors::prelude::*;
    pub(crate) use crate::fields::prelude::*;
    pub use crate::forms::prelude::*;
    pub use crate::home::prelude::*;
    pub(crate) use crate::plugin::prelude::*;
    pub(crate) use crate::styles::prelude::*;
    pub use crate::table::prelude::*;
    pub use crate::tui::prelude::*;
    pub use anyhow::{Error, Ok as Aok, Result};
    pub(crate) use crossterm::event::{
        Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers,
    };
    pub(crate) use ratatui::{prelude::*, widgets::*};
}

mod actions;
mod app;
mod center_widget;
mod cli;
mod component;
mod config;
mod db;
mod dialogue;
mod errors;
mod fields;
mod forms;
mod home;
mod plugin;
mod styles;
mod table;
mod tui;
