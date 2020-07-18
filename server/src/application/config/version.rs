use std::num::ParseIntError;
use std::str::FromStr;

#[cfg(not(test))]
use pkg_version::pkg_version_major;
use thiserror::Error;

#[cfg(not(test))]
const API_VERSION_LATEST: ApiVersion = ApiVersion {
    version: pkg_version_major!(),
};

#[cfg(test)]
const API_VERSION_LATEST: ApiVersion = ApiVersion { version: 1 };

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
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

impl FromStr for ApiVersion {
    type Err = ParseApiVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let without_v: &str = s.trim_start_matches('v');
        let number = without_v.parse::<u32>()?;
        Ok(ApiVersion::new(number))
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub(crate) enum ParseApiVersionError {
    #[error("{0}")]
    ParseVersionIntError(#[from] ParseIntError),
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

    #[test]
    fn parses_valid_version() {
        asserting("parses a valid version")
            .that(&ApiVersion::from_str("v0").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parses_valid_version_without_prefix() {
        asserting("parses a valid version")
            .that(&ApiVersion::from_str("0").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parses_multi_digit_number() {
        asserting("parses a valid multi-digit version")
            .that(&ApiVersion::from_str("v12").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parse_unsuccessfully_with_invalid_version() {
        let res: Result<ApiVersion, ParseApiVersionError> = ApiVersion::from_str("vwhoops");
        asserting("fails to parse an invalid version")
            .that(&res)
            .is_err();
    }

    #[test]
    fn parse_unsuccessfully_with_signed_integer_value() {
        let result: Result<ApiVersion, ParseApiVersionError> = ApiVersion::from_str("v-1");
        asserting("fails to parse with a negative version number")
            .that(&result)
            .is_err();
    }
}
