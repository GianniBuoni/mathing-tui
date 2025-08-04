use crate::prelude::*;

use clap::Parser;

pub mod prelude {
    pub use super::Args;
}

#[derive(Debug, Parser)]
#[command(version = version(), about = clap::crate_description!())]
pub struct Args;

fn version() -> String {
    let version = clap::crate_version!();
    let author = clap::crate_authors!();
    let config_dirs = ConfigDirs::get().unwrap_or_default();

    format!(
        "\
{version}

Author: {author}

Config directory: {}
Data directory: {}",
        config_dirs.keymap, config_dirs.db
    )
}
