use std::fs::{self, OpenOptions};
use std::io::Write;

use chrono::Local;

use super::LogOptions;

#[cfg(not(test))]
pub fn initialise_logger() {
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
        fs::rename(&path, &old_log_file_path).unwrap();
    }

    // Create a new log file for each log option if it doesn't exist
    let log_options = vec!["error", "state", "action", "changes", "info"];
    for option in log_options {
        let log_file_path = format!("logs/{}.log", option);
        if !fs::metadata(&log_file_path).is_ok() {
            fs::File::create(&log_file_path).unwrap();
        }
    }
}

fn print_log(log_options: &LogOptions, message: &str) {
    match log_options {
        LogOptions::State => (),
        LogOptions::Error => println!("\x1b[31mError: \x1b[0m{}", message),
        LogOptions::Action => println!("\x1b[34mAction: \x1b[0m{}", message),
        LogOptions::Changes => {
            println!("\x1b[32mChanges: \x1b[0m{}", message);
        }
        LogOptions::Info => println!("\x1b[33mInfo: \x1b[0m{}", message),
    }
}

#[cfg(not(test))]
fn log(log_options: LogOptions, message: &str) {
    let log_file_path = match log_options {
        LogOptions::State => "logs/state.log",
        LogOptions::Error => "logs/error.log",
        LogOptions::Action => "logs/action.log",
        LogOptions::Changes => "logs/changes.log",
        LogOptions::Info => "logs/info.log",
    };

    print_log(&log_options, message);

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

#[cfg(test)]
fn log(log_options: LogOptions, message: &str) {
    print_log(&log_options, message);
}

pub fn state(message: &str) {
    log(LogOptions::State, message);
}

pub fn error(message: &str) {
    log(LogOptions::Error, message);
}

pub fn action(message: &str) {
    log(LogOptions::Action, message);
}

pub fn changes(message: &str) {
    log(LogOptions::Changes, message);
}

pub fn info(message: &str) {
    log(LogOptions::Info, message);
}
