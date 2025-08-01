use std::cmp::Ordering;

use serde::Deserialize;
use strum::EnumDiscriminants;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, EnumDiscriminants,
)]
#[strum_discriminants(derive(PartialOrd, Ord))]
pub enum Action {
    Quit,
    AddToReceipt,
    DeleteSelected,
    EditSelected,
    EnterInsert,
    EnterNormal,
    Help,
    MakeSelection,
    NavigateLeft,
    NavigateDown,
    NavigateUp,
    NavigateRight,
    Refresh,
    Reset,
    Search,
    SelectForward,
    SelectBackward,
    Submit,
    HandleInput(crossterm::event::KeyEvent),
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_discriminant: ActionDiscriminants = self.into();
        let other_discriminant: ActionDiscriminants = other.into();
        match (self_discriminant, other_discriminant) {
            _ if self_discriminant < other_discriminant => Ordering::Less,
            _ if self_discriminant > other_discriminant => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Quit => "Quit",
            Self::AddToReceipt => "Add to receipt",
            Self::DeleteSelected => "Delete selected",
            Self::EditSelected => "Edit selected",
            Self::EnterInsert => "Add new item or user",
            Self::EnterNormal => "Cancel",
            Self::Help => "Help",
            Self::MakeSelection => "Make selection",
            Self::NavigateLeft => "Next page",
            Self::NavigateDown => "Next row",
            Self::NavigateUp => "Prev row",
            Self::NavigateRight => "Prev page.",
            Self::Refresh => "Reset tables",
            Self::Reset => "Clear/new receipt",
            Self::Search => "Search items",
            Self::SelectForward => "Select next table",
            Self::SelectBackward => "Select previous table",
            Self::Submit => "Submit form",
            Self::HandleInput(_) => "Sends key event to a text input",
        };
        write!(f, "{desc}")
    }
}
