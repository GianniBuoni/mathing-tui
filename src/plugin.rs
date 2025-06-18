use anyhow::Result;

use crate::prelude::{Component, ComponentTracker};

pub mod prelude {
    pub use super::{Plugin, PluginParent};
}

/// Descripes a parent child relationship between Components
pub trait Plugin: Component + Sized {
    type Parent: PluginParent;

    // required methods
    /// plugin() is called and is primarily responsible for
    /// adding itself to the parent component.
    /// Any other pieces that the child has that the parent needs are
    /// called handled here as well.
    fn plugin(self, parent: &mut Self::Parent);
    /// init is called when the parent is fully ready to be built:
    /// all child -> parent relationships should be defined.
    /// the init resovles any parent -> child relationships.
    fn init(&mut self, index: usize, tracker: ComponentTracker);
}

pub trait PluginParent {
    // required method
    fn init(&mut self) -> Result<()>;

    // provided method
    fn add_plugins(&mut self, plugin: fn(&mut Self)) -> &mut Self {
        plugin(self);
        self
    }
}
