use super::*;

mod component;
mod plugin;

impl<T> Choice<T>
where
    T: Debug + Default + Copy,
{
    const HEIGHT: u16 = 1;

    pub fn new(display: impl Display) -> Self {
        Self {
            display: display.to_string().into(),
            ..Default::default()
        }
    }
    pub fn with_value(mut self, value: T) -> Self {
        self.value = value;
        self
    }
    pub fn get_display(&self) -> Line {
        let marker = match self.is_active() {
            true => "> ",
            false => "  ",
        };
        let radio = match self.selected {
            true => "◉︎ ",
            false => "○︎ ",
        };
        let style: AppStyles = AppColors::get(self.is_active()).into();

        Line::from_iter([marker, radio, self.display.as_ref()])
            .style(style.highlight_style)
    }
}

impl<T> Field for Choice<T>
where
    T: Debug + Default + Copy,
{
    fn submit(&self) -> Result<()> {
        todo!()
    }
    fn get_rect_height(&self) -> u16 {
        Self::HEIGHT
    }
}
