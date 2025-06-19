use crate::prelude::{Component, ComponentBuilder, ComponentTracker};

pub mod prelude {
    pub use super::{Plugin, PluginInit, PluginParent};
}

/// Descripes a parent child relationship between Components
pub trait Plugin: Component + PluginInit + Sized {
    type Parent: PluginParent + ComponentBuilder;

    // required methods
    /// plugin() is primarily responsible for adding itself to the
    /// parent component. Any other pieces that the child has that
    /// the parent needs are called and handled here as well.
    fn plugin(self, parent: &mut Self::Parent);

    /// plugin_group() can batch plug in several plugins of the same
    /// type into the parent.
    fn plugin_group(parent: &mut Self::Parent);
}

pub trait PluginInit {
    /// init is called when the parent is fully ready to be built:
    /// all child -> parent relationships should be defined in plugin().
    /// the init resovles any parent -> child relationships.
    fn init(&mut self, index: usize, tracker: ComponentTracker);
}

pub trait PluginParent {
    // provided method
    fn add_plugins(&mut self, plugin: fn(&mut Self)) -> &mut Self {
        plugin(self);
        self
    }
}
