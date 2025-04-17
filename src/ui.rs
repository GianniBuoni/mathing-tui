use crate::prelude::*;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let context_menu = Line::from(vec![
            " Quit ".gray(),
            "<q>".dark_gray(),
            " | ".gray(),
            "Switch pane ".gray(),
            "<Tab> ".dark_gray(),
        ])
        .centered();

        let main_block = Block::default().title_bottom(context_menu);

        let chunks: [Rect; 2] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .areas(main_block.inner(area));

        self.models
            .iter()
            .zip(chunks)
            .for_each(|((_, model), inner_area)| {
                let color = if model.is_active() {
                    Color::White
                } else {
                    Color::DarkGray
                };
                model_block(color, model).render(inner_area, buf);
            });

        main_block.render(area, buf);
    }
}

pub fn model_block<'a>(color: Color, model: &Box<dyn Model>) -> Block<'a> {
    Block::bordered()
        .border_style(Style::default().fg(color))
        .border_type(BorderType::Rounded)
        .title(model.title())
}
