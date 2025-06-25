use super::*;

impl<T> Component for SelectionField<T>
where
    T: Debug + Default + Copy,
{
    fn handle_action(&mut self, action: Option<Action>) {
        let Some(action) = action else {
            return;
        };
        match action {
            Action::NavigateDown => self.cycle_next(1),
            Action::NavigateUp => self.cycle_next(-1),
            _ => self
                .choices
                .iter_mut()
                .for_each(|f| f.handle_action(Some(action))),
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let styles: AppStyles = AppColors::get(self.is_active()).into();

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.title.as_ref())
            .padding(Padding::uniform(1))
            .style(styles.block_style);

        let inner_area = block.inner(rect);
        let lines = Layout::vertical(
            self.choices
                .iter()
                .map(|_| Constraint::Fill(1))
                .collect::<Vec<Constraint>>(),
        )
        .split(inner_area);

        block.render(rect, frame.buffer_mut());
        self.choices.iter_mut().zip(lines.iter()).for_each(
            |(choice, choice_area)| {
                choice.draw(frame, *choice_area);
            },
        );
    }
    fn is_active(&self) -> bool {
        self.active_field.inner() == self.index
    }
}
