use super::*;

#[derive(Debug, Default)]
pub struct AppBuilder {
    models: HashMap<CurrentModel, Box<dyn Model>>,
    forms: HashMap<(CurrentModel, FormAction), Box<dyn Form>>,
}

impl App {
    /// Creates an [`AppBuilder`] for App, AppBuilder's `build` method
    /// must be called to finish the process for constucting an App struct.
    pub fn new_builder() -> AppBuilder {
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
            true => Err("Key already is registered".into()),
            false => {
                self.models.insert(key, model);
                Ok(self)
            }
        }
    }
    pub fn build(self) -> App {
        let mut app = App {
            models: self.models,
            ..Default::default()
        };
        app.toggle_current_model();
        app
    }
}
