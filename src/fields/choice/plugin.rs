use super::*;

impl<T> PluginInit for Choice<T>
where
    T: Debug + Default + Copy,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.index = index;
        self.active_choice = tracker;
    }
}

impl<T> Plugin for Choice<T>
where
    T: Debug + Default + Copy,
{
    type Parent = SelectionBuilder<T>;

    fn plugin(self, parent: &mut Self::Parent) -> Result<()> {
        parent.choices.push(self);
        Ok(())
    }

    fn plugin_group(parent: &mut Self::Parent) -> Result<()> {
        let _ = parent;
        todo!()
    }
}
