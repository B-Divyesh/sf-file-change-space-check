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
