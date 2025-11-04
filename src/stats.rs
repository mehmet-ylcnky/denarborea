use crate::FileInfo;
use humansize::{format_size, DECIMAL};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TreeStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub file_types: HashMap<String, usize>,
    pub largest_file: Option<(String, u64)>,
    pub smallest_file: Option<(String, u64)>,
    pub avg_file_size: u64,
    pub symlinks: usize,
}

impl TreeStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, info: &FileInfo) {
        if info.is_dir {
            self.total_dirs += 1;
        } else {
            self.total_files += 1;
            self.total_size += info.size;

            // Track file types
            let extension = info
                .path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("no extension")
                .to_lowercase();

            *self.file_types.entry(extension).or_insert(0) += 1;

            // Track largest file
            if let Some((_, largest_size)) = &self.largest_file {
                if info.size > *largest_size {
                    self.largest_file = Some((info.name.clone(), info.size));
                }
            } else {
                self.largest_file = Some((info.name.clone(), info.size));
            }

            // Track smallest file
            if let Some((_, smallest_size)) = &self.smallest_file {
                if info.size < *smallest_size {
                    self.smallest_file = Some((info.name.clone(), info.size));
                }
            } else {
                self.smallest_file = Some((info.name.clone(), info.size));
            }
        }
        if info.is_symlink {
            self.symlinks += 1;
        }
    }

    pub fn finalize(&mut self) {
        if self.total_files > 0 {
            self.avg_file_size = self.total_size / self.total_files as u64;
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("\n Tree Statistics:\n");
        output.push_str(&format!("Directories: {}\n", self.total_dirs));
        output.push_str(&format!("Files: {}\n", self.total_files));
        output.push_str(&format!(
            "Total size: {}\n",
            format_size(self.total_size, DECIMAL)
        ));

        if self.total_files > 0 {
            output.push_str(&format!(
                "Average file size: {}\n",
                format_size(self.avg_file_size, DECIMAL)
            ));
        }

        if let Some((name, size)) = &self.largest_file {
            output.push_str(&format!(
                "Largest file: {} ({})\n",
                name,
                format_size(*size, DECIMAL)
            ));
        }

        if let Some((name, size)) = &self.smallest_file {
            output.push_str(&format!(
                "Smallest file: {} ({})\n",
                name,
                format_size(*size, DECIMAL)
            ));
        }

        if self.symlinks > 0 {
            output.push_str(&format!("Symlinks: {}\n", self.symlinks));
        }

        if !self.file_types.is_empty() {
            output.push_str("File types:\n");
            let mut types: Vec<_> = self.file_types.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1)); // sort by count, descending

            for (ext, count) in types.iter().take(10) {
                // show top 10
                output.push_str(&format!("{} files: {}\n", ext, count));
            }
        }
        output
    }
}
