pub mod prelude {
    pub use crate::app::prelude::*;
    pub use crate::db::prelude::*;
    pub use crate::events::send_key_event;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::styles::prelude::*;
    pub use crate::table::prelude::*;
    pub(crate) use ratatui::{crossterm::event, prelude::*, widgets::*};
}

mod app;
mod db;
mod events;
mod model;
mod styles;
mod table;
#[cfg(test)]
mod test_cases;
mod ui;
