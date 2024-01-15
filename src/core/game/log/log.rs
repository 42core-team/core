use lazy_static::lazy_static;
use std::fs::{self, create_dir_all, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use chrono::Utc;

use super::LogOptions;

lazy_static! {
    static ref LOGGER: Arc<RwLock<Option<Logger>>> = {
        let logger = Logger::new().expect("Failed to initialize logger");
        let logger = Some(logger);
        Arc::new(RwLock::new(logger))
    };
}

pub struct Logger {
    log_folder: PathBuf,
    old_logs_folder: PathBuf,
    old_logs_copied: bool, // Flag to track whether old logs have been copied
}

impl Logger {
    pub fn new() -> Result<Self, io::Error> {
        let log_folder = Path::new("logs");
        let old_logs_folder = Path::new("old-logs");

        create_dir_all(log_folder)?;
        create_dir_all(old_logs_folder)?;

        Ok(Logger {
            log_folder: log_folder.to_path_buf(),
            old_logs_folder: old_logs_folder.to_path_buf(),
            old_logs_copied: false, // Initialize the flag to false
        })
    }

    fn get_log_file_path(&self, log_option: &LogOptions) -> PathBuf {
        let timestamp = Utc::now();
        let log_filename = match log_option {
            LogOptions::State => format!("state-{}.log", timestamp.format("%Y-%m-%d-%H-%M-%S")),
            LogOptions::Error => format!("error-{}.log", timestamp.format("%Y-%m-%d-%H-%M-%S")),
            LogOptions::Action => format!("action-{}.log", timestamp.format("%Y-%m-%d-%H-%M-%S")),
            LogOptions::Changes => format!("changes-{}.log", timestamp.format("%Y-%m-%d-%H-%M-%S")),
            LogOptions::Info => format!("info-{}.log", timestamp.format("%Y-%m-%d-%H-%M-%S")),
        };

        self.log_folder.join(log_filename)
    }

    fn get_old_logs_folder(&self, timestamp: &str) -> PathBuf {
        self.old_logs_folder.join(timestamp)
    }

    fn copy_logs_to_old_folder(&mut self) -> Result<(), io::Error> {
        if self.old_logs_copied {
            return Ok(());
        }

        let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let old_logs_folder = self.get_old_logs_folder(&timestamp);

        create_dir_all(&old_logs_folder)?;

        for log_option in &[
            LogOptions::State,
            LogOptions::Error,
            LogOptions::Action,
            LogOptions::Changes,
            LogOptions::Info,
        ] {
            let log_file_path = self.get_log_file_path(log_option);
            let old_log_file_path = old_logs_folder.join(log_file_path.file_name().unwrap());

            fs::copy(&log_file_path, &old_log_file_path)?;
        }

        self.old_logs_copied = true; // Set the flag to true after copying
        Ok(())
    }

    pub fn copy_old_logs(&mut self) -> Result<(), io::Error> {
        self.copy_logs_to_old_folder()
    }

    pub fn log(&mut self, log_option: LogOptions, text: &str) -> Result<(), io::Error> {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_file_path = self.get_log_file_path(&log_option);

        // Check if old logs need to be copied
        self.copy_logs_to_old_folder()?;

        // Open the log file in append mode or create if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;

        // Write the log entry to the file
        writeln!(&mut file, "[{}] - {:?}: {}", timestamp, log_option, text)?;

        Ok(())
    }
}

pub fn log(log_option: LogOptions, text: &str) {
    if let Ok(logger) = LOGGER.read() {
        if let Some(logger) = logger.as_ref() {
            if let Ok(mut logger) = LOGGER.write() {
                if let Some(logger) = logger.as_mut() {
                    let _ = logger.log(log_option, text);
                }
            }
        }
    }
}
