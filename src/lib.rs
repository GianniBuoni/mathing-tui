pub mod prelude {
    pub use crate::app::prelude::*;
    pub(crate) use crate::component::prelude::*;
    pub use crate::db::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::styles::prelude::*;
    pub use crate::table::prelude::*;
    pub use crate::tui::prelude::*;
    pub(crate) use ratatui::{prelude::*, widgets::*};
}

mod app;
mod component;
mod db;
mod model;
mod styles;
mod table;
#[cfg(test)]
mod test_cases;
mod tui;
mod ui;
