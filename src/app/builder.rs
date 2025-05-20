use super::*;

impl<'a> App<'a> {
    pub fn new(home: Home<'a>) -> Self {
        App {
            component: home,
            ..Default::default()
        }
    }
}
