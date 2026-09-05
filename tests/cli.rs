use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("fcsc-cli-{}-{nonce}", std::process::id()));
        fs::create_dir_all(root.join("source")).unwrap();
        fs::create_dir_all(root.join("destination")).unwrap();
        Self(root)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_fcsc")
}

#[test]
fn documented_json_example_emits_manifest() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("source/archive.bin"), b"payload").unwrap();
    let output = Command::new(binary())
        .args([
            fixture.0.join("source").to_str().unwrap(),
            fixture.0.join("destination").to_str().unwrap(),
            "--policy",
            "overwrite",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["verdict"], "sufficient");
    assert_eq!(value["actions"][0]["operation"], "copy");
}

#[test]
fn unchecked_plan_uses_stable_exit_code_three() {
    let fixture = Fixture::new();
    let output = Command::new(binary())
        .args([
            fixture.0.join("source").to_str().unwrap(),
            fixture.0.join("destination").to_str().unwrap(),
            "--policy",
            "skip",
            "--no-space-check",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stdout).contains("SPACE NOT CHECKED"));
}

#[test]
fn invalid_source_is_an_actionable_error() {
    let fixture = Fixture::new();
    let output = Command::new(binary())
        .args([
            fixture.0.join("missing").to_str().unwrap(),
            fixture.0.join("destination").to_str().unwrap(),
            "--policy",
            "skip",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot read"));
}

#[test]
fn invalid_policy_uses_the_documented_input_error_exit_code() {
    let fixture = Fixture::new();
    let output = Command::new(binary())
        .args([
            fixture.0.join("source").to_str().unwrap(),
            fixture.0.join("destination").to_str().unwrap(),
            "--policy",
            "not-a-policy",
            "--json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid value 'not-a-policy'"));
    assert!(output.stdout.is_empty());
}

#[test]
fn demo_runs_in_a_new_temporary_sandbox() {
    let working = Fixture::new();
    let before = fs::read_dir(&working.0).unwrap().count();
    let output = Command::new(binary())
        .arg("--demo")
        .current_dir(&working.0)
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("DEMO — sample data in a temporary sandbox"));
    assert!(stdout.contains("photos.raw"));
    assert!(stdout.contains("field-laptop.img"));
    assert!(stdout.contains("demo-manifest.json"));
    assert_eq!(fs::read_dir(&working.0).unwrap().count(), before);

    let sandbox = stdout
        .lines()
        .find_map(|line| line.strip_prefix("Sample sandbox  "))
        .map(PathBuf::from)
        .expect("demo prints its sandbox path");
    assert!(sandbox.starts_with(std::env::temp_dir()));
    assert!(sandbox.join("demo-manifest.json").is_file());
    fs::remove_dir_all(sandbox).unwrap();
}

#[test]
fn demo_json_is_machine_readable_and_uses_requested_policy() {
    let output = Command::new(binary())
        .args([
            "--demo",
            "--policy",
            "keep-both",
            "--sparse",
            "expand",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["policy"], "keep-both");
    assert_eq!(value["sparse"], "expand");
    assert_eq!(value["verdict"], "unchecked");
    assert!(value["summary"]["conflicts"].as_u64().unwrap() >= 1);
    let sandbox = PathBuf::from(value["source"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    fs::remove_dir_all(sandbox).unwrap();
}

#[test]
fn demo_rejects_a_redundant_no_space_check_flag() {
    let output = Command::new(binary())
        .args(["--demo", "--no-space-check"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--demo"));
    assert!(stderr.contains("--no-space-check"));
}
