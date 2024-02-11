use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_millis() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as u64
}
