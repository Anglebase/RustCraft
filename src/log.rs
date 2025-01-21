use colored::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

struct Logger {
    level: Level,
    file: Option<String>,
}

impl Logger {
    fn new() -> Self {
        Self {
            level: Level::Info,
            file: None,
        }
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn set_file(&mut self, file: Option<String>) {
        self.file = file;
    }

    fn log(&self, level: Level, owner: &str, message: &str) {
        if level >= self.level {
            if let Some(ref file) = self.file {
                let result = match level {
                    Level::Debug => format!("[DEBUG] {} |: {}\n", owner, message),
                    Level::Info => format!("[INFO]  {} |: {}\n", owner, message),
                    Level::Warn => format!("[WARN]  {} |: {}\n", owner, message),
                    Level::Error => format!("[ERROR] {} |: {}\n", owner, message),
                };
                use std::fs::OpenOptions;
                use std::io::Write;
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .unwrap();
                file.write_all(result.as_bytes()).unwrap();
            } else {
                let result = match level {
                    Level::Debug => format!("[DEBUG] {} |: {}", owner, message).green(),
                    Level::Info => format!("[INFO]  {} |: {}", owner, message).blue(),
                    Level::Warn => format!("[WARN]  {} |: {}", owner, message).yellow(),
                    Level::Error => format!("[ERROR] {} |: {}", owner, message).red(),
                };
                if level == Level::Error {
                    eprintln!("{}", result);
                } else {
                    println!("{}", result);
                }
            }
        }
    }
}

use crate::RustCraftWrapper;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: RustCraftWrapper<Logger> = RustCraftWrapper::new(Logger::new());
}

pub fn set_level(level: Level) {
    LOGGER.apply(|logger| {
        logger.set_level(level);
    });
}

pub fn set_file(file: Option<String>) {
    LOGGER.apply(|logger| {
        logger.set_file(file);
    });
}

pub fn log(level: Level, owner: &str, message: &str) {
    LOGGER.apply(|logger| {
        logger.log(level, owner, message);
    });
}

#[macro_export]
macro_rules! debug {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Debug, $owner, &format_args!($($arg)*).to_string());
    };
}

#[macro_export]
macro_rules! info {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Info, $owner, &format_args!($($arg)*).to_string());
    };
}

#[macro_export]
macro_rules! warn {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Warn, $owner, &format_args!($($arg)*).to_string());
    };
}

#[macro_export]
macro_rules! error {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Error, $owner, &format_args!($($arg)*).to_string());
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        set_level(Level::Debug);
        debug!("test", "test message");
        info!("test", "test message");
        warn!("test", "test message");
        error!("test", "test message");

        debug!("outsized_name:test", "test message");
    }

    #[test]
    fn test_file() {
        set_file(Some(r"./log.txt".to_string()));
        debug!("test", "test message");
        info!("test", "test message");
        warn!("test", "test message");
        error!("test", "test message");
    }
}
