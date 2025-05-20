// TODO: move to db, connect to FromActions struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueryRequest;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Quit,
    SwitchPane,
    EnterNormal,
    EnterInsert,
    TableNavigateDown,
    TableNavigateUp,
    Query(QueryRequest),
}
