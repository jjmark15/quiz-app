use std::fmt::Formatter;

pub(crate) trait LogEntry {
    fn log_contents(&self) -> String {
        format::join_entries(self.log_entry_kvps())
    }

    fn log_entry_kvps(&self) -> Vec<LogEntryKVP>;
}

pub(crate) struct LogEntryKVP {
    key: String,
    value: LogEntryValue,
}

impl LogEntryKVP {
    pub(crate) fn new<K: AsRef<str>, V: Into<LogEntryValue>>(key: K, value: V) -> LogEntryKVP {
        LogEntryKVP {
            key: key.as_ref().to_string(),
            value: value.into(),
        }
    }
}

impl std::fmt::Display for LogEntryKVP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

pub(crate) struct LogEntryValue {
    value: String,
}

impl std::fmt::Display for LogEntryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl LogEntryValue {
    pub(crate) fn new(value: String) -> LogEntryValue {
        LogEntryValue { value }
    }
}

impl<T: AsRef<str>> From<T> for LogEntryValue {
    fn from(f: T) -> Self {
        let s = {
            let to_string = f.as_ref().to_string();

            if to_string.contains(' ') {
                format!("\"{}\"", to_string)
            } else {
                to_string
            }
        };
        LogEntryValue::new(s)
    }
}

pub(crate) fn log_string<T: LogEntry>(entry: &T) -> String {
    entry.log_contents()
}

mod format {
    use crate::logging::LogEntryKVP;

    pub(crate) fn join_entries(entries: Vec<LogEntryKVP>) -> String {
        let entry_strings: Vec<String> =
            entries.iter().map(move |entry| entry.to_string()).collect();

        entry_strings.join(" ")
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use spectral::prelude::*;

    use super::*;

    struct TwoFieldStruct {
        first_field: String,
        second_field: String,
    }

    impl Default for TwoFieldStruct {
        fn default() -> Self {
            TwoFieldStruct {
                first_field: "first_value".to_string(),
                second_field: "second_value".to_string(),
            }
        }
    }

    impl LogEntry for TwoFieldStruct {
        fn log_entry_kvps(&self) -> Vec<LogEntryKVP> {
            vec![
                LogEntryKVP::new("first_field", self.first_field.clone()),
                LogEntryKVP::new("second_field", self.second_field.clone()),
            ]
        }
    }

    #[test]
    fn returns_log_string_with_two_fields_and_both_are_included_in_log() {
        let entry = TwoFieldStruct::default();
        let expected = "first_field=first_value second_field=second_value";
        asserting("log string contains both keys and values")
            .that(&log_string(&entry))
            .starts_with(expected);
    }

    #[test]
    fn returns_quoted_entry_value_when_string_contains_spaces() {
        let entry = TwoFieldStruct {
            first_field: "includes spaces".to_string(),
            ..TwoFieldStruct::default()
        };
        let expected = "first_field=\"includes spaces\" second_field=second_value";

        asserting("first field of log string has a quoted value")
            .that(&log_string(&entry))
            .starts_with(expected);
    }
}
