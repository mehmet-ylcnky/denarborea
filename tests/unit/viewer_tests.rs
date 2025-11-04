use denarborea::{FileViewer, ViewerFormat};
use std::fs;

use crate::common::test_helpers::TestFixture;

#[test]
fn test_text_viewer() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("test.txt", "Hello\nWorld\nTest");

    let viewer = FileViewer::new(ViewerFormat::Text);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("📄 Text File"));
    assert!(result.contains("Hello"));
    assert!(result.contains("World"));
    assert!(result.contains("Test"));
}

#[test]
fn test_json_viewer() {
    let fixture = TestFixture::new();
    let json_content = r#"{"name": "test", "value": 42, "items": [1, 2, 3]}"#;
    let file_path = fixture.create_file("test.json", json_content);

    let viewer = FileViewer::new(ViewerFormat::Json);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("📋 JSON File"));
    assert!(result.contains("test"));
    assert!(result.contains("42"));
    assert!(result.contains("📊 Summary"));
}

#[test]
fn test_csv_viewer_default_delimiter() {
    let fixture = TestFixture::new();
    let csv_content = "name,age,city\nJohn,30,NYC\nJane,25,LA";
    let file_path = fixture.create_file("test.csv", csv_content);

    let viewer = FileViewer::new(ViewerFormat::Csv);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("📊 CSV File"));
    assert!(result.contains("name"));
    assert!(result.contains("age"));
    assert!(result.contains("city"));
    assert!(result.contains("John"));
    assert!(result.contains("Jane"));
    assert!(result.contains("📈 Summary: 3 columns"));
}

#[test]
fn test_csv_viewer_custom_delimiter() {
    let fixture = TestFixture::new();
    let csv_content = "name;age;city\nJohn;30;NYC\nJane;25;LA";
    let file_path = fixture.create_file("test.csv", csv_content);

    let viewer = FileViewer::new(ViewerFormat::Csv).with_delimiter(';');
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("📊 CSV File"));
    assert!(result.contains("name"));
    assert!(result.contains("age"));
    assert!(result.contains("John"));
    assert!(result.contains("30"));
    assert!(result.contains("NYC"));
}

#[test]
fn test_binary_viewer() {
    let fixture = TestFixture::new();
    let binary_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header
    let file_path = fixture.root_path.join("test.bin");
    fs::write(&file_path, &binary_data).unwrap();

    let viewer = FileViewer::new(ViewerFormat::Binary);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("🔢 Binary File"));
    assert!(result.contains("PNG Image"));
    assert!(result.contains("Hex Dump"));
    assert!(result.contains("89504e47"));
}

#[test]
fn test_yaml_viewer() {
    let fixture = TestFixture::new();
    let yaml_content = "name: test\nvalue: 42\nitems:\n  - one\n  - two";
    let file_path = fixture.create_file("test.yaml", yaml_content);

    let viewer = FileViewer::new(ViewerFormat::Yaml);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("📄 YAML File"));
    assert!(result.contains("name: test"));
    assert!(result.contains("value: 42"));
}

#[test]
fn test_toml_viewer() {
    let fixture = TestFixture::new();
    let toml_content = "[package]\nname = \"test\"\nversion = \"1.0.0\"";
    let file_path = fixture.create_file("test.toml", toml_content);

    let viewer = FileViewer::new(ViewerFormat::Toml);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("⚙️  TOML File"));
    assert!(result.contains("package"));
    assert!(result.contains("name = \"test\""));
}

#[test]
fn test_auto_format_detection() {
    let fixture = TestFixture::new();

    // Test JSON auto-detection
    let json_file = fixture.create_file("data.json", r#"{"test": true}"#);
    let viewer = FileViewer::new(ViewerFormat::Auto);
    let result = viewer.view_file(&json_file).unwrap();
    assert!(result.contains("📋 JSON File"));

    // Test CSV auto-detection
    let csv_file = fixture.create_file("data.csv", "a,b,c\n1,2,3");
    let result = viewer.view_file(&csv_file).unwrap();
    assert!(result.contains("📊 CSV File"));

    // Test text auto-detection
    let txt_file = fixture.create_file("data.txt", "Hello World");
    let result = viewer.view_file(&txt_file).unwrap();
    assert!(result.contains("📄 Text File"));
}

#[test]
fn test_max_lines_limit() {
    let fixture = TestFixture::new();
    let content = (0..200)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let file_path = fixture.create_file("large.txt", &content);

    let viewer = FileViewer::new(ViewerFormat::Text).with_limits(Some(50), None);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("Line 0"));
    assert!(result.contains("Line 49"));
    assert!(!result.contains("Line 100"));
    assert!(result.contains("showing first 50 lines"));
}

#[test]
fn test_max_bytes_limit() {
    let fixture = TestFixture::new();
    let binary_data = vec![0u8; 2048]; // 2KB of zeros
    let file_path = fixture.root_path.join("large.bin");
    fs::write(&file_path, &binary_data).unwrap();

    let viewer = FileViewer::new(ViewerFormat::Binary).with_limits(None, Some(512));
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("🔢 Binary File"));
    assert!(result.contains("showing first 512 bytes"));
}

#[test]
fn test_invalid_json() {
    let fixture = TestFixture::new();
    let invalid_json = r#"{"invalid": json content"#;
    let file_path = fixture.create_file("invalid.json", invalid_json);

    let viewer = FileViewer::new(ViewerFormat::Json);
    let result = viewer.view_file(&file_path);

    assert!(result.is_err());
}

#[test]
fn test_invalid_yaml() {
    let fixture = TestFixture::new();
    let invalid_yaml = "invalid:\n  - yaml\n    content";
    let file_path = fixture.create_file("invalid.yaml", invalid_yaml);

    let viewer = FileViewer::new(ViewerFormat::Yaml);
    let result = viewer.view_file(&file_path).unwrap();

    assert!(result.contains("⚠️  Invalid YAML format"));
    assert!(result.contains("Raw content"));
}

#[test]
fn test_empty_csv() {
    let fixture = TestFixture::new();
    let file_path = fixture.create_file("empty.csv", "");

    let viewer = FileViewer::new(ViewerFormat::Csv);
    let result = viewer.view_file(&file_path);

    // Should handle empty CSV gracefully
    assert!(result.is_err() || result.unwrap().contains("CSV File"));
}

#[test]
fn test_nonexistent_file() {
    let viewer = FileViewer::new(ViewerFormat::Auto);
    let result = viewer.view_file(std::path::Path::new("/nonexistent/file.txt"));

    assert!(result.is_err());
}
