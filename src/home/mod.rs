use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::prelude::*;

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
        if self.components.len() == 0 {
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
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        let action = match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Tab => Action::SwitchPane,
                KeyCode::Char('i') => Action::EnterInsert,
                _ => {
                    let current = self.component_tracker.borrow();
                    if let Some(component) =
                        self.components.get_mut(*current.deref())
                    {
                        return component.handle_events(Some(Event::Key(key)));
                    } else {
                        return None;
                    };
                }
            },
            Mode::Insert => match key.code {
                KeyCode::Esc => Action::EnterNormal,
                _ => {
                    return None;
                }
            },
        };
        Some(action)
    }

    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::EnterInsert) => self.mode = Mode::Insert,
            Some(Action::EnterNormal) => self.mode = Mode::Normal,
            Some(Action::SwitchPane) => {
                self.cycle_view();
                self.components
                    .iter_mut()
                    .for_each(|component| component.update(action));
            }
            Some(Action::Query(_q_params)) => {
                todo!()
            }
            Some(_) => {}
            None => {}
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

impl<'a> HomeBuilder<'a> {
    pub fn add_component(mut self, component: TableTui<'a>) -> HomeBuilder<'a> {
        self.components.push(component);
        self
    }
}

impl<'a> ComponentBuilder<Home<'a>> for HomeBuilder<'a> {
    fn build(mut self) -> Home<'a> {
        self.components.iter_mut().for_each(|component| {
            component.add_tracker(self.component_tracker.clone());
            component.init();
        });
        Home {
            components: self.components,
            component_tracker: self.component_tracker,
            keymap: self.keymap,
            ..Default::default()
        }
    }
}
