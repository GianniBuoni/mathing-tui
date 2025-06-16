use super::*;

impl Component for Home {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        match (key.code, key.modifiers) {
            (KeyCode::Char(_), KeyModifiers::NONE)
            | (KeyCode::Char(_), KeyModifiers::SHIFT)
            | (KeyCode::Backspace, KeyModifiers::NONE)
                if self.form.is_some() =>
            {
                Some(Action::HandleInput(key))
            }
            _ => self.keymap.get(&key).copied(),
        }
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match self.mode {
            Mode::Insert => match action {
                Some(Action::EnterNormal) => {
                    self.form = None;
                    self.mode = Mode::Normal
                }
                Some(Action::Submit) => {
                    self.handle_submit();
                }
                Some(_) => {
                    if let Some(form) = &mut self.form {
                        form.handle_action(action);
                    }
                }
                None => {
                    if let Some(form) = &mut self.form {
                        form.handle_action(action);
                    }
                }
            },
            Mode::Normal => match action {
                Some(Action::EnterInsert) => {
                    self.mode = Mode::Insert;
                    // TODO: replace with appropriate form builder
                    let (form, payload_builder) = Form::new_item_form();

                    self.form = Some(FormTui::ItemForm(form));
                    self.from_params = payload_builder;
                }
                Some(Action::SelectForward) => {
                    self.cycle_active(1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.handle_action(action));
                }
                Some(Action::SelectBackward) => {
                    self.cycle_active(-1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.handle_action(action));
                }
                Some(_) => {
                    self.components.iter_mut().for_each(|c| {
                        c.handle_action(action);
                    });
                }
                None => {
                    self.components.iter_mut().for_each(|c| {
                        c.handle_action(action);
                    });
                }
            },
        }
    }

    fn handle_repsonse(&mut self, res: Option<&DbResponse>) {
        self.components
            .iter_mut()
            .for_each(|component| component.handle_repsonse(res));
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

        // first split of the ui
        let [small_chunk, r_chunk]: [Rect; 2] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .spacing(1)
        .areas(main_block.inner(rect));

        // split the smaller ui element even further
        let [item_chunk, user_chunk]: [Rect; 2] = Layout::vertical([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .areas(small_chunk);

        // rearrange the chunks to match the order of the tables plugins
        let chunks = [item_chunk, r_chunk, user_chunk];

        main_block.render(rect, frame.buffer_mut());

        self.components.iter_mut().zip(chunks).for_each(
            |(component, chunk)| {
                component.draw(frame, chunk);
            },
        );

        if let Some(form) = &mut self.form {
            form.draw(frame, rect);
        }
    }
}
