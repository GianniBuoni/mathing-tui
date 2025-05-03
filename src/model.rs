use ratatui::{buffer::Buffer, layout::Rect};

pub(crate) mod prelude {
    pub(crate) use super::Model;
}

pub trait Model {
    fn title(&self) -> String;
    fn is_active(&self) -> bool;
    fn index(&self) -> u8;
    fn toggle(&mut self);
    fn render(&self, area: Rect, buf: &mut Buffer);
}
