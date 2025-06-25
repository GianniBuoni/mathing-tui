use std::fmt::Display;

use crate::prelude::*;

pub mod prelude {
    pub use super::app::AppError;
    pub use super::component::ComponentError;
    pub use super::form::FormError;
}

mod app;
mod component;
mod form;
