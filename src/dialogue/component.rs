use super::*;

impl Component for Dialogue {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        // get centered area
        let centered_rect = center_widget(
            rect,
            Constraint::Length(self.rect.width),
            Constraint::Length(self.rect.height),
        );
        // render clear in centered area
        frame.render_widget(Clear, centered_rect);

        // render message in block
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1));
        let inner_block = block.inner(centered_rect);
        let mut message = Line::from(self.message.as_ref());

        if self.error {
            let styles: AppStyles = AppColors::ACTIVE.into();
            message = message.style(styles.error_style);
        }

        block.render(centered_rect, frame.buffer_mut());
        message.render(inner_block, frame.buffer_mut());
    }
    fn handle_action(&mut self, action: Option<Action>) {
        let _ = action;
        todo!()
    }
}
