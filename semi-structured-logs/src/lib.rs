// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    }
}

pub fn info(message: &str) -> String {
    return format("INFO", message);
}

pub fn warn(message: &str) -> String {
    return format("WARNING", message);
}

pub fn error(message: &str) -> String {
    return format("ERROR", message);
}

pub fn debug(message: &str) -> String {
    return format("DEBUG", message);
}

fn format(level: &str, message: &str) -> String {
    return format!("[{}]: {}", level, message);
}
