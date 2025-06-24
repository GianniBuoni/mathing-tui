use super::*;

impl TableData {
    pub fn new_builder() -> TableBuilder {
        TableBuilder::default()
    }
    pub fn max(&self) -> usize {
        self.items.len() - 1
    }
    pub fn add_items(&mut self, items: Vec<DbTable>) {
        match items.len() {
            1 => {
                self.items.push(items.first().unwrap().clone());
            }
            _ => self.items = items,
        }
    }
    pub fn new_form(&self) -> Option<Result<Form>> {
        let Some(table_type) = &self.table_type else {
            return None;
        };
        match table_type {
            AppArm::Items => Some(Form::new_item()),
            AppArm::Users => Some(Form::new_user()),
            AppArm::Receipts => None,
        }
    }
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn get_active_item(&self) -> Option<&DbTable> {
        self.items.get(self.table_index)
    }

    pub(super) fn next_row(&mut self) {
        if !self.is_active() {
            return;
        }
        if self.items.is_empty() {
            return;
        }
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }
    pub(super) fn prev_row(&mut self) {
        if !self.is_active() {
            return;
        }
        if self.items.is_empty() {
            return;
        }
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }
}

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
        match action {
            Some(Action::NavigateDown) => {
                self.next_row();
            }
            Some(Action::NavigateUp) => {
                self.prev_row();
            }
            Some(_) => {}
            None => {}
        }
    }
    fn handle_response(&mut self, res: Option<&DbResponse>) {
        let Some(res) = res else {
            return;
        };
        let Some(table_type) = &self.table_type else {
            return;
        };
        match (table_type, &res.payload) {
            (AppArm::Items, DbPayload::Item(_) | DbPayload::Items(_)) => {
                self.add_items(res.payload.clone().into());
            }
            (
                AppArm::Receipts,
                DbPayload::Receipt(_) | DbPayload::Receipts(_),
            ) => {
                self.add_items(res.payload.clone().into());
            }
            (AppArm::Users, DbPayload::User(_) | DbPayload::Users(_)) => {
                self.add_items(res.payload.clone().into());
            }
            _ => {}
        }
    }
}
