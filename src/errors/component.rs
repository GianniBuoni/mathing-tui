use super::*;

#[derive(Debug)]
pub enum ComponentError {
    NoData,
    NotFound(String),
    Mapping(AppArm, AppArm),
}

impl Display for ComponentError {
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

impl std::error::Error for ComponentError {}

impl ComponentError {
    pub fn not_found(want: impl Display) -> Self {
        Self::NotFound(want.to_string())
    }
}
