# Rust CLI Tree Visualizer

A powerful command-line tool written in Rust for recursively scanning and displaying directory structures with advanced filtering, git integration, and multiple output formats.

## Features

### Core Functionality
- **Recursive directory traversal** with configurable depth limits
- **Multiple output formats**: Tree, JSON, XML, CSV, Markdown
- **Colorized terminal output** with file type-specific colors
- **Git integration** with status indicators and .gitignore support
- **Comprehensive filtering** by extension, size, patterns, and file types

### Display Options
- **File metadata**: Size, permissions, timestamps, checksums (MD5)
- **Directory statistics**: File/folder counts, largest/smallest files
- **Git status indicators**: Modified, added, deleted, untracked files
- **Symlink resolution** with target path display
- **File type coloring**: Directories (blue), executables (green), code files by language

### Filtering & Sorting
- **Extension filtering**: Include/exclude specific file types
- **Size filtering**: Min/max file size limits with unit support (KB, MB, GB)
- **Pattern matching**: Regex and glob-style patterns
- **Hidden files**: Show/hide dotfiles
- **Git ignore**: Respect .gitignore rules
- **Sorting options**: Name, size, time, extension, type (ascending/descending)

### Configuration
- **Persistent settings**: TOML configuration file support
- **CLI overrides**: Command-line flags override config file settings
- **Cross-platform**: Unix permissions on supported systems

## Dependencies

- **clap**: Command-line argument parsing
- **walkdir**: Directory traversal
- **ignore**: .gitignore support
- **git2**: Git repository integration (optional)
- **colored**: Terminal color output
- **serde**: Serialization for JSON/TOML
- **humansize**: Human-readable file sizes
- **chrono**: Date/time formatting
- **md5**: Checksum calculation
- **regex**: Pattern matching

## Project Structure

```
src/
├── main.rs          # Entry point (placeholder)
├── lib.rs           # Library interface
├── config.rs        # Configuration management
├── tree.rs          # Core tree visualization logic
├── display.rs       # Output formatting
├── git.rs           # Git integration
├── stats.rs         # Statistics collection
└── utils.rs         # Utility functions
```

## Configuration Options

The tool supports extensive configuration through CLI flags and config files:

- **Depth control**: `max_depth`, `limit`
- **Display options**: `show_hidden`, `show_size`, `show_permissions`, `show_time`, `show_count`, `show_checksum`, `show_stats`
- **Filtering**: `filter_extension`, `min_size`, `max_size`, `exclude_patterns`, `include_patterns`
- **Git options**: `git_ignore`, `git_status`
- **Output**: `output_format`, `use_colors`, `full_path`
- **Sorting**: `sort_by`, `reverse_sort`
- **File types**: `directories_only`, `files_only`, `follow_links`

## Output Formats

1. **Tree**: Traditional tree view with Unicode connectors
2. **JSON**: Structured data with nested hierarchy
3. **CSV**: Tabular format with headers
4. **Markdown**: Table format for documentation
5. **XML**: Structured markup with metadata

## Build Features

- **Default**: Includes git integration
- **git**: Optional git2 dependency for repository features

## Status

Core functionality implemented. CLI interface pending implementation.