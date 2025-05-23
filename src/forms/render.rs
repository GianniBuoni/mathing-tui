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
