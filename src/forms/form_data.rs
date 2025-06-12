use std::ops::Deref;

use super::*;

impl<T> Component for Form<T>
where
    T: Debug + Default,
{
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

impl<T> Form<T>
where
    T: Debug + Default,
{
    pub fn render_block(&self) -> Rc<[Block]> {
        let popup_block = Block::new().title(self.title.deref());
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
}

impl Form<ItemParams> {
    pub fn submit(&mut self) -> Result<()> {
        let _params = ItemParams::new();

        match self.request_type {
            RequestType::Get => {
                unimplemented!()
            }
            RequestType::Post => {
                if self.fields.is_empty() {
                    return Err(FormErrors::malformed("fields").into());
                }
                self.fields.iter().try_for_each(|f| f.submit())?;
            }
            RequestType::Delete => {
                unimplemented!()
            }
            RequestType::Update => {
                unimplemented!()
            }
            RequestType::None => {
                if self.fields.is_empty() {
                    return Err(FormErrors::malformed("fields").into());
                }
                return Err(FormErrors::malformed("request type").into());
            }
            _ => {}
        }

        Ok(())
    }
}

impl Form<UserParams> {
    pub fn submit(&mut self) -> Result<()> {
        todo!()
    }
}

impl Form<JoinedReceiptParams> {
    pub fn submit(&mut self) -> Result<()> {
        todo!()
    }
}
