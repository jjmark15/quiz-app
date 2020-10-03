use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Warp Server")]
pub struct CliOptions {
    #[structopt(name = "CONFIG FILE PATH", parse(from_os_str))]
    config_file_path: PathBuf,
}

impl CliOptions {
    pub fn config_file_path(&self) -> &PathBuf {
        &self.config_file_path
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    impl Default for CliOptions {
        fn default() -> Self {
            CliOptions {
                config_file_path: PathBuf::from("placeholder"),
            }
        }
    }

    #[test]
    fn returns_config_file_path() {
        assert_that(&CliOptions::default().config_file_path())
            .is_equal_to(&PathBuf::from("placeholder"))
    }
}
