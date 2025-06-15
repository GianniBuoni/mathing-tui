use super::*;

impl Component for Home {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char(_) | KeyCode::Backspace if self.form.is_some() => {
                Some(Action::HandleInput(key))
            }
            _ => self.keymap.get(&key).copied(),
        }
    }

    fn update(
        &mut self,
        action: Option<Action>,
        response: Option<&DbResponse>,
    ) {
        match self.mode {
            Mode::Insert => match action {
                Some(Action::EnterNormal) => {
                    self.form = None;
                    self.mode = Mode::Normal
                }
                Some(Action::Submit) => {
                    // form submit
                    if let Some(form) = self.form.as_mut() {
                        if let Err(e) = form.submit() {
                            form.map_err(Some(e));
                            return;
                        }
                        // TODO: remove this message?
                        form.map_err(Some(anyhow::Error::msg(
                            "Valid! Submitting...",
                        )));
                    }

                    // formulate request
                    // ISSUE: form params's build method is consuming
                    // send request via tx
                }
                Some(_) => {
                    if let Some(form) = &mut self.form {
                        form.update(action, response);
                    }
                }
                None => {}
            },
            Mode::Normal => match action {
                Some(Action::EnterInsert) => {
                    self.mode = Mode::Insert;
                    // TODO: replace with appropriate form builder
                    let (form, paylod_builder) = Form::new_item_form();

                    self.form = Some(FormTui::ItemForm(form));
                    self.from_params = Some(paylod_builder);
                }
                Some(Action::SelectForward) => {
                    self.cycle_active(1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.update(action, None));
                }
                Some(Action::SelectBackward) => {
                    self.cycle_active(-1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.update(action, None));
                }
                Some(_) => {
                    self.components.iter_mut().for_each(|c| {
                        c.update(action, response);
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

        if let Some(form) = &mut self.form {
            form.draw(frame, rect);
        }
    }
}
