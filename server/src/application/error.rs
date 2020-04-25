use crate::application::logging::LogEntry;

pub(crate) trait ApplicationError: std::error::Error + LogEntry + std::fmt::Display {}
