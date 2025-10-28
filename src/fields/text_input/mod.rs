use std::{any::type_name, ops::Deref};

use tui_input::backend::crossterm::EventHandler;

use super::*;
use crate::forms::FormBuilder;

mod component;
mod plugin;

impl<T> InputField<T>
where
    T: Debug + Default + Display + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    const HEIGHT: u16 = 3;

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_title(&mut self, title: impl Display) -> &mut Self {
        let title = format!(" {title} ");
        self.title = title.into();
        self
    }

    pub fn with_field_type(&mut self, field_type: AppArm) -> &mut Self {
        self.field_type = Some(field_type);
        self
    }

    pub fn with_default_value(&mut self, default_value: T) -> &mut Self {
        let input = Input::new(default_value.to_string());
        self.input = input;
        self
    }

    pub fn render_block(&self, style: Style) -> Block<'_> {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.title.deref())
            .style(style)
    }

    pub fn render_input(&self, style: Style) -> Paragraph<'_> {
        Paragraph::new(self.input.value())
            .style(style)
            .add_modifier(Modifier::RAPID_BLINK)
    }

    pub fn validate(&self) -> Result<T> {
        let inner_value = self.input.value();

        if inner_value.is_empty() {
            return Err({
                FormError::no_data(self.title.deref().trim()).into()
            });
        }

        inner_value.parse::<T>().map_err(|_| {
            FormError::validation(inner_value, type_name::<T>()).into()
        })
    }
}

impl<T> Field for InputField<T>
where
    T: Debug + Default + Display + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn submit(&self) -> Result<()> {
        let new_value = self.validate()?;
        self.value.map_value(new_value);
        Ok(())
    }
    fn get_rect_height(&self) -> u16 {
        Self::HEIGHT
    }
    fn handles_input(&self) -> bool {
        true
    }
}
