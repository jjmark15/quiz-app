use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[cfg(test)]
pub(crate) use mocks::{MockApiVersion, FROM_STR_MUTEX};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        const API_VERSION_LATEST: ApiVersionImpl = ApiVersionImpl { version: 1 };
    } else {
        use pkg_version::pkg_version_major;
        const API_VERSION_LATEST: ApiVersionImpl = ApiVersionImpl {
            version: pkg_version_major!(),
        };
    }
}

pub(crate) trait ApiVersion {
    fn latest() -> &'static ApiVersionImpl;

    fn version(&self) -> u32;

    fn new(version: u32) -> ApiVersionImpl;

    fn is_latest(&self) -> bool;
}

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
pub(crate) struct ApiVersionImpl {
    version: u32,
}

impl ApiVersion for ApiVersionImpl {
    fn latest() -> &'static ApiVersionImpl {
        &API_VERSION_LATEST
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn new(version: u32) -> ApiVersionImpl {
        ApiVersionImpl { version }
    }

    fn is_latest(&self) -> bool {
        self.eq(ApiVersionImpl::latest())
    }
}

impl From<u32> for ApiVersionImpl {
    fn from(u: u32) -> Self {
        ApiVersionImpl::new(u)
    }
}

impl FromStr for ApiVersionImpl {
    type Err = ParseApiVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let without_v: &str = s.trim_start_matches('v');
        let number = without_v.parse::<u32>()?;
        Ok(ApiVersionImpl::new(number))
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub(crate) enum ParseApiVersionError {
    #[error(transparent)]
    ParseVersionIntError(#[from] ParseIntError),
    #[cfg(test)]
    #[error("test error")]
    Testing,
}

#[cfg(test)]
mod mocks {
    use std::fmt::Debug;
    use std::sync::Mutex;

    use mockall::mock;
    use mockall::predicate::*;

    use super::*;

    lazy_static! {
        // required as static method mocks are global - see https://docs.rs/mockall/0.7.1/mockall/index.html?search=#static-methods
        pub static ref FROM_STR_MUTEX: Mutex<()> = Mutex::new(());
    }

    mock! {
        pub(crate) ApiVersion {}

        trait ApiVersion {
            fn latest() -> &'static ApiVersionImpl;

            fn version(&self) -> u32;

            fn new(version: u32) -> ApiVersionImpl;

            fn is_latest(&self) -> bool;
        }

        trait Clone {
            fn clone(&self) -> Self;
        }

        trait PartialEq {
            fn eq(&self, other: &MockApiVersion) -> bool;
        }

        trait Eq {}

        trait FromStr {
            type Err=ParseApiVersionError;
            fn from_str(s: &str) -> Result<Self, ParseApiVersionError>;
        }

        trait Debug {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn recognises_latest_api_version() {
        let version = ApiVersionImpl::new(1);
        asserting("version is the latest")
            .that(&version.is_latest())
            .is_true();
    }

    #[test]
    fn recognises_old_api_version_with_lower_number() {
        let version = ApiVersionImpl::new(0);
        asserting("version is not the latest")
            .that(&version.is_latest())
            .is_false();
    }

    #[test]
    fn recognises_old_api_version_with_higher_number() {
        let version = ApiVersionImpl::new(2);
        asserting("version is not the latest")
            .that(&version.is_latest())
            .is_false();
    }

    #[test]
    fn parses_valid_version() {
        asserting("parses a valid version")
            .that(&ApiVersionImpl::from_str("v0").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parses_valid_version_without_prefix() {
        asserting("parses a valid version")
            .that(&ApiVersionImpl::from_str("0").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parses_multi_digit_number() {
        asserting("parses a valid multi-digit version")
            .that(&ApiVersionImpl::from_str("v12").is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn parse_unsuccessfully_with_invalid_version() {
        let res: Result<ApiVersionImpl, ParseApiVersionError> = ApiVersionImpl::from_str("vwhoops");
        asserting("fails to parse an invalid version")
            .that(&res)
            .is_err();
    }

    #[test]
    fn parse_unsuccessfully_with_signed_integer_value() {
        let result: Result<ApiVersionImpl, ParseApiVersionError> = ApiVersionImpl::from_str("v-1");
        asserting("fails to parse with a negative version number")
            .that(&result)
            .is_err();
    }
}
