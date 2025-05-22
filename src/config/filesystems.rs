use std::{
    env,
    fs::{File, create_dir_all},
    io::{Error, ErrorKind, Write},
    path::PathBuf,
    sync::Once,
};

use super::*;

static CONFIG_CHECK: Once = Once::new();

pub fn config_dir() -> Result<PathBuf> {
    let mut path = match env::var("PLATFORM")?.as_ref() {
        "development" => PathBuf::from(env::var("PWD")?).join(".config"),
        _ => dirs::config_dir().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidFilename,
                "Could not parse config filepath for this platform.",
            )
        })?,
    };

    match env::var("MATHING_CONFIG") {
        Ok(dir) => {
            path = path.join(dir).join("config.toml");
        }
        Err(_) => DEFAULT_CONFIG_PATH.iter().for_each(|s| {
            path = path.join(s);
        }),
    }

    Ok(path)
}

pub fn config_exists() -> Result<bool> {
    Ok(config_dir()?.exists() && config_dir()?.is_file())
}

fn config_check() -> Result<()> {
    if !config_exists()? {
        let path = config_dir()?;

        create_dir_all(path.parent().ok_or_else(|| {
            Error::new(
                ErrorKind::NotADirectory,
                "Could not parse parent dir for config file.",
            )
        })?)?;

        let mut f = File::create_new(path)?;
        f.write_all(DEFAULT_CONFIG)?;
        //TODO: add logging feature to report that file was created.
    }

    Ok(())
}

pub fn config_check_once() -> Result<()> {
    let mut res = Ok(());
    CONFIG_CHECK.call_once(|| {
        res = config_check();
    });

    res
}
