pub struct ApiVersion;

impl ApiVersion {
    #[cfg(not(test))]
    pub fn version_string() -> &'static str {
        "v0"
    }

    #[cfg(test)]
    pub fn version_string() -> &'static str {
        "vlatest"
    }
}
