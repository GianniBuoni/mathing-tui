use std::{borrow::Cow, cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use crate::prelude::*;
use builder::TableBuilder;

mod builder;
mod interactions;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{TableData, TableDisplay};
}

pub trait TableDisplay: Debug + Default {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

#[derive(Debug, Default)]
pub struct TableData<'a, T>
where
    T: TableDisplay,
{
    title: Cow<'a, str>,
    headings: Rc<[Cow<'a, str>]>,
    items: Rc<[T]>,
    table_index: usize,
    app_index: usize,
    tracker: Rc<RefCell<usize>>,
    active: bool,
}

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn new_builder() -> TableBuilder<'a, T> {
        TableBuilder::default()
    }
}

impl<T> Component for TableData<'_, T>
where
    T: TableDisplay,
{
    fn handle_key_events(&mut self, _key: KeyEvent) -> Option<Action> {
        None
    }
    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SwitchPane) => {
                self.check_active();
            }
            Some(_) => {}
            None => {}
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let colors = AppColors::get(self.active);

        let block = self
            .render_block(&colors.border_fg)
            .padding(Padding::proportional(1));

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(
                self.render_table(&colors.into()),
                block.inner(rect),
                frame.buffer_mut(),
                &mut state,
            );
        }

        block.render(rect, frame.buffer_mut())
    }
    fn add_tracker(&mut self, tracker: Rc<RefCell<usize>>) {
        self.tracker = tracker;
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn init(&mut self) {
        self.check_active();
    }
}
