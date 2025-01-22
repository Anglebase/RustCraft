use colored::*;

/// 日志级别标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志记录器
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
        use chrono::*;
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        if level >= self.level {
            if let Some(ref file) = self.file {
                let result = match level {
                    Level::Debug => format!("{} [DEBUG] {:>20} |: {}\n", timestamp, owner, message),
                    Level::Info => format!("{} [INFO]  {:>20} |: {}\n", timestamp, owner, message),
                    Level::Warn => format!("{} [WARN]  {:>20} |: {}\n", timestamp, owner, message),
                    Level::Error => format!("{} [ERROR] {:>20} |: {}\n", timestamp, owner, message),
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
                    Level::Debug => format!(
                        "{} {:<7} {:>20} |: {}",
                        timestamp,
                        "[DEBUG]".green().italic().underline(),
                        owner,
                        message
                    )
                    .green(),
                    Level::Info => format!(
                        "{} {:<7} {:>20} |: {}",
                        timestamp,
                        "[INFO]".blue(),
                        owner,
                        message
                    )
                    .blue(),
                    Level::Warn => format!(
                        "{} {:<7} {:>20} |: {}",
                        timestamp,
                        "[WARN]".yellow().bold(),
                        owner,
                        message
                    )
                    .yellow(),
                    Level::Error => format!(
                        "{} {:<7} {:>20} |: {}",
                        timestamp,
                        "[ERROR]".red().bold().underline(),
                        owner,
                        message
                    )
                    .red(),
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

pub struct Log;

impl Log {
    /// 设置日志级别
    /// 默认情况下，日志级别为 `Info`
    pub fn set_level(level: Level) {
        LOGGER.apply(|logger| {
            logger.set_level(level);
        });
    }

    /// 设置日志输出文件
    /// 默认情况下，日志输出到控制台
    pub fn set_file(file: Option<String>) {
        LOGGER.apply(|logger| {
            logger.set_file(file);
        });
    }
}

/// 日志输出函数
pub fn log(level: Level, owner: &str, message: &str) {
    LOGGER.apply(|logger| {
        logger.log(level, owner, message);
    });
}

/// 日志输出宏
#[macro_export]
macro_rules! debug {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Debug, $owner, &format_args!($($arg)*).to_string());
    };
}

/// 日志输出宏
#[macro_export]
macro_rules! info {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Info, $owner, &format_args!($($arg)*).to_string());
    };
}

/// 日志输出宏
#[macro_export]
macro_rules! warn {
    ($owner:expr, $($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Warn, $owner, &format_args!($($arg)*).to_string());
    };
}

/// 日志输出宏
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
        Log::set_level(Level::Debug);
        debug!("test", "test message");
        info!("test", "test message");
        warn!("test", "test message");
        error!("test", "test message");
    }

    #[test]
    fn test_file() {
        Log::set_file(Some(r"out/log.txt".to_string()));
        debug!("test", "test message");
        info!("test", "test message");
        warn!("test", "test message");
        error!("test", "test message");
    }
}
