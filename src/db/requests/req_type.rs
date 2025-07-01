use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RequestType {
    #[default]
    None,
    GetAll,
    Get,
    Post,
    Update,
    Delete,
    Reset,
    Refresh,
    Count,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::GetAll => write!(f, "Get all"),
            Self::Get => write!(f, "Get"),
            Self::Post => write!(f, "Post"),
            Self::Update => write!(f, "Update"),
            Self::Delete => write!(f, "Delete"),
            Self::Reset => write!(f, "Reset"),
            Self::Refresh => write!(f, "Refresh"),
            Self::Count => write!(f, "Count"),
        }
    }
}
