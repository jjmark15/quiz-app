use predicates::str::starts_with;
use std::path::PathBuf;

#[test]
fn application_uses_config_path_passed_in_cli() {
    let mut cmd = assert_cmd::Command::cargo_bin("server_backend").unwrap();
    let config_file_path: PathBuf = ["configs", "placeholder.yml"].iter().collect();

    let assert = cmd.arg(config_file_path.as_os_str()).assert();
    assert.code(1).failure().stderr(starts_with(
        "Config error occurred: error occurred reading config file: could not read from config file",
    ));
}
