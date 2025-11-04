use denarborea::{Config, OutputFormat, SortBy};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();

    assert_eq!(config.max_depth, None);
    assert!(!config.show_hidden);
    assert!(!config.show_size);
    assert!(!config.show_permissions);
    assert!(!config.show_time);
    assert!(!config.show_count);
    assert!(config.use_colors);
    assert!(matches!(config.sort_by, SortBy::Name));
    assert!(!config.reverse_sort);
    assert_eq!(config.filter_extension, None);
    assert!(!config.directories_only);
    assert!(!config.files_only);
    assert_eq!(config.min_size, None);
    assert_eq!(config.max_size, None);
    assert!(config.exclude_patterns.is_empty());
    assert_eq!(config.include_patterns, None);
    assert!(!config.git_ignore);
    assert!(!config.git_status);
    assert_eq!(config.limit, None);
    assert!(matches!(config.output_format, OutputFormat::Tree));
    assert!(!config.follow_links);
    assert!(!config.full_path);
    assert!(!config.show_checksum);
    assert!(!config.show_stats);
    assert!(!config.interactive);
}

#[test]
fn test_get_extension_single() {
    let mut config = Config::default();
    config.filter_extension = Some("rs".to_string());

    let extensions = config.get_extension();
    assert_eq!(extensions, vec!["rs"]);
}

#[test]
fn test_get_extension_multiple() {
    let mut config = Config::default();
    config.filter_extension = Some("rs,py,js".to_string());

    let extensions = config.get_extension();
    assert_eq!(extensions, vec!["rs", "py", "js"]);
}

#[test]
fn test_get_extension_with_spaces() {
    let mut config = Config::default();
    config.filter_extension = Some("rs, py , js".to_string());

    let extensions = config.get_extension();
    assert_eq!(extensions, vec!["rs", "py", "js"]);
}

#[test]
fn test_get_extension_empty() {
    let config = Config::default();
    let extensions = config.get_extension();
    assert!(extensions.is_empty());
}

#[test]
fn test_matches_size_filter_no_limits() {
    let config = Config::default();
    assert!(config.matches_size_filter(0));
    assert!(config.matches_size_filter(1000));
    assert!(config.matches_size_filter(u64::MAX));
}

#[test]
fn test_matches_size_filter_min_only() {
    let mut config = Config::default();
    config.min_size = Some(100);

    assert!(!config.matches_size_filter(50));
    assert!(!config.matches_size_filter(99));
    assert!(config.matches_size_filter(100));
    assert!(config.matches_size_filter(200));
}

#[test]
fn test_matches_size_filter_max_only() {
    let mut config = Config::default();
    config.max_size = Some(1000);

    assert!(config.matches_size_filter(0));
    assert!(config.matches_size_filter(500));
    assert!(config.matches_size_filter(1000));
    assert!(!config.matches_size_filter(1001));
    assert!(!config.matches_size_filter(2000));
}

#[test]
fn test_matches_size_filter_both_limits() {
    let mut config = Config::default();
    config.min_size = Some(100);
    config.max_size = Some(1000);

    assert!(!config.matches_size_filter(50));
    assert!(!config.matches_size_filter(99));
    assert!(config.matches_size_filter(100));
    assert!(config.matches_size_filter(500));
    assert!(config.matches_size_filter(1000));
    assert!(!config.matches_size_filter(1001));
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("denarborea");
    fs::create_dir_all(&config_dir).unwrap();

    // Mock the config directory
    std::env::set_var("HOME", temp_dir.path());

    let mut original_config = Config::default();
    original_config.show_size = true;
    original_config.max_depth = Some(5);
    original_config.sort_by = SortBy::Size;

    // Test save
    let config_path = config_dir.join("config.toml");
    let toml_content = toml::to_string_pretty(&original_config).unwrap();
    fs::write(&config_path, toml_content).unwrap();

    // Test load
    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: Config = toml::from_str(&loaded_content).unwrap();

    assert_eq!(loaded_config.show_size, original_config.show_size);
    assert_eq!(loaded_config.max_depth, original_config.max_depth);
    assert!(matches!(loaded_config.sort_by, SortBy::Size));
}
