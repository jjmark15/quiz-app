use log::debug;
use regex::Regex;
use warp::{Filter, Rejection};

use crate::config::version::ApiVersion;
use crate::logging;
use crate::web::filters::validate::api::error::ApiValidationError;

pub(crate) fn valid_api_version() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::optional::<String>("accept")
        .and_then(validate_api_version)
        .untuple_one()
}

async fn validate_api_version(optional_accept_string: Option<String>) -> Result<(), Rejection> {
    if let Some(accept_string) = optional_accept_string {
        debug!("accept header is present");
        match extract_api_version_from_accept_header(accept_string) {
            Ok(api_version) => {
                if !api_version.is_latest() {
                    return handle_old_api_version(api_version);
                }
                Ok(())
            }
            Err(err) => handle_failed_api_version_extraction(err),
        }
    } else {
        debug!("accept header is NOT present");
        Ok(())
    }
}

fn handle_old_api_version(version: ApiVersion) -> Result<(), Rejection> {
    let err: ApiValidationError = ApiValidationError::WrongApiVersion(version);
    debug!("{}", logging::log_string(&err));
    Err(warp::reject::custom(err))
}

fn handle_failed_api_version_extraction(err: ApiValidationError) -> Result<(), Rejection> {
    debug!("{}", logging::log_string(&err));
    Err(warp::reject::custom(err))
}

fn extract_api_version_from_accept_header<T: AsRef<str>>(
    accept_header: T,
) -> Result<ApiVersion, ApiValidationError> {
    let re: Regex = Regex::new(r"(?i)application/vnd\.warpj\.v(\d+)\+?\w*").unwrap();
    let result = re.captures(accept_header.as_ref());

    match result {
        Some(captures) => match captures.get(1) {
            Some(m) => match m.as_str().parse::<u32>() {
                Ok(u) => Ok(ApiVersion::from(u)),
                Err(e) => Err(e.into()),
            },
            None => Err(ApiValidationError::MissingMatch),
        },
        None => Err(ApiValidationError::MissingMatch),
    }
}

#[cfg(test)]
pub(crate) mod tests {
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
            .that(&e)
            .is_equal_to(ApiValidationError::MissingMatch);
    }
}
