use crate::Result;
use memmap2::Mmap;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;

const SMALL_FILE_THRESHOLD: u64 = 1_000_000; // 1MB
const LARGE_FILE_THRESHOLD: u64 = 100_000_000; // 100MB
const DEFAULT_PREVIEW_SIZE: usize = 64 * 1024; // 64KB
const MAX_PREVIEW_ITEMS: usize = 100;

#[derive(Debug, Clone)]
pub enum ViewStrategy {
    FullLoad,
    Streaming,
    MemoryMapped,
}

pub fn determine_strategy(path: &Path) -> Result<ViewStrategy> {
    let metadata = std::fs::metadata(path)?;
    let size = metadata.len();

    Ok(match size {
        0..SMALL_FILE_THRESHOLD => ViewStrategy::FullLoad,
        SMALL_FILE_THRESHOLD..LARGE_FILE_THRESHOLD => ViewStrategy::Streaming,
        _ => ViewStrategy::MemoryMapped,
    })
}

pub fn view_large_json(path: &Path, max_items: Option<usize>) -> Result<String> {
    let strategy = determine_strategy(path)?;
    let max_items = max_items.unwrap_or(MAX_PREVIEW_ITEMS);

    match strategy {
        ViewStrategy::FullLoad => view_json_full_load(path),
        ViewStrategy::Streaming => view_json_streaming(path, max_items),
        ViewStrategy::MemoryMapped => view_json_memory_mapped(path, max_items),
    }
}

pub fn view_large_csv(path: &Path, max_rows: Option<usize>) -> Result<String> {
    let strategy = determine_strategy(path)?;
    let max_rows = max_rows.unwrap_or(MAX_PREVIEW_ITEMS);

    match strategy {
        ViewStrategy::FullLoad => view_csv_full_load(path, max_rows),
        ViewStrategy::Streaming => view_csv_streaming(path, max_rows),
        ViewStrategy::MemoryMapped => view_csv_memory_mapped(path, max_rows),
    }
}

pub fn view_large_text(path: &Path, max_lines: Option<usize>) -> Result<String> {
    let strategy = determine_strategy(path)?;
    let max_lines = max_lines.unwrap_or(MAX_PREVIEW_ITEMS);

    match strategy {
        ViewStrategy::FullLoad => view_text_full_load(path, max_lines),
        ViewStrategy::Streaming => view_text_streaming(path, max_lines),
        ViewStrategy::MemoryMapped => view_text_memory_mapped(path, max_lines),
    }
}

fn view_json_full_load(path: &Path) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    Ok(serde_json::to_string_pretty(&json)?)
}

fn view_json_streaming(path: &Path, max_items: usize) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut output = String::new();
    output.push_str(&format!("📋 JSON File: {} (Streaming)\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    let mut items_shown = 0;
    for result in stream {
        if items_shown >= max_items {
            output.push_str(&format!(
                "\n... (showing first {} items of large file)\n",
                max_items
            ));
            break;
        }

        match result {
            Ok(value) => {
                if items_shown > 0 {
                    output.push('\n');
                }
                output.push_str(&serde_json::to_string_pretty(&value)?);
                items_shown += 1;
            }
            Err(e) => {
                output.push_str(&format!(
                    "\nError parsing JSON at item {}: {}",
                    items_shown + 1,
                    e
                ));
                break;
            }
        }
    }

    Ok(output)
}

fn view_json_memory_mapped(path: &Path, max_items: usize) -> Result<String> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let mut output = String::new();
    let file_size = mmap.len();
    output.push_str(&format!(
        "📋 JSON File: {} ({} bytes, Memory-Mapped)\n",
        path.display(),
        file_size
    ));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    // Use a reasonable preview size for very large files
    let preview_size = DEFAULT_PREVIEW_SIZE.min(file_size);
    let preview_data = &mmap[..preview_size];

    let cursor = Cursor::new(preview_data);
    let stream = serde_json::Deserializer::from_reader(cursor).into_iter::<serde_json::Value>();

    let mut items_shown = 0;
    for result in stream {
        if items_shown >= max_items {
            output.push_str(&format!(
                "\n... (showing first {} items from {} bytes preview)\n",
                max_items, preview_size
            ));
            break;
        }

        match result {
            Ok(value) => {
                if items_shown > 0 {
                    output.push('\n');
                }
                output.push_str(&serde_json::to_string_pretty(&value)?);
                items_shown += 1;
            }
            Err(_) => break, // Stop on parse error in preview
        }
    }

    if file_size > preview_size {
        output.push_str(&format!(
            "\n📊 File too large - showing preview only ({} of {} bytes)\n",
            preview_size, file_size
        ));
    }

    Ok(output)
}

fn view_csv_full_load(path: &Path, max_rows: usize) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    parse_csv_content(&content, ',', max_rows)
}

