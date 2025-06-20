use super::*;

impl<T> Choice<T>
where
    T: Debug + Default,
{
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
    pub fn is_selected(&self) -> bool {
        self.selected_choice.inner() == self.index
    }

    pub fn get_display(&self) -> Line {
        let marker = match self.is_active() {
            true => "> ",
            false => "  ",
        };
        let radio = match self.is_selected() {
            true => "◉︎ ",
            false => "○︎ ",
        };
        let style: AppStyles = AppColors::get(self.is_active()).into();

        Line::from_iter([marker, radio, self.display.as_ref()])
            .style(style.highlight_style)
    }
}

impl<T> Component for Choice<T>
where
    T: Debug + Default,
{
    fn handle_action(&mut self, action: Option<Action>) {
        let Some(action) = action else {
            return;
        };
        let Action::HandleInput(key_event) = action else {
            return;
        };
        if let KeyCode::Char(' ') = key_event.code {
            self.selected_choice.go_to(self.index);
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.get_display().render(rect, frame.buffer_mut());
    }
    fn is_active(&self) -> bool {
        self.active_choice.inner() == self.index
    }
}

impl<T> Field for Choice<T>
where
    T: Debug + Default,
{
    fn submit(&self) -> Result<()> {
        todo!()
    }
    fn get_rect_height(&self) -> u16 {
        todo!()
    }
}

impl<T> PluginInit for Choice<T>
where
    T: Debug + Default,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.index = index;
        self.active_choice = tracker;
    }
}

// each choice type will need its own implementation of Plugin
