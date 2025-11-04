use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

use crate::common::test_helpers::TestFixture;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Directory visualizer for Dena"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_cli_basic_tree() {
    let fixture = TestFixture::new();
    fixture.create_file("file1.txt", "content");
    fixture.create_file("subdir/file2.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("subdir"));
}

#[test]
fn test_cli_show_size() {
    let fixture = TestFixture::new();
    fixture.create_file("test.txt", "hello world");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--size")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("11")); // File size
}

#[test]
fn test_cli_max_depth() {
    let fixture = TestFixture::new();
    fixture.create_file("level1/level2/level3/deep.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--max-depth")
        .arg("2")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("level2"))
        .stdout(predicate::str::contains("deep.txt").not());
}

#[test]
fn test_cli_filter_extension() {
    let fixture = TestFixture::new();
    fixture.create_file("test.rs", "rust code");
    fixture.create_file("test.py", "python code");
    fixture.create_file("test.txt", "text");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--extension")
        .arg("rs")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test.rs"))
        .stdout(predicate::str::contains("test.py").not())
        .stdout(predicate::str::contains("test.txt").not());
}

#[test]
fn test_cli_directories_only() {
    let fixture = TestFixture::new();
    fixture.create_file("file.txt", "content");
    fixture.create_dir("directory");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--directories-only")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("directory"))
        .stdout(predicate::str::contains("file.txt").not());
}

#[test]
fn test_cli_files_only() {
    let fixture = TestFixture::new();
    fixture.create_file("file.txt", "content");
    fixture.create_dir("directory");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--files-only")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("file.txt"))
        .stdout(predicate::str::contains("directory").not());
}

#[test]
fn test_cli_json_output() {
    let fixture = TestFixture::new();
    fixture.create_file("test.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--format")
        .arg("json")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("{"))
        .stdout(predicate::str::contains("test.txt"));
}

#[test]
fn test_cli_output_to_file() {
    let fixture = TestFixture::new();
    fixture.create_file("input.txt", "content");
    let output_file = fixture.root_path.join("output.txt");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--output")
        .arg(&output_file)
        .arg(fixture.path())
        .assert()
        .success();
    
    assert!(output_file.exists());
    let content = fs::read_to_string(&output_file).unwrap();
    assert!(content.contains("input.txt"));
}

#[test]
fn test_cli_sort_by_size() {
    let fixture = TestFixture::new();
    fixture.create_file("small.txt", "a");
    fixture.create_file("large.txt", "this is a much larger file");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--sort")
        .arg("size")
        .arg("--size")
        .arg(fixture.path())
        .assert()
        .success();
}

#[test]
fn test_cli_reverse_sort() {
    let fixture = TestFixture::new();
    fixture.create_file("a.txt", "content");
    fixture.create_file("z.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--reverse")
        .arg(fixture.path())
        .assert()
        .success();
}

#[test]
fn test_cli_show_hidden() {
    let fixture = TestFixture::new();
    fixture.create_file(".hidden", "secret");
    fixture.create_file("visible.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--all")
        .arg(fixture.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"));
}

#[test]
fn test_cli_no_color() {
    let fixture = TestFixture::new();
    fixture.create_file("test.txt", "content");
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--no-color")
        .arg(fixture.path())
        .assert()
        .success();
}

#[test]
fn test_cli_invalid_path() {
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("/nonexistent/path")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_cli_invalid_extension() {
    let fixture = TestFixture::new();
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--extension")
        .arg("")
        .arg(fixture.path())
        .assert()
        .success(); // Empty extension should be handled gracefully
}

#[test]
fn test_cli_conflicting_flags() {
    let fixture = TestFixture::new();
    
    let mut cmd = Command::cargo_bin("denarborea").unwrap();
    cmd.arg("--directories-only")
        .arg("--files-only")
        .arg(fixture.path())
        .assert()
        .success(); // Should handle conflicting flags gracefully
}
