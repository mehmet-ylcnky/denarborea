use denarborea::FileInfo;
use std::fs;
use std::path::Path;

use crate::common::test_helpers::{create_unicode_test_files, TestFixture};

#[test]
fn test_fileinfo_from_regular_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "hello world");

    let file_info = FileInfo::from_path(&file_path).unwrap();

    assert_eq!(file_info.name, "test.txt");
    assert_eq!(file_info.size, 11); // "hello world" is 11 bytes
    assert!(!file_info.is_dir);
    assert!(!file_info.is_symlink);
    assert!(file_info.modified_time.is_some());
    assert_eq!(file_info.file_count, None);
    assert_eq!(file_info.dir_count, None);
}

#[test]
fn test_fileinfo_from_directory() {
    let fixture = TestFixture::new();
    let dir_path = fixture.create_dir("testdir");
    fixture.create_file("testdir/file1.txt", "content");
    fixture.create_file("testdir/file2.txt", "content");
    fixture.create_dir("testdir/subdir");

    let file_info = FileInfo::from_path(&dir_path).unwrap();

    assert_eq!(file_info.name, "testdir");
    assert!(file_info.is_dir);
    assert!(!file_info.is_symlink);
    assert!(file_info.modified_time.is_some());
    assert_eq!(file_info.file_count, Some(2));
    assert_eq!(file_info.dir_count, Some(1));
}

#[test]
fn test_fileinfo_from_empty_directory() {
    let fixture = TestFixture::new();
    let dir_path = fixture.create_dir("empty");

    let file_info = FileInfo::from_path(&dir_path).unwrap();

    assert_eq!(file_info.name, "empty");
    assert!(file_info.is_dir);
    assert_eq!(file_info.file_count, Some(0));
    assert_eq!(file_info.dir_count, Some(0));
}

#[test]
fn test_fileinfo_from_zero_byte_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("empty.txt", "");

    let file_info = FileInfo::from_path(&file_path).unwrap();

    assert_eq!(file_info.name, "empty.txt");
    assert_eq!(file_info.size, 0);
    assert!(!file_info.is_dir);
}

#[test]
fn test_fileinfo_unicode_filename() {
    let fixture = TestFixture::new();
    create_unicode_test_files(&fixture);

    #[cfg(windows)]
    {
        let emoji_path = fixture.root_path.join("emoji_file.txt");
        let file_info = FileInfo::from_path(&emoji_path).unwrap();
        assert_eq!(file_info.name, "emoji_file.txt");

        let unicode_path = fixture.root_path.join("unicode_file.txt");
        let file_info = FileInfo::from_path(&unicode_path).unwrap();
        assert_eq!(file_info.name, "unicode_file.txt");

        let chinese_path = fixture.root_path.join("chinese_file.txt");
        let file_info = FileInfo::from_path(&chinese_path).unwrap();
        assert_eq!(file_info.name, "chinese_file.txt");
    }

    #[cfg(not(windows))]
    {
        let emoji_path = fixture.root_path.join("emoji_😀.txt");
        let file_info = FileInfo::from_path(&emoji_path).unwrap();
        assert_eq!(file_info.name, "emoji_😀.txt");

        let unicode_path = fixture.root_path.join("unicode_ñáéíóú.txt");
        let file_info = FileInfo::from_path(&unicode_path).unwrap();
        assert_eq!(file_info.name, "unicode_ñáéíóú.txt");

        let chinese_path = fixture.root_path.join("chinese_中文.txt");
        let file_info = FileInfo::from_path(&chinese_path).unwrap();
        assert_eq!(file_info.name, "chinese_中文.txt");
    }
}

#[test]
fn test_fileinfo_special_characters() {
    let fixture = TestFixture::new();

    #[cfg(windows)]
    let file_path = fixture.create_file("special_chars.txt", "content");

    #[cfg(not(windows))]
    let file_path = fixture.create_file("special!@#$%^&*().txt", "content");

    let file_info = FileInfo::from_path(&file_path).unwrap();

    #[cfg(windows)]
    assert_eq!(file_info.name, "special_chars.txt");

    #[cfg(not(windows))]
    assert_eq!(file_info.name, "special!@#$%^&*().txt");
}

#[test]
fn test_fileinfo_spaces_in_name() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("file with spaces.txt", "content");

    let file_info = FileInfo::from_path(&file_path).unwrap();
    assert_eq!(file_info.name, "file with spaces.txt");
}

#[test]
fn test_fileinfo_hidden_file() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file(".hidden", "secret");

    let file_info = FileInfo::from_path(&file_path).unwrap();
    assert_eq!(file_info.name, ".hidden");
    assert_eq!(file_info.size, 6);
}

#[test]
fn test_fileinfo_no_extension() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("README", "documentation");

    let file_info = FileInfo::from_path(&file_path).unwrap();
    assert_eq!(file_info.name, "README");
}

#[test]
fn test_fileinfo_multiple_extensions() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("archive.tar.gz", "compressed");

    let file_info = FileInfo::from_path(&file_path).unwrap();
    assert_eq!(file_info.name, "archive.tar.gz");
}

#[cfg(unix)]
#[test]
fn test_fileinfo_permissions_unix() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "content");

    // Make file executable
    let metadata = fs::metadata(&file_path).unwrap();
    let mut permissions = metadata.permissions();
    use std::os::unix::fs::PermissionsExt;
    permissions.set_mode(0o755);
    fs::set_permissions(&file_path, permissions).unwrap();

    let file_info = FileInfo::from_path(&file_path).unwrap();
    assert!(file_info.is_executable);
    assert!(file_info.permissions.is_some());
}

#[test]
fn test_fileinfo_symlink() {
    let fixture = TestFixture::new();
    let target_path = fixture.create_file("target.txt", "target content");
    let link_path = fixture.create_symlink("target.txt", "link.txt");

    let file_info = FileInfo::from_path(&link_path).unwrap();
    assert_eq!(file_info.name, "link.txt");
    assert!(file_info.is_symlink);
}

#[test]
fn test_fileinfo_calculate_checksum() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "hello");

    let mut file_info = FileInfo::from_path(&file_path).unwrap();
    assert_eq!(file_info.checksum, None);

    let checksum = file_info.calculate_checksum();
    assert!(checksum.is_some());
    assert_eq!(checksum.unwrap(), "5d41402abc4b2a76b9719d911017c592"); // MD5 of "hello"
}

#[test]
fn test_fileinfo_calculate_checksum_directory() {
    let fixture = TestFixture::new();
    let dir_path = fixture.create_dir("testdir");

    let mut file_info = FileInfo::from_path(&dir_path).unwrap();
    let checksum = file_info.calculate_checksum();
    assert!(checksum.is_none()); // Directories don't have checksums
}

#[test]
fn test_fileinfo_nonexistent_file() {
    let result = FileInfo::from_path(Path::new("/nonexistent/file.txt"));
    assert!(result.is_err());
}
