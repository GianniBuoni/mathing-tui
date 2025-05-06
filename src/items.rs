use std::borrow::Cow;

use rust_decimal::dec;

use crate::{prelude::*, ui::model_block};

pub(crate) mod prelude {
    pub(crate) use super::Items;
}

pub struct Items<'a> {
    table: TableData<'a, MockItems>,
    title: String,
    index: u8,
    active: bool,
}

impl<'a> Items<'a> {
    pub fn new() -> Self {
        let display_items = [
            MockItems::new("Slamon", dec!(9.49)),
            MockItems::new("Pretzels", dec!(5.59)),
            MockItems::new("Blueberries", dec!(4.59)),
        ];

        let active = false;

        let headings = ["Items", "Price"]
            .iter()
            .map(|string| Cow::Borrowed(*string))
            .collect();

        let table = TableData::new(headings, display_items, active);

        Self {
            title: "Grocery Items".into(),
            index: u8::default(),
            active,
            table,
        }
    }
}

impl<'a> WidgetRef for Items<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = model_block(self).padding(Padding::uniform(1));
        let inner_area = block.inner(area);

        self.table.render_table(inner_area, buf);
        block.render(area, buf)
    }
}

impl<'a> Model for Items<'a> {
    fn title(&self) -> String {
        format!(" [{}] {} ", self.index, self.title)
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn index(&self) -> u8 {
        self.index
    }
    fn toggle(&mut self) {
        self.active = !self.active;
        self.table.sync_block(self.active)
    }
    fn next_row(&mut self) {
        self.table.next_row();
    }
    fn prev_row(&mut self) {
        self.table.prev_row();
    }
}
