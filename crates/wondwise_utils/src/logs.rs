// Copyright 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

//! ## Logs
//! There are parts of the application where the use or registration of logs
//! is needed either to show them to the user graphically or to register
//! them in session files.
//!
//! This is where the mod logs appears, which contains functions
//! that will help us to do this job.

use colored::*;
use std::env;

/// these are possible levels for a log.
/// we can use them as follows:
/// ```rust
/// use wondwise_utils::logs::LogLevel;
///
/// let log_level_error = LogLevel::Error;
/// let log_level_warning = LogLevel::Warning;
/// ```
/// the functionality possible here is very basic,
/// that is why it is usually used together with the `Log` structure
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Error,
    Warning,
}

/// this is the structure that a log should have, this is global for any level
/// we can use them as follows:
/// ```rust
/// use wondwise_utils::logs::{Log, LogLevel};
///
/// // this print a log
/// Log::new(LogLevel::Warning, 0, "This is a test log.").show();
///
/// // this return a log
/// let my_log: Log = Log::new(LogLevel::Warning, 0, "This is a test log.");
///
/// // we can also print an existing log
/// my_log.show();
/// ```
/// these are some uses that this structure can be given.
#[derive(Debug)]
pub struct Log {
    level: LogLevel,
    code: u16,
    message: String,
}

impl Log {
    /// This function allows us to create a new log in a very intuitive and easy way,
    /// it also allows us to do extra things like print it in the console
    pub fn new(level: LogLevel, code: u16, message: &str) -> Self {
        Self {
            level,
            code,
            message: String::from(message),
        }
    }

    pub fn show(&self) {
        match self.level {
            LogLevel::Info => {
                println!("{} {}", "INFO".bold().green(), self.message);
            }
            LogLevel::Debug => {
                let log_level = match env::var("LOG_LEVEL") {
                    Ok(value) => value,
                    Err(_) => String::from("info"),
                };

                if log_level == *"debug" {
                    println!("{} {}", "DEBUG".bold().blue(), self.message);
                }
            }
            LogLevel::Error => {
                eprintln!("{} {}", "ERROR".bold().red(), self.message);
            }
            LogLevel::Warning => {
                println!("{} {}", "WARNING".bold().yellow(), self.message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log() {
        let log_new = Log::new(LogLevel::Error, 1, "message");

        assert_eq!(log_new.code, 1);
        assert_eq!(log_new.message, "message");
    }
}
