use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use regex::Regex;
use warp::reject::Reject;
use warp::{Filter, Rejection};

use crate::config::version::ApiVersion;

pub fn validate_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::header("accept")
        .and_then(|accept_string: String| async move {
            match extract_api_version_from_accept_header(accept_string.as_str()) {
                Ok(api_version) => {
                    if api_version == ApiVersion::latest() {
                        Ok(())
                    } else {
                        Err(warp::reject::custom(ApiValidationError::new(
                            ApiValidationErrorKind::WrongApiVersion,
                        )))
                    }
                }
                Err(e) => Err(warp::reject::custom(e)),
            }
        })
        .untuple_one()
}

#[derive(Debug)]
pub struct ApiValidationError {
    kind: ApiValidationErrorKind,
    cause: Option<String>,
}

impl ApiValidationError {
    pub fn description(&self) -> String {
        let kind_description = match self.kind {
            ApiValidationErrorKind::MissingMatch => "could not find a version in accept header",
            ApiValidationErrorKind::UnableToParse => "version in accept header could not be parsed",
            ApiValidationErrorKind::WrongApiVersion => "api version is incorrect",
            ApiValidationErrorKind::Unknown => "unknown error",
        };

        match &self.cause {
            Some(s) => format!("{} : {}", kind_description, s),
            None => kind_description.to_string(),
        }
    }

    pub fn new(kind: ApiValidationErrorKind) -> ApiValidationError {
        ApiValidationError { kind, cause: None }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum ApiValidationErrorKind {
    MissingMatch,
    UnableToParse,
    WrongApiVersion,
    Unknown,
}

impl From<ParseIntError> for ApiValidationError {
    fn from(p: ParseIntError) -> Self {
        ApiValidationError {
            kind: ApiValidationErrorKind::UnableToParse,
            cause: Some(p.description().to_string()),
        }
    }
}

impl From<&ApiValidationError> for ApiValidationError {
    fn from(original: &ApiValidationError) -> Self {
        ApiValidationError {
            kind: original.kind,
            cause: original.cause.clone(),
        }
    }
}

impl Error for ApiValidationError {}

impl Display for ApiValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Reject for ApiValidationError {}

fn extract_api_version_from_accept_header(
    accept_header: &str,
) -> Result<ApiVersion, ApiValidationError> {
    let re: Regex = Regex::new(r"(?i)application/vnd\.warpj\.v(\d)+\+?\w*").unwrap();
    let result = re.captures(accept_header);

    match result {
        Some(captures) => match captures.get(1) {
            Some(m) => match m.as_str().parse::<u16>() {
                Ok(u) => Ok(ApiVersion::from(u)),
                Err(e) => Err(e.into()),
            },
            None => Err(ApiValidationError::new(
                ApiValidationErrorKind::MissingMatch,
            )),
        },
        None => Err(ApiValidationError::new(
            ApiValidationErrorKind::MissingMatch,
        )),
    }
}

#[cfg(test)]
pub mod tests {
    use spectral::prelude::*;

    use crate::config::version::ApiVersion;

    use super::*;

    #[test]
    fn extracts_successfully_with_valid_version() {
        let header = "application/vnd.warpj.v0+text";
        let result: Result<ApiVersion, ApiValidationError> =
            extract_api_version_from_accept_header(header);

        asserting("successfully extracts a valid version")
            .that(&result.is_ok())
            .is_equal_to(true)
    }

    #[test]
    fn extracts_unsuccessfully_with_invalid_version() {
        let header = "application/vnd.warpj.vwhoops+text";
        let result: Result<ApiVersion, ApiValidationError> =
            extract_api_version_from_accept_header(header);

        asserting("fails to extract a valid version")
            .that(&result.is_ok())
            .is_equal_to(false);

        let e = result.unwrap_err();

        asserting("regex parser could not match a version number")
            .that(&e.kind)
            .is_equal_to(ApiValidationErrorKind::MissingMatch);
    }
}
