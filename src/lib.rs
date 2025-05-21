pub mod prelude {
    pub use crate::app::prelude::*;
    pub use crate::component::prelude::*;
    pub use crate::config::prelude::*;
    pub use crate::db::prelude::*;
    pub(crate) use crate::home::prelude::*;
    pub(crate) use crate::styles::prelude::*;
    pub(crate) use crate::table::prelude::*;
    pub use crate::tui::prelude::*;
    pub(crate) use crossterm::event::{
        Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers,
    };
    pub(crate) use ratatui::{prelude::*, widgets::*};
}

mod app;
mod component;
mod config;
mod db;
mod home;
mod styles;
mod table;
mod tui;
