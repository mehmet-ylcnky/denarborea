use denarborea::{FileViewer, ViewerFormat, ViewerStrategy};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

struct LargeFileTest {
    temp_dir: TempDir,
}

impl LargeFileTest {
    fn new() -> Self {
        Self {
            temp_dir: TempDir::new().unwrap(),
        }
    }

    fn create_huge_json(&self, size_gb: f32) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("huge_{}_gb.json", size_gb));
        let mut file = File::create(&path).unwrap();

        writeln!(file, "[").unwrap();
        let items_per_gb = 150_000; // Approximate items per GB
        let total_items = (size_gb * items_per_gb as f32) as usize;

        println!(
            "Creating {}GB JSON file with {} items...",
            size_gb, total_items
        );

        for i in 0..total_items {
            if i % 10000 == 0 {
                println!(
                    "Progress: {}/{} items ({:.1}%)",
                    i,
                    total_items,
                    (i as f32 / total_items as f32) * 100.0
                );
            }

            let item = format!(
                r#"  {{
    "id": {},
    "name": "User_{}",
    "email": "user{}@example.com",
    "address": {{
      "street": "{} Main St",
      "city": "City_{}",
      "zip": "{:05}"
    }},
    "data": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    "metadata": {{
      "created": "2024-01-{:02}T10:30:00Z",
      "tags": ["tag_{}", "category_{}", "type_{}"],
      "score": {:.2}
    }}
  }}{}"#,
                i,
                i,
                i,
                i,
                i % 1000,
                i % 100000,
                (i % 28) + 1,
                i % 100,
                i % 50,
                i % 10,
                (i as f32 * 0.123) % 100.0,
                if i < total_items - 1 { "," } else { "" }
            );
            writeln!(file, "{}", item).unwrap();
        }
        writeln!(file, "]").unwrap();

        println!(
            "Created file: {} bytes",
            std::fs::metadata(&path).unwrap().len()
        );
        path
    }

    fn create_huge_csv(&self, size_gb: f32) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("huge_{}_gb.csv", size_gb));
        let mut file = File::create(&path).unwrap();

        writeln!(file, "id,name,email,address,city,zip,data,score,created").unwrap();
        let rows_per_gb = 2_000_000; // Approximate rows per GB
        let total_rows = (size_gb * rows_per_gb as f32) as usize;

        println!(
            "Creating {}GB CSV file with {} rows...",
            size_gb, total_rows
        );

        for i in 0..total_rows {
            if i % 100000 == 0 {
                println!(
                    "Progress: {}/{} rows ({:.1}%)",
                    i,
                    total_rows,
                    (i as f32 / total_rows as f32) * 100.0
                );
            }

            writeln!(
                file,
                "{},User_{},user{}@example.com,\"{} Main St\",City_{},{:05},\"Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua\",{:.2},2024-01-{:02}T10:30:00Z",
                i, i, i, i, i % 1000, i % 100000, (i as f32 * 0.123) % 100.0, (i % 28) + 1
            ).unwrap();
        }

        println!(
            "Created file: {} bytes",
            std::fs::metadata(&path).unwrap().len()
        );
        path
    }

    fn get_memory_usage() -> Option<u64> {
        // Get current process memory usage in KB
        let output = Command::new("ps")
            .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
            .output()
            .ok()?;

        let memory_str = String::from_utf8(output.stdout).ok()?;
        memory_str.trim().parse::<u64>().ok()
    }
}

#[test]
fn test_huge_json_memory_mapped() {
    let test = LargeFileTest::new();

    // Create 1GB JSON file
    let json_path = test.create_huge_json(1.0);

    let memory_before = LargeFileTest::get_memory_usage().unwrap_or(0);
    println!("Memory before: {} KB", memory_before);

    let viewer = FileViewer::new(ViewerFormat::Json)
        .with_strategy(ViewerStrategy::MemoryMapped)
        .with_limits(Some(5), None); // Only show first 5 items

    let start = std::time::Instant::now();
    let result = viewer.view_file(&json_path).unwrap();
    let duration = start.elapsed();

    let memory_after = LargeFileTest::get_memory_usage().unwrap_or(0);
    println!("Memory after: {} KB", memory_after);
    println!(
        "Memory increase: {} KB",
        memory_after.saturating_sub(memory_before)
    );
    println!("Processing time: {:?}", duration);
    println!("Output length: {} chars", result.len());

    // Verify we got valid JSON output
    assert!(result.contains("📋 JSON File"));
    assert!(result.contains("Memory-Mapped"));

    // Memory increase should be minimal (< 50MB for 1GB file)
    let memory_increase = memory_after.saturating_sub(memory_before);
    assert!(
        memory_increase < 50_000,
        "Memory usage too high: {} KB",
        memory_increase
    );

    // Should be fast (< 1 second for preview)
    assert!(duration.as_secs() < 1, "Too slow: {:?}", duration);
}

