use regex::Regex;
use thiserror::Error;

use crate::application::config::version::{ApiVersionTrait, ParseApiVersionError};

#[derive(Debug, Clone)]
pub(crate) struct AcceptHeader<A: ApiVersionTrait> {
    api_version: A,
}

impl<A: ApiVersionTrait> AcceptHeader<A> {
    fn new(api_version: A) -> Self {
        AcceptHeader { api_version }
    }

    pub(crate) fn parse<S: AsRef<str>>(accept_string: S) -> Result<Self, ParseAcceptHeaderError> {
        let api_version: A = extract_api_version_from_accept_header(accept_string)?;
        Ok(AcceptHeader::new(api_version))
    }

    pub(crate) fn api_version(&self) -> &A {
        &self.api_version
    }
}

fn extract_api_version_from_accept_header<T: AsRef<str>, A: ApiVersionTrait>(
    accept_header: T,
) -> Result<A, ParseAcceptHeaderError> {
    let re: Regex = Regex::new(r"(?i)application/vnd\.warpj\.(v\d*)\+?\w*").unwrap();
    let result = re.captures(accept_header.as_ref());

    match result {
        Some(captures) => match captures.get(1) {
            Some(m) => {
                let s = m.as_str();
                let api_version = A::from_str(s)?;
                Ok(api_version)
            }
            None => Err(ParseAcceptHeaderError::MissingApiVersion),
        },
        None => Err(ParseAcceptHeaderError::Malformed(
            accept_header.as_ref().to_string(),
        )),
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub(crate) enum ParseAcceptHeaderError {
    #[error("received malformed accept header \"{0}\"")]
    Malformed(String),
    #[error("missing api version in accept header")]
    MissingApiVersion,
    #[error("bad api version in accept header: {0}")]
    BadApiVersion(#[from] ParseApiVersionError),
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::application::config::version::{MockApiVersion, FROM_STR_MUTEX};

    use super::*;

    #[test]
    fn finds_api_version_in_valid_header_string() {
        let _m = FROM_STR_MUTEX.lock().unwrap();
        let header_string = "application/vnd.warpj.v12+text";
        let ctx = MockApiVersion::from_str_context();
        ctx.expect().return_once(|_s| Ok(MockApiVersion::default()));

        asserting("finds api version in valid header string")
            .that(&AcceptHeader::<MockApiVersion>::parse(header_string))
            .is_ok();
    }

    #[test]
    fn errors_if_no_version_present() {
        let _m = FROM_STR_MUTEX.lock().unwrap();
        let header_string = "application/vnd.warpj.v+text";
        let ctx = MockApiVersion::from_str_context();
        ctx.expect()
            .return_once(|_s| Err(ParseApiVersionError::Testing));

        asserting("errors if no api version present")
            .that(&AcceptHeader::<MockApiVersion>::parse(header_string))
            .is_err_containing(&ParseAcceptHeaderError::BadApiVersion(
                ParseApiVersionError::Testing,
            ));
    }

    #[test]
    fn errors_if_malformed_header_string() {
        let header_string = "application/vnd.warpjv12+text";

        asserting("errors when parsing a malformed header string")
            .that(&AcceptHeader::<MockApiVersion>::parse(header_string))
            .is_err_containing(ParseAcceptHeaderError::Malformed(header_string.to_string()));
    }

    #[test]
    fn errors_if_contains_non_digit_version_string() {
        let _m = FROM_STR_MUTEX.lock().unwrap();
        let header_string = "application/vnd.warpj.vtwelve+text";
        let ctx = MockApiVersion::from_str_context();
        ctx.expect()
            .return_once(|_s| Err(ParseApiVersionError::Testing));

        asserting("errors when parsing a non-digit version string")
            .that(&AcceptHeader::<MockApiVersion>::parse(header_string))
            .is_err_containing(&ParseAcceptHeaderError::BadApiVersion(
                ParseApiVersionError::Testing,
            ));
    }
}
