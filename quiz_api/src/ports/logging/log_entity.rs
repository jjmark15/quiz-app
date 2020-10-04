use crate::ports::logging::log_entry::LogEntry;

pub trait LogEntity {
    fn log_entry(&self) -> LogEntry;
}
