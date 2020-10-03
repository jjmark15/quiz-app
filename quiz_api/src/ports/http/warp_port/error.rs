use crate::ports::http::warp_port::response::ErrorResponseBody;
use crate::ports::logging::{LogEntry, LogEntryKVP};

impl LogEntry for ErrorResponseBody {
    fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
        vec![
            LogEntryKVP::new("type", "rejection"),
            LogEntryKVP::new("message", self.message()),
        ]
    }
}
