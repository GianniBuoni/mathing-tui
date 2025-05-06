use super::*;

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    pub fn render_table(&self, area: Rect, buf: &mut Buffer) {
        let colors = match self.active {
            true => AppColors::ACTIVE,
            false => AppColors::INACTIVE,
        };

        let header_style = Style::default()
            .fg(colors.header_fg)
            .bg(colors.header_bg)
            .bold();

        let highlight_style = Style::default().fg(colors.selected_row_fg);

        let row_style = Style::default().fg(colors.row_fg).bg(colors.row_bg);

        let header = self
            .headings
            .into_iter()
            .map(|cow| Cell::from(cow.deref()))
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = self.items.iter().map(|data| {
            data.ref_array()
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .style(row_style)
        });

        let fills = vec![1 as u16; self.headings.len()];

        let t = Table::new(rows, Constraint::from_fills(fills))
            .header(header)
            .row_highlight_style(highlight_style);

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(t, area, buf, &mut state);
        }
    }
}
