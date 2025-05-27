use crate::prelude::*;

/// Provide a width and a height of the inner rect.
/// Centering on the vertical with non-perfect numbers adds
/// extra line to the y of the rect.
/// Centering on the horizontal with non-perfect numbers adds
/// extra to the left of the rect.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centering_rects() {
        let test_cases = [
            (
                Rect::new(0, 0, 56, 11),
                Rect::new(0, 0, 50, 8),
                Rect::new(3, 2, 50, 8),
                "Test not quite centerable height",
            ),
            (
                Rect::new(0, 0, 57, 10),
                Rect::new(0, 0, 50, 8),
                Rect::new(4, 1, 50, 8),
                "Test not quite centerable width",
            ),
            (
                Rect::new(0, 0, 56, 10),
                Rect::new(0, 0, 50, 8),
                Rect::new(3, 1, 50, 8),
                "Test perfecty centerable rect",
            ),
        ];

        test_cases.iter().for_each(|(outer, inner, want, desc)| {
            let got = center_widget(
                *outer,
                Constraint::Length(inner.width),
                Constraint::Length(inner.height),
            );

            assert_eq!(want.x, got.x, "{desc}; field x");
            assert_eq!(want.y, got.y, "{desc} field y");
            assert_eq!(want.width, got.width, "{desc} field width");
            assert_eq!(want.height, got.height, "{desc} field height");
        });
    }
}
