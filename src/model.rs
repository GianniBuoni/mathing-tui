pub(crate) mod prelude {
    pub(crate) use super::Model;
}

pub trait Model {
    fn title(&self) -> String;
    fn index(&self) -> u8;
}
