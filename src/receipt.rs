use std::borrow::Cow;

use rust_decimal::dec;

use crate::{prelude::*, ui::model_block};

pub(crate) mod prelude {
    pub(crate) use super::Receipt;
}

pub struct Receipt<'a> {
    table: TableData<'a, MockReceipt>,
    title: String,
    index: u8,
    active: bool,
}

impl<'a> Default for Receipt<'a> {
    fn default() -> Self {
        let mock_receipts = [
            MockReceipt::new("Slamon", "Jon, Noodle", dec!(9.49), 1),
            MockReceipt::new("Blueberries", "Jon", dec!(5.59), 4),
        ];

        let active = false;
        let headings = ["Item Name", "Item Price", "Item Qty", "Payees"]
            .iter()
            .map(|string| Cow::Borrowed(*string))
            .collect();

        let table = TableData::new(headings, mock_receipts, active);

        Self {
            table,
            title: "Receipt".into(),
            index: 1,
            active,
        }
    }
}

impl<'a> WidgetRef for Receipt<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = model_block(self).padding(Padding::uniform(1));
        let inner_area = block.inner(area);

        self.table.render_table(inner_area, buf);
        block.render(area, buf)
    }
}

impl<'a> Model for Receipt<'a> {
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
