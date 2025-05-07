use super::*;

impl<T> WidgetRef for TableData<'_, T>
where
    T: TableDisplay,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let colors = AppColors::get(self.active);

        let block = self
            .render_block(&colors.border_fg)
            .padding(Padding::proportional(1));

        let styles: AppTableStyles = colors.into();

        let inner_area = block.inner(area);

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(
                self.render_table(&styles),
                inner_area,
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
    pub fn render_block<'a>(&'a self, fg: &Color) -> Block<'a> {
        Block::bordered()
            .border_style(Style::default().fg(*fg))
            .border_type(BorderType::Rounded)
            .title(self.title())
    }

    pub fn render_rows(&self, styles: &AppTableStyles) -> Vec<Row> {
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

    pub fn render_headers(&self, styles: &AppTableStyles) -> Row {
        self.headings
            .iter()
            .map(|cow| Cell::from(cow.deref()))
            .collect::<Row>()
            .style(styles.header_style)
            .height(1)
    }

    pub fn render_table(&self, styles: &AppTableStyles) -> Table {
        Table::new(
            self.render_rows(styles),
            Constraint::from_fills(vec![1; self.headings.len()]),
        )
        .header(self.render_headers(styles))
        .row_highlight_style(styles.highlight_style)
    }
}
