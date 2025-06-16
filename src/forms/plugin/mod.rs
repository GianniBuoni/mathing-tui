use super::*;

mod new_item;
mod new_receipt;
mod new_user;
mod update_item;

impl Form {
    fn form_rect(height: u16) -> Rect {
        Rect::new(0, 0, 60, height)
    }
}

impl FormBuilder {
    fn try_map_input<T>(
        &mut self,
        value: &ParamOption<T>,
        field_name: impl Display,
    ) -> Result<()>
    where
        T: Debug + Default + Clone + FromStr + 'static,
        <T as FromStr>::Err: Debug,
    {
        match value.clone_inner() {
            Ok(value) => {
                let field = InputField::<T>::new(field_name).map_value(value);
                self.add_field(field);
                Ok(())
            }
            Err(_) => Err(FormErrors::unmapped(field_name).into()),
        }
    }
}
