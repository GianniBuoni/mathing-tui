use std::fmt::Display;

#[derive(Debug)]
pub enum RequestError {
    MissingParam(String),
    NotFound(String, String),
    Unhandled(String, String),
    Connection,
}

impl RequestError {
    pub fn missing_param(field: impl ToString) -> Self {
        Self::MissingParam(field.to_string())
    }
    pub fn not_found(id: impl ToString, table: impl ToString) -> Self {
        Self::NotFound(id.to_string(), table.to_string())
    }
    pub fn unhandled(request: impl ToString, payload: impl ToString) -> Self {
        Self::Unhandled(request.to_string(), payload.to_string())
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
            Self::Unhandled(request, payload) => {
                write!(f, "Unhandled Request, invalid {request}: {payload}.")
            }
            Self::Connection => {
                write!(f, "DB Connection unreachable.")
            }
        }
    }
}

impl std::error::Error for RequestError {}
