use std::{borrow::Cow, rc::Rc};

use rust_decimal::Decimal;

use crate::prelude::*;

pub struct AppColors {
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    row_bg: Color,
    selected_row_fg: Color,
}

impl AppColors {
    fn new() -> Self {
        Self {
            header_bg: Color::Yellow,
            header_fg: Color::Black,
            row_fg: Color::Reset,
            row_bg: Color::Reset,
            selected_row_fg: Color::Black,
        }
    }
}

pub struct TableData<T>
where
    T: TableDisplay + Sized,
{
    state: TableState,
    items: Rc<[T]>,
    colors: AppColors,
}

impl<T> TableData<T>
where
    T: TableDisplay,
{
    pub fn new(items: impl Into<Rc<[T]>>) -> Self {
        let items: Rc<[T]> = items.into();
        Self {
            state: TableState::default(),
            items,
            colors: AppColors::new(),
        }
    }
}

pub trait TableView {
    fn render_table(&mut self, area: Rect, buf: &mut Buffer);
    fn next_row(&mut self);
    fn prev_row(&mut self);
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

impl<T> TableView for TableData<T>
where
    T: TableDisplay,
{
    fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        let highlight_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_fg);

        let row_style = Style::default()
            .fg(self.colors.row_fg)
            .bg(self.colors.row_bg);

        let header = ["Item Name", "Price"]
            .into_iter()
            .map(Cell::from)
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
            StatefulWidget::render(t, area, buf, &mut self.state);
        }
    }
    fn next_row(&mut self) {
        todo!()
    }
    fn prev_row(&mut self) {
        todo!()
    }
}
