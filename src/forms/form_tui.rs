use anyhow::Error;

use super::*;

impl Component for FormTui<'_> {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self {
            FormTui::ItemForm(i) => i.update(action),
            FormTui::ReceiptForm(r) => r.update(action),
        }
    }
}

impl<'a> FormTui<'a> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::ItemForm(form) => {
                let values = form.submit()?;
                let mut values = values.iter();

                if let Some(name) = values.next() {
                    match name {
                        FormValue::String(_) => {}
                        _ => return Err(Error::msg("")),
                    }
                };

                if let Some(price) = values.next() {
                    match price {
                        FormValue::Decimal(_) => {}
                        _ => return Err(Error::msg("")),
                    }
                };

                Ok(())
            }

            Self::ReceiptForm(form) => {
                let values = form.submit()?;
                let mut values = values.iter();

                if let Some(qty) = values.next() {
                    match qty {
                        FormValue::Integer(_) => {}
                        _ => return Err(Error::msg("")),
                    }
                };

                if let Some(payees) = values.next() {
                    match payees {
                        FormValue::String(_) => {}
                        _ => return Err(Error::msg("")),
                    }
                }

                Ok(())
            }
        }
    }
}
