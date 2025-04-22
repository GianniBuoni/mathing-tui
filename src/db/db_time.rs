use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_time() -> Result<i64, Box<dyn Error>> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    Ok(time)
}
