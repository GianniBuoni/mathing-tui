use super::*;

impl App {
    pub fn register_model(
        &mut self,
        key: CurrentModel,
        model: Box<dyn Model>,
    ) -> Result<(), Box<dyn Error>> {
        // early return if key is already registered
        match self.models.contains_key(&key) {
            true => return Err("Key already is registered".into()),
            false => {
                self.models.insert(key, model);
            }
        }

        Ok(())
    }

    pub fn list_models(&self) -> Vec<&dyn Model> {
        let mut models = self
            .models
            .values()
            .map(|model| model.as_ref())
            .collect::<Vec<&dyn Model>>();
        models.sort_by_key(|model| model.index());
        models
    }
}
