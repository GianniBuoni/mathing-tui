use super::*;

#[derive(Debug)]
pub enum AppError {
    Mapping(AppArm, AppArm),
    Config(String),
    ConfigInit,
    StoreTotalMutex,
    StoreTotalKey(i64),
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
                write!(f, "Config error: {msg}.")
            }
            Self::ConfigInit => {
                write!(f, "Config error: has not yet been initialized.")
            }
            Self::StoreTotalMutex => {
                write!(
                    f,
                    "StoreTotal Mutex Error: Current thread couldn't obtain lock."
                )
            }
            Self::StoreTotalKey(key) => {
                write!(
                    f,
                    "StoreTotal Error: No total found for given key \"{key}\"."
                )
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
