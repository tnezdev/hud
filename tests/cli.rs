use assert_cmd::Command;

fn hud() -> Command {
    Command::cargo_bin("hud").unwrap()
}

#[test]
fn help_exits_successfully() {
    hud()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}

#[test]
fn version_exits_successfully() {
    hud()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("0.1.0"));
}

#[test]
fn demo_check_config_exits_successfully() {
    hud()
        .args(["--demo", "--check-config"])
        .assert()
        .success()
        .stdout(predicates::str::contains("hud config OK"));
}

#[test]
fn kitchen_sink_check_config_exits_successfully() {
    hud()
        .args(["--config", "examples/kitchen-sink.toml", "--check-config"])
        .assert()
        .success()
        .stdout(predicates::str::contains("hud config OK"));
}

#[test]
fn dogfood_check_config_exits_successfully() {
    hud()
        .args(["--config", "examples/dogfood.toml", "--check-config"])
        .assert()
        .success()
        .stdout(predicates::str::contains("hud config OK"));
}

#[test]
fn unknown_argument_exits_nonzero() {
    hud()
        .arg("--bogus")
        .assert()
        .failure()
        .stderr(predicates::str::contains("unknown argument: --bogus"));
}

#[test]
fn missing_config_path_exits_nonzero() {
    hud()
        .args(["--config", "/no/such/path.toml"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("missing hud config"));
}

#[test]
fn config_without_path_exits_nonzero() {
    hud()
        .arg("--config")
        .assert()
        .failure()
        .stderr(predicates::str::contains("--config requires a path"));
}
