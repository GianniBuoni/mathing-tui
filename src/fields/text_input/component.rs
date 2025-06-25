use super::*;

impl<T> Component for InputField<T>
where
    T: Debug + Default + Display + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let style = Into::<AppStyles>::into(AppColors::get(self.is_active()));

        let block = self.render_block(style.block_style);
        let input = self.render_input(style.input_style);
        let input_area = block.inner(rect);

        block.render_ref(rect, frame.buffer_mut());
        input.render_ref(input_area, frame.buffer_mut());

        if self.is_active() {
            frame.set_cursor_position((
                input_area.x + self.input.visual_cursor() as u16,
                input_area.y,
            ));
        }
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match action {
            Some(Action::HandleInput(key)) => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
            }
            Some(_) => {}
            None => {}
        }
    }

    fn is_active(&self) -> bool {
        self.active_field.inner() == self.index
    }
}
