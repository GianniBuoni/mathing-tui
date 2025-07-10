use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};

use futures::future::try_join_all;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use crate::prelude::*;
use tables::{StoreCount, StoreJoinPrices, StoreJoinRaw, StoreReceipt};

mod params;
mod payloads;
mod requests;
mod response;
mod tables;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::params::prelude::*;
    pub use super::payloads::prelude::*;
    pub use super::requests::prelude::*;
    pub use super::response::prelude::*;
    pub use super::tables::prelude::*;
    // TODO: remove later when i fgure out how to make param building
    // a consuming operation?
    pub use super::params::{
        items::ItemParamsBuilder, join_row::JoinParamsBuilder,
        users::UserParamsBuilder,
    };
}
