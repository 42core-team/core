use std::time::{UNIX_EPOCH, SystemTime};

use super::LogOptions;


pub fn log(log_option: LogOptions, text: &str) {
    let timestamp_in_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    match log_option {
        LogOptions::State => {
            println!("{:?} - State: {:?}",timestamp_in_ms, text);
        },
        LogOptions::Error => {
            println!("{:?} - Error: {:?}",timestamp_in_ms, text);
        },
        LogOptions::Action => {
            println!("{:?} - Action: {:?}",timestamp_in_ms, text);
        },
        LogOptions::Changes => {
            println!("{:?} - Changes: {:?}",timestamp_in_ms, text);
        }
    }
}