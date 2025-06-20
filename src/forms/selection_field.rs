use super::*;

impl<T> Component for SelectionField<T>
where
    T: Debug + Default,
{
    fn handle_action(&mut self, action: Option<Action>) {
        todo!()
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        todo!()
    }
    fn is_active(&self) -> bool {
        todo!()
    }
}

impl<T> Field for SelectionField<T>
where
    T: Debug + Default,
{
    fn submit(&self) -> Result<()> {
        todo!()
    }
}

impl<T> PluginInit for SelectionField<T>
where
    T: Debug + Default,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        todo!()
    }
}
