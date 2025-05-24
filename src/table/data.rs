use super::*;

use builder::TableBuilder;

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn new_builder() -> TableBuilder<'a, T> {
        TableBuilder::default()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl<T> Component for TableData<'_, T>
where
    T: TableDisplay,
{
    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SwitchPane) => {
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
}
