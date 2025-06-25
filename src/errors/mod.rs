use std::fmt::Display;

use crate::prelude::*;

pub mod prelude {
    pub use super::HomeErrors;
}

#[derive(Debug)]
pub enum HomeErrors {
    NoData,
    NotFound(String),
    Mapping(AppArm, AppArm),
}

impl Display for HomeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => {
                write!(
                    f,
                    "Home or tables have no items/components, check your builders."
                )
            }
            Self::NotFound(want) => write!(f, "{want} was not found."),
            Self::Mapping(got, want) => {
                write!(
                    f,
                    "Opperation matched with {got:?} but expected {want:?}."
                )
            }
        }
    }
}

impl std::error::Error for HomeErrors {}

impl HomeErrors {
    pub fn not_found(want: impl Display) -> Self {
        Self::NotFound(want.to_string())
    }
}
