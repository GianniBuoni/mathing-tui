use super::*;

impl HomeBuilder {
    pub fn add_component(&mut self, component: TableData) -> &mut Self {
        self.components.push(component);
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

impl ComponentBuilder for HomeBuilder {
    type Output = Home;

    fn build(mut self) -> Result<Self::Output> {
        self.components.iter_mut().enumerate().for_each(
            |(index, component)| {
                component.init(index, self.component_tracker.clone())
            },
        );
        Ok(Home {
            components: self.components,
            req_tx: self.req_tx,
            component_tracker: self.component_tracker,
            ..Default::default()
        })
    }
}
