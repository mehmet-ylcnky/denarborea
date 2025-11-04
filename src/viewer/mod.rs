pub mod binary;
pub mod csv_viewer;
pub mod parquet_viewer;
pub mod structured;
pub mod text;

use crate::Result;
use std::path::Path;

#[derive(Debug, Clone)]
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

pub struct FileViewer {
    format: ViewerFormat,
    max_lines: Option<usize>,
    max_bytes: Option<usize>,
}

impl FileViewer {
    pub fn new(format: ViewerFormat) -> Self {
        Self {
            format,
            max_lines: Some(100),
            max_bytes: Some(1024 * 1024), // 1MB
        }
    }

    pub fn with_limits(mut self, max_lines: Option<usize>, max_bytes: Option<usize>) -> Self {
        self.max_lines = max_lines;
        self.max_bytes = max_bytes;
        self
    }

    pub fn view_file(&self, path: &Path) -> Result<String> {
        let format = match self.format {
            ViewerFormat::Auto => self.detect_format(path),
            _ => self.format.clone(),
        };

        match format {
            ViewerFormat::Text => text::view_text_file(path, self.max_lines),
            ViewerFormat::Binary => binary::view_binary_file(path, self.max_bytes),
            ViewerFormat::Json => structured::view_json_file(path),
            ViewerFormat::Yaml => structured::view_yaml_file(path),
            ViewerFormat::Toml => structured::view_toml_file(path),
            ViewerFormat::Csv => csv_viewer::view_csv_file(path, self.max_lines),
            ViewerFormat::Parquet => parquet_viewer::view_parquet_file(path, self.max_lines),
            ViewerFormat::Auto => unreachable!(),
        }
    }

    fn detect_format(&self, path: &Path) -> ViewerFormat {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "json" => ViewerFormat::Json,
                "yaml" | "yml" => ViewerFormat::Yaml,
                "toml" => ViewerFormat::Toml,
                "csv" => ViewerFormat::Csv,
                "parquet" => ViewerFormat::Parquet,
                "txt" | "md" | "rs" | "py" | "js" | "ts" | "html" | "css" | "xml" => {
                    ViewerFormat::Text
                }
                _ => {
                    // Check if file is binary
                    if self.is_binary_file(path) {
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

    fn is_binary_file(&self, path: &Path) -> bool {
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
}
