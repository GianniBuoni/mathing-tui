use std::fmt::Display;

use super::*;

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
