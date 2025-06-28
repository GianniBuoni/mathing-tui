use super::*;

#[derive(Debug)]
pub enum AppError {
    Mapping(AppArm, AppArm),
    Config(String),
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
            Self::Config(msg) => {
                write!(f, "Config error: {msg}")
            }
        }
    }
}

impl AppError {
    pub fn config(message: impl Display) -> Self {
        Self::Config(message.to_string())
    }
}

impl std::error::Error for AppError {}
