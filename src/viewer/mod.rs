pub mod binary;
pub mod csv_viewer;
pub mod parquet_viewer;
pub mod structured;
pub mod text;

use crate::Result;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewerFormat {
    Auto,
    Text,
    Binary,
    Json,
    Yaml,
    Toml,
    Csv,
    Parquet,
}

#[derive(Debug, Clone)]
pub struct ViewerOptions {
    pub max_lines: Option<usize>,
    pub max_bytes: Option<usize>,
    pub delimiter: char,
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            max_lines: Some(100),
            max_bytes: Some(1024 * 1024), // 1MB
            delimiter: ',',
        }
    }
}

pub struct FileViewer {
    format: ViewerFormat,
    max_lines: Option<usize>,
    max_bytes: Option<usize>,
    delimiter: char,
}

impl FileViewer {
    pub fn new(format: ViewerFormat) -> Self {
        Self {
            format,
            max_lines: Some(100),
            max_bytes: Some(1024 * 1024), // 1MB
            delimiter: ',',
        }
    }

    pub fn with_limits(mut self, max_lines: Option<usize>, max_bytes: Option<usize>) -> Self {
        self.max_lines = max_lines;
        self.max_bytes = max_bytes;
        self
    }

    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn view_file(&self, path: &Path) -> Result<String> {
        let format = match self.format {
            ViewerFormat::Auto => detect_format(path, None),
            _ => self.format.clone(),
        };

        match format {
            ViewerFormat::Text => text::view_text_file(path, self.max_lines),
            ViewerFormat::Binary => binary::view_binary_file(path, self.max_bytes),
            ViewerFormat::Json => structured::view_json_file(path),
            ViewerFormat::Yaml => structured::view_yaml_file(path),
            ViewerFormat::Toml => structured::view_toml_file(path),
            ViewerFormat::Csv => csv_viewer::view_csv_file(path, self.max_lines, self.delimiter),
            ViewerFormat::Parquet => parquet_viewer::view_parquet_file(path, self.max_lines),
            ViewerFormat::Auto => unreachable!(),
        }
    }
}

// Public functions for testing
pub fn detect_format(path: &Path, override_format: Option<ViewerFormat>) -> ViewerFormat {
    if let Some(format) = override_format {
        return format;
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "json" => ViewerFormat::Json,
            "yaml" | "yml" => ViewerFormat::Yaml,
            "toml" => ViewerFormat::Toml,
            "csv" => ViewerFormat::Csv,
            "parquet" => ViewerFormat::Parquet,
            "txt" | "md" | "rs" | "py" | "js" | "ts" | "html" | "css" | "xml" => ViewerFormat::Text,
            _ => {
                if is_binary_file(path) {
                    ViewerFormat::Binary
                } else {
                    ViewerFormat::Text
                }
            }
        }
    } else {
        ViewerFormat::Text
    }
}

pub fn is_binary_file(path: &Path) -> bool {
    use std::fs::File;
    use std::io::Read;

    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 512];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            // Check for null bytes (common in binary files)
            return buffer[..bytes_read].contains(&0);
        }
    }
    false
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f = bytes as f64;
    let unit_index = (bytes_f.log2() / THRESHOLD.log2()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);

    if unit_index == 0 {
        format!("{} B", bytes)
    } else {
        let size = bytes_f / THRESHOLD.powi(unit_index as i32);
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn truncate_content(
    content: &str,
    max_lines: Option<usize>,
    max_bytes: Option<usize>,
) -> String {
    let mut result = content;

    // Apply byte limit first
    if let Some(max_bytes) = max_bytes {
        if content.len() > max_bytes {
            result = &content[..max_bytes];
        }
    }

    // Apply line limit
    if let Some(max_lines) = max_lines {
        let lines: Vec<&str> = result.lines().take(max_lines).collect();
        lines.join("\n")
    } else {
        result.to_string()
    }
}

pub fn escape_control_chars(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            '\r' => "\\r".to_string(),
            c if c.is_control() => format!("\\x{:02x}", c as u8),
            c => c.to_string(),
        })
        .collect()
}

