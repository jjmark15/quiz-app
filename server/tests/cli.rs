use predicates::str::starts_with;
use std::path::PathBuf;

#[test]
fn application_uses_config_path_passed_in_cli() {
    let mut cmd = assert_cmd::Command::cargo_bin("server").unwrap();
    let mut config_file_path = PathBuf::from(".").join("configs").join("placeholder");
    config_file_path.set_extension("yml");

    let assert = cmd.arg(config_file_path.as_os_str()).assert();
    assert.code(1).failure().stderr(starts_with(
        "Config error occurred: could not read from config file",
    ));
}
