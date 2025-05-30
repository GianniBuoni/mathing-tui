use std::fmt::Display;

#[derive(Debug)]
pub enum RequestError {
    MissingParam(String),
    NotFound(String, String),
}

impl RequestError {
    pub fn missing_param(field: impl ToString) -> Self {
        Self::MissingParam(field.to_string())
    }
    pub fn not_found(id: impl ToString, table: impl ToString) -> Self {
        Self::NotFound(id.to_string(), table.to_string())
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
            Self::NotFound(id, table) => {
                write!(
                    f,
                    "Database error: cound not find {id} in table: {table}."
                )
            }
        }
    }
}

impl std::error::Error for RequestError {}
