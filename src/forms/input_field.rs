use std::any::type_name;

use anyhow::Error;
use tui_input::backend::crossterm::EventHandler;

use super::*;

impl<T> InputField<'_, T>
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
            .title(format!(" {} ", self.title))
            .style(style)
    }

    pub fn render_input(&self, style: Style) -> Paragraph {
        Paragraph::new(self.input.value())
            .style(style)
            .add_modifier(Modifier::RAPID_BLINK)
    }
}

impl<T> Component for InputField<'_, T>
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

impl<T> Field for InputField<'_, T>
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

    fn validate(&self) -> Result<()> {
        let inner_value = self.input.value();

        if inner_value.is_empty() {
            let message = format!("{} is unset.", self.title);
            return Err(Error::msg(message));
        }

        match inner_value.parse::<T>() {
            Ok(_) => Ok(()),
            Err(_) => {
                let message = format!(
                    "Unable to parse \"{}\" as {}.",
                    inner_value,
                    type_name::<T>()
                );
                Err(Error::msg(message))
            }
        }
    }

    fn submit(&self) -> Result<()> {
        match &self.value {
            None => {
                let message =
                    format!("{} is not mapped to any value.", self.title);
                Err(Error::msg(message))
            }
            Some(value) => {
                self.validate()?;
                let new_value = self.input.value().parse::<T>().unwrap();
                *value.borrow_mut() = new_value;
                Ok(())
            }
        }
    }
}
