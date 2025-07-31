use std::ops::Deref;

use super::*;

impl Message {
    pub(super) fn new_error(message: impl Display) -> Self {
        Self::Error(message.to_string().into())
    }
    pub(super) fn render(&self, rect: Rect, buf: &mut Buffer) {
        match self {
            Self::Confirmation(str) => {
                Line::from(str.deref()).render(rect, buf);
            }
            Self::Error(str) => {
                Line::from(str.deref()).red().render(rect, buf);
            }
            Self::Paragraph(strs) => {
                let lines = strs
                    .iter()
                    .map(|(str, color)| {
                        Line::from(str.deref())
                            .style(Style::default().fg(*color))
                    })
                    .collect::<Vec<Line>>();
                Text::from(lines).render(rect, buf);
            }
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::Confirmation(Rc::default())
    }
}

impl From<Vec<(Rc<str>, Color)>> for Message {
    fn from(value: Vec<(Rc<str>, Color)>) -> Self {
        match value.len() {
            1 => Self::Confirmation(value.first().unwrap().0.clone()),
            _ => Self::Paragraph(value.into()),
        }
    }
}
