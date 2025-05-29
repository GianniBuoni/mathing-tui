use std::fmt::Display;

#[derive(Debug)]
pub enum RequestErrors {
    MissingParam(String),
}

impl RequestErrors {
    pub fn missing_param(field: impl ToString) -> Self {
        Self::MissingParam(field.to_string())
    }
}

impl Display for RequestErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParam(field) => {
                write!(
                    f,
                    "Malformed params: required field \"{field}\" is missing."
                )
            }
        }
    }
}

impl std::error::Error for RequestErrors {}
