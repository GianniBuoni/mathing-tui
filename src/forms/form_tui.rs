use super::*;

impl Component for FormTui {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.mut_inner(|f| f.draw(frame, rect));
    }
    fn handle_action(&mut self, action: Option<Action>) {
        self.mut_inner(|f| f.handle_action(action));
    }
}

impl FormTui {
    pub fn init_payload(&mut self) {
        match self {
            Self::UserFrom(u) => {
                if u.payload.is_none() {
                    u.payload = Some(DbPayloadBuilder::UserParams(
                        UserParams::builder(),
                    ))
                }
            }
            Self::ItemForm(i) => {
                if i.payload.is_none() {
                    i.payload = Some(DbPayloadBuilder::ItemParams(
                        ItemParams::builder(),
                    ))
                }
            }
            Self::ReceiptForm(r) => {
                if r.payload.is_none() {
                    r.payload = Some(DbPayloadBuilder::ReceiptParams(
                        JoinedReceiptParams::builder(),
                    ))
                }
            }
        }
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Form) -> Form,
    {
        match self {
            Self::UserFrom(u) => Self::UserFrom(f(u)),
            Self::ItemForm(i) => Self::ItemForm(f(i)),
            Self::ReceiptForm(r) => Self::ReceiptForm(f(r)),
        }
    }

    pub fn get_inner<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Form) -> T,
    {
        match self {
            Self::UserFrom(u) => f(u),
            Self::ItemForm(i) => f(i),
            Self::ReceiptForm(r) => f(r),
        }
    }

    pub fn mut_inner<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Form),
    {
        match self {
            Self::UserFrom(u) => f(u),
            Self::ItemForm(i) => f(i),
            Self::ReceiptForm(r) => f(r),
        }
    }

    pub fn try_mut_inner<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Form) -> Result<T>,
    {
        match self {
            Self::UserFrom(u) => f(u),
            Self::ItemForm(i) => f(i),
            Self::ReceiptForm(r) => f(r),
        }
    }
}

impl PluginParent for FormTui {}
