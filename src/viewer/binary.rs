use crate::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn view_binary_file(path: &Path, max_bytes: Option<usize>) -> Result<String> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    let read_size = max_bytes.unwrap_or(1024).min(file_size as usize);
    let mut buffer = vec![0u8; read_size];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    let mut output = String::new();
    output.push_str(&format!(
        "🔢 Binary File: {} ({} bytes)\n",
        path.display(),
        file_size
    ));
    output.push_str("─".repeat(80).as_str());
    output.push('\n');

    // File signature detection
    if let Some(file_type) = detect_file_type(&buffer) {
        output.push_str(&format!("File Type: {}\n", file_type));
        output.push('\n');
    }

    // Hex dump
    output.push_str("Hex Dump:\n");
    for (i, chunk) in buffer.chunks(16).enumerate() {
        let offset = i * 16;

        // Offset
        output.push_str(&format!("{:08x}  ", offset));

        // Hex bytes
        for (j, &byte) in chunk.iter().enumerate() {
            if j == 8 {
                output.push(' '); // Extra space in the middle
            }
            output.push_str(&format!("{:02x} ", byte));
        }

        // Padding for incomplete lines
        let padding = (16 - chunk.len()) * 3;
        if chunk.len() <= 8 {
            output.push(' '); // Account for middle space
        }
        output.push_str(&" ".repeat(padding));

        // ASCII representation
        output.push_str(" |");
        for &byte in chunk {
            if byte.is_ascii_graphic() || byte == b' ' {
                output.push(byte as char);
            } else {
                output.push('.');
            }
        }
        output.push_str("|\n");
    }

    if bytes_read < file_size as usize {
        output.push_str(&format!(
            "\n... (showing first {} bytes of {})\n",
            bytes_read, file_size
        ));
    }

    Ok(output)
}

fn detect_file_type(buffer: &[u8]) -> Option<&'static str> {
    if buffer.len() < 4 {
        return None;
    }

    match &buffer[0..4] {
        [0x89, 0x50, 0x4E, 0x47] => Some("PNG Image"),
        [0xFF, 0xD8, 0xFF, _] => Some("JPEG Image"),
        [0x47, 0x49, 0x46, 0x38] => Some("GIF Image"),
        [0x25, 0x50, 0x44, 0x46] => Some("PDF Document"),
        [0x50, 0x4B, 0x03, 0x04] => Some("ZIP Archive"),
        [0x50, 0x4B, 0x05, 0x06] => Some("ZIP Archive (empty)"),
        [0x50, 0x4B, 0x07, 0x08] => Some("ZIP Archive (spanned)"),
        [0x1F, 0x8B, 0x08, _] => Some("GZIP Archive"),
        [0x42, 0x5A, 0x68, _] => Some("BZIP2 Archive"),
        [0x7F, 0x45, 0x4C, 0x46] => Some("ELF Executable"),
        [0x4D, 0x5A, _, _] => Some("Windows Executable"),
        _ => {
            // Check for text-based formats
            if buffer
                .iter()
                .all(|&b| b.is_ascii() || b == b'\n' || b == b'\r' || b == b'\t')
            {
                Some("Text File")
            } else {
                Some("Unknown Binary")
            }
        }
    }
}
