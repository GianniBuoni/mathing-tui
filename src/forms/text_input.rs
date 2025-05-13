use std::{borrow::Cow, cell::RefCell, error::Error};

use super::*;

#[derive(PartialEq, Default, Debug)]
pub enum InputMode {
    Editing,
    #[default]
    Normal,
}

#[derive(Default, Debug)]
pub struct InputWidget<'a> {
    title: Cow<'a, str>,
    cursor_pos: RefCell<Position>,
    pub input: Input,
    pub input_mode: InputMode,
}

impl<'a> InputWidget<'a> {
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Cow::Borrowed(title);
        self
    }
    pub fn toggle(&mut self) {
        match self.input_mode {
            InputMode::Editing => self.input_mode = InputMode::Normal,
            InputMode::Normal => self.input_mode = InputMode::Editing,
        }
    }
    pub fn get_cursor(&self) -> Result<Position, Box<dyn Error>> {
        let pos = self.cursor_pos.try_borrow()?;
        Ok(pos.clone())
    }
    pub fn output(&self) -> Cow<str> {
        Cow::Borrowed(self.input.value())
    }
}

impl WidgetRef for InputWidget<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let style: AppTableStyles = match self.input_mode {
            InputMode::Normal => AppColors::INACTIVE.into(),
            InputMode::Editing => AppColors::ACTIVE.into(),
        };

        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        let input = Paragraph::new(self.input.value())
            .style(style.input_style)
            .add_modifier(Modifier::RAPID_BLINK)
            .scroll((0, scroll as u16))
            .block(
                Block::bordered()
                    .title(format!(" {} ", self.title))
                    .style(style.block_style)
                    .border_type(BorderType::Rounded),
            );

        Clear.render(area, buf);
        input.render(area, buf);

        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            {
                let mut cursor = self.cursor_pos.borrow_mut();
                cursor.x = x as u16;
                cursor.y = area.y + 1
            }
        }
    }
}