#[test]
fn test_huge_csv_memory_mapped() {
    let test = LargeFileTest::new();

    // Create 1GB CSV file
    let csv_path = test.create_huge_csv(1.0);

    let memory_before = LargeFileTest::get_memory_usage().unwrap_or(0);
    println!("Memory before: {} KB", memory_before);

    let viewer = FileViewer::new(ViewerFormat::Csv)
        .with_strategy(ViewerStrategy::MemoryMapped)
        .with_limits(Some(10), None); // Only show first 10 rows

    let start = std::time::Instant::now();
    let result = viewer.view_file(&csv_path).unwrap();
    let duration = start.elapsed();

    let memory_after = LargeFileTest::get_memory_usage().unwrap_or(0);
    println!("Memory after: {} KB", memory_after);
    println!(
        "Memory increase: {} KB",
        memory_after.saturating_sub(memory_before)
    );
    println!("Processing time: {:?}", duration);
    println!("Output length: {} chars", result.len());

    // Verify we got valid CSV output
    assert!(result.contains("📊 CSV File"));
    assert!(result.contains("Memory-Mapped"));
    assert!(result.contains("Headers:"));

    // Memory increase should be minimal
    let memory_increase = memory_after.saturating_sub(memory_before);
    assert!(
        memory_increase < 50_000,
        "Memory usage too high: {} KB",
        memory_increase
    );

    // Should be fast
    assert!(duration.as_secs() < 1, "Too slow: {:?}", duration);
}

#[test]
fn test_streaming_vs_memory_mapped_performance() {
    let test = LargeFileTest::new();

    // Create 500MB JSON file
    let json_path = test.create_huge_json(0.5);

    // Test streaming
    let start = std::time::Instant::now();
    let viewer_streaming = FileViewer::new(ViewerFormat::Json)
        .with_strategy(ViewerStrategy::Streaming)
        .with_limits(Some(10), None);
    let result_streaming = viewer_streaming.view_file(&json_path).unwrap();
    let streaming_duration = start.elapsed();

    // Test memory-mapped
    let start = std::time::Instant::now();
    let viewer_mmap = FileViewer::new(ViewerFormat::Json)
        .with_strategy(ViewerStrategy::MemoryMapped)
        .with_limits(Some(10), None);
    let result_mmap = viewer_mmap.view_file(&json_path).unwrap();
    let mmap_duration = start.elapsed();

    println!("Streaming time: {:?}", streaming_duration);
    println!("Memory-mapped time: {:?}", mmap_duration);
    println!("Streaming output length: {}", result_streaming.len());
    println!("Memory-mapped output length: {}", result_mmap.len());

    // Both should produce valid output
    assert!(result_streaming.contains("JSON File"));
    assert!(result_mmap.contains("JSON File"));

    // Both should be reasonably fast (< 15 seconds for CI environment)
    assert!(
        streaming_duration.as_secs() < 15,
        "Streaming too slow: {:?}",
        streaming_duration
    );
    assert!(
        mmap_duration.as_secs() < 15,
        "Memory-mapped too slow: {:?}",
        mmap_duration
    );
}

#[test]
fn test_auto_strategy_selection() {
    let test = LargeFileTest::new();

    // Create files of different sizes
    let small_json = test.create_huge_json(0.001); // ~1MB
    let large_json = test.create_huge_json(0.1); // ~100MB

    // Test auto strategy selection
    let viewer = FileViewer::new(ViewerFormat::Json).with_limits(Some(3), None);

    // Small file should use full load or streaming
    let start = std::time::Instant::now();
    let result_small = viewer.view_file(&small_json).unwrap();
    let small_duration = start.elapsed();

    // Large file should use memory-mapped
    let start = std::time::Instant::now();
    let result_large = viewer.view_file(&large_json).unwrap();
    let large_duration = start.elapsed();

    println!("Small file time: {:?}", small_duration);
    println!("Large file time: {:?}", large_duration);

    // Both should work
    assert!(result_small.contains("JSON File"));
    assert!(result_large.contains("JSON File"));

    // Large file processing should not be excessively slower in CI environment
    // (memory-mapped should have reasonable performance)
    let ratio = large_duration.as_millis() as f64 / small_duration.as_millis() as f64;
    assert!(
        ratio < 500.0,
        "Large file processing too slow compared to small file: {}x",
        ratio
    );
}
