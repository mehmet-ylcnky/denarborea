# DenArborea Documentation

Comprehensive guides and examples for using DenArborea effectively.

## Quick Start

- [Basic Usage](basic-usage.md) - Get started with fundamental commands
- [Display Options](display-options.md) - Configure output appearance and information

## Core Features

- [Filtering](filtering.md) - Filter files by type, size, patterns, and more
- [Sorting](sorting.md) - Sort output by name, size, time, and other criteria
- [Git Integration](git-integration.md) - Work with Git repositories and status
- [Output Formats](output-formats.md) - Export in JSON, XML, CSV, and Markdown
- [File Viewer](file-viewer.md) - View and analyze file contents

## Advanced Usage

- [Advanced Examples](advanced-examples.md) - Real-world scenarios and complex workflows

## Command Reference

### Basic Commands
```bash
denarborea [PATH] [OPTIONS]
```

### Common Options
| Option | Description |
|--------|-------------|
| `-L, --level <NUM>` | Maximum depth to display |
| `-a, --all` | Show hidden files |
| `-s, --size` | Show file sizes |
| `-p, --permissions` | Show file permissions |
| `-t, --time` | Show modification times |
| `-c, --count` | Show file/directory counts |
| `--stats` | Show statistics summary |

### Filtering Options
| Option | Description |
|--------|-------------|
| `-e, --extension <EXT>` | Filter by file extension |
| `-d, --directories-only` | Show only directories |
| `-f, --files-only` | Show only files |
| `--min-size <SIZE>` | Minimum file size |
| `--max-size <SIZE>` | Maximum file size |
| `--exclude <PATTERN>` | Exclude patterns |
| `--include <PATTERN>` | Include patterns |

### Git Options
| Option | Description |
|--------|-------------|
| `--git-status` | Show git status indicators |
| `--git-ignore` | Respect .gitignore rules |

### Output Options
| Option | Description |
|--------|-------------|
| `--format <FORMAT>` | Output format (tree, json, xml, csv, markdown) |
| `-o, --output <FILE>` | Save output to file |
| `--sort <FIELD>` | Sort by field (name, size, time, extension, type) |
| `-r, --reverse` | Reverse sort order |

### File Viewer Options
| Option | Description |
|--------|-------------|
| `--view <FILE>` | View file contents |
| `--viewer-format <FORMAT>` | Force format (auto, text, json, csv, yaml, toml, binary, parquet) |
| `--max-lines <NUM>` | Limit displayed lines |
| `--max-bytes <NUM>` | Limit displayed bytes |
| `--delimiter <CHAR>` | CSV delimiter character |

## Examples by Use Case

### Development
```bash
# Analyze project structure
denarborea --stats --git-status --exclude "target"

# Find large files in build output
denarborea target/ --min-size 1MB --sort size -s

# Generate project documentation
denarborea --format markdown -L 3 -o structure.md
```

### System Administration
```bash
# Find large log files
denarborea /var/log --min-size 100MB --sort size -s

# Check directory permissions
denarborea /etc -p --directories-only

# Monitor disk usage
denarborea /home --sort size -s --directories-only --limit 10
```

### Data Analysis
```bash
# Inspect CSV structure
denarborea --view data.csv --max-lines 10

# Analyze parquet schema
denarborea --view dataset.parquet

# Export file inventory
denarborea --format csv --checksum -o inventory.csv
```

## Tips and Best Practices

### Performance
- Use `--limit` for large directories
- Combine `--git-ignore` to exclude build artifacts
- Use specific extensions (`-e`) to focus on relevant files

### Automation
- Export to JSON/CSV for programmatic processing
- Use in CI/CD pipelines for build analysis
- Combine with other tools via pipes

### Troubleshooting
- Use `--help` for command-specific help
- Check file permissions if access is denied
- Use `--viewer-format binary` for unknown file types

## Getting Help

- Run `denarborea --help` for command help
- Check individual documentation files for detailed examples
- Report issues on the project repository
