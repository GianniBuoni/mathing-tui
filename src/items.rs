use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Items;
}

pub struct Items {
    index: u8,
    title: String,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            index: 0,
            title: "Grocery Items".into(),
        }
    }
}

impl Model for Items {
    fn index(&self) -> u8 {
        self.index
    }
    fn title(&self) -> String {
        format!(" [{}] {} ", self.index, self.title)
    }
}
