pub mod prelude {
    pub use crate::app::prelude::*;
    pub use crate::events::send_key_event;
    pub(crate) use crate::items::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::receipt::prelude::*;
    pub(crate) use ratatui::{crossterm::event, prelude::*, widgets::*};
}

mod app;
mod events;
mod items;
mod model;
mod receipt;
mod ui;
