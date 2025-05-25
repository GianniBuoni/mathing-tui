use anyhow::Error;

use super::*;

impl Default for FormValue<'_> {
    fn default() -> Self {
        Self::String(Cow::Borrowed(""))
    }
}

impl TryFrom<&Input> for FormValue<'_> {
    type Error = Error;

    fn try_from(value: &Input) -> std::result::Result<Self, Self::Error> {
        let inner_value = value.value();

        match inner_value {
            float if float.contains(".") => {
                let f = inner_value.parse::<f64>()?;
                Ok(Self::Decimal(f))
            }
            int if int.parse::<i64>().is_ok() => {
                let i = inner_value.parse::<i64>()?;
                Ok(Self::Integer(i))
            }
            _ => Ok(Self::String(Cow::Owned(inner_value.into()))),
        }
    }
}
