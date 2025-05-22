use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

pub fn get_time() -> Result<i64> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    Ok(time)
}
