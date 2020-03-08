use crate::config::version::ApiVersion;

pub(crate) fn get_application_header_prefix() -> String {
    format!("application/vnd.warpj.{}", ApiVersion::version_string())
}
