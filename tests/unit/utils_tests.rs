use denarborea::utils::{parse_size, count_files_in_dir};
use std::path::Path;

use crate::common::test_helpers::TestFixture;

#[test]
fn test_parse_size_bytes() {
    assert_eq!(parse_size(&Some("100".to_string())).unwrap(), Some(100));
    assert_eq!(parse_size(&Some("0".to_string())).unwrap(), Some(0));
}

#[test]
fn test_parse_size_kilobytes() {
    assert_eq!(parse_size(&Some("1KB".to_string())).unwrap(), Some(1024));
    assert_eq!(parse_size(&Some("1kb".to_string())).unwrap(), Some(1024));
    assert_eq!(parse_size(&Some("1K".to_string())).unwrap(), Some(1024));
    assert_eq!(parse_size(&Some("1k".to_string())).unwrap(), Some(1024));
    assert_eq!(parse_size(&Some("2KB".to_string())).unwrap(), Some(2048));
}

#[test]
fn test_parse_size_megabytes() {
    assert_eq!(parse_size(&Some("1MB".to_string())).unwrap(), Some(1024 * 1024));
    assert_eq!(parse_size(&Some("1mb".to_string())).unwrap(), Some(1024 * 1024));
    assert_eq!(parse_size(&Some("1M".to_string())).unwrap(), Some(1024 * 1024));
    assert_eq!(parse_size(&Some("2MB".to_string())).unwrap(), Some(2 * 1024 * 1024));
}

#[test]
fn test_parse_size_gigabytes() {
    assert_eq!(parse_size(&Some("1GB".to_string())).unwrap(), Some(1024 * 1024 * 1024));
    assert_eq!(parse_size(&Some("1gb".to_string())).unwrap(), Some(1024 * 1024 * 1024));
    assert_eq!(parse_size(&Some("1G".to_string())).unwrap(), Some(1024 * 1024 * 1024));
}

#[test]
fn test_parse_size_terabytes() {
    assert_eq!(parse_size(&Some("1TB".to_string())).unwrap(), Some(1024_u64.pow(4)));
    assert_eq!(parse_size(&Some("1tb".to_string())).unwrap(), Some(1024_u64.pow(4)));
    assert_eq!(parse_size(&Some("1T".to_string())).unwrap(), Some(1024_u64.pow(4)));
}

#[test]
fn test_parse_size_none() {
    assert_eq!(parse_size(&None).unwrap(), None);
}

#[test]
fn test_parse_size_invalid() {
    assert!(parse_size(&Some("invalid".to_string())).is_err());
    assert!(parse_size(&Some("".to_string())).is_err());
    assert!(parse_size(&Some("1XB".to_string())).is_err());
    assert!(parse_size(&Some("-1MB".to_string())).is_err());
}

#[test]
fn test_parse_size_decimal() {
    assert_eq!(parse_size(&Some("1.5KB".to_string())).unwrap(), Some(1536)); // 1.5 * 1024
    assert_eq!(parse_size(&Some("2.5MB".to_string())).unwrap(), Some((2.5 * 1024.0 * 1024.0) as u64));
}

#[test]
fn test_parse_size_with_spaces() {
    assert_eq!(parse_size(&Some(" 1MB ".to_string())).unwrap(), Some(1024 * 1024));
    assert_eq!(parse_size(&Some("1 MB".to_string())).unwrap(), Some(1024 * 1024));
}

#[test]
fn test_count_files_in_empty_dir() {
    let fixture = TestFixture::new();
    let empty_dir = fixture.create_dir("empty");
    
    let (files, dirs) = count_files_in_dir(&empty_dir);
    assert_eq!(files, 0);
    assert_eq!(dirs, 0);
}

#[test]
fn test_count_files_in_dir_with_files() {
    let fixture = TestFixture::new();
    fixture.create_file("file1.txt", "content");
    fixture.create_file("file2.txt", "content");
    fixture.create_file("file3.txt", "content");
    
    let (files, dirs) = count_files_in_dir(fixture.path());
    assert_eq!(files, 3);
    assert_eq!(dirs, 0);
}

#[test]
fn test_count_files_in_dir_with_subdirs() {
    let fixture = TestFixture::new();
    fixture.create_dir("subdir1");
    fixture.create_dir("subdir2");
    fixture.create_file("file1.txt", "content");
    
    let (files, dirs) = count_files_in_dir(fixture.path());
    assert_eq!(files, 1);
    assert_eq!(dirs, 2);
}

#[test]
fn test_count_files_nested_structure() {
    let fixture = TestFixture::new();
    fixture.create_file("subdir1/file1.txt", "content");
    fixture.create_file("subdir1/file2.txt", "content");
    fixture.create_file("subdir2/nested/file3.txt", "content");
    
    let (files, dirs) = count_files_in_dir(fixture.path());
    assert_eq!(files, 0); // Only direct children
    assert_eq!(dirs, 2); // subdir1 and subdir2
}

#[test]
fn test_count_files_with_hidden_files() {
    let fixture = TestFixture::new();
    fixture.create_file(".hidden", "content");
    fixture.create_file("visible.txt", "content");
    
    let (files, dirs) = count_files_in_dir(fixture.path());
    assert_eq!(files, 2); // Both hidden and visible files are counted
    assert_eq!(dirs, 0);
}

#[test]
fn test_count_files_nonexistent_dir() {
    let (files, dirs) = count_files_in_dir(Path::new("/nonexistent/path"));
    assert_eq!(files, 0);
    assert_eq!(dirs, 0);
}
