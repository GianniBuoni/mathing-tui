use std::fmt::Display;

use crate::prelude::*;

pub mod prelude {
    pub use super::app::AppError;
    pub use super::component::ComponentError;
}

mod app;
mod component;
