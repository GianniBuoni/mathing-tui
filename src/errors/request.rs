use super::*;

#[derive(Debug)]
pub enum RequestError {
    Connection,
    MissingParam(RequestType, String, String),
    NotFound(String, String),
    Unhandled(String, String),
}

impl RequestError {
    pub fn missing_param(
        req: RequestType,
        param: impl Display,
        field: impl Display,
    ) -> Self {
        Self::MissingParam(req, param.to_string(), field.to_string())
    }
    pub fn not_found(id: impl ToString, table: impl Display) -> Self {
        Self::NotFound(id.to_string(), table.to_string())
    }
    pub fn unhandled(request: impl ToString, payload: impl Display) -> Self {
        Self::Unhandled(request.to_string(), payload.to_string())
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParam(r_type, param_type, field) => {
                write!(
                    f,
                    "{r_type} {param_type} params malformed: \"{field}\" is missing."
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
