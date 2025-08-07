#![allow(dead_code)]
#![allow(unused_imports)]

use std::{env, path::PathBuf};

use self::prelude::*;

pub mod prelude {
    pub use super::init_test_config::try_init_test_config;
    pub use super::init_test_db::{
        MOCK_ITEMS, MOCK_RECEIPTS, MOCK_RU, MOCK_USERS, try_init_test_db,
    };
    pub use super::init_test_paging_db::{
        basic_get_req, test_req, try_init_paging_test, try_process_req,
    };
    pub use futures::future::try_join_all;
    pub use mathing_tui::prelude::*;
    pub use sqlx::{QueryBuilder, Sqlite, SqlitePool};
}

mod init_test_config;
mod init_test_db;
mod init_test_paging_db;
