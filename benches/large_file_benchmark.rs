use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use denarborea::{FileViewer, ViewerFormat, ViewerStrategy};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

struct BenchmarkSetup {
    temp_dir: TempDir,
}

impl BenchmarkSetup {
    fn new() -> Self {
        Self {
            temp_dir: TempDir::new().unwrap(),
        }
    }

    fn create_large_json(&self, size_mb: usize) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("large_{}_mb.json", size_mb));
        let mut file = File::create(&path).unwrap();

        writeln!(file, "[").unwrap();
        let items_per_mb = 150; // Approximate items per MB
        let total_items = size_mb * items_per_mb;

        for i in 0..total_items {
            let item = format!(
                r#"  {{
    "id": {},
    "name": "User_{}",
    "email": "user{}@example.com",
    "data": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
  }}{}"#,
                i,
                i,
                i,
                if i < total_items - 1 { "," } else { "" }
            );
            writeln!(file, "{}", item).unwrap();
        }
        writeln!(file, "]").unwrap();
        path
    }

    fn create_large_csv(&self, size_mb: usize) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("large_{}_mb.csv", size_mb));
        let mut file = File::create(&path).unwrap();

        writeln!(file, "id,name,email,data").unwrap();
        let rows_per_mb = 2000; // Approximate rows per MB
        let total_rows = size_mb * rows_per_mb;

        for i in 0..total_rows {
            writeln!(
                file,
                "{},User_{},user{}@example.com,\"Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua\"",
                i, i, i
            ).unwrap();
        }
        path
    }

    fn create_large_text(&self, size_mb: usize) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("large_{}_mb.txt", size_mb));
        let mut file = File::create(&path).unwrap();

        let lines_per_mb = 15000; // Approximate lines per MB
        let total_lines = size_mb * lines_per_mb;

        for i in 0..total_lines {
            writeln!(
                file,
                "Line {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                i
            ).unwrap();
        }
        path
    }

    fn create_large_yaml(&self, size_mb: usize) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("large_{}_mb.yaml", size_mb));
        let mut file = File::create(&path).unwrap();

        writeln!(file, "users:").unwrap();
        let items_per_mb = 200; // Approximate items per MB for YAML
        let total_items = size_mb * items_per_mb;

        for i in 0..total_items {
            writeln!(file, "  - id: {}", i).unwrap();
            writeln!(file, "    name: User_{}", i).unwrap();
            writeln!(file, "    email: user{}@example.com", i).unwrap();
            writeln!(
                file,
                "    data: Lorem ipsum dolor sit amet consectetur adipiscing elit"
            )
            .unwrap();
        }
        path
    }

    fn create_large_toml(&self, size_mb: usize) -> std::path::PathBuf {
        let path = self
            .temp_dir
            .path()
            .join(format!("large_{}_mb.toml", size_mb));
        let mut file = File::create(&path).unwrap();

        let sections_per_mb = 100; // Approximate sections per MB for TOML
        let total_sections = size_mb * sections_per_mb;

        for i in 0..total_sections {
            writeln!(file, "[user_{}]", i).unwrap();
            writeln!(file, "id = {}", i).unwrap();
            writeln!(file, "name = \"User_{}\"", i).unwrap();
            writeln!(file, "email = \"user{}@example.com\"", i).unwrap();
            writeln!(
                file,
                "data = \"Lorem ipsum dolor sit amet consectetur adipiscing elit\""
            )
            .unwrap();
            writeln!(file).unwrap();
        }
        path
    }
}

