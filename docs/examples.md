# Examples

The `docs/example_files/` directory contains sample files to test the file viewer functionality:

- **config.json** - JSON configuration with nested objects and arrays
- **data.csv** - Employee data with multiple columns
- **docker-compose.yml** - YAML service configuration
- **Cargo.toml** - TOML project configuration
- **README.txt** - Plain text documentation
- **sample.bin** - Binary file with hex dump display
- **employees.parquet** - Parquet file with schema and metadata

## Try the File Viewer

```bash
# View all example files
denarborea docs/example_files/

# Test different formats
denarborea --view docs/example_files/config.json
denarborea --view docs/example_files/data.csv  
denarborea --view docs/example_files/docker-compose.yml
denarborea --view docs/example_files/Cargo.toml
denarborea --view docs/example_files/README.txt
denarborea --view docs/example_files/sample.bin
denarborea --view docs/example_files/employees.parquet
```

## Quick Test Commands

```bash
# JSON with pretty formatting
denarborea --view docs/example_files/config.json

# CSV as formatted table
denarborea --view docs/example_files/data.csv

# Binary file as hex dump
denarborea --view docs/example_files/sample.bin

# Parquet with schema and data
denarborea --view docs/example_files/employees.parquet
```
