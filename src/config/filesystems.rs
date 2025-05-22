use std::{
    env,
    fs::{File, create_dir_all},
    io::{Error, ErrorKind, Write},
    path::PathBuf,
};

use super::*;

pub fn config_dir() -> Result<PathBuf> {
    let notfound_error = Error::new(
        ErrorKind::NotFound,
        "Could not parse config filepath for this platform.",
    );

    let mut path = match env::var("PLATFORM") {
        Ok(str) => match str {
            str if str == "development" => {
                PathBuf::from(env::var("PWD")?).join(".config")
            }
            _ => dirs::config_dir().ok_or(notfound_error)?,
        },
        Err(_) => dirs::config_dir().ok_or(notfound_error)?,
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

fn config_check() -> Result<()> {
    let config_exists = config_dir()?.exists() && config_dir()?.is_file();

    if !config_exists {
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
