use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

use crate::common::test_helpers::TestFixture;

#[test]
fn test_cli_view_text_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "Hello World\nLine 2");

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄 test.txt (Plain Text)"))
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_cli_view_json_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.json", r#"{"name": "test", "value": 42}"#);

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"))
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("42"));
}

#[test]
fn test_cli_view_csv_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.csv", "name,age\nJohn,30\nJane,25");

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("John"));
}

#[test]
fn test_cli_view_csv_with_custom_delimiter() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.csv", "name;age\nJohn;30\nJane;25");

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .arg("--delimiter")
        .arg(";")
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("John"))
        .stdout(predicate::str::contains("30"));
}

#[test]
fn test_cli_view_binary_file() {
    let fixture = TestFixture::new();
    let binary_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let file_path = fixture.root_path.join("test.png");
    fs::write(&file_path, &binary_data).unwrap();

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("binary")
        .assert()
        .success()
        .stdout(predicate::str::contains("🔢 Binary File"))
        .stdout(predicate::str::contains("PNG Image"))
        .stdout(predicate::str::contains("Hex Dump"));
}

#[test]
fn test_cli_view_with_max_lines() {
    let fixture = TestFixture::new();
    let content = (0..100)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let file_path = fixture.create_file("large.txt", &content);

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 0"))
        .stdout(predicate::str::contains("Line 9"))
        .stdout(predicate::str::contains("showing first 10 lines"))
        .stdout(predicate::str::contains("Line 50").not());
}

#[test]
fn test_cli_view_with_max_bytes() {
    let fixture = TestFixture::new();
    let binary_data = vec![0u8; 2048];
    let file_path = fixture.root_path.join("large.bin");
    fs::write(&file_path, &binary_data).unwrap();

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("binary")
        .arg("--max-bytes")
        .arg("256")
        .assert()
        .success()
        .stdout(predicate::str::contains("🔢 Binary File"))
        .stdout(predicate::str::contains("showing first 256 bytes"));
}

#[test]
fn test_cli_view_yaml_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.yaml", "name: test\nvalue: 42");

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄 YAML File"))
        .stdout(predicate::str::contains("name: test"));
}

#[test]
fn test_cli_view_toml_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.toml", "[package]\nname = \"test\"");

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("⚙️  TOML File"))
        .stdout(predicate::str::contains("package"));
}

#[test]
fn test_cli_view_nonexistent_file() {
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg("/nonexistent/file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error viewing file"));
}

#[test]
fn test_cli_view_auto_format_detection() {
    let fixture = TestFixture::new();

    // Test that .json files are auto-detected as JSON
    let json_file = fixture.create_file("auto.json", r#"{"auto": true}"#);
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&json_file)
        .arg("--viewer-format")
        .arg("auto")
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"));
}

#[test]
fn test_cli_view_format_override() {
    let fixture = TestFixture::new();
    // Create a .txt file but view it as JSON
    let file_path = fixture.create_file("data.txt", r#"{"name": "test"}"#);

    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"));
}
