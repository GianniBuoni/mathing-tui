use super::*;

impl Form {
    pub fn builder() -> FormBuilder {
        FormBuilder::default()
    }
    pub fn render_block(&self, area: Rect, buf: &mut Buffer) -> Rc<[Rect]> {
        let block = Block::bordered().border_type(BorderType::Rounded);

        // split inner_area equally and return rects
        let constraints = self
            .fields
            .iter()
            .map(|f| Constraint::Length(f.get_rect_height()))
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
        self.fields.iter().try_for_each(|f| f.submit())?;

        Ok(())
    }

    pub fn cycle_active(&mut self, add: i32) {
        if self.fields.is_empty() {
            return;
        }
        let max = self.fields.len() - 1;
        let new_index = self.active_field.inner() as i32 + add;

        match new_index {
            int if int > max as i32 => self.active_field.go_to(0),
            int if int < 0 => self.active_field.go_to(max),
            _ => self.active_field.go_to(new_index as usize),
        }
    }

    pub fn map_err(&mut self, err: impl Display) {
        self.error = Some(format!(" {err} "))
    }

    pub fn try_get_payload(&self) -> Result<DbPayload> {
        self.payload
            .as_ref()
            .map(|payload| payload.build())
            .ok_or(FormError::malformed("payload").into())
    }

    pub fn get_req_type(&self) -> RequestType {
        self.request_type
    }
}
