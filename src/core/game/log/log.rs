use std::fs::{self, OpenOptions};
use std::io::Write;

use chrono::Local;

use super::LogOptions;

pub struct Logger {}
#[cfg(not(test))]
impl Logger {
    #[cfg(not(test))]
    pub fn new() -> Logger {
        // Create logs folder if it doesn't exist
        fs::create_dir_all("logs").unwrap();

        // Create old-logs folder if it doesn't exist
        fs::create_dir_all("old-logs").unwrap();

        // Move existing log files to old-logs folder
        let paths = fs::read_dir("logs").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let old_log_file_path = format!("old-logs/{}", file_name);
            if fs::metadata(&path).is_ok() && fs::metadata(&old_log_file_path).is_ok() {
                fs::rename(&path, &old_log_file_path).unwrap();
            }
        }

        // Create a new log file for each log option if it doesn't exist
        let log_options = vec!["error", "state", "action", "changes", "info"];
        for option in log_options {
            let log_file_path = format!("logs/{}.log", option);
            if !fs::metadata(&log_file_path).is_ok() {
                fs::File::create(&log_file_path).unwrap();
            }
        }
        Logger {}
    }

    pub fn log(log_options: LogOptions, message: String) {
        let log_file_path = match log_options {
            LogOptions::State => "logs/state.log",
            LogOptions::Error => "logs/error.log",
            LogOptions::Action => "logs/action.log",
            LogOptions::Changes => "logs/changes.log",
            LogOptions::Info => "logs/info.log",
        };

        // Get the current timestamp with milliseconds
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        // Open the file in append mode and write the message with timestamp
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .unwrap();
        writeln!(file, "{} - {}", current_time, message).unwrap();
    }
}

#[cfg(not(test))]
pub fn log(log_options: LogOptions, message: &str) {
    Logger::log(log_options, message.to_string());
}

#[cfg(test)]
pub fn log(log_options: LogOptions, message: &str) {
    // Only the log function is implemented for tests
    // You might want to replace this with an implementation that suits your testing needs
    println!("Test log: {:?} - {}", log_options, message);
}
