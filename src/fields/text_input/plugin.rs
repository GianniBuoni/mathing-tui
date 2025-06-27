use super::*;

impl<T> PluginInit for InputField<T>
where
    T: Debug + Default + Display + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.index = index;
        self.active_field = tracker;
    }
}

impl<T> Plugin for InputField<T>
where
    T: Debug + Default + Display + Clone + FromStr + 'static,
    <T as FromStr>::Err: Debug,
{
    type Parent = FormBuilder;

    fn plugin(self, parent: &mut Self::Parent) -> Result<()> {
        let Some(input) = &self.field_type else {
            let e = FormError::malformed("field type").into();
            return Err(e);
        };
        let Some(form) = &parent.form_type else {
            let e = FormError::malformed("form type").into();
            return Err(e);
        };
        if !(input == form) {
            let e = AppError::Mapping(*input, *form).into();
            return Err(e);
        }
        parent.with_field(self);
        Ok(())
    }
    fn plugin_group(parent: &mut Self::Parent) -> Result<()> {
        let _ = parent;
        todo!()
    }
}
