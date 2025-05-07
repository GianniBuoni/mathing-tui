use std::borrow::Cow;

use super::*;

use rust_decimal::dec;

impl Default for App {
    fn default() -> Self {
        let mut models = HashMap::new();

        let mock_items = [
            MockItems::new("Slamon", dec!(9.49)),
            MockItems::new("Pretzels", dec!(5.59)),
            MockItems::new("Blueberries", dec!(4.59)),
        ];
        let item_headings = [" Items ", " Price "]
            .iter()
            .map(|string| Cow::Borrowed(*string))
            .collect();
        let items = Box::new(TableData::new(
            "Grocery Items",
            item_headings,
            mock_items,
            0,
        ));

        let mock_receipt = [
            MockReceipt::new("Slamon", "Jon, Noodle", dec!(9.49), 1),
            MockReceipt::new("Blueberries", "Jon", dec!(5.59), 4),
        ];
        let rec_headings =
            [" Item Name ", " Item Price ", " Item Qty ", " Payees "]
                .iter()
                .map(|string| Cow::Borrowed(*string))
                .collect();
        let receipt = Box::new(TableData::new(
            "Receipt Items",
            rec_headings,
            mock_receipt,
            1,
        ));

        models.insert(CurrentModel::Items, items as Box<dyn Model>);
        models.insert(CurrentModel::Receipt, receipt as Box<dyn Model>);

        let mut app = Self {
            models,
            current_model: CurrentModel::default(),
            should_exit: bool::default(),
        };

        app.init_view();
        app
    }
}
