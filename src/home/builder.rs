use super::*;

impl HomeBuilder {
    pub fn add_component(
        &mut self,
        component: impl Component + 'static,
    ) -> &mut Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn add_request_handler(
        &mut self,
        req_tx: UnboundedSender<DbRequest>,
    ) -> &mut Self {
        self.req_tx = Some(req_tx);
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
            req_tx: self.req_tx,
            component_tracker: self.component_tracker,
            keymap: self.keymap,
            ..Default::default()
        }
    }

    fn add_key_event_handler(
        &mut self,
        keymap: HashMap<KeyEvent, Action>,
    ) -> &mut Self {
        self.keymap = keymap;
        self
    }
}
