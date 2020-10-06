use crate::LogSection;

#[derive(Debug, Eq, PartialEq)]
pub struct LogEntry {
    sections: Vec<LogSection>,
}

impl LogEntry {
    pub fn new(sections: Vec<LogSection>) -> Self {
        LogEntry { sections }
    }

    pub fn sections(&self) -> &Vec<LogSection> {
        &self.sections
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn returns_sections() {
        let log_section = LogSection::new("key".to_string(), "value".to_string());
        let under_test = LogEntry::new(vec![log_section.clone()]);
        assert_that(&under_test.sections()).is_equal_to(&vec![log_section]);
    }
}
