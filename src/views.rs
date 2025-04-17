pub(crate) mod prelude {
    pub(crate) use super::CurrentModel;
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub enum CurrentModel {
    #[default]
    Items,
    Receipt,
}
