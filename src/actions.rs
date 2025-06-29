use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Action {
    Quit,
    AddToReceipt,
    DeleteSelected,
    EditSelected,
    EnterInsert,
    EnterNormal,
    MakeSelection,
    NavigateLeft,
    NavigateDown,
    NavigateUp,
    NavigateRight,
    Refresh,
    SelectForward,
    SelectBackward,
    Submit,
    HandleInput(crossterm::event::KeyEvent),
}
