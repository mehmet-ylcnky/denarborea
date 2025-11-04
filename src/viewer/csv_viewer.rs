use crate::Result;
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

pub fn view_csv_file(path: &Path, max_rows: Option<usize>, delimiter: char) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(file);

    let mut output = String::new();
    output.push_str(&format!("📊 CSV File: {}\n", path.display()));
    output.push_str("─".repeat(80).as_str());
    output.push('\n');

    // Get headers
    let headers = reader.headers()?.clone();
    let col_count = headers.len();

    // Calculate column widths
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len().max(10)).collect();

    // Sample some rows to determine better column widths
    let mut records = Vec::new();
    for (i, result) in reader.records().enumerate() {
        if let Some(max) = max_rows {
            if i >= max {
                break;
            }
        }

        match result {
            Ok(record) => {
                for (j, field) in record.iter().enumerate() {
                    if j < col_widths.len() {
                        col_widths[j] = col_widths[j].max(field.len().min(30));
                    }
                }
                records.push(record);
            }
            Err(e) => {
                output.push_str(&format!("Error reading row {}: {}\n", i + 1, e));
            }
        }
    }

    // Print headers
    output.push('│');
    for (i, header) in headers.iter().enumerate() {
        output.push_str(&format!(
            " {:width$} │",
            truncate_string(header, col_widths[i]),
            width = col_widths[i]
        ));
    }
    output.push('\n');

    // Print separator
    output.push('├');
    for (i, &width) in col_widths.iter().enumerate() {
        output.push_str(&"─".repeat(width + 2));
        if i < col_widths.len() - 1 {
            output.push('┼');
        }
    }
    output.push_str("┤\n");

    // Print data rows
    for record in records.iter() {
        output.push('│');
        for (i, field) in record.iter().enumerate() {
            if i < col_widths.len() {
                output.push_str(&format!(
                    " {:width$} │",
                    truncate_string(field, col_widths[i]),
                    width = col_widths[i]
                ));
            }
        }
        output.push('\n');
    }

    // Summary
    output.push('\n');
    output.push_str(&format!(
        "📈 Summary: {} columns, {} rows shown",
        col_count,
        records.len()
    ));

    if let Some(max) = max_rows {
        if records.len() >= max {
            output.push_str(&format!(" (limited to {} rows)", max));
        }
    }
    output.push('\n');

    Ok(output)
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}
