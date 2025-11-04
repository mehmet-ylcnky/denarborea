use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::common::test_helpers::TestFixture;

// Empty and minimal files
#[test]
fn test_view_empty_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("empty.txt", "");

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("Plain Text"));
}

#[test]
fn test_view_single_character_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("single.txt", "a");

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("a"));
}

// Unicode and encoding tests
#[test]
fn test_view_unicode_file() {
    let fixture = TestFixture::new();
    let content = "Hello 世界 🌍 Здравствуй мир\n日本語テスト\n🚀🎉✨";
    let file_path = fixture.create_file("unicode.txt", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("世界"))
        .stdout(predicate::str::contains("🌍"))
        .stdout(predicate::str::contains("日本語"));
}

#[test]
fn test_view_mixed_line_endings() {
    let fixture = TestFixture::new();
    let content = "Unix\nWindows\r\nMac\rMixed\n\r\n";
    let file_path = fixture.create_file("mixed_endings.txt", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("Unix"))
        .stdout(predicate::str::contains("Windows"));
}

// Large file handling
#[test]
fn test_view_very_large_file() {
    let fixture = TestFixture::new();
    let content = "x".repeat(100_000);
    let file_path = fixture.create_file("huge.txt", &content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("5")
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("Plain Text"));
}

#[test]
fn test_view_many_lines_file() {
    let fixture = TestFixture::new();
    let lines: Vec<String> = (1..=10000).map(|i| format!("Line {}", i)).collect();
    let content = lines.join("\n");
    let file_path = fixture.create_file("many_lines.txt", &content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 1"))
        .stdout(predicate::str::contains("Line 3"))
        .stdout(predicate::str::contains("showing first 3 lines"))
        .stdout(predicate::str::contains("Line 100").not());
}

// Binary file edge cases
#[test]
fn test_view_null_bytes_file() {
    let fixture = TestFixture::new();
    let binary_data = vec![0x00, 0x01, 0x02, 0x00, 0xFF, 0x00];
    let file_path = fixture.root_path.join("nulls.bin");
    fs::write(&file_path, &binary_data).unwrap();

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("binary")
        .assert()
        .success()
        .stdout(predicate::str::contains("🔢 Binary File"))
        .stdout(predicate::str::contains("00 01 02"));
}

#[test]
fn test_view_mixed_binary_text() {
    let fixture = TestFixture::new();
    let mut data = b"Hello".to_vec();
    data.extend_from_slice(&[0x00, 0xFF, 0x80]);
    data.extend_from_slice(b" World");
    let file_path = fixture.root_path.join("mixed.dat");
    fs::write(&file_path, &data).unwrap();

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("binary")
        .assert()
        .success()
        .stdout(predicate::str::contains("🔢 Binary File"))
        .stdout(predicate::str::contains("Hello"));
}

// JSON edge cases
#[test]
fn test_view_malformed_json() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("bad.json", r#"{"incomplete": true"#);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error viewing file"));
}

#[test]
fn test_view_nested_json() {
    let fixture = TestFixture::new();
    let content = r#"{
        "level1": {
            "level2": {
                "level3": {
                    "deep": "value",
                    "array": [1, 2, {"nested": true}]
                }
            }
        }
    }"#;
    let file_path = fixture.create_file("nested.json", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"))
        .stdout(predicate::str::contains("level3"))
        .stdout(predicate::str::contains("nested"));
}

#[test]
fn test_view_json_array() {
    let fixture = TestFixture::new();
    let content = r#"[{"id": 1}, {"id": 2}, {"id": 3}]"#;
    let file_path = fixture.create_file("array.json", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"));
}

// CSV edge cases
#[test]
fn test_view_csv_with_quotes() {
    let fixture = TestFixture::new();
    let content = r#"name,description
"John Doe","A person with ""quotes"""
"Jane, Smith","Has comma in name""#;
    let file_path = fixture.create_file("quotes.csv", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"))
        .stdout(predicate::str::contains("John Doe"));
}

#[test]
fn test_view_csv_empty_fields() {
    let fixture = TestFixture::new();
    let content = "a,b,c\n1,,3\n,2,\n,,";
    let file_path = fixture.create_file("empty_fields.csv", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"));
}

#[test]
fn test_view_csv_single_column() {
    let fixture = TestFixture::new();
    let content = "values\n1\n2\n3";
    let file_path = fixture.create_file("single.csv", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"))
        .stdout(predicate::str::contains("values"));
}

#[test]
fn test_view_csv_unusual_delimiter() {
    let fixture = TestFixture::new();
    let content = "a|b|c\n1|2|3";
    let file_path = fixture.create_file("pipe.csv", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--delimiter")
        .arg("|")
        .assert()
        .success()
        .stdout(predicate::str::contains("📊 CSV File"));
}

// YAML edge cases
#[test]
fn test_view_malformed_yaml() {
    let fixture = TestFixture::new();
    let content = "key: value\n  invalid: indentation";
    let file_path = fixture.create_file("bad.yaml", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄 YAML File"));
}

#[test]
fn test_view_complex_yaml() {
    let fixture = TestFixture::new();
    let content = r#"
services:
  web:
    image: nginx
    ports:
      - "80:80"
    environment:
      - NODE_ENV=production
    volumes:
      - ./data:/data
"#;
    let file_path = fixture.create_file("docker.yaml", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄 YAML File"))
        .stdout(predicate::str::contains("services"));
}

// TOML edge cases
#[test]
fn test_view_malformed_toml() {
    let fixture = TestFixture::new();
    let content = "[section\nkey = value";
    let file_path = fixture.create_file("bad.toml", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("⚙️  TOML File"));
}

// File permission edge cases
#[test]
#[cfg(unix)]
fn test_view_unreadable_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("secret.txt", "secret content");

    // Make file unreadable
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&file_path, perms).unwrap();

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error viewing file"));
}

// Special file types
#[test]
fn test_view_directory_as_file() {
    let fixture = TestFixture::new();
    let dir_path = fixture.create_dir("testdir");

    // On Windows, viewing directories may fail with permission errors
    // On Unix, directories can be viewed as binary files
    let result = Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&dir_path)
        .assert();

    if cfg!(windows) {
        // Windows may deny access to directories
        result
            .failure()
            .stderr(predicate::str::contains("Error viewing file"));
    } else {
        // Unix systems can view directories as binary files
        result.success().stdout(predicate::str::contains("📄"));
    }
}

// Format detection edge cases
#[test]
fn test_view_no_extension_json_content() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("noext", r#"{"type": "json"}"#);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("auto")
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"));
}

#[test]
fn test_view_wrong_extension_correct_format() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("data.txt", r#"{"actually": "json"}"#);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("📋 JSON File"));
}

// Boundary conditions
#[test]
fn test_view_max_lines_zero() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "line1\nline2\nline3");

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("0")
        .assert()
        .success()
        .stdout(predicate::str::contains("showing first 0 lines"));
}

#[test]
fn test_view_max_bytes_zero() {
    let fixture = TestFixture::new();
    let binary_data = vec![1, 2, 3, 4, 5];
    let file_path = fixture.root_path.join("test.bin");
    fs::write(&file_path, &binary_data).unwrap();

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--viewer-format")
        .arg("binary")
        .arg("--max-bytes")
        .arg("0")
        .assert()
        .success()
        .stdout(predicate::str::contains("showing first 0 bytes"));
}

#[test]
fn test_view_max_lines_exceeds_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("short.txt", "line1\nline2");

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("100")
        .assert()
        .success()
        .stdout(predicate::str::contains("line1"))
        .stdout(predicate::str::contains("line2"));
}

// Control characters and special content
#[test]
fn test_view_control_characters() {
    let fixture = TestFixture::new();
    let content = "Line1\x07\x08\x09\nLine2\x1B[31mRed\x1B[0m";
    let file_path = fixture.create_file("control.txt", content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"));
}

#[test]
fn test_view_very_long_lines() {
    let fixture = TestFixture::new();
    let long_line = "x".repeat(10000);
    let content = format!("short\n{}\nshort", long_line);
    let file_path = fixture.create_file("long_lines.txt", &content);

    Command::cargo_bin("denarborea")
        .unwrap()
        .arg("--view")
        .arg(&file_path)
        .arg("--max-lines")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("📄"))
        .stdout(predicate::str::contains("short"));
}
