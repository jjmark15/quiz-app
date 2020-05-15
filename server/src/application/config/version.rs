#[cfg(not(test))]
use pkg_version::pkg_version_major;

#[cfg(not(test))]
const API_VERSION_LATEST: ApiVersion = ApiVersion {
    version: pkg_version_major!(),
};

#[cfg(test)]
const API_VERSION_LATEST: ApiVersion = ApiVersion { version: 1 };

#[derive(Eq, PartialEq, Debug, Default)]
pub(crate) struct ApiVersion {
    version: u32,
}

impl ApiVersion {
    pub(crate) fn latest() -> &'static ApiVersion {
        &API_VERSION_LATEST
    }

    pub(crate) fn version(&self) -> u32 {
        self.version
    }

    pub(crate) fn new(version: u32) -> ApiVersion {
        ApiVersion { version }
    }

    pub(crate) fn is_latest(&self) -> bool {
        self.eq(ApiVersion::latest())
    }
}

impl From<u32> for ApiVersion {
    fn from(u: u32) -> Self {
        ApiVersion::new(u)
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn recognises_latest_api_version() {
        let version = ApiVersion::new(1);
        asserting("version is the latest")
            .that(&version.is_latest())
            .is_true();
    }

    #[test]
    fn recognises_old_api_version_with_lower_number() {
        let version = ApiVersion::new(0);
        asserting("version is not the latest")
            .that(&version.is_latest())
            .is_false();
    }

    #[test]
    fn recognises_old_api_version_with_higher_number() {
        let version = ApiVersion::new(2);
        asserting("version is not the latest")
            .that(&version.is_latest())
            .is_false();
    }
}
