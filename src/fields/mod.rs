use crate::prelude::*;

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

use tui_input::Input;

pub mod prelude {
    pub use super::Field;
    pub use super::plugin::prelude::*;
    #[cfg(test)]
    pub use super::tests::test_text_inputs;
}

mod choice;
mod plugin;
mod selection_field;
#[cfg(test)]
mod tests;
mod text_input;

pub trait Field: Component + PluginInit {
    fn submit(&self) -> Result<()>;
    fn get_rect_height(&self) -> u16;
    fn handles_input(&self) -> bool {
        false
    }
}

#[derive(Default, Debug)]
pub struct InputField<T>
where
    T: Debug + FromStr + Default + Clone,
    <T as FromStr>::Err: Debug,
{
    input: Input,
    title: Rc<str>,
    active_field: ComponentTracker,
    value: ParamOption<T>,
    index: usize,
    field_type: Option<AppArm>,
}

#[derive(Debug, Default)]
pub struct SelectionField<T>
where
    T: Debug + Default + Copy,
{
    choices: Vec<Choice<T>>,
    values: Rc<RefCell<Vec<T>>>,
    active_field: ComponentTracker,
    active_choice: ComponentTracker,
    title: Rc<str>,
    index: usize,
    multiselect: bool,
}

#[derive(Debug, Default)]
pub struct SelectionBuilder<T>
where
    T: Debug + Default + Copy,
{
    choices: Vec<Choice<T>>,
    active_choice: ComponentTracker,
    title: Rc<str>,
    multiselect: bool,
}

#[derive(Debug, Default)]
pub struct Choice<T>
where
    T: Debug + Default + Copy,
{
    value: T,
    display: Rc<str>,
    active_choice: ComponentTracker,
    index: usize,
    selected: bool,
}
