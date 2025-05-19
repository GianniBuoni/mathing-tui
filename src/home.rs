use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Home;
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Default, Debug)]
pub struct Home {
    action_tx: Option<UnboundedSender<Action>>,
    components: Vec<Box<dyn Component>>,
    current_model: usize,
    keymap: HashMap<KeyEvent, Action>,
    last_events: Vec<KeyEvent>,
    mode: Mode,
}

#[derive(Default, Debug)]
pub struct HomeBuilder {
    action_tx: Option<UnboundedSender<Action>>,
    keymap: HashMap<KeyEvent, Action>,
}

impl Home {
    pub fn new_builder() -> HomeBuilder {
        HomeBuilder::default()
    }
}

impl Component for Home {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        self.last_events.push(key);
        let action = match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Tab => Action::SwitchPane,
                KeyCode::Char('i') => Action::EnterInsert,
                _ => {
                    return None;
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
            Some(Action::SwitchPane) => {
                todo!()
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

impl ComponentBuilder<HomeBuilder, Home> for HomeBuilder {
    fn build(&self) -> Home {
        Home {
            action_tx: self.action_tx.clone(),
            keymap: self.keymap.clone(),
            ..Default::default()
        }
    }
    fn add_action_handler(
        mut self,
        tx: UnboundedSender<Action>,
    ) -> HomeBuilder {
        self.action_tx = Some(tx);
        self
    }
}
