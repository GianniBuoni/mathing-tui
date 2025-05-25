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

        if let (Some(block), Some(area)) = (blocks.last(), areas.last()) {
            let field_areas = self.render_feild_areas(block.inner(*area));

            // render feilds
            self.fields.iter_mut().zip(field_areas.iter()).for_each(
                |(field, area)| {
                    field.draw(frame, *area);
                },
            );
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SelectForward) => {
                self.cycle_active(1);
                self.fields.iter_mut().for_each(|f| f.update(action));
            }
            Some(Action::SelectBackward) => {
                self.cycle_active(-1);
                self.fields.iter_mut().for_each(|f| f.update(action));
            }
            Some(_) => {
                if let Some(active) =
                    self.fields.get_mut(*self.active_field.borrow())
                {
                    active.update(action);
                }
            }
            None => {}
        }
    }
}

impl Form<'_> {
    pub fn submit(&self) -> Result<Rc<[FormValue]>> {
        self.fields
            .iter()
            .map(|input| TryInto::<FormValue>::try_into(&input.input))
            .collect()
    }

    pub fn cycle_active(&mut self, add: i32) {
        if self.fields.is_empty() {
            return;
        }

        let max = self.fields.len() - 1;
        let mut current_index = self.active_field.borrow_mut();

        match *current_index as i32 + add {
            int if int > max as i32 => *current_index = 0,
            int if int < 0 => *current_index = max,
            _ => *current_index = (*current_index as i32 + add) as usize,
        }
    }

    fn check_active(&mut self) {}
}
