use super::{
    response_matching::{match_post_get, match_update},
    *,
};

impl Component for TableData {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let styles = Into::<AppStyles>::into(AppColors::get(self.is_active()));

        let block = self
            .render_block(styles.block_style)
            .padding(Padding::uniform(1));

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(
                self.render_table(&styles),
                block.inner(rect),
                frame.buffer_mut(),
                &mut state,
            );
        }

        block.render(rect, frame.buffer_mut())
    }
    fn is_active(&self) -> bool {
        self.tracker.inner() == self.app_index
    }
    fn handle_action(&mut self, action: Option<Action>) {
        let Some(action) = action else {
            return;
        };
        match action {
            Action::NavigateDown => {
                self.next_row();
            }
            Action::NavigateUp => {
                self.prev_row();
            }
            _ => {}
        }
    }
    fn handle_response(&mut self, res: Option<&DbResponse>) {
        let Some(res) = res else {
            return;
        };
        let Some(table_type) = &self.table_type else {
            return;
        };
        match (table_type, &res.req_type, &res.payload) {
            // Get and Post Responses
            item if match_post_get(item) => {
                self.add_items(res.payload.clone().into());
            }
            // Update Responses
            item if match_update(item) => {
                let new_element: Vec<DbTable> = res.payload.clone().into();
                let Some(new_element) = new_element.first() else {
                    return;
                };
                self.items[self.table_index] = new_element.to_owned();
            }
            // Delete responses
            (_, RequestType::Delete, DbPayload::AffectedRows(i)) => {
                if self.is_active() && !self.items.is_empty() && *i == 1 {
                    self.items.remove(self.table_index);
                }
            }
            _ => {}
        }
    }
}
