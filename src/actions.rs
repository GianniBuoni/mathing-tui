// TODO: move to db, connect to FromActions struct
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueryRequest;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Action {
    Quit,
    EnterNormal,
    EnterInsert,
    SelectForward,
    SelectBackward,
    Submit,
    TableNavigateDown,
    TableNavigateUp,
    HandleInput(crossterm::event::KeyEvent),
}
