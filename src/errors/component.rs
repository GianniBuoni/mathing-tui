use super::*;

#[derive(Debug)]
pub enum ComponentError {
    /// Idicates that function expects component to have a data collection
    /// to opperate on, but none was found.
    NoData,
    NotFound(String),
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => {
                write!(f, "No Data: current table is empty.")
            }
            Self::NotFound(want) => {
                write!(f, "Not found: {want} was not found.")
            }
        }
    }
}

impl std::error::Error for ComponentError {}

impl ComponentError {
    /// Ididates that component is missing a field, or field
    /// that should be [`Some`] is [`None`].
    /// Capitalize the params.
    pub fn not_found(want: impl Display) -> Self {
        Self::NotFound(want.to_string())
    }
}
