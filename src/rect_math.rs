use crate::prelude::*;

pub fn center_widget(
    area: Rect,
    horizontal: Constraint,
    vertical: Constraint,
) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::Center)
        .areas(area);
    area
}
