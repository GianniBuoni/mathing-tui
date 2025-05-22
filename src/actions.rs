// TODO: move to db, connect to FromActions struct
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueryRequest;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Action {
    Quit,
    SwitchPane,
    EnterNormal,
    EnterInsert,
    TableNavigateDown,
    TableNavigateUp,
}
