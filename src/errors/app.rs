use super::*;

#[derive(Debug)]
pub enum AppError {
    Mapping(AppArm, AppArm),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mapping(want, got) => {
                write!(
                    f,
                    "Opperation wanted to match {want:?}, but matched {got:?}."
                )
            }
        }
    }
}

impl std::error::Error for AppError {}
