#[derive(Eq, PartialEq, Debug)]
pub struct ApiVersion {
    version: u16,
}

impl ApiVersion {
    pub fn latest() -> ApiVersion {
        ApiVersion { version: 0 }
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn version_string(&self) -> String {
        format!("v{}", self.version())
    }

    pub fn new(version: u16) -> ApiVersion {
        ApiVersion { version }
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::latest()
    }
}

impl From<u16> for ApiVersion {
    fn from(u: u16) -> Self {
        ApiVersion::new(u)
    }
}
