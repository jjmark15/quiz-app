pub trait Log {
    fn log_contents(&self) -> String;
}

pub fn log_string<T: Log>(entry: &T) -> String {
    entry.log_contents()
}
