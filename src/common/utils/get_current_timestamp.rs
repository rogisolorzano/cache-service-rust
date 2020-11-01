use std::time::{SystemTime, UNIX_EPOCH, SystemTimeError};

/// Get the current timestamp in seconds.
pub fn get_current_timestamp() -> Result<u64, SystemTimeError> {
    Ok(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
    )
}
