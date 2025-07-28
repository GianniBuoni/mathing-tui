use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Quit => "Quit the app.",
            Self::AddToReceipt => {
                "Add the current Store Item to the current Receipt table."
            }
            Self::DeleteSelected => {
                "Delete active table's currently selected item."
            }
            Self::EditSelected => {
                "Edit active table's currently selected item."
            }
            Self::EnterInsert => "Add new item to the active table.",
            Self::EnterNormal => "Cancel current form/action.",
            Self::Help => {
                "Show current key mappings and config directory location."
            }
            Self::MakeSelection => {
                "For muti-select forms: add active choice to selection."
            }
            Self::NavigateLeft => "Go to active table's next page.",
            Self::NavigateDown => "Select active table's next item.",
            Self::NavigateUp => "Select active table's previous item.",
            Self::NavigateRight => "Go to active table's previous page.",
            Self::Refresh => "Refetch all data from the database.",
            Self::Reset => "Reset receipt table; deletes all items from it.",
            Self::Search => "Search for a Store Item.",
            Self::SelectForward => "Select/activate the next table.",
            Self::SelectBackward => "Select/activate the previous table.",
            Self::Submit => "For forms: submit current form.",
            Self::HandleInput(_) => "Sends key event to a text input.",
        };
        write!(f, "{desc}")
    }
}
