use super::*;

impl Default for App {
    fn default() -> Self {
        let mut models = HashMap::new();

        let items = Box::new(Items::new());
        let reciept = Box::new(Receipt::default());

        models.insert(CurrentModel::Items, items as Box<dyn Model>);
        models.insert(CurrentModel::Receipt, reciept as Box<dyn Model>);

        let mut app = Self {
            models,
            current_model: CurrentModel::default(),
            should_exit: bool::default(),
        };

        app.init_view();
        app
    }
}
