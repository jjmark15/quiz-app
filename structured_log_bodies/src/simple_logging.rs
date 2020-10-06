use crate::LogEntity;

pub fn simple_log(log_entity: impl LogEntity) -> String {
    log_entity
        .log_entry()
        .sections()
        .iter()
        .map(|section| {
            format!(
                "{}={}",
                quote_string_if_contains_space(section.key()),
                quote_string_if_contains_space(section.value())
            )
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn quote_string_if_contains_space<S: AsRef<str>>(string: S) -> String {
    if string.as_ref().contains(' ') {
        format!("\"{}\"", string.as_ref())
    } else {
        string.as_ref().to_string()
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;
    use crate::{LogEntry, LogSection};

    struct SingleSectionLogEntity {
        key: String,
        value: String,
    }

    impl SingleSectionLogEntity {
        fn new(key: String, value: String) -> Self {
            SingleSectionLogEntity { key, value }
        }
    }

    impl LogEntity for SingleSectionLogEntity {
        fn log_entry(&self) -> LogEntry {
            let log_section = LogSection::new(self.key.clone(), self.value.clone());
            LogEntry::new(vec![log_section])
        }
    }

    #[derive(Default)]
    struct MultiSectionLogEntity;

    impl LogEntity for MultiSectionLogEntity {
        fn log_entry(&self) -> LogEntry {
            let log_section_1 = LogSection::new("key1".to_string(), "value1".to_string());
            let log_section_2 = LogSection::new("key2".to_string(), "value2".to_string());
            LogEntry::new(vec![log_section_1, log_section_2])
        }
    }

    #[test]
    fn key_and_value_are_joined_with_equals_symbol() {
        let test_log_entity = SingleSectionLogEntity::new("key".to_string(), "value".to_string());
        assert_that(&simple_log(test_log_entity)).is_equal_to(&"key=value".to_string());
    }

    #[test]
    fn multiple_log_sections_are_separated_by_a_space() {
        let test_log_entity = MultiSectionLogEntity::default();
        assert_that(&simple_log(test_log_entity))
            .is_equal_to(&"key1=value1 key2=value2".to_string());
    }

    #[test]
    fn quotes_log_section_key_when_contains_a_space() {
        let test_log_entity =
            SingleSectionLogEntity::new("with space".to_string(), "value".to_string());
        assert_that(&simple_log(test_log_entity)).is_equal_to(&"\"with space\"=value".to_string());
    }

    #[test]
    fn quotes_log_section_value_when_contains_a_space() {
        let test_log_entity =
            SingleSectionLogEntity::new("key".to_string(), "with space".to_string());
        assert_that(&simple_log(test_log_entity)).is_equal_to(&"key=\"with space\"".to_string());
    }
}
