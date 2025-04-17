use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Receipt;
}

pub struct Receipt {
    index: u8,
    title: String,
}

impl Default for Receipt {
    fn default() -> Self {
        Self {
            index: 1,
            title: "Receipt".into(),
        }
    }
}

impl Model for Receipt {
    fn title(&self) -> String {
        format!(" [{}] {} ", self.index, self.title)
    }
    fn index(&self) -> u8 {
        self.index
    }
}
