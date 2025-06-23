use super::*;

#[derive(Debug, PartialEq)]
pub enum FormErrors {
    Malformed(String),
    Validation(String, String),
    NoData(String),
    Mapping(AppArm, AppArm),
}

impl Display for FormErrors {
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
            Self::Mapping(input, form) => {
                write!(f, "{input:?} input is not mappable to {form:?} form.")
            }
        }
    }
}

impl std::error::Error for FormErrors {}

impl FormErrors {
    pub fn malformed(item: impl ToString) -> Self {
        Self::Malformed(item.to_string())
    }
    pub fn validation(value: impl ToString, want_type: impl ToString) -> Self {
        Self::Validation(value.to_string(), want_type.to_string())
    }
    pub fn no_data(input_value: impl ToString) -> Self {
        Self::NoData(input_value.to_string())
    }
    pub fn mapping(input: AppArm, form: AppArm) -> Self {
        Self::Mapping(input, form)
    }
}
