pub enum ApiVersion {
    V0,
}

impl ApiVersion {
    pub fn version_string(&self) -> &str {
        match self {
            ApiVersion::V0 => "v0",
        }
    }

    pub fn latest() -> Self {
        ApiVersion::V0
    }
}
