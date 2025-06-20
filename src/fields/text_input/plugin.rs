use super::*;

impl<T> PluginInit for InputField<T>
where
    T: Debug + Default + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.index = index;
        self.active_field = tracker;
    }
}

impl<T> Plugin for InputField<T>
where
    T: Debug + Default + Clone + FromStr + 'static,
    <T as FromStr>::Err: Debug,
{
    type Parent = FormBuilder;

    fn plugin(self, parent: &mut Self::Parent) {
        let Some(field_type) = &self.field_type else {
            return;
        };
        let Some(form_type) = &parent.form_type else {
            return;
        };
        if field_type == form_type {
            parent.with_field(self);
        }
    }
    fn plugin_group(parent: &mut Self::Parent) {
        let _ = parent;
        todo!()
    }
}

pub fn new_item(parent: &mut FormBuilder) {
    let Some(DbPayloadBuilder::ItemParams(params)) = &mut parent.payload else {
        return;
    };

    let mut name_input = InputField::<String>::new();
    name_input
        .with_title("Item Name")
        .with_field_type(AppArm::Items);
    let name = name_input.value.clone();

    let mut price_input = InputField::<f64>::new();
    price_input
        .with_title("Item Price")
        .with_field_type(AppArm::Items);
    let price = price_input.value.clone();

    params.item_name(name);
    params.item_price(price);
    name_input.plugin(parent);
    price_input.plugin(parent);
}
