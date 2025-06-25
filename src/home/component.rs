use super::*;

impl Component for Home {
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        if let Some(form) = &self.form {
            return form.handle_key_events(key);
        }
        let keymap = Config::get_config();
        keymap.get(key)
    }

    fn handle_action(&mut self, action: Option<Action>) {
        let Some(action) = action else {
            return;
        };
        match self.mode {
            Mode::Insert => match action {
                Action::EnterNormal => {
                    self.form = None;
                    self.message = None;
                    self.mode = Mode::Normal
                }
                Action::Submit => {
                    self.handle_submit();
                }
                _ => {
                    if let Some(form) = &mut self.form {
                        form.handle_action(Some(action));
                    }
                }
            },
            Mode::Normal => match action {
                Action::AddToReceipt => {
                    let Some(table) = self.components.first() else {
                        return;
                    };
                    let Some(item) = table.get_active_item() else {
                        return;
                    };
                    let DbTable::Item(item) = item else {
                        let err = "Error getting item id for new receipt. First table is not the item table.";
                        self.map_err(err);
                        return;
                    };
                    let Some(table) = self.components.get(2) else {
                        return;
                    };
                    let users = table.get_items();
                    let users = users
                        .iter()
                        .filter_map(|f| match f {
                            DbTable::User(u) => Some(u),
                            _ => None,
                        })
                        .collect::<Vec<&StoreUser>>();

                    match Form::new_receipt(item, users) {
                        Ok(form) => {
                            self.component_tracker.go_to(0);
                            self.form = Some(form);
                            self.mode = Mode::Insert;
                        }
                        Err(err) => self.map_err(err),
                    }
                }
                Action::EnterInsert => {
                    let Some(table) =
                        self.components.get(self.component_tracker.inner())
                    else {
                        return;
                    };
                    let Some(form) = table.new_form() else {
                        return;
                    };
                    match form {
                        Ok(form) => {
                            self.form = Some(form);
                            self.mode = Mode::Insert;
                        }
                        Err(err) => self.map_err(err),
                    }
                }
                Action::DeleteSelected => {
                    let Some(table) =
                        self.components.get(self.component_tracker.inner())
                    else {
                        return;
                    };
                    let Some(dialogue) = table.delete_form() else {
                        return;
                    };
                    match dialogue {
                        Ok(dialogue) => {
                            self.message = Some(dialogue);
                            self.mode = Mode::Insert;
                        }
                        Err(err) => self.map_err(err),
                    }
                }
                Action::EditSelected => {
                    let Some(table) =
                        self.components.get(self.component_tracker.inner())
                    else {
                        return;
                    };

                    if let Some(AppArm::Receipts) = table.table_type {
                        // get receipts
                        let Some(DbTable::Receipt(current_r)) =
                            table.get_active_item()
                        else {
                            return;
                        };
                        // get users
                        let Some(table) = self.components.get(2) else {
                            return;
                        };
                        let users = table.get_items();
                        let users = users
                            .iter()
                            .filter_map(|f| match f {
                                DbTable::User(u) => Some(u),
                                _ => None,
                            })
                            .collect::<Vec<&StoreUser>>();

                        match Form::edit_receipt(current_r, users) {
                            Ok(form) => {
                                self.form = Some(form);
                                self.mode = Mode::Insert;
                            }
                            Err(err) => self.map_err(err),
                        }
                    } else {
                        let Some(form) = table.edit_form() else {
                            return;
                        };
                        match form {
                            Ok(form) => {
                                self.form = Some(form);
                                self.mode = Mode::Insert
                            }
                            Err(err) => self.map_err(err),
                        }
                    }
                }
                Action::SelectForward => {
                    self.cycle_active(1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.handle_action(Some(action)));
                }
                Action::SelectBackward => {
                    self.cycle_active(-1);
                    self.components
                        .iter_mut()
                        .for_each(|c| c.handle_action(Some(action)));
                }
                _ => {
                    self.components.iter_mut().for_each(|c| {
                        c.handle_action(Some(action));
                    });
                }
            },
        }
    }

    fn handle_response(&mut self, res: Option<&DbResponse>) {
        let Some(res) = res else {
            return;
        };
        if let Some(err) = &res.error {
            self.map_err(err);
            return;
        }
        self.components
            .iter_mut()
            .for_each(|component| component.handle_response(Some(res)));
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
