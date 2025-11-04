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

<div align="center">

A directory visualizer written in Rust with advanced filtering, Git integration, and multiple output formats.

</div>

**DenArborea** is a modern command-line directory visualization tool that extends the classic Unix `tree` command with powerful features for developers and system administrators. Built in Rust for performance and reliability, it provides an intuitive way to explore, analyze, and document directory structures with advanced filtering, Git integration, and multiple export formats.

Whether you're navigating complex codebases, analyzing disk usage, documenting project structures, or integrating with CI/CD pipelines, DenArborea transforms directory exploration from a simple listing into a comprehensive analysis tool. With built-in file content viewing, Git status integration, and export capabilities, it bridges the gap between basic file system navigation and advanced project analysis.

## Installation

```bash
# Clone the repository
git clone https://github.com/mehmet-ylcnky/denarborea.git
cd denarborea

# Build release version
cargo build --release

# Binary will be at target/release/denarborea
```

## Features

<details>
<summary>📊 Display Options</summary>

- **Tree View** - Classic tree structure with Unicode connectors
- **File Sizes** - Human-readable format (KB, MB, GB)
- **Permissions** - Unix-style permission display (rwxr-xr-x)
- **Timestamps** - Last modified time
- **File Counts** - Show file/directory counts for folders
- **MD5 Checksums** - Calculate and display file checksums
- **Symlinks** - Display symlink targets with arrows
- **Full Paths** - Show absolute paths instead of relative

</details>

<details>
<summary>🎨 Color Coding</summary>

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

</details>

<details>
<summary>🔍 Filtering & Sorting</summary>

- **Extension Filter** - Show only specific file types
- **Size Filters** - Min/max file size limits
- **Pattern Matching** - Include/exclude with regex or glob patterns
- **Hidden Files** - Show/hide dotfiles
- **Directories Only** - Show only folders
- **Files Only** - Show only files
- **Sort Options** - By name, size, time, extension, or type
- **Reverse Sort** - Reverse any sort order
- **Limit Results** - Cap the number of displayed items

</details>

<details>
<summary>🔧 Git Integration</summary>

- **Git Status** - Show file status indicators (M, A, D, ??, etc.)
- **Status Colors** - Modified (yellow), Added (green), Deleted (red), Untracked (red)
- **Git Ignore** - Respect .gitignore rules automatically

</details>

<details>
<summary>📤 Output Formats</summary>

- **Tree** - Traditional tree view
- **JSON** - Structured hierarchical data
- **XML** - Markup format with metadata
- **CSV** - Tabular format with headers
- **Markdown** - Table format for documentation

</details>

<details>
<summary>📄 File Viewer</summary>

- **Multi-format Support** - View JSON, CSV, YAML, TOML, and text files
- **Syntax Highlighting** - Color-coded display for better readability
- **Structured Formatting** - Pretty-printed JSON, organized CSV tables
- **Binary File Detection** - Hex dump view for binary files with file type detection
- **Content Limits** - Configurable line and byte limits for large files
- **Custom Delimiters** - Support for different CSV separators (comma, semicolon, tab, pipe)
- **Format Override** - Force specific format interpretation
- **Error Handling** - Graceful handling of malformed files

</details>

<details>
<summary>📈 Statistics</summary>

- Total files and directories
- Total size with human-readable format
- Average file size
- Largest and smallest files
- File type distribution (top 10)
- Symlink count

</details>

## Code Quality Standards

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

### Development Guidelines

All contributions must:
- Pass all existing tests
- Include tests for new functionality
- Follow Rust formatting standards (`cargo fmt`)
- Pass clippy linting (`cargo clippy`)
- Have accompanying tests for new features

## Usage

📚 **[Complete Documentation](docs/README.md)**

### Quick Start
```bash
# Show current directory
denarborea

# Show with file sizes and git status
denarborea -s --git-status

# View file contents
denarborea --view config.json
```

### Documentation Sections
- **[Basic Usage](docs/basic-usage.md)** - Fundamental commands and options
- **[Display Options](docs/display-options.md)** - Sizes, permissions, timestamps, and more
- **[Filtering](docs/filtering.md)** - Filter by extension, size, patterns
- **[Sorting](docs/sorting.md)** - Sort by name, size, time, type
- **[Git Integration](docs/git-integration.md)** - Git status and .gitignore support
- **[Output Formats](docs/output-formats.md)** - JSON, XML, CSV, Markdown export
- **[File Viewer](docs/file-viewer.md)** - View JSON, CSV, YAML, binary files
- **[Advanced Examples](docs/advanced-examples.md)** - Real-world scenarios and workflows

## Examples

📁 **[Example Files and Usage](docs/examples.md)** - Sample files and commands to test the file viewer functionality.

## Inspiration

While watching my daughter, Dena, exploring the plants near our home, I noticed how she'd stop at each one, tracing the branches with her eyes from trunk to the smallest twig. "Look, Babash," she said, pointing at one leaf, "she knows where it belongs."

Her observation reminded me of the Unix `tree` command that has served well for decades for developers who spend their daily life in `Terminal` (including myself). Directory structures are just like trees—each folder a branch, each file a leaf, all connected in a hierarchy that makes sense once you can see it clearly.

Than I found myself wanting more: *What if I could see which files changed in Git? What if I could filter by size or extension? What if I could export the structure as JSON for documentation?* That's when Dena's love for plants and my developer needs came together. **DenArborea** combines my daughter's name with *arborea* (Latin for "tree-like"), creating a tool that builds upon the classic `tree` command with modern features:
- **Git integration** to see modified files at a glance (`--git-status`)
- **Smart filtering** to find large files or specific extensions (`--min-size 100MB -e rs`)
- **Multiple output formats** for documentation and automation (`--format json`)
- **Rich metadata** like checksums, permissions, and statistics (`--checksum --stats`)

It transforms the abstract maze of nested directories into something as clear and natural as the trees she loves to climb—with the power tools developers need today.

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
