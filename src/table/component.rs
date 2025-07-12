use super::{
    response_handling::{
        match_count, match_post_get, match_reset, match_update,
        try_add_store_total,
    },
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
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        let Some(action) = action else {
            return;
        };
        match action {
            Action::NavigateDown => {
                self.row_increment(1);
            }
            Action::NavigateUp => {
                self.row_increment(-1);
            }
            Action::NavigateLeft => {
                self.page_increment(-1);
            }
            Action::NavigateRight => {
                self.page_increment(1);
            }
            _ => {}
        }
    }
    fn handle_response(&mut self, res: Option<&DbResponse>) -> Result<()> {
        let Some(res) = res else {
            return Ok(());
        };
        let table_type = &self
            .table_type
            .ok_or(ComponentError::not_found("Table type"))?;

        match (table_type, &res.req_type, &res.payload) {
            // Count Responses
            item if match_count(item) => {
                self.set_count(res.payload.to_owned());
                Ok(())
            }
            // Get and Post Responses
            item if match_post_get(item) => {
                self.add_items(res.payload.to_owned());
                try_add_store_total(item)
            }
            // Update Responses
            item if match_update(item) => {
                let new_element: Vec<DbTable> = res.payload.clone().into();
                // Update Response payloads should not be empty
                let new_element =
                    new_element.first().ok_or(ComponentError::NoData)?;

                self.items[self.table_index] = new_element.to_owned();
                try_add_store_total(item)
            }
            // Delete responses
            (_, RequestType::Delete, DbPayload::AffectedRows(i)) => {
                if self.is_active() && !self.items.is_empty() && *i == 1 {
                    self.items.remove(self.table_index);
                }
                Ok(())
            }
            // Reset Responses
            item if match_reset(item) => {
                self.items = vec![];
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
