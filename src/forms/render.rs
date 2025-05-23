use super::*;

impl Form<'_> {
    pub fn render_block(&self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::new().title(format!(" {} ", self.title));
        let popup_area = center_widget(
            area,
            Constraint::Length(self.rect.width),
            Constraint::Length(self.rect.height),
        );

        let bordered_block = Block::bordered().border_type(BorderType::Rounded);
        let bordered_area = popup_block.inner(popup_area);

        popup_block.render(popup_area, buf);
        bordered_block.render(bordered_area, buf);
    }
}

impl FormField<'_> {
    pub fn render_block(&self, area: Rect, buf: &mut Buffer) {
        let styles: AppStyles = AppColors::get(self.active).into();

        let field_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", self.title))
            .style(styles.block_style);

        let input_area = field_block.inner(area);
        let input = Paragraph::new(self.input.value())
            .style(styles.input_style)
            .add_modifier(Modifier::RAPID_BLINK);

        input.render(input_area, buf);
        field_block.render(area, buf);
    }
}
