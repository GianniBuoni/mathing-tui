use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use crate::prelude::*;

mod builder;
#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

pub(crate) mod prelude {
    pub(crate) use super::Home;
    #[cfg(test)]
    pub(crate) use super::test_cases::test_home;
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Default, Debug)]
pub struct Home<'a> {
    components: Vec<TableTui<'a>>,
    component_tracker: Rc<RefCell<usize>>,
    keymap: HashMap<KeyEvent, Action>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder<'a> {
    components: Vec<TableTui<'a>>,
    component_tracker: Rc<RefCell<usize>>,
    keymap: HashMap<KeyEvent, Action>,
}

impl<'a> Home<'a> {
    pub fn new_builder() -> HomeBuilder<'a> {
        HomeBuilder::default()
    }
    fn cycle_view(&mut self) {
        if self.components.is_empty() {
            return;
        }

        let mut current = self.component_tracker.borrow_mut();

        *current = if *current.deref() < self.components.len() - 1 {
            current.deref() + 1
        } else {
            0
        };
    }
}

impl Component for Home<'_> {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        self.keymap.get(&key).copied()
    }

    fn update(&mut self, action: Option<Action>) {
        match self.mode {
            Mode::Insert => match action {
                Some(Action::EnterNormal) => self.mode = Mode::Normal,
                Some(_) => {}
                None => {}
            },
            Mode::Normal => match action {
                Some(Action::EnterInsert) => self.mode = Mode::Insert,
                Some(Action::SelectForward) => {
                    self.cycle_view();
                    self.components
                        .iter_mut()
                        .for_each(|component| component.update(action));
                }
                //TODO: add cycle backwards
                Some(_) => {
                    self.components.iter_mut().for_each(|component| {
                        component.update(action);
                    });
                }
                None => {}
            },
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let context_menu = Line::from(vec![
            " Quit ".gray(),
            "<CTRL-c>".dark_gray(),
            " | ".gray(),
            "Switch pane ".gray(),
            "<Tab> ".dark_gray(),
        ])
        .centered();

        let main_block = Block::default().title_bottom(context_menu);

        let chunks: [Rect; 2] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .spacing(1)
        .areas(main_block.inner(rect));

        main_block.render(rect, frame.buffer_mut());

        self.components.iter_mut().zip(chunks).for_each(
            |(component, chunk)| {
                component.draw(frame, chunk);
            },
        );
    }
}
