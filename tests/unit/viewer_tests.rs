use denarborea::viewer::*;
use denarborea::ViewerStrategy;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

struct TestSetup {
    temp_dir: TempDir,
}

impl TestSetup {
    fn new() -> Self {
        Self {
            temp_dir: TempDir::new().unwrap(),
        }
    }

    fn create_file(&self, name: &str, content: &[u8]) -> std::path::PathBuf {
        let path = self.temp_dir.path().join(name);
        fs::write(&path, content).unwrap();
        path
    }
}

#[test]
fn test_detect_format_by_extension() {
    assert_eq!(
        detect_format(Path::new("test.json"), None),
        ViewerFormat::Json
    );
    assert_eq!(
        detect_format(Path::new("test.csv"), None),
        ViewerFormat::Csv
    );
    assert_eq!(
        detect_format(Path::new("test.yaml"), None),
        ViewerFormat::Yaml
    );
    assert_eq!(
        detect_format(Path::new("test.yml"), None),
        ViewerFormat::Yaml
    );
    assert_eq!(
        detect_format(Path::new("test.toml"), None),
        ViewerFormat::Toml
    );
    assert_eq!(
        detect_format(Path::new("test.txt"), None),
        ViewerFormat::Text
    );
    // .bin extension defaults to Text since file doesn't exist to check if binary
    assert_eq!(
        detect_format(Path::new("test.bin"), None),
        ViewerFormat::Text
    );
}

#[test]
fn test_detect_format_override() {
    assert_eq!(
        detect_format(Path::new("test.txt"), Some(ViewerFormat::Json)),
        ViewerFormat::Json
    );
    assert_eq!(
        detect_format(Path::new("test.json"), Some(ViewerFormat::Binary)),
        ViewerFormat::Binary
    );
}

#[test]
fn test_detect_format_no_extension() {
    assert_eq!(detect_format(Path::new("noext"), None), ViewerFormat::Text);
    assert_eq!(
        detect_format(Path::new("path/noext"), None),
        ViewerFormat::Text
    );
}

#[test]
fn test_detect_format_case_insensitive() {
    assert_eq!(
        detect_format(Path::new("test.JSON"), None),
        ViewerFormat::Json
    );
    assert_eq!(
        detect_format(Path::new("test.CSV"), None),
        ViewerFormat::Csv
    );
    assert_eq!(
        detect_format(Path::new("test.YAML"), None),
        ViewerFormat::Yaml
    );
}

#[test]
fn test_is_binary_file_detection() {
    let setup = TestSetup::new();

    // Text file
    let text_file = setup.create_file("text.txt", b"Hello World\nLine 2");
    assert!(!is_binary_file(&text_file));

    // Binary file with null bytes
    let binary_file = setup.create_file("binary.bin", &[0x00, 0x01, 0xFF, 0x80]);
    assert!(is_binary_file(&binary_file));

    // Mixed content (should be detected as binary due to null bytes)
    let mixed_file = setup.create_file("mixed.dat", b"Hello\x00World");
    assert!(is_binary_file(&mixed_file));

    // UTF-8 with high bytes (should not be binary)
    let utf8_file = setup.create_file("utf8.txt", "Hello 世界 🌍".as_bytes());
    assert!(!is_binary_file(&utf8_file));
}

