const API_VERSION_LATEST: ApiVersion = ApiVersion { version: 0 };

#[derive(Eq, PartialEq, Debug, Default)]
pub struct ApiVersion {
    version: u16,
}

impl ApiVersion {
    pub fn latest() -> &'static ApiVersion {
        &API_VERSION_LATEST
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

    pub fn is_latest(&self) -> bool {
        self.eq(ApiVersion::latest())
    }
}

impl From<u16> for ApiVersion {
    fn from(u: u16) -> Self {
        ApiVersion::new(u)
    }
}
