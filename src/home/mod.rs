use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tokio::sync::mpsc::UnboundedSender;

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
pub struct Home {
    keymap: HashMap<KeyEvent, Action>,
    form: Option<FormTui>,
    components: Vec<TableTui>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder {
    keymap: HashMap<KeyEvent, Action>,
    components: Vec<TableTui>,
    component_tracker: Rc<RefCell<usize>>,
    req_tx: Option<UnboundedSender<DbRequest>>,
}

impl Home {
    pub fn new_builder() -> HomeBuilder {
        HomeBuilder::default()
    }
    fn cycle_active(&mut self, add: i32) {
        if self.components.is_empty() {
            return;
        }

        let max = self.components.len() - 1;
        let mut current = self.component_tracker.borrow_mut();

        match *current as i32 + add {
            int if int > max as i32 => *current = 0,
            int if int < 0 => *current = max,
            _ => *current = (*current as i32 + add) as usize,
        }
    }
}

impl Component for Home {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char(_) if self.form.is_some() => {
                Some(Action::HandleInput(key))
            }
            _ => self.keymap.get(&key).copied(),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self.mode {
            Mode::Insert => match action {
                Some(Action::EnterNormal) => {
                    self.mode = Mode::Normal;
                    self.form = None
                }
                Some(_) => {
                    if let Some(form) = &mut self.form {
                        form.update(action);
                    }
                }
                None => {}
            },
            Mode::Normal => match action {
                Some(Action::EnterInsert) => {
                    self.mode = Mode::Insert;
                    // TODO: replace with appropriate form builder
                    self.form = Some(FormTui::ItemForm(Form::default()))
                }
                Some(Action::SelectForward) => {
                    self.cycle_active(1);
                    self.components.iter_mut().for_each(|c| c.update(action));
                }
                Some(Action::SelectBackward) => {
                    self.cycle_active(-1);
                    self.components.iter_mut().for_each(|c| c.update(action));
                }
                Some(_) => {
                    self.components.iter_mut().for_each(|c| {
                        c.update(action);
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
