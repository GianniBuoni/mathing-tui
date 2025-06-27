use super::*;

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
        }
    }
}
impl Display for DbPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::AffectedRows(_) => write!(f, "AffectedRows"),
            Self::ItemParams(_) => write!(f, "ItemParams"),
            Self::Item(_) => write!(f, "Item"),
            Self::Items(_) => write!(f, "Items"),
            Self::ReceiptParams(_) => write!(f, "ReceiptParams"),
            Self::Receipt(_) => write!(f, "Receipt"),
            Self::Receipts(_) => write!(f, "Receipts"),
            Self::UserParams(_) => write!(f, "UserParams"),
            Self::User(_) => write!(f, "User"),
            Self::Users(_) => write!(f, "Users"),
        }
    }
}
impl DbPayloadBuilder {
    pub fn build(&self) -> DbPayload {
        match self {
            Self::ItemParams(i) => DbPayload::ItemParams(i.build()),
            Self::UserParams(u) => DbPayload::UserParams(u.build()),
            Self::ReceiptParams(r) => DbPayload::ReceiptParams(r.build()),
        }
    }
}
