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
                    form.add_field(self);
                }
            }
            FormTui::ItemForm(form) => {
                if let DbPayloadBuilder::ItemParams(i) =
                    form.payload.as_mut().unwrap()
                {
                    i.item_name(self.value.clone());
                    form.add_field(self);
                }
            }
            _ => {}
        }
    }
}

impl Plugin for InputField<f64> {
    type Parent = FormTui;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.init_payload();
        let FormTui::ItemForm(form) = parent else {
            return;
        };
        if let DbPayloadBuilder::ItemParams(i) = form.payload.as_mut().unwrap()
        {
            i.item_price(self.value.clone());
            form.add_field(self);
        }
    }
}

impl Plugin for InputField<i64> {
    type Parent = FormTui;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.init_payload();
        let FormTui::ReceiptForm(form) = parent else {
            return;
        };
        if let DbPayloadBuilder::ReceiptParams(r) =
            form.payload.as_mut().unwrap()
        {
            r.item_qty(self.value.clone());
            form.add_field(self);
        }
    }
}
