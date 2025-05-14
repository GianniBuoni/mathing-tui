use super::*;

impl WidgetRef for FormWidget<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::default().title(format!(" {}", self.title));
        let inner_popup = popup_block.inner(area);
        let form_block = Block::bordered().border_type(BorderType::Rounded);

        let chunks = Layout::vertical(self.layout.iter())
            .split(form_block.inner(inner_popup));

        chunks
            .iter()
            .zip(self.inputs.iter())
            .for_each(|(chunk, widget)| {
                widget.render(*chunk, buf);
            });

        popup_block.render(area, buf);
        form_block.render(inner_popup, buf);
    }
}