fn view_csv_streaming(path: &Path, max_rows: usize) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut output = String::new();
    output.push_str(&format!("📊 CSV File: {} (Streaming)\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    // Get headers
    if let Ok(headers) = csv_reader.headers() {
        output.push_str(&format!(
            "Headers: {}\n\n",
            headers.iter().collect::<Vec<_>>().join(" | ")
        ));

        let mut rows_shown = 0;
        for result in csv_reader.records() {
            if rows_shown >= max_rows {
                output.push_str(&format!(
                    "\n... (showing first {} rows of large file)\n",
                    max_rows
                ));
                break;
            }

            match result {
                Ok(record) => {
                    output.push_str(&format!(
                        "Row {}: {}\n",
                        rows_shown + 1,
                        record.iter().collect::<Vec<_>>().join(" | ")
                    ));
                    rows_shown += 1;
                }
                Err(e) => {
                    output.push_str(&format!("Error reading row {}: {}\n", rows_shown + 1, e));
                    break;
                }
            }
        }
    }

    Ok(output)
}

fn view_csv_memory_mapped(path: &Path, max_rows: usize) -> Result<String> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let mut output = String::new();
    let file_size = mmap.len();
    output.push_str(&format!(
        "📊 CSV File: {} ({} bytes, Memory-Mapped)\n",
        path.display(),
        file_size
    ));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    let preview_size = DEFAULT_PREVIEW_SIZE.min(file_size);
    let preview_data = &mmap[..preview_size];

    let cursor = Cursor::new(preview_data);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(cursor);

    if let Ok(headers) = csv_reader.headers() {
        output.push_str(&format!(
            "Headers: {}\n\n",
            headers.iter().collect::<Vec<_>>().join(" | ")
        ));

        let mut rows_shown = 0;
        for result in csv_reader.records() {
            if rows_shown >= max_rows {
                output.push_str(&format!(
                    "\n... (showing first {} rows from preview)\n",
                    max_rows
                ));
                break;
            }

            match result {
                Ok(record) => {
                    output.push_str(&format!(
                        "Row {}: {}\n",
                        rows_shown + 1,
                        record.iter().collect::<Vec<_>>().join(" | ")
                    ));
                    rows_shown += 1;
                }
                Err(_) => break,
            }
        }
    }

    if file_size > preview_size {
        output.push_str(&format!(
            "\n📊 File too large - showing preview only ({} of {} bytes)\n",
            preview_size, file_size
        ));
    }

    Ok(output)
}

fn view_text_full_load(path: &Path, max_lines: usize) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().take(max_lines).collect();
    Ok(lines.join("\n"))
}

fn view_text_streaming(path: &Path, max_lines: usize) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut output = String::new();
    output.push_str(&format!("📄 Text File: {} (Streaming)\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    let mut lines_shown = 0;
    for (line_num, line) in reader.lines().enumerate() {
        if lines_shown >= max_lines {
            output.push_str(&format!(
                "\n... (showing first {} lines of large file)\n",
                max_lines
            ));
            break;
        }

        match line {
            Ok(content) => {
                output.push_str(&format!("{:4} │ {}\n", line_num + 1, content));
                lines_shown += 1;
            }
            Err(e) => {
                output.push_str(&format!(
                    "{:4} │ <error reading line: {}>\n",
                    line_num + 1,
                    e
                ));
                break;
            }
        }
    }

    Ok(output)
}

fn view_text_memory_mapped(path: &Path, max_lines: usize) -> Result<String> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let mut output = String::new();
    let file_size = mmap.len();
    output.push_str(&format!(
        "📄 Text File: {} ({} bytes, Memory-Mapped)\n",
        path.display(),
        file_size
    ));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    let preview_size = DEFAULT_PREVIEW_SIZE.min(file_size);
    let preview_data = &mmap[..preview_size];

    let preview_str = String::from_utf8_lossy(preview_data);

    for (lines_shown, (line_num, line)) in preview_str.lines().enumerate().enumerate() {
        if lines_shown >= max_lines {
            output.push_str(&format!(
                "\n... (showing first {} lines from preview)\n",
                max_lines
            ));
            break;
        }

        output.push_str(&format!("{:4} │ {}\n", line_num + 1, line));
    }

    if file_size > preview_size {
        output.push_str(&format!(
            "\n📊 File too large - showing preview only ({} of {} bytes)\n",
            preview_size, file_size
        ));
    }

    Ok(output)
}

fn parse_csv_content(content: &str, delimiter: char, max_rows: usize) -> Result<String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(std::io::Cursor::new(content));

    let mut output = String::new();

    if let Ok(headers) = reader.headers() {
        output.push_str(&format!(
            "Headers: {}\n\n",
            headers.iter().collect::<Vec<_>>().join(" | ")
        ));

        for (i, result) in reader.records().enumerate() {
            if i >= max_rows {
                output.push_str(&format!("\n... (showing first {} rows)\n", max_rows));
                break;
            }

            if let Ok(record) = result {
                output.push_str(&format!(
                    "Row {}: {}\n",
                    i + 1,
                    record.iter().collect::<Vec<_>>().join(" | ")
                ));
            }
        }
    }

    Ok(output)
}
