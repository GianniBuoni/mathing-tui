use std::{any::type_name, ops::Deref};

use tui_input::backend::crossterm::EventHandler;

use super::*;

impl<T> InputField<T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn map_value(mut self, source: Rc<RefCell<T>>) -> Self {
        self.value = Some(source);
        self
    }

    pub fn render_block(&self, style: Style) -> Block {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.title.deref())
            .style(style)
    }

    pub fn render_input(&self, style: Style) -> Paragraph {
        Paragraph::new(self.input.value())
            .style(style)
            .add_modifier(Modifier::RAPID_BLINK)
    }

    pub fn validate(&self) -> Result<T> {
        let inner_value = self.input.value();

        if inner_value.is_empty() {
            return Err({
                FormErrors::no_data(self.title.deref().trim()).into()
            });
        }

        inner_value.parse::<T>().map_err(|_| {
            FormErrors::validation(inner_value, type_name::<T>()).into()
        })
    }
}

impl<T> Component for InputField<T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        self.index = index;
        self.active_field = tracker;
        self.check_active();
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let style = Into::<AppStyles>::into(AppColors::get(self.active));

        let block = self.render_block(style.block_style);
        let input = self.render_input(style.input_style);

        block.render_ref(rect, frame.buffer_mut());
        input.render_ref(block.inner(rect), frame.buffer_mut());
    }

    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SelectForward) | Some(Action::SelectBackward) => {
                self.check_active();
            }
            Some(Action::HandleInput(key)) => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
            }
            Some(_) => {}
            None => {}
        }
    }
}

impl<T> Field for InputField<T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn assign_index(&mut self, index: usize) {
        self.index = index
    }

    fn check_active(&mut self) {
        if self.index == *self.active_field.borrow() {
            self.active = true
        }
    }

    fn submit(&self) -> Result<()> {
        match &self.value {
            None => Err(FormErrors::unmapped(self.title.deref().trim()).into()),
            Some(value) => {
                let new_value = self.validate()?;
                *value.borrow_mut() = new_value;
                Ok(())
            }
        }
    }
}
