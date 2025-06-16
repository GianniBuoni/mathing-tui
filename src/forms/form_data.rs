use ratatui::text::ToLine;

use super::*;

impl Component for Form {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        // get areas
        let [centered_area, title, form_block_area, error_area] =
            self.get_block_areas(rect);

        // render Clear in centered area
        frame.render_widget(Clear, centered_area);

        Line::from(self.title.as_ref()).render(title, frame.buffer_mut());

        let error = match &self.error {
            Some(e) => e
                .to_line()
                .style(Into::<AppStyles>::into(AppColors::ACTIVE).error_style),
            None => "".to_line(),
        };

        error.render(error_area, frame.buffer_mut());

        // render nested field blocks in form block
        let field_areas =
            self.render_block(form_block_area, frame.buffer_mut());

        self.fields.iter_mut().zip(field_areas.iter()).for_each(
            |(field, field_area)| {
                field.draw(frame, *field_area);
            },
        );
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SelectForward) => {
                self.cycle_active(1);
                self.fields.iter_mut().for_each(|f| f.handle_action(action));
            }
            Some(Action::SelectBackward) => {
                self.cycle_active(-1);
                self.fields.iter_mut().for_each(|f| f.handle_action(action));
            }
            Some(_) => {
                if let Some(active) =
                    self.fields.get_mut(*self.active_field.borrow())
                {
                    active.handle_action(action);
                }
            }
            None => {}
        }
    }
}

impl Form {
    pub fn render_block(&self, area: Rect, buf: &mut Buffer) -> Rc<[Rect]> {
        let block = Block::bordered().border_type(BorderType::Rounded);

        // split inner_area equally and return rects
        let constraints = self
            .fields
            .iter()
            .map(|_| Constraint::Fill(1))
            .collect::<Vec<Constraint>>();
        let areas = Layout::vertical(constraints).split(block.inner(area));

        // render block before returning
        block.render(area, buf);
        areas
    }

    pub fn get_block_areas(&self, full_area: Rect) -> [Rect; 4] {
        // center self.rect
        let centered_area = center_widget(
            full_area,
            Constraint::Length(self.rect.width),
            Constraint::Length(self.rect.height),
        );

        // add padding
        let form_area = Block::new()
            .padding(Padding::uniform(1))
            .inner(centered_area);

        // split new block into [Line, Block, Line]
        let [title, form_block_area, error_area]: [Rect; 3] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(form_area);

        [centered_area, title, form_block_area, error_area]
    }

    pub fn submit(&mut self) -> Result<()> {
        if self.fields.is_empty() {
            return Err(FormErrors::malformed("fields").into());
        }
        if self.request_type == RequestType::None {
            return Err(FormErrors::malformed("request type").into());
        }
        self.fields.iter().try_for_each(|f| f.submit())?;

        Ok(())
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

    pub fn map_err(&mut self, err: Option<anyhow::Error>) {
        match err {
            None => self.error = None,
            Some(e) => self.error = Some(format!(" {} ", e)),
        }
    }
}
