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
        let Some(some_act) = action else {
            return;
        };
        match some_act {
            Action::SelectForward => {
                self.cycle_active(1);
                self.fields.iter_mut().for_each(|f| f.handle_action(action));
            }
            Action::SelectBackward => {
                self.cycle_active(-1);
                self.fields.iter_mut().for_each(|f| f.handle_action(action));
            }
            _ => {
                if let Some(active) =
                    self.fields.get_mut(self.active_field.inner())
                {
                    active.handle_action(action);
                }
            }
        }
    }

    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        let keymap = Config::get_config();
        let default = keymap.get(key);

        if self.fields.is_empty() {
            return default;
        }
        let Some(field) = self.fields.get(self.active_field.inner()) else {
            return default;
        };
        if !field.handles_input() {
            return default;
        }
        match (key.code, key.modifiers) {
            (
                KeyCode::Char(_) | KeyCode::Backspace,
                KeyModifiers::NONE | KeyModifiers::SHIFT,
            ) => Some(Action::HandleInput(key)),
            _ => default,
        }
    }
}
