use crate::Result;
use arrow_array::Array;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

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

    // Read and display actual data
    match read_parquet_data(path, max_rows) {
        Ok(data_output) => {
            output.push_str(&data_output);
        }
        Err(e) => {
            output.push_str(&format!("Error reading data: {}\n", e));
            
            // Fallback to metadata-only view
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
        }
    }

    Ok(output)
}

fn read_parquet_data(path: &Path, max_rows: Option<usize>) -> Result<String> {
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;
    
    let mut output = String::new();
    let mut total_rows_shown = 0;
    let max_display_rows = max_rows.unwrap_or(10);
    
    // Read record batches
    let mut batches = Vec::new();
    for batch_result in reader {
        let batch = batch_result?;
        batches.push(batch);
        
        if total_rows_shown >= max_display_rows {
            break;
        }
    }
    
    if batches.is_empty() {
        output.push_str("No data found in parquet file.\n");
        return Ok(output);
    }
    
    // Get schema from first batch
    let schema = batches[0].schema();
    let column_names: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();
    
    output.push_str("Data:\n");
    output.push_str("─".repeat(80).as_str());
    output.push('\n');
    
    // Calculate column widths
    let mut col_widths: Vec<usize> = column_names.iter().map(|name| name.len().max(10)).collect();
    
    // Sample some data to determine better column widths
    for batch in &batches {
        for (col_idx, column) in batch.columns().iter().enumerate() {
            let sample_size = (batch.num_rows()).min(5);
            for row_idx in 0..sample_size {
                let value_str = format_array_value(column, row_idx);
                if col_idx < col_widths.len() {
                    col_widths[col_idx] = col_widths[col_idx].max(value_str.len().min(20));
                }
            }
        }
    }
    
    // Print headers
    output.push('│');
    for (i, name) in column_names.iter().enumerate() {
        output.push_str(&format!(
            " {:width$} │",
            truncate_string(name, col_widths[i]),
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
    for batch in &batches {
        for row_idx in 0..batch.num_rows() {
            if total_rows_shown >= max_display_rows {
                break;
            }
            
            output.push('│');
            for (col_idx, column) in batch.columns().iter().enumerate() {
                let value_str = format_array_value(column, row_idx);
                output.push_str(&format!(
                    " {:width$} │",
                    truncate_string(&value_str, col_widths[col_idx]),
                    width = col_widths[col_idx]
                ));
            }
            output.push('\n');
            total_rows_shown += 1;
        }
        
        if total_rows_shown >= max_display_rows {
            break;
        }
    }
    
    output.push('\n');
    output.push_str(&format!(
        "📈 Summary: {} columns, {} rows shown",
        column_names.len(),
        total_rows_shown
    ));
    
    if let Some(max) = max_rows {
        if total_rows_shown >= max {
            output.push_str(&format!(" (limited to {} rows)", max));
        }
    }
    output.push('\n');
    
    Ok(output)
}

fn format_array_value(array: &Arc<dyn Array>, row_idx: usize) -> String {
    use arrow_array::*;
    
    if array.is_null(row_idx) {
        return "null".to_string();
    }
    
    match array.data_type() {
        arrow::datatypes::DataType::Boolean => {
            let array = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Int8 => {
            let array = array.as_any().downcast_ref::<Int8Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Int16 => {
            let array = array.as_any().downcast_ref::<Int16Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Int32 => {
            let array = array.as_any().downcast_ref::<Int32Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Int64 => {
            let array = array.as_any().downcast_ref::<Int64Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::UInt8 => {
            let array = array.as_any().downcast_ref::<UInt8Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::UInt16 => {
            let array = array.as_any().downcast_ref::<UInt16Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::UInt32 => {
            let array = array.as_any().downcast_ref::<UInt32Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::UInt64 => {
            let array = array.as_any().downcast_ref::<UInt64Array>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Float32 => {
            let array = array.as_any().downcast_ref::<Float32Array>().unwrap();
            format!("{:.2}", array.value(row_idx))
        }
        arrow::datatypes::DataType::Float64 => {
            let array = array.as_any().downcast_ref::<Float64Array>().unwrap();
            format!("{:.2}", array.value(row_idx))
        }
        arrow::datatypes::DataType::Utf8 => {
            let array = array.as_any().downcast_ref::<StringArray>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::LargeUtf8 => {
            let array = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
            array.value(row_idx).to_string()
        }
        arrow::datatypes::DataType::Binary => {
            let array = array.as_any().downcast_ref::<BinaryArray>().unwrap();
            format!("<binary {} bytes>", array.value(row_idx).len())
        }
        arrow::datatypes::DataType::Date32 => {
            let array = array.as_any().downcast_ref::<Date32Array>().unwrap();
            format!("date({})", array.value(row_idx))
        }
        arrow::datatypes::DataType::Date64 => {
            let array = array.as_any().downcast_ref::<Date64Array>().unwrap();
            format!("date({})", array.value(row_idx))
        }
        _ => format!("<{:?}>", array.data_type()),
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
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
