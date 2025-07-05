use super::*;

impl Component for Home {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        if let Some(form) = &self.form {
            return form.handle_key_events(key);
        }
        KeyMap::get()?.get_action(key)
    }

    fn handle_action(&mut self, action: Option<Action>) {
        let Some(act) = action else {
            return;
        };
        match self.mode {
            Mode::Insert => match act {
                Action::EnterNormal => self.reset_mode(),
                Action::Submit => self.handle_submit(),
                // pass any other action to an active form
                _ => {
                    if let Some(form) = &mut self.form {
                        form.handle_action(action);
                    }
                }
            },
            Mode::Normal => match act {
                Action::AddToReceipt => self.new_receipt(),
                Action::EnterInsert => self.enter_insert(),
                Action::DeleteSelected => self.delete_selected(),
                Action::EditSelected => self.edit_selected(),
                Action::Search => self.handle_search(),
                Action::SelectForward => self.cycle_active(1),
                Action::SelectBackward => self.cycle_active(-1),
                Action::NavigateLeft | Action::NavigateRight => {
                    self.handle_paging(action)
                }
                Action::Refresh => self.handle_refresh(),
                _ => {
                    self.components.iter_mut().for_each(|c| {
                        c.handle_action(action);
                    });
                }
            },
        }
    }
    /// Home's response handler maps any error into a dialoge message.
    fn handle_response(&mut self, res: Option<&DbResponse>) -> Result<()> {
        let Some(res) = res else {
            return Ok(());
        };
        if let Some(err) = &res.error {
            self.map_err(err);
            return Ok(());
        }
        let res = self
            .components
            .iter_mut()
            .try_for_each(|component| component.handle_response(Some(res)));

        if let Err(err) = res {
            self.map_err(err);
        }
        Ok(())
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
        if let Some(dialogue) = &mut self.message {
            dialogue.draw(frame, rect);
        }
    }
}