pub fn detect_csv_delimiter(content: &str) -> char {
    let delimiters = [',', ';', '\t', '|'];
    let mut counts = [0; 4];

    for line in content.lines().take(5) {
        for (i, &delim) in delimiters.iter().enumerate() {
            counts[i] += line.matches(delim).count();
        }
    }

    // Return delimiter with highest count, fallback to comma
    delimiters[counts
        .iter()
        .position(|&x| x == *counts.iter().max().unwrap())
        .unwrap_or(0)]
}

pub fn highlight_syntax(code: &str, language: &str) -> String {
    // Simple syntax highlighting - in real implementation would use syntect
    match language {
        "rs" | "rust" => {
            // Basic Rust keyword highlighting
            let mut result = code.to_string();
            let keywords = ["let", "fn", "struct", "impl", "pub", "use"];
            for keyword in keywords {
                result = result.replace(keyword, &format!("\x1b[94m{}\x1b[0m", keyword));
            }
            result
        }
        _ => code.to_string(), // Return original if language not supported
    }
}

pub fn parse_json_content(content: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| content.to_string()),
        Err(e) => format!("Error parsing JSON: {}\n\nRaw content:\n{}", e, content),
    }
}

pub fn parse_yaml_content(content: &str) -> String {
    match serde_yaml::from_str::<serde_yaml::Value>(content) {
        Ok(value) => serde_yaml::to_string(&value).unwrap_or_else(|_| content.to_string()),
        Err(_) => content.to_string(), // Return original if parsing fails
    }
}

pub fn parse_toml_content(content: &str) -> String {
    match toml::from_str::<toml::Value>(content) {
        Ok(value) => toml::to_string_pretty(&value).unwrap_or_else(|_| content.to_string()),
        Err(_) => content.to_string(), // Return original if parsing fails
    }
}

pub fn format_hex_dump(data: &[u8], offset: usize) -> String {
    let mut result = String::new();

    for (i, chunk) in data.chunks(16).enumerate() {
        let addr = offset + i * 16;
        result.push_str(&format!("{:08x}  ", addr));

        // Hex bytes
        for (j, &byte) in chunk.iter().enumerate() {
            if j == 8 {
                result.push(' ');
            }
            result.push_str(&format!("{:02x} ", byte));
        }

        // Padding for incomplete lines
        for j in chunk.len()..16 {
            if j == 8 {
                result.push(' ');
            }
            result.push_str("   ");
        }

        result.push_str(" |");

        // ASCII representation
        for &byte in chunk {
            if byte.is_ascii_graphic() || byte == b' ' {
                result.push(byte as char);
            } else {
                result.push('.');
            }
        }

        result.push_str("|\n");
    }

    result
}

pub fn detect_binary_file_type(data: &[u8]) -> &'static str {
    if data.len() >= 8 && &data[0..8] == b"\x89PNG\r\n\x1a\n" {
        "PNG Image"
    } else if data.len() >= 3 && &data[0..3] == b"\xFF\xD8\xFF" {
        "JPEG Image"
    } else if data.len() >= 5 && &data[0..5] == b"%PDF-" {
        "PDF Document"
    } else if data.len() >= 4 && &data[0..4] == b"PK\x03\x04" {
        "ZIP Archive"
    } else {
        "Unknown Binary"
    }
}

pub fn parse_csv_content(content: &str, delimiter: char) -> String {
    use csv::ReaderBuilder;
    use std::io::Cursor;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(Cursor::new(content));

    let mut result = String::new();

    // Get headers
    if let Ok(headers) = reader.headers() {
        result.push_str(&format!(
            "Headers: {}\n",
            headers.iter().collect::<Vec<_>>().join(", ")
        ));

        // Read a few rows
        for (i, record_result) in reader.records().enumerate() {
            if i >= 5 {
                break;
            } // Limit for testing

            if let Ok(record) = record_result {
                result.push_str(&format!(
                    "Row {}: {}\n",
                    i + 1,
                    record.iter().collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }

    result
}
