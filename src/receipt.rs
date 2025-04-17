use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Receipt;
}

pub struct Receipt {
    title: String,
    index: u8,
    active: bool,
}

impl Default for Receipt {
    fn default() -> Self {
        Self {
            title: "Receipt".into(),
            index: 1,
            active: false,
        }
    }
}

impl Model for Receipt {
    fn title(&self) -> String {
        format!(" [{}] {} ", self.index, self.title)
    }
    fn is_active(&self) -> bool {
        self.active
    }
}
