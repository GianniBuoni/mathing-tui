use super::*;

impl App {
    pub fn new(home: Home) -> Self {
        App {
            component: home,
            ..Default::default()
        }
    }
}
