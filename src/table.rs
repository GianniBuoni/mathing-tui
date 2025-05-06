use std::{borrow::Cow, ops::Deref, rc::Rc};

use rust_decimal::Decimal;

use crate::prelude::*;

pub struct TableData<'a, T>
where
    T: TableDisplay + Sized,
{
    headings: Rc<[Cow<'a, str>]>,
    items: Rc<[T]>,
    table_index: usize,
    active: bool,
}

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn new(
        headings: Rc<[Cow<'a, str>]>,
        items: impl Into<Rc<[T]>>,
        active: bool,
    ) -> Self {
        let items: Rc<[T]> = items.into();

        Self {
            headings,
            items,
            table_index: 0,
            active,
        }
    }
}

pub trait TableDisplay {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

pub struct MockItems {
    name: String,
    price: Decimal,
}

impl MockItems {
    pub fn new(name: &str, price: Decimal) -> Self {
        let name = name.into();
        Self { name, price }
    }
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(self.name.as_str())
    }
    fn price(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self.price))
    }
}

impl TableDisplay for MockItems {
    fn ref_array(&self) -> Vec<Cow<str>> {
        vec![self.name(), self.price()]
    }
}

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

        let t = Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(header)
        .row_highlight_style(highlight_style);

        {
            use ratatui::widgets::StatefulWidget;
            let mut state = TableState::new().with_selected(self.table_index);
            StatefulWidget::render(t, area, buf, &mut state);
        }
    }
    pub fn next_row(&mut self) {
        todo!()
    }
    pub fn prev_row(&mut self) {
        todo!()
    }
    pub fn sync_block(&mut self, active: bool) {
        self.active = active;
    }
}
