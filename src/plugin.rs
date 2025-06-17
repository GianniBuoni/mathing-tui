use crate::prelude::Component;

pub mod prelude {
    pub use super::{Plugin, PluginParent};
}

pub trait Plugin: Component + Sized {
    type Parent: PluginParent;

    // required method
    fn plugin(self, parent: &mut Self::Parent);
}

pub trait PluginParent {
    // provided method
    fn add_plugins(&mut self, plugin: fn(&mut Self)) -> &mut Self {
        plugin(self);
        self
    }
}
