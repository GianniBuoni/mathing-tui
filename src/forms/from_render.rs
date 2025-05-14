use super::*;

impl<T> WidgetRef for FormWidget<'_, T>
where
    T: Default + Debug,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::default().title(format!(" {}", self.title));

        let form_block = Block::bordered().border_type(BorderType::Rounded);

        let chunks = Layout::vertical(self.layout.iter())
            .spacing(1)
            .split(form_block.inner(area));

        chunks
            .iter()
            .zip(self.inputs.iter())
            .for_each(|(chunk, widget)| {
                widget.render(*chunk, buf);
            });

        form_block.render(popup_block.inner(area), buf);
        popup_block.render(area, buf);
    }
}
