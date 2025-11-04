use crate::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn view_text_file(path: &Path, max_lines: Option<usize>) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    // Try syntax highlighting
    if let Ok(highlighted) = highlight_file(path, max_lines) {
        return Ok(highlighted);
    }

    // Fallback to plain text
    output.push_str(&format!("📄 Text File: {}\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    for (line_num, line) in reader.lines().enumerate() {
        if let Some(max) = max_lines {
            if line_num >= max {
                output.push_str(&format!("\n... (showing first {} lines)\n", max));
                break;
            }
        }

        match line {
            Ok(content) => {
                output.push_str(&format!("{:4} │ {}\n", line_num + 1, content));
            }
            Err(_) => {
                output.push_str(&format!("{:4} │ <invalid UTF-8>\n", line_num + 1));
            }
        }
    }

    Ok(output)
}

fn highlight_file(path: &Path, max_lines: Option<usize>) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_for_file(path)?
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let content = std::fs::read_to_string(path)?;

    let mut output = String::new();
    output.push_str(&format!(
        "📄 {} ({})\n",
        path.file_name().unwrap_or_default().to_string_lossy(),
        syntax.name
    ));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    for (line_num, line) in LinesWithEndings::from(&content).enumerate() {
        if let Some(max) = max_lines {
            if line_num >= max {
                output.push_str(&format!("\n... (showing first {} lines)\n", max));
                break;
            }
        }

        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&format!("{:4} │ {}", line_num + 1, escaped));
    }

    Ok(output)
}
