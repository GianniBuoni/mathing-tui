use super::*;

impl Form<'_> {
    pub fn render_block(&self) -> Rc<[Block]> {
        let popup_block = Block::new().title(format!(" {} ", self.title));
        let bordered_block = Block::bordered().border_type(BorderType::Rounded);
        [popup_block, bordered_block].into()
    }

    pub fn render_block_areas(&self, outer: &Block, area: Rect) -> Rc<[Rect]> {
        let outer_area = center_widget(
            area,
            Constraint::Length(self.rect.width),
            Constraint::Length(self.rect.height),
        );

        let inner_area = outer.inner(outer_area);
        [outer_area, inner_area].into()
    }

    pub fn render_feild_areas(&self, area: Rect) -> Rc<[Rect]> {
        let divisions = self.fields.len();
        Layout::vertical(Constraint::from_lengths(vec![3; divisions]))
            .split(area)
    }
}

impl FormField<'_> {
    pub fn render_block(&self, style: Style) -> Block {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", self.title))
            .style(style)
    }

    pub fn render_input(&self, style: Style) -> Paragraph {
        Paragraph::new(self.input.value())
            .style(style)
            .add_modifier(Modifier::RAPID_BLINK)
    }
}
