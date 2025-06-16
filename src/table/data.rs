use super::*;

use builder::TableBuilder;

impl<T> TableData<T>
where
    T: TableDisplay,
{
    pub fn new_builder() -> TableBuilder<T> {
        TableBuilder::default()
    }
    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SelectForward) | Some(Action::SelectBackward) => {
                self.check_active();
            }
            Some(Action::TableNavigateDown) => {
                self.next_row();
            }
            Some(Action::TableNavigateUp) => {
                self.prev_row();
            }
            Some(_) => {}
            None => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let styles = Into::<AppStyles>::into(AppColors::get(self.active));

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

    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        self.app_index = index;
        self.tracker = tracker;
        self.check_active();
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

impl Component for TableData<StoreItem> {
    fn is_active(&self) -> bool {
        self.is_active()
    }
    fn handle_action(&mut self, action: Option<Action>) {
        self.handle_action(action);
    }
    fn handle_repsonse(&mut self, res: Option<&DbResponse>) {
        if res.is_none() {
            return;
        }
        if let DbPayload::Item(i) = &res.unwrap().payload {
            self.add_item(i.to_owned())
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.draw(frame, rect);
    }
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        self.init(index, tracker);
    }
}

impl Component for TableData<StoreJoinRow> {
    fn is_active(&self) -> bool {
        self.is_active()
    }
    fn handle_action(&mut self, action: Option<Action>) {
        self.handle_action(action);
    }
    fn handle_repsonse(&mut self, res: Option<&DbResponse>) {
        if res.is_none() {
            return;
        }
        if let DbPayload::Receipt(r) = &res.unwrap().payload {
            self.add_item(r.to_owned())
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.draw(frame, rect);
    }
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        self.init(index, tracker);
    }
}
