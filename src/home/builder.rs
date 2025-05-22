use super::*;

impl<'a> HomeBuilder<'a> {
    pub fn add_component(mut self, component: TableTui<'a>) -> HomeBuilder<'a> {
        self.components.push(component);
        self
    }
}

impl<'a> ComponentBuilder<Home<'a>> for HomeBuilder<'a> {
    fn build(mut self) -> Home<'a> {
        self.components.iter_mut().for_each(|component| {
            component.add_tracker(self.component_tracker.clone());
            component.init();
        });
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