#[test]
fn test_format_file_size() {
    assert_eq!(format_file_size(0), "0 B");
    assert_eq!(format_file_size(1023), "1023 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1536), "1.5 KB");
    assert_eq!(format_file_size(1048576), "1.0 MB");
    assert_eq!(format_file_size(1073741824), "1.0 GB");
}

#[test]
fn test_truncate_content_by_lines() {
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";

    assert_eq!(
        truncate_content(content, Some(3), None),
        "Line 1\nLine 2\nLine 3"
    );
    assert_eq!(truncate_content(content, Some(0), None), "");
    assert_eq!(truncate_content(content, Some(10), None), content);
    assert_eq!(truncate_content(content, None, None), content);
}

#[test]
fn test_truncate_content_by_bytes() {
    let content = "Hello World";

    assert_eq!(truncate_content(content, None, Some(5)), "Hello");
    assert_eq!(truncate_content(content, None, Some(0)), "");
    assert_eq!(truncate_content(content, None, Some(100)), content);
}

#[test]
fn test_truncate_content_both_limits() {
    let content = "Line 1\nLine 2\nLine 3\nLine 4";

    // Bytes limit should take precedence if smaller
    assert_eq!(truncate_content(content, Some(5), Some(10)), "Line 1\nLin");

    // Lines limit should take precedence if smaller
    assert_eq!(
        truncate_content(content, Some(2), Some(100)),
        "Line 1\nLine 2"
    );
}

#[test]
fn test_escape_control_characters() {
    assert_eq!(escape_control_chars("Hello\nWorld"), "Hello\\nWorld");
    assert_eq!(escape_control_chars("Tab\tSeparated"), "Tab\\tSeparated");
    assert_eq!(
        escape_control_chars("Carriage\rReturn"),
        "Carriage\\rReturn"
    );
    assert_eq!(escape_control_chars("Bell\x07Sound"), "Bell\\x07Sound");
    assert_eq!(escape_control_chars("Normal text"), "Normal text");
}

#[test]
fn test_detect_csv_delimiter() {
    assert_eq!(detect_csv_delimiter("a,b,c\n1,2,3"), ',');
    assert_eq!(detect_csv_delimiter("a;b;c\n1;2;3"), ';');
    assert_eq!(detect_csv_delimiter("a\tb\tc\n1\t2\t3"), '\t');
    assert_eq!(detect_csv_delimiter("a|b|c\n1|2|3"), '|');
    assert_eq!(detect_csv_delimiter("no delimiters here"), ','); // fallback
}

#[test]
fn test_highlight_syntax_basic() {
    let result = highlight_syntax("let x = 42;", "rs");
    assert!(result.contains("let"));
    assert!(result.contains("42"));
}

#[test]
fn test_highlight_syntax_unsupported_language() {
    let code = "some code";
    let result = highlight_syntax(code, "unknown");
    assert_eq!(result, code); // Should return original if highlighting fails
}

#[test]
fn test_parse_json_valid() {
    let json = r#"{"name": "test", "value": 42}"#;
    let result = parse_json_content(json);
    assert!(result.contains("name"));
    assert!(result.contains("test"));
    assert!(result.contains("42"));
}

#[test]
fn test_parse_json_invalid() {
    let json = r#"{"invalid": json"#;
    let result = parse_json_content(json);
    assert!(result.contains("Error parsing JSON"));
}

#[test]
fn test_parse_yaml_valid() {
    let yaml = "name: test\nvalue: 42";
    let result = parse_yaml_content(yaml);
    assert!(result.contains("name"));
    assert!(result.contains("test"));
}

#[test]
fn test_parse_yaml_invalid() {
    let yaml = "invalid:\n  yaml:\nstructure";
    let result = parse_yaml_content(yaml);
    // Should still show content even if parsing fails
    assert!(result.contains("invalid"));
}

#[test]
fn test_parse_toml_valid() {
    let toml = "[package]\nname = \"test\"";
    let result = parse_toml_content(toml);
    assert!(result.contains("package"));
    assert!(result.contains("name"));
}

#[test]
fn test_parse_toml_invalid() {
    let toml = "[invalid\ntoml = structure";
    let result = parse_toml_content(toml);
    // Should show error or fallback to plain text
    assert!(!result.is_empty());
}

#[test]
fn test_format_hex_dump() {
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let result = format_hex_dump(&data, 0);

    assert!(result.contains("48 65 6c 6c 6f")); // lowercase hex
    assert!(result.contains("Hello"));
    assert!(result.contains("00000000"));
}

#[test]
fn test_format_hex_dump_with_offset() {
    let data = vec![0x41, 0x42, 0x43]; // "ABC"
    let result = format_hex_dump(&data, 16);

    assert!(result.contains("41 42 43"));
    assert!(result.contains("ABC"));
    assert!(result.contains("00000010")); // offset 16 in hex
}

#[test]
fn test_format_hex_dump_non_printable() {
    let data = vec![0x00, 0x01, 0xFF, 0x80];
    let result = format_hex_dump(&data, 0);

    assert!(result.contains("00 01 ff 80")); // lowercase hex
    assert!(result.contains("...."));
}

#[test]
fn test_detect_binary_file_type() {
    // PNG signature
    let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(detect_binary_file_type(&png_data), "PNG Image");

    // JPEG signature
    let jpeg_data = vec![0xFF, 0xD8, 0xFF];
    assert_eq!(detect_binary_file_type(&jpeg_data), "JPEG Image");

    // PDF signature
    let pdf_data = b"%PDF-1.4".to_vec();
    assert_eq!(detect_binary_file_type(&pdf_data), "PDF Document");

    // Unknown binary
    let unknown_data = vec![0x12, 0x34, 0x56, 0x78];
    assert_eq!(detect_binary_file_type(&unknown_data), "Unknown Binary");
}

#[test]
fn test_parse_csv_with_custom_delimiter() {
    let csv_content = "name;age;city\nJohn;30;NYC\nJane;25;LA";
    let result = parse_csv_content(csv_content, ';');

    assert!(result.contains("name"));
    assert!(result.contains("John"));
    assert!(result.contains("30"));
    assert!(result.contains("NYC"));
}

#[test]
fn test_parse_csv_with_quotes() {
    let csv_content = r#"name,description
"John Doe","A person with ""quotes"""
"Jane, Smith","Has comma in name""#;
    let result = parse_csv_content(csv_content, ',');

    assert!(result.contains("John Doe"));
    assert!(result.contains("Jane, Smith"));
}

#[test]
fn test_parse_csv_empty_fields() {
    let csv_content = "a,b,c\n1,,3\n,2,";
    let result = parse_csv_content(csv_content, ',');

    assert!(result.contains("a"));
    assert!(result.contains("1"));
    assert!(result.contains("3"));
}

#[test]
fn test_viewer_options_defaults() {
    let options = ViewerOptions::default();
    assert_eq!(options.max_lines, Some(100)); // Default has limits
    assert_eq!(options.max_bytes, Some(1024 * 1024));
    assert_eq!(options.delimiter, ',');
}

#[test]
fn test_viewer_options_with_limits() {
    let options = ViewerOptions {
        max_lines: Some(10),
        max_bytes: Some(1024),
        delimiter: ';',
        strategy: ViewerStrategy::Streaming,
        preview_size: 32 * 1024,
    };

    assert_eq!(options.max_lines, Some(10));
    assert_eq!(options.max_bytes, Some(1024));
    assert_eq!(options.delimiter, ';');
    assert_eq!(options.strategy, ViewerStrategy::Streaming);
    assert_eq!(options.preview_size, 32 * 1024);
}
