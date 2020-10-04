#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LogSection {
    key: String,
    value: String,
}

impl LogSection {
    pub fn new(key: String, value: String) -> Self {
        LogSection { key, value }
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    fn log_section() -> LogSection {
        LogSection::new("key".to_string(), "value".to_string())
    }

    #[test]
    fn returns_a_key() {
        assert_that(&log_section().key()).is_equal_to(&"key".to_string());
    }

    #[test]
    fn returns_a_value() {
        assert_that(&log_section().value()).is_equal_to(&"value".to_string());
    }
}
