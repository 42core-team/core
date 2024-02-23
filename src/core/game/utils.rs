use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn get_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .expect("Failed to calculate duration")
}
