pub(crate) mod prelude {
    pub(crate) use super::Model;
}

pub trait Model {
    fn title(&self) -> String;
    fn is_active(&self) -> bool;
    fn index(&self) -> u8;
    fn toggle(&mut self);
}
