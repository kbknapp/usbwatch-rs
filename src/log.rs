use once_cell::sync::OnceCell;

pub static LOG_LEVEL: OnceCell<LogLevel> = OnceCell::new();

pub fn log_level() -> &'static LogLevel { LOG_LEVEL.get_or_init(|| LogLevel::Info) }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}
