use crate::LogEntry;

pub trait LogEntity {
    fn log_entry(&self) -> LogEntry;
}
