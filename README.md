<div align="center">
<img src="denarborea.png" alt="DenArborea Logo" width="300">
</div>

<div align="center">

[![CI](https://github.com/mehmet-ylcnky/denarborea/actions/workflows/ci.yml/badge.svg)](https://github.com/mehmet-ylcnky/denarborea/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/mehmet-ylcnky/denarborea/branch/main/graph/badge.svg)](https://codecov.io/gh/mehmet-ylcnky/denarborea)
[![Tests](https://img.shields.io/badge/tests-124%20passing-brightgreen)](https://github.com/mehmet-ylcnky/denarborea)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Clippy](https://img.shields.io/badge/clippy-passing-brightgreen)](https://github.com/mehmet-ylcnky/denarborea)

</div>

A directory visualizer written in Rust with advanced filtering, Git integration, and multiple output formats.

## Inspiration

While watching my daughter, Dena, exploring the plants near our home, I noticed how she'd stop at each one, tracing the branches with her eyes from trunk to the smallest twig. "Look, Babash," she said, pointing at one leaf, "she knows where it belongs."

Her observation reminded me of the Unix `tree` command that has served well for decades for developers who spend their daily life in `Terminal` (including myself). Directory structures are just like trees—each folder a branch, each file a leaf, all connected in a hierarchy that makes sense once you can see it clearly.

Than I found myself wanting more: *What if I could see which files changed in Git? What if I could filter by size or extension? What if I could export the structure as JSON for documentation?* That's when Dena's love for plants and my developer needs came together. **DenArborea** combines my daughter's name with *arborea* (Latin for "tree-like"), creating a tool that builds upon the classic `tree` command with modern features:
- **Git integration** to see modified files at a glance (`--git-status`)
- **Smart filtering** to find large files or specific extensions (`--min-size 100MB -e rs`)
- **Multiple output formats** for documentation and automation (`--format json`)
- **Rich metadata** like checksums, permissions, and statistics (`--checksum --stats`)

It transforms the abstract maze of nested directories into something as clear and natural as the trees she loves to climb—with the power tools developers need today.

## Development

### Code Quality Standards

This project follows strict code quality standards:

```bash
# Format code (required before commit)
make fmt

# Run linting checks
make clippy

# Run all tests
make test

# Run all checks (fmt + clippy + test)
make check
```

**Pre-commit hooks** automatically run `fmt` and `clippy` checks. All code must:
- Pass `cargo fmt --check`
- Pass `cargo clippy -- -D warnings`
- Have accompanying tests for new features

## Features

### 📊 Display Options
- **Tree View** - Classic tree structure with Unicode connectors
- **File Sizes** - Human-readable format (KB, MB, GB)
- **Permissions** - Unix-style permission display (rwxr-xr-x)
- **Timestamps** - Last modified time
- **File Counts** - Show file/directory counts for folders
- **MD5 Checksums** - Calculate and display file checksums
- **Symlinks** - Display symlink targets with arrows
- **Full Paths** - Show absolute paths instead of relative

### 🎨 Color Coding
- **Directories** - Blue
- **Executables** - Bold green
- **Symlinks** - Cyan
- **Rust files** - Red
- **Python files** - Yellow
- **JavaScript/TypeScript** - Bright yellow
- **HTML/CSS** - Magenta
- **Markdown** - Bright blue
- **Config files** (JSON, YAML, TOML) - Bright green
- **Images** (JPG, PNG, GIF, SVG) - Bright magenta
- **Archives** (ZIP, TAR, GZ) - Bold red

### 🔍 Filtering & Sorting
- **Extension Filter** - Show only specific file types
- **Size Filters** - Min/max file size limits
- **Pattern Matching** - Include/exclude with regex or glob patterns
- **Hidden Files** - Show/hide dotfiles
- **Directories Only** - Show only folders
- **Files Only** - Show only files
- **Sort Options** - By name, size, time, extension, or type
- **Reverse Sort** - Reverse any sort order
- **Limit Results** - Cap the number of displayed items

### 🔧 Git Integration
- **Git Status** - Show file status indicators (M, A, D, ??, etc.)
- **Status Colors** - Modified (yellow), Added (green), Deleted (red), Untracked (red)
- **Git Ignore** - Respect .gitignore rules automatically

### 📤 Output Formats
- **Tree** - Traditional tree view
- **JSON** - Structured hierarchical data
- **XML** - Markup format with metadata
- **CSV** - Tabular format with headers
- **Markdown** - Table format for documentation

### 📄 File Viewer
- **Multi-format Support** - View JSON, CSV, YAML, TOML, and text files
- **Syntax Highlighting** - Color-coded display for better readability
- **Structured Formatting** - Pretty-printed JSON, organized CSV tables
- **Binary File Detection** - Hex dump view for binary files with file type detection
- **Content Limits** - Configurable line and byte limits for large files
- **Custom Delimiters** - Support for different CSV separators (comma, semicolon, tab, pipe)
- **Format Override** - Force specific format interpretation
- **Error Handling** - Graceful handling of malformed files

### 📈 Statistics
- Total files and directories
- Total size with human-readable format
- Average file size
- Largest and smallest files
- File type distribution (top 10)
- Symlink count

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/denarborea.git
cd denarborea

# Build release version
cargo build --release

# Binary will be at target/release/denarborea
```

## Usage

### Basic Usage

```bash
# Show current directory
denarborea

# Show specific directory
denarborea /path/to/directory

# Limit depth to 2 levels
denarborea -L 2

# Show hidden files
denarborea -a
```

### Display Options

```bash
# Show file sizes
denarborea -s

# Show permissions
denarborea -p

# Show timestamps
denarborea -t

# Show file/directory counts
denarborea -c

# Show everything
denarborea -s -p -t -c

# Show statistics summary
denarborea --stats

# Show MD5 checksums
denarborea --checksum

# Show full absolute paths
denarborea --full-path
```

### Filtering

```bash
# Show only Rust files
denarborea -e rs

# Show multiple extensions
denarborea -e rs,toml,md

# Show only directories
denarborea -d

# Show only files
denarborea -f

# Files larger than 1MB
denarborea --min-size 1MB

# Files smaller than 100KB
denarborea --max-size 100KB

# Exclude patterns
denarborea --exclude "target" --exclude "node_modules"

# Include patterns
denarborea --include "src"

# Limit to 50 items
denarborea --limit 50
```

### Sorting

```bash
# Sort by name (default)
denarborea --sort name

# Sort by size
denarborea --sort size

# Sort by modification time
denarborea --sort time

# Sort by extension
denarborea --sort extension

# Sort by type (directories first)
denarborea --sort type

# Reverse sort order
denarborea --sort size -r
```

### Git Integration

```bash
# Show git status indicators
denarborea --git-status

# Respect .gitignore files (hides target/, .git/, etc.)
denarborea --git-ignore

# Both git features together
denarborea --git-status --git-ignore
```

**Git Status Indicators:**
- `M ` - Modified files
- `A ` - Added (staged new files)
- `D ` - Deleted files
- `??` - Untracked files
- `R ` - Renamed files
- `I ` - Ignored files

**Example outputs:**

```bash
# Show what files have changed in your repo
$ denarborea --git-status -L 1
.
|-- M Cargo.lock
|-- M Cargo.toml
|-- M README.md
|--   src
'-- I target

# Hide ignored files (cleaner output)
$ denarborea --git-ignore -L 1
.
|-- Cargo.lock
|-- Cargo.toml
|-- README.md
'-- src

# Show modified files with sizes
$ denarborea --git-status --git-ignore -s -L 2
.
|-- M Cargo.lock[39.36 kB]
|-- M Cargo.toml[528 B]
|-- M README.md[8.78 kB]
'--   src
   |-- M config.rs[3.89 kB]
   |--   display.rs[10.75 kB]
   |-- M main.rs[4.99 kB]

# Find only modified files
$ denarborea --git-status --git-ignore -L 2 | grep "M "
|-- M Cargo.lock
|-- M Cargo.toml
|-- M README.md
   |-- M config.rs
   |-- M main.rs

# Analyze modified files with statistics
$ denarborea src --git-status -s --stats
src
|-- M config.rs[3.89 kB]
|--   display.rs[10.75 kB]
|-- M main.rs[4.99 kB]
|--   tree.rs[18.64 kB]

 Tree Statistics:
Directories: 0
Files: 8
Total size: 50.58 kB
Average file size: 6.32 kB
Largest file: tree.rs (18.64 kB)
```

### Output Formats

```bash
# JSON output
denarborea --format json

# XML output
denarborea --format xml

# CSV output
denarborea --format csv

# Markdown table
denarborea --format markdown

# Save to file
denarborea -o output.txt

# JSON to file
denarborea --format json -o tree.json
```

### File Viewer

```bash
# View JSON file with pretty formatting
denarborea --view examples/config.json

# View CSV file as formatted table
denarborea --view examples/data.csv

# View YAML file with syntax highlighting
denarborea --view examples/docker-compose.yml

# View TOML configuration
denarborea --view examples/Cargo.toml

# View text file with line numbers
denarborea --view examples/README.txt

# View binary file as hex dump
denarborea --view examples/sample.bin

# View parquet file metadata and schema
denarborea --view examples/employees.parquet

# View binary file with explicit format
denarborea --view image.png --viewer-format binary

# Limit output to first 10 lines
denarborea --view large-file.txt --max-lines 10

# Limit output to first 1KB
denarborea --view large-file.txt --max-bytes 1024

# CSV with custom delimiter
denarborea --view data.tsv --delimiter $'\t'

# Force format interpretation
denarborea --view data.txt --viewer-format json
```

**Example Outputs:**

**JSON File:**
```
📋 JSON File: examples/config.json
────────────────────────────────────────────────────────────
{
  "features": {
    "file_viewer": true,
    "git_integration": true,
    "multiple_formats": [
      "json",
      "csv", 
      "yaml",
      "toml"
    ],
    "tree_visualization": true
  },
  "name": "DenArborea",
  "settings": {
    "color_output": true,
    "max_depth": 10,
    "show_hidden": false
  },
  "version": "0.1.0"
}

📊 Summary: Object with 4 keys
```

**CSV File:**
```
📊 CSV File: examples/data.csv
────────────────────────────────────────────────────────────────────────────────
│ name          │ age        │ city          │ occupation        │
├───────────────┼────────────┼───────────────┼───────────────────┤
│ Alice Johnson │ 28         │ New York      │ Software Engineer │
│ Bob Smith     │ 34         │ San Francisco │ Data Scientist    │
│ Carol Davis   │ 25         │ Chicago       │ Designer          │
│ David Wilson  │ 31         │ Seattle       │ Product Manager   │
│ Eve Brown     │ 29         │ Austin        │ DevOps Engineer   │

📈 Summary: 4 columns, 5 rows shown
```

**Text File:**
```
📄 README.txt (Plain Text)
────────────────────────────────────────────────────────────
   1 │ DenArborea Examples
   2 │ ==================
   3 │ 
   4 │ This directory contains example files to demonstrate the file viewer functionality.
   5 │ 
   6 │ Files included:
   7 │ - config.json: JSON configuration example
   8 │ - data.csv: CSV data with employee information
   9 │ - docker-compose.yml: YAML service configuration
  10 │ - Cargo.toml: TOML project configuration
```

**Binary File:**
```
🔢 Binary File: examples/sample.bin (42 bytes)
────────────────────────────────────────────────────────────────────────────────
File Type: Unknown Binary

Hex Dump:
00000000  50 4e 47 89 50 4e 47 0d  0a 1a 0a 00 00 00 0d 48  |PNG.PNG........H|
00000010  65 6c 6c 6f 00 57 6f 72  6c 64 00 42 69 6e 61 72  |ello.World.Binar|
00000020  79 00 44 61 74 61 ff fe  fd fc                    |y.Data....|
```

**Parquet File:**
```
📊 Parquet File: examples/employees.parquet
────────────────────────────────────────────────────────────────────────────────
Version: 2
Created by: parquet-cpp-arrow version 20.0.0
Number of rows: 5
Number of row groups: 1

Schema:
  1: id (int64)
  2: name (byte_array (string))
  3: age (int64)
  4: salary (double)
  5: active (boolean)

Data:
────────────────────────────────────────────────────────────────────────────────
│ id         │ name       │ age        │ salary     │ active     │
├────────────┼────────────┼────────────┼────────────┼────────────┤
│ 1          │ Alice      │ 25         │ 50000.00   │ true       │
│ 2          │ Bob        │ 30         │ 60000.00   │ true       │
│ 3          │ Charlie    │ 35         │ 70000.00   │ false      │
│ 4          │ Diana      │ 28         │ 55000.00   │ true       │
│ 5          │ Eve        │ 32         │ 65000.00   │ true       │

📈 Summary: 5 columns, 5 rows shown
```

### Advanced Examples

```bash
# Show Rust project structure (no target dir, with stats)
denarborea --exclude "target" -e rs,toml --stats

# Find large files in home directory
denarborea ~ --min-size 100MB --sort size -r -f

# Show git-tracked files with status
denarborea --git-ignore --git-status -s

# Export directory structure to JSON
denarborea --format json -o structure.json

# Show only modified files in git repo
denarborea --git-status --git-ignore -L 3

# Analyze file types in project
denarborea --stats --no-color > analysis.txt

# Show executable files only
denarborea -f --sort size -r | grep "→"
```

## Configuration File

Denarborea supports persistent configuration via TOML file at `~/.config/denarborea/config.toml`:

```toml
max_depth = 3
show_hidden = false
show_size = true
show_permissions = false
show_time = false
show_count = true
use_colors = true
sort_by = "Name"
reverse_sort = false
git_ignore = true
git_status = false
output_format = "Tree"
follow_links = false
full_path = false
show_checksum = false
show_stats = false
```

## Command-Line Options

```
Usage: denarborea [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the directory to visualize [default: .]

Options:
  -L, --max-depth <MAX_DEPTH>      Limit the depth of recursion
  -a, --all                        Show hidden files and directories
  -s, --size                       Show file sizes in human readable format
  -p, --permissions                Show file permissions
  -t, --time                       Show last modified time
  -c, --count                      Show file/directory counts
      --no-color                   Disable colored output
      --sort <SORT>                Sort files and directories [default: name] [possible values: name, size, time, extension, type]
  -e, --extension <EXTENSION>      Filter by file extension (e.g., rs, py, js)
  -d, --directories-only           Show only directories
  -f, --files-only                 Show only files
      --min-size <MIN_SIZE>        Show only files larger than specified size (e.g., 1MB, 500KB)
      --max-size <MAX_SIZE>        Show only files smaller than specified size (e.g., 10MB, 2GB)
      --exclude <EXCLUDE>          Exclude path matching pattern (can be used multiple times)
      --include <INCLUDE>          Include path matching pattern (can be used multiple times)
      --git-ignore                 Respect .gitignore files and global git config
      --git-status                 Show Git status indicators (requires git repo)
      --limit <LIMIT>              Limit the number of files displayed
      --format <FORMAT>            Output format [default: tree] [possible values: tree, json, xml, csv, markdown]
  -o, --output <OUTPUT>            Output to file
      --follow-links               Follow symbolic links
      --full-path                  Show full absolute paths
      --checksum                   Show MD5 checksums for files
  -r, --reverse                    Reverse the sort order
      --stats                      Show summary statistics at the end
  -h, --help                       Print help
  -V, --version                    Print version
```

## Project Structure

```
denarborea/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── lib.rs        # Library interface
│   ├── config.rs     # Configuration management
│   ├── tree.rs       # Core tree visualization logic
│   ├── display.rs    # Output formatting and colors
│   ├── git.rs        # Git integration
│   ├── stats.rs      # Statistics collection
│   └── utils.rs      # Utility functions
├── Cargo.toml        # Dependencies and metadata
└── README.md         # This file
```

## Examples

The `examples/` directory contains sample files to test the file viewer functionality:

- **config.json** - JSON configuration with nested objects and arrays
- **data.csv** - Employee data with multiple columns
- **docker-compose.yml** - YAML service configuration
- **Cargo.toml** - TOML project configuration
- **README.txt** - Plain text documentation
- **sample.bin** - Binary file with hex dump display
- **employees.parquet** - Parquet file with schema and metadata

Try the file viewer with these examples:

```bash
# View all example files
denarborea examples/

# Test different formats
denarborea --view examples/config.json
denarborea --view examples/data.csv  
denarborea --view examples/docker-compose.yml
denarborea --view examples/Cargo.toml
denarborea --view examples/README.txt
denarborea --view examples/sample.bin
denarborea --view examples/employees.parquet
```

## Dependencies

- **clap** - Command-line argument parsing with derive macros
- **colored** - Terminal color output
- **walkdir** - Efficient directory traversal
- **ignore** - .gitignore support
- **git2** - Git repository integration (optional feature)
- **serde** / **serde_json** - Serialization for JSON/TOML
- **humansize** - Human-readable file sizes
- **chrono** - Date/time formatting
- **md5** - Checksum calculation
- **regex** - Pattern matching
- **toml** - Configuration file parsing
- **dirs** - Cross-platform config directory

## Build Features

```bash
# Default build (includes git support)
cargo build --release

# Without git support
cargo build --release --no-default-features
```
