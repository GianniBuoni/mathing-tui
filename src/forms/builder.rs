use super::*;

impl FormBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_title(&mut self, title: impl Display) -> &mut Self {
        let title = format!(" {} ", title);
        self.title = title.into();
        self
    }
    pub fn with_request_type(&mut self, req_type: RequestType) -> &mut Self {
        self.request_type = req_type;
        self
    }
    pub fn with_field(&mut self, field: impl Field + 'static) -> &mut Self {
        self.fields.push(Box::new(field) as Box<dyn Field>);
        self
    }

    pub fn with_form_type(&mut self, form_type: AppArm) -> &mut Self {
        match form_type {
            AppArm::Items => {
                let params = ItemParams::builder();
                self.payload = Some(DbPayloadBuilder::ItemParams(params))
            }
            AppArm::Receipts => {
                let params = JoinedReceiptParams::builder();
                self.payload = Some(DbPayloadBuilder::ReceiptParams(params))
            }
            AppArm::Users => {
                let params = UserParams::builder();
                self.payload = Some(DbPayloadBuilder::UserParams(params))
            }
        }
        self.form_type = Some(form_type);
        self
    }

    pub fn calc_rect(&mut self) -> Option<Rect> {
        let height = self
            .fields
            .iter()
            .map(|f| f.get_rect_height())
            .reduce(|acc, f| acc + f)?;

        Some(Rect::new(0, 0, Form::WIDTH, height + Form::HEIGHT))
    }
}

impl PluginParent for FormBuilder {}

impl ComponentBuilder for FormBuilder {
    type Output = Form;
    fn build(mut self) -> Result<Self::Output> {
        if self.request_type == RequestType::None {
            let err = FormErrors::malformed("request type").into();
            return Err(err);
        }
        let Some(_) = self.form_type else {
            let err = FormErrors::malformed("form type").into();
            return Err(err);
        };
        let Some(_) = self.payload else {
            let err = FormErrors::malformed("payload").into();
            return Err(err);
        };
        if self.fields.is_empty() {
            let err = FormErrors::malformed("fields").into();
            return Err(err);
        }
        let Some(rect) = self.calc_rect() else {
            let err = FormErrors::malformed("rect").into();
            return Err(err);
        };

        self.fields
            .iter_mut()
            .enumerate()
            .for_each(|(index, f)| f.init(index, self.active_field.clone()));

        Ok(Form {
            error: None,
            fields: self.fields,
            title: self.title,
            active_field: self.active_field,
            rect,
            request_type: self.request_type,
            payload: self.payload,
        })
    }
}
