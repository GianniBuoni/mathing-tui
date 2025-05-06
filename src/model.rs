use ratatui::widgets::WidgetRef;

pub(crate) mod prelude {
    pub(crate) use super::Model;
}

pub trait Model: WidgetRef {
    fn title(&self) -> String;
    fn is_active(&self) -> bool;
    fn index(&self) -> u8;
    fn toggle(&mut self);
    fn next_row(&mut self);
    fn prev_row(&mut self);
}
