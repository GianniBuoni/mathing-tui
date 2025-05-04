use rust_decimal::dec;

use crate::{
    prelude::*,
    table::{MockItems, TableData, TableView},
    ui::model_block,
};

pub(crate) mod prelude {
    pub(crate) use super::Items;
}

pub struct Items {
    title: String,
    index: u8,
    active: bool,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            title: "Grocery Items".into(),
            index: 0,
            active: false,
        }
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
        self.active = !self.active
    }
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = model_block(self).padding(Padding::uniform(1));
        let inner_area = block.inner(area);

        let display_items = [
            MockItems::new("Slamon", dec!(9.49)),
            MockItems::new("Pretzels", dec!(5.59)),
            MockItems::new("Blueberries", dec!(4.59)),
        ];

        let mut t = TableData::new(display_items);
        t.render_table(inner_area, buf);
        block.render(area, buf);
    }
}
