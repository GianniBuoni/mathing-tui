use super::*;

impl<T> Plugin for SelectionField<T>
where
    T: Debug + Default + Copy + 'static,
{
    type Parent = FormBuilder;

    fn plugin(self, parent: &mut Self::Parent) -> Result<()> {
        parent.with_field(self);
        Ok(())
    }
    fn plugin_group(parent: &mut Self::Parent) -> Result<()> {
        let _ = parent;
        todo!()
    }
}
