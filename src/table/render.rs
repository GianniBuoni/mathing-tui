use super::*;

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    pub(super) fn render_block<'a>(&'a self, fg: &Color) -> Block<'a> {
        Block::bordered()
            .border_style(Style::default().fg(*fg))
            .border_type(BorderType::Rounded)
            .title(self.title())
    }

    pub(super) fn render_rows(&self, styles: &AppStyles) -> Vec<Row> {
        self.items
            .iter()
            .map(|data| {
                data.ref_array()
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Row>()
                    .style(styles.row_style)
            })
            .collect()
    }

    pub(super) fn render_heading(&self, styles: &AppStyles) -> Row {
        self.headings
            .iter()
            .map(|cow| Cell::from(cow.deref()))
            .collect::<Row>()
            .style(styles.header_style)
            .height(1)
    }

    pub(super) fn render_table(&self, styles: &AppStyles) -> Table {
        Table::new(
            self.render_rows(styles),
            Constraint::from_fills(vec![1; self.headings.len()]),
        )
        .header(self.render_heading(styles))
        .row_highlight_style(styles.highlight_style)
    }
}
