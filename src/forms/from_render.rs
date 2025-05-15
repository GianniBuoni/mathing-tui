use super::*;

impl<T> WidgetRef for FormWidget<'_, T>
where
    T: Debug + Default,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let widget_area = center_widget(
            area,
            Constraint::Length(self.area.width),
            Constraint::Length(self.area.height),
        );

        let popup_block = Block::default().title(format!(" {}", self.title));
        let inner_popup = popup_block.inner(widget_area);
        let form_block = Block::bordered().border_type(BorderType::Rounded);

        let chunks = Layout::vertical(self.layout.iter())
            .split(form_block.inner(inner_popup));

        chunks
            .iter()
            .zip(self.inputs.iter())
            .for_each(|(chunk, widget)| {
                widget.render(*chunk, buf);
            });

        popup_block.render(widget_area, buf);
        form_block.render(inner_popup, buf);
    }
}
