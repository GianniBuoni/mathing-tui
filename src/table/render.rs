use super::*;

impl<T> WidgetRef for TableData<'_, T>
where
    T: TableDisplay,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let styles: AppTableStyles = AppColors::get(self.active).into();

        let block = self
            .render_block(&styles.block_style)
            .padding(Padding::proportional(1));

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(
                self.render_table(&styles),
                block.inner(area),
                buf,
                &mut state,
            );
        }

        block.render(area, buf)
    }
}

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    pub(super) fn render_block<'a>(&'a self, block_style: &Style) -> Block<'a> {
        Block::bordered()
            .border_style(*block_style)
            .border_type(BorderType::Rounded)
            .title(self.title())
    }

    pub(super) fn render_rows(&self, styles: &AppTableStyles) -> Vec<Row> {
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

    pub(super) fn render_heading(&self, styles: &AppTableStyles) -> Row {
        self.headings
            .iter()
            .map(|cow| Cell::from(cow.deref()))
            .collect::<Row>()
            .style(styles.header_style)
            .height(1)
    }

    pub(super) fn render_table(&self, styles: &AppTableStyles) -> Table {
        Table::new(
            self.render_rows(styles),
            Constraint::from_fills(vec![1; self.headings.len()]),
        )
        .header(self.render_heading(styles))
        .row_highlight_style(styles.highlight_style)
    }
}