fn benchmark_file_viewing(c: &mut Criterion) {
    let setup = BenchmarkSetup::new();

    // Test different file sizes (in MB)
    let sizes = vec![1, 10, 50, 100, 500];

    let mut group = c.benchmark_group("large_file_viewing");
    group.sample_size(10); // Reduce sample size for large files

    for size in sizes {
        // JSON benchmarks
        let json_path = setup.create_large_json(size);

        group.bench_with_input(BenchmarkId::new("json_auto", size), &size, |b, _| {
            let viewer = FileViewer::new(ViewerFormat::Json);
            b.iter(|| {
                black_box(viewer.view_file(&json_path).unwrap());
            });
        });

        group.bench_with_input(
            BenchmarkId::new("json_memory_mapped", size),
            &size,
            |b, _| {
                let viewer =
                    FileViewer::new(ViewerFormat::Json).with_strategy(ViewerStrategy::MemoryMapped);
                b.iter(|| {
                    black_box(viewer.view_file(&json_path).unwrap());
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("json_streaming", size), &size, |b, _| {
            let viewer =
                FileViewer::new(ViewerFormat::Json).with_strategy(ViewerStrategy::Streaming);
            b.iter(|| {
                black_box(viewer.view_file(&json_path).unwrap());
            });
        });

        // CSV benchmarks
        let csv_path = setup.create_large_csv(size);

        group.bench_with_input(BenchmarkId::new("csv_auto", size), &size, |b, _| {
            let viewer = FileViewer::new(ViewerFormat::Csv);
            b.iter(|| {
                black_box(viewer.view_file(&csv_path).unwrap());
            });
        });

        group.bench_with_input(
            BenchmarkId::new("csv_memory_mapped", size),
            &size,
            |b, _| {
                let viewer =
                    FileViewer::new(ViewerFormat::Csv).with_strategy(ViewerStrategy::MemoryMapped);
                b.iter(|| {
                    black_box(viewer.view_file(&csv_path).unwrap());
                });
            },
        );

        // Text benchmarks
        let text_path = setup.create_large_text(size);

        group.bench_with_input(BenchmarkId::new("text_auto", size), &size, |b, _| {
            let viewer = FileViewer::new(ViewerFormat::Text);
            b.iter(|| {
                black_box(viewer.view_file(&text_path).unwrap());
            });
        });

        group.bench_with_input(
            BenchmarkId::new("text_memory_mapped", size),
            &size,
            |b, _| {
                let viewer =
                    FileViewer::new(ViewerFormat::Text).with_strategy(ViewerStrategy::MemoryMapped);
                b.iter(|| {
                    black_box(viewer.view_file(&text_path).unwrap());
                });
            },
        );
    }

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let setup = BenchmarkSetup::new();

    // Create very large files for memory testing
    let large_sizes = vec![100, 500, 1000]; // MB

    let mut group = c.benchmark_group("memory_efficiency");
    group.sample_size(5); // Even fewer samples for very large files

    for size in large_sizes {
        let json_path = setup.create_large_json(size);

        // Test memory-mapped vs full load for very large files
        group.bench_with_input(
            BenchmarkId::new("large_json_memory_mapped", size),
            &size,
            |b, _| {
                let viewer = FileViewer::new(ViewerFormat::Json)
                    .with_strategy(ViewerStrategy::MemoryMapped)
                    .with_limits(Some(10), None); // Only show first 10 items
                b.iter(|| {
                    black_box(viewer.view_file(&json_path).unwrap());
                });
            },
        );

        // Only test full load for smaller files to avoid OOM
        if size <= 100 {
            group.bench_with_input(
                BenchmarkId::new("large_json_full_load", size),
                &size,
                |b, _| {
                    let viewer = FileViewer::new(ViewerFormat::Json)
                        .with_strategy(ViewerStrategy::FullLoad)
                        .with_limits(Some(10), None);
                    b.iter(|| {
                        black_box(viewer.view_file(&json_path).unwrap());
                    });
                },
            );
        }
    }

    group.finish();
}

fn benchmark_startup_time(c: &mut Criterion) {
    let setup = BenchmarkSetup::new();

    // Test startup time for different file sizes
    let sizes = vec![10, 100, 500, 1000];

    let mut group = c.benchmark_group("startup_time");
    group.sample_size(10);

    for size in sizes {
        let json_path = setup.create_large_json(size);

        group.bench_with_input(
            BenchmarkId::new("startup_memory_mapped", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let viewer = FileViewer::new(ViewerFormat::Json)
                        .with_strategy(ViewerStrategy::MemoryMapped)
                        .with_limits(Some(1), None); // Just first item
                    black_box(viewer.view_file(&json_path).unwrap());
                });
            },
        );

        if size <= 100 {
            group.bench_with_input(
                BenchmarkId::new("startup_full_load", size),
                &size,
                |b, _| {
                    b.iter(|| {
                        let viewer = FileViewer::new(ViewerFormat::Json)
                            .with_strategy(ViewerStrategy::FullLoad)
                            .with_limits(Some(1), None);
                        black_box(viewer.view_file(&json_path).unwrap());
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_file_viewing,
    benchmark_memory_usage,
    benchmark_startup_time
);
criterion_main!(benches);
