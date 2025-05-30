use std::fmt::Display;

#[derive(Debug)]
pub enum RequestError {
    MissingParam(String),
}

impl RequestError {
    pub fn missing_param(field: impl ToString) -> Self {
        Self::MissingParam(field.to_string())
    }
}

impl Display for RequestError {
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

impl std::error::Error for RequestError {}
