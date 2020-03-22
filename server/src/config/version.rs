use pkg_version::pkg_version_major;

const API_VERSION_LATEST: ApiVersion = ApiVersion {
    version: pkg_version_major!(),
};

#[derive(Eq, PartialEq, Debug, Default)]
pub(crate) struct ApiVersion {
    version: u32,
}

impl ApiVersion {
    pub(crate) fn latest() -> &'static ApiVersion {
        &API_VERSION_LATEST
    }

    #[cfg(test)]
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
