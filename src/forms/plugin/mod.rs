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

impl Plugin for InputField<String> {
    type Parent = FormTui;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.init_payload();
        match parent {
            FormTui::UserFrom(form) => {
                if let DbPayloadBuilder::UserParams(u) =
                    form.payload.as_mut().unwrap()
                {
                    u.user_name(self.value.clone());
                }
                form.add_field(self);
            }
            FormTui::ItemForm(form) => {
                if let DbPayloadBuilder::ItemParams(i) =
                    form.payload.as_mut().unwrap()
                {
                    i.item_name(self.value.clone());
                }
                form.add_field(self);
            }
            _ => {}
        }
    }
}

impl Plugin for InputField<f64> {
    type Parent = FormTui;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.init_payload();
        if let FormTui::ItemForm(form) = parent {
            if let DbPayloadBuilder::ItemParams(i) =
                form.payload.as_mut().unwrap()
            {
                i.item_price(self.value.clone());
            }
            form.add_field(self);
        }
    }
}

impl Plugin for InputField<i64> {
    type Parent = FormTui;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.init_payload();

        match parent {
            FormTui::UserFrom(form) => {
                if let DbPayloadBuilder::UserParams(u) =
                    form.payload.as_mut().unwrap()
                {
                    u.user_id(self.value.clone());
                }
                form.add_field(self);
            }
            FormTui::ItemForm(form) => {
                if let DbPayloadBuilder::ItemParams(i) =
                    form.payload.as_mut().unwrap()
                {
                    i.item_id(self.value.clone());
                }
                form.add_field(self);
            }
            FormTui::ReceiptForm(form) => {
                if let DbPayloadBuilder::ReceiptParams(r) =
                    form.payload.as_mut().unwrap()
                {
                    match form.fields.len() {
                        0 => {
                            r.item_id(self.value.clone());
                        }
                        1 => {
                            r.item_qty(self.value.clone());
                        }
                        3 => {
                            r.r_id(self.value.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
