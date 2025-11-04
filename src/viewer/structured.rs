use crate::Result;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub fn view_json_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    let json: JsonValue = serde_json::from_str(&content)?;

    let mut output = String::new();
    output.push_str(&format!("📋 JSON File: {}\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    // Pretty print JSON
    let pretty = serde_json::to_string_pretty(&json)?;
    output.push_str(&pretty);
    output.push('\n');

    // Add summary
    output.push('\n');
    output.push_str(&format!("📊 Summary: {}", analyze_json(&json)));

    Ok(output)
}

pub fn view_yaml_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;

    let mut output = String::new();
    output.push_str(&format!("📄 YAML File: {}\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    // Try to parse and reformat
    match serde_yaml::from_str::<serde_yaml::Value>(&content) {
        Ok(yaml) => {
            let formatted = serde_yaml::to_string(&yaml)?;
            output.push_str(&formatted);

            // Convert to JSON for analysis
            if let Ok(json) = serde_json::to_value(&yaml) {
                output.push_str(&format!("\n📊 Summary: {}", analyze_json(&json)));
            }
        }
        Err(e) => {
            output.push_str("⚠️  Invalid YAML format\n");
            output.push_str(&format!("Error: {}\n\n", e));
            output.push_str("Raw content:\n");
            output.push_str(&content);
        }
    }

    Ok(output)
}

pub fn view_toml_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;

    let mut output = String::new();
    output.push_str(&format!("⚙️  TOML File: {}\n", path.display()));
    output.push_str("─".repeat(60).as_str());
    output.push('\n');

    // Try to parse and reformat
    match toml::from_str::<toml::Value>(&content) {
        Ok(toml_val) => {
            let formatted = toml::to_string_pretty(&toml_val)?;
            output.push_str(&formatted);

            // Convert to JSON for analysis
            if let Ok(json) = serde_json::to_value(&toml_val) {
                output.push_str(&format!("\n📊 Summary: {}", analyze_json(&json)));
            }
        }
        Err(e) => {
            output.push_str("⚠️  Invalid TOML format\n");
            output.push_str(&format!("Error: {}\n\n", e));
            output.push_str("Raw content:\n");
            output.push_str(&content);
        }
    }

    Ok(output)
}

fn analyze_json(value: &JsonValue) -> String {
    match value {
        JsonValue::Object(map) => {
            format!("Object with {} keys", map.len())
        }
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                "Empty array".to_string()
            } else {
                let first_type = match &arr[0] {
                    JsonValue::Object(_) => "objects",
                    JsonValue::Array(_) => "arrays",
                    JsonValue::String(_) => "strings",
                    JsonValue::Number(_) => "numbers",
                    JsonValue::Bool(_) => "booleans",
                    JsonValue::Null => "nulls",
                };
                format!("Array of {} items ({})", arr.len(), first_type)
            }
        }
        JsonValue::String(s) => {
            format!("String ({} characters)", s.len())
        }
        JsonValue::Number(n) => {
            format!("Number ({})", n)
        }
        JsonValue::Bool(b) => {
            format!("Boolean ({})", b)
        }
        JsonValue::Null => "Null value".to_string(),
    }
}
