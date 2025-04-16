use crate::prelude::*;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Span::styled(" Hello World ", Style::default().bold().bg(Color::Green))
            .render(area, buf);
    }
}
