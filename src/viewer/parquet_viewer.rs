use crate::Result;
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;

pub fn view_parquet_file(path: &Path, max_rows: Option<usize>) -> Result<String> {
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let metadata = reader.metadata();

    let mut output = String::new();
    output.push_str(&format!("📊 Parquet File: {}\n", path.display()));
    output.push_str("─".repeat(80).as_str());
    output.push('\n');

    // File metadata
    output.push_str(&format!(
        "Version: {}\n",
        metadata.file_metadata().version()
    ));
    output.push_str(&format!(
        "Created by: {}\n",
        metadata.file_metadata().created_by().unwrap_or("Unknown")
    ));
    output.push_str(&format!(
        "Number of rows: {}\n",
        metadata.file_metadata().num_rows()
    ));
    output.push_str(&format!(
        "Number of row groups: {}\n",
        metadata.num_row_groups()
    ));
    output.push('\n');

    // Schema information
    output.push_str("Schema:\n");
    let schema = metadata.file_metadata().schema_descr();
    for (i, column) in schema.columns().iter().enumerate() {
        output.push_str(&format!(
            "  {}: {} ({})\n",
            i + 1,
            column.name(),
            format_parquet_type(column.physical_type(), column.logical_type())
        ));
    }
    output.push('\n');

    // Row group statistics
    output.push_str("Row Groups:\n");
    for i in 0..metadata.num_row_groups() {
        let row_group = metadata.row_group(i);
        output.push_str(&format!(
            "  Group {}: {} rows, {} bytes\n",
            i + 1,
            row_group.num_rows(),
            row_group.total_byte_size()
        ));
    }
    output.push('\n');

    // Column statistics
    output.push_str("Column Statistics:\n");
    if let Some(row_group) = metadata.row_groups().first() {
        for (i, column_chunk) in row_group.columns().iter().enumerate() {
            let column_descriptor = schema.column(i);
            let column_name = column_descriptor.name();
            output.push_str(&format!("  {}: ", column_name));

            if let Some(stats) = column_chunk.statistics() {
                output.push_str(&format!(
                    "null_count={:?}, distinct_count={:?}",
                    stats.null_count_opt(),
                    stats.distinct_count_opt()
                ));

                // Add min/max if available
                if stats.min_bytes_opt().is_some() && stats.max_bytes_opt().is_some() {
                    output.push_str(", min/max available");
                }
            } else {
                output.push_str("no statistics");
            }
            output.push('\n');
        }
    }

    // Sample data (if requested)
    if max_rows.unwrap_or(0) > 0 {
        output.push('\n');
        output.push_str("Sample Data:\n");
        output.push_str("(Note: Full data reading requires Arrow integration)\n");

        // For now, just show that we detected the structure
        // Full implementation would require Arrow RecordBatch reading
        let sample_rows = max_rows.unwrap_or(5).min(10);
        output.push_str(&format!("Would show first {} rows here\n", sample_rows));
    }

    Ok(output)
}

fn format_parquet_type(
    physical_type: parquet::basic::Type,
    logical_type: Option<parquet::basic::LogicalType>,
) -> String {
    let base_type = match physical_type {
        parquet::basic::Type::BOOLEAN => "boolean",
        parquet::basic::Type::INT32 => "int32",
        parquet::basic::Type::INT64 => "int64",
        parquet::basic::Type::INT96 => "int96",
        parquet::basic::Type::FLOAT => "float",
        parquet::basic::Type::DOUBLE => "double",
        parquet::basic::Type::BYTE_ARRAY => "byte_array",
        parquet::basic::Type::FIXED_LEN_BYTE_ARRAY => "fixed_len_byte_array",
    };

    if let Some(logical) = logical_type {
        format!("{} ({})", base_type, format_logical_type(logical))
    } else {
        base_type.to_string()
    }
}

fn format_logical_type(logical_type: parquet::basic::LogicalType) -> String {
    match logical_type {
        parquet::basic::LogicalType::String => "string".to_string(),
        parquet::basic::LogicalType::Map => "map".to_string(),
        parquet::basic::LogicalType::List => "list".to_string(),
        parquet::basic::LogicalType::Enum => "enum".to_string(),
        parquet::basic::LogicalType::Decimal { precision, scale } => {
            format!("decimal({}, {})", precision, scale)
        }
        parquet::basic::LogicalType::Date => "date".to_string(),
        parquet::basic::LogicalType::Time { .. } => "time".to_string(),
        parquet::basic::LogicalType::Timestamp { .. } => "timestamp".to_string(),
        parquet::basic::LogicalType::Integer {
            bit_width,
            is_signed,
        } => {
            format!(
                "int{}_{}",
                bit_width,
                if is_signed { "signed" } else { "unsigned" }
            )
        }
        parquet::basic::LogicalType::Unknown => "unknown".to_string(),
        parquet::basic::LogicalType::Json => "json".to_string(),
        parquet::basic::LogicalType::Bson => "bson".to_string(),
        parquet::basic::LogicalType::Uuid => "uuid".to_string(),
        parquet::basic::LogicalType::Float16 => "float16".to_string(),
    }
}
