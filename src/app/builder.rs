use super::*;

#[derive(Debug, Default)]
pub struct AppBuilder {
    models: HashMap<CurrentModel, Box<dyn Model>>,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder::default()
    }
}

impl AppBuilder {
    pub fn register_model(
        mut self,
        key: CurrentModel,
        model: Box<dyn Model>,
    ) -> Result<Self, Box<dyn Error>> {
        // early return if key is already registered
        match self.models.contains_key(&key) {
            true => return Err("Key already is registered".into()),
            false => {
                self.models.insert(key, model);
                Ok(self)
            }
        }
    }
    pub fn build(self) -> App {
        let mut app = App::default();
        app.models = self.models;
        app.toggle_current_model();
        app
    }
}
