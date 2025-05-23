use super::*;

impl Component for Form<'_> {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        // blocks and areas hard coded to return a len of 2
        let blocks = self.render_block();
        let areas = self.render_block_areas(blocks.first().unwrap(), rect);

        // clear area before rendering blocks
        frame.render_widget(Clear, *areas.first().unwrap());

        // render blocks
        blocks.iter().zip(areas.iter()).for_each(|(block, area)| {
            block.render(*area, frame.buffer_mut());
        });

        // render feilds
        let field_areas = self.render_feild_areas(*areas.last().unwrap());
        self.fields.iter_mut().zip(field_areas.iter()).for_each(
            |(field, area)| {
                field.draw(frame, *area);
            },
        );
    }

    fn update(&mut self, _action: Option<Action>) {
        todo!()
    }
}

impl Component for FormField<'_> {
    fn init(&mut self) {
        todo!()
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let style = Into::<AppStyles>::into(AppColors::get(self.active));

        let block = self.render_block(style.block_style);
        let input = self.render_input(style.input_style);

        block.render_ref(rect, frame.buffer_mut());
        input.render_ref(block.inner(rect), frame.buffer_mut());
    }

    fn update(&mut self, _action: Option<Action>) {
        todo!()
    }
}

impl Component for FormTui<'_> {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self {
            FormTui::ItemForm(i) => i.update(action),
            FormTui::ReceiptForm(r) => r.update(action),
        }
    }
}
