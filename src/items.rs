use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Items;
}

pub struct Items {
    title: String,
    index: u8,
    active: bool,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            title: "Grocery Items".into(),
            index: 0,
            active: false,
        }
    }
}

impl Model for Items {
    fn title(&self) -> String {
        format!(" [{}] {} ", self.index, self.title)
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn index(&self) -> u8 {
        self.index
    }
    fn toggle(&mut self) {
        self.active = !self.active
    }
}
