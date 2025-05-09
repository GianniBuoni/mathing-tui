use std::{borrow::Cow, fmt::Debug};

use ratatui::widgets::WidgetRef;

pub(crate) mod prelude {
    pub(crate) use super::Model;
}

pub trait Model: WidgetRef + Debug {
    fn index(&self) -> u8;
    fn is_active(&self) -> bool;
    fn title(&self) -> Cow<str>;
    fn toggle(&mut self);
    fn next_row(&mut self);
    fn prev_row(&mut self);
}
