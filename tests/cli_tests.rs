use assert_cmd::Command;

#[test]
fn test_cli_init() {
    let mut cmd = Command::cargo_bin("locknote").unwrap();
    cmd.arg("init").assert().success();
}
