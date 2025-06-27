use super::*;

#[derive(Debug, PartialEq)]
pub enum FormError {
    Malformed(String),
    Validation(String, String),
    NoData(String),
}

impl Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(item) => {
                write!(f, "Malformed: form has no {item}.")
            }
            Self::Validation(value, want_type) => {
                write!(
                    f,
                    "Input invalid: unable to parse \"{value}\" as type {want_type}."
                )
            }
            Self::NoData(input_value) => {
                write!(f, "No data: {input_value} field is empty.")
            }
        }
    }
}

impl std::error::Error for FormError {}

impl FormError {
    pub fn malformed(item: impl ToString) -> Self {
        Self::Malformed(item.to_string())
    }
    pub fn validation(value: impl ToString, want_type: impl ToString) -> Self {
        Self::Validation(value.to_string(), want_type.to_string())
    }
    pub fn no_data(input_value: impl ToString) -> Self {
        Self::NoData(input_value.to_string())
    }
}
