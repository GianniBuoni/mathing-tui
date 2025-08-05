use std::ops::Deref;

use super::*;

impl TableData {
    pub(super) fn render_block<'a>(&'a self, style: Style) -> Block<'a> {
        let current = self.count.min(self.limit * self.current_page);
        let title = format!(" {} of {} ", current, self.count);
        let title = Line::from(title).right_aligned();

        Block::bordered()
            .border_style(style)
            .border_type(BorderType::Rounded)
            .title(format!(" [{}] {} ", self.app_index, self.title))
            .title_bottom(title)
    }

    pub(super) fn render_rows(&self, style: Style) -> Vec<Row> {
        let mut rows = self
            .items
            .iter()
            .map(|data| data.into_row().style(style))
            .collect::<Vec<Row>>();

        if let Some(AppArm::Users) = self.table_type {
            (|| {
                let t = StoreTotal::try_get()?
                    .lock()
                    .map_err(|_| AppError::StoreTotalMutex)?;
                t.sum_total(&mut rows);
                Aok(())
            })()
            // TODO: handle the error properly
            .unwrap_or_default()
        }
        rows
    }

    pub(super) fn render_heading(&self, styles: &AppStyles) -> Row {
        self.headings
            .iter()
            .map(|heading| Cell::from(heading.deref()))
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
