use crate::ports::logging::{LogEntity, LogEntry, LogSection};
use std::error::Error;

pub(crate) struct RejectionLogEntity {
    message: String,
}

impl RejectionLogEntity {
    pub(crate) fn new(message: String) -> Self {
        RejectionLogEntity { message }
    }

    pub(crate) fn from_error<E: Error>(error: &E) -> Self {
        let message: String = format!("{}", error);
        Self::new(message)
    }
}

impl LogEntity for RejectionLogEntity {
    fn log_entry(&self) -> LogEntry {
        let log_type = LogSection::new("type".to_string(), "rejection".to_string());
        let cause = LogSection::new("cause".to_string(), self.message.clone());
        LogEntry::new(vec![log_type, cause])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn rejection_log_entity_creates_a_rejection_log_with_a_cause() {
        let under_test = RejectionLogEntity::new("cause message".to_string());
        let expected_sections = vec![
            LogSection::new("type".to_string(), "rejection".to_string()),
            LogSection::new("cause".to_string(), "cause message".to_string()),
        ];
        let expected = LogEntry::new(expected_sections);
        assert_that(&under_test.log_entry()).is_equal_to(&expected);
    }
}
