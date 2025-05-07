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
        .spacing(1)
        .areas(main_block.inner(area));

        self.list_models().into_iter().zip(chunks).for_each(
            |(model, inner_area)| {
                model.render_ref(inner_area, buf);
            },
        );

        main_block.render(area, buf);
    }
}
