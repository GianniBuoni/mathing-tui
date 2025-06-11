use super::*;

impl<T> TableData<T>
where
    T: TableDisplay,
{
    pub(super) fn render_block<'a>(&'a self, style: Style) -> Block<'a> {
        Block::bordered()
            .border_style(style)
            .border_type(BorderType::Rounded)
            .title(format!(" [{}] {} ", self.app_index, self.title))
    }

    pub(super) fn render_rows(&self, style: Style) -> Vec<Row> {
        self.items
            .iter()
            .map(|data| {
                data.ref_array().into_iter().collect::<Row>().style(style)
            })
            .collect()
    }

    pub(super) fn render_heading(&self, styles: &AppStyles) -> Row {
        self.headings
            .iter()
            .map(|heading| Cell::from(format!(" {} ", heading)))
            .collect::<Row>()
            .style(styles.header_style)
            .height(1)
    }

    pub(super) fn render_table(&self, styles: &AppStyles) -> Table {
        Table::new(
            self.render_rows(styles.row_style),
            Constraint::from_fills(vec![1; self.headings.len()]),
        )
        .header(self.render_heading(styles))
        .row_highlight_style(styles.highlight_style)
    }
}
