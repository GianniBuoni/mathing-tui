use super::*;

impl HomeBuilder {
    pub fn add_component(mut self, component: TableTui) -> Self {
        self.components.push(component);
        self
    }
}

impl ComponentBuilder<Home> for HomeBuilder {
    fn build(mut self) -> Home {
        self.components.iter_mut().enumerate().for_each(
            |(index, component)| {
                component.init(index, self.component_tracker.clone());
            },
        );
        Home {
            components: self.components,
            component_tracker: self.component_tracker,
            keymap: self.keymap,
            ..Default::default()
        }
    }

    fn add_key_event_handler(
        mut self,
        keymap: HashMap<KeyEvent, Action>,
    ) -> Self {
        self.keymap = keymap;
        self
    }
}
