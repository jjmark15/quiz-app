use log::debug;
use regex::Regex;
use warp::{Filter, Rejection};

use crate::config::version::ApiVersion;
use crate::error::api_validation_error::{ApiValidationError, ApiValidationErrorKind};
use crate::logging;

pub fn validate_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::optional::<String>("accept")
        .and_then(|optional_accept_string: Option<String>| async move {
            if let Some(accept_string) = optional_accept_string {
                match extract_api_version_from_accept_header(accept_string.as_str()) {
                    Ok(api_version) => {
                        if api_version.version() == ApiVersion::latest().version() {
                            Ok(())
                        } else {
                            let err =
                                ApiValidationError::new(ApiValidationErrorKind::WrongApiVersion);
                            debug!("{}", logging::log_string(&err));
                            Err(warp::reject::custom(err))
                        }
                    }
                    Err(err) => {
                        debug!("{}", logging::log_string(&err));
                        Err(warp::reject::custom(err))
                    }
                }
            } else {
                Ok(())
            }
        })
        .untuple_one()
}

fn extract_api_version_from_accept_header(
    accept_header: &str,
) -> Result<ApiVersion, ApiValidationError> {
    let re: Regex = Regex::new(r"(?i)application/vnd\.warpj\.v(\d+)\+?\w*").unwrap();
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
    fn extracts_successfully_with_multi_digit_number() {
        let header = "application/vnd.warpj.v12+text";
        let result: Result<ApiVersion, ApiValidationError> =
            extract_api_version_from_accept_header(header);

        asserting("successfully extracts a multi digit number")
            .that(&result.unwrap().version())
            .is_equal_to(12)
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
            .that(&e.kind())
            .is_equal_to(ApiValidationErrorKind::MissingMatch);
    }
}
