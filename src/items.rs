use rust_decimal::dec;

use crate::{
    prelude::*,
    table::{MockItems, TableData},
    ui::model_block,
};

pub(crate) mod prelude {
    pub(crate) use super::Items;
}

pub struct Items {
    table: TableData<MockItems>,
    title: String,
    index: u8,
    active: bool,
}

impl Items {
    pub fn new() -> Self {
        let display_items = [
            MockItems::new("Slamon", dec!(9.49)),
            MockItems::new("Pretzels", dec!(5.59)),
            MockItems::new("Blueberries", dec!(4.59)),
        ];

        let active = false;
        let table = TableData::new(display_items, active);

        Self {
            title: "Grocery Items".into(),
            index: u8::default(),
            active,
            table,
        }
    }
}

impl WidgetRef for Items {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = model_block(self).padding(Padding::uniform(1));
        let inner_area = block.inner(area);

        self.table.render_table(inner_area, buf);
        block.render(area, buf);
    }
}

impl Model for Items {
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
}
