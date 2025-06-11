use super::*;

#[derive(Debug, PartialEq)]
pub enum FormErrors {
    Malformed,
    Validation(String, String),
    NoData(String),
    Unmapped(String),
}

impl Display for FormErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed => write!(f, "Malformed: form has no fields."),
            Self::Validation(value, want_type) => {
                write!(
                    f,
                    "Input invalid: unable to parse \"{value}\" as type {want_type}."
                )
            }
            Self::NoData(input_value) => {
                write!(f, "No data: {input_value} field is empty.")
            }
            Self::Unmapped(input) => {
                write!(f, "{input} is not mapped to any value.")
            }
        }
    }
}

impl std::error::Error for FormErrors {}

impl FormErrors {
    pub fn validation(value: impl ToString, want_type: impl ToString) -> Self {
        Self::Validation(value.to_string(), want_type.to_string())
    }
    pub fn no_data(input_value: impl ToString) -> Self {
        Self::NoData(input_value.to_string())
    }
    pub fn unmapped(input: impl ToString) -> Self {
        Self::Unmapped(input.to_string())
    }
}
