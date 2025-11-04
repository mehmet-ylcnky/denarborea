# Output Formats

Export directory structures in various formats for documentation and automation.

## Tree Format (Default)

```bash
# Standard tree output
denarborea
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   └── lib.rs
└── target/
    └── release/
        └── denarborea
```

## JSON Format

```bash
# Structured JSON output
denarborea --format json
```

**Output:**
```json
{
  "name": ".",
  "type": "directory",
  "size": 1048576,
  "modified": "2024-11-04T15:30:22Z",
  "children": [
    {
      "name": "Cargo.toml",
      "type": "file",
      "size": 1234,
      "modified": "2024-11-04T15:30:22Z"
    },
    {
      "name": "README.md",
      "type": "file",
      "size": 8704,
      "modified": "2024-11-04T16:05:15Z"
    },
    {
      "name": "src",
      "type": "directory",
      "size": 4096,
      "modified": "2024-11-04T14:20:10Z",
      "children": [
        {
          "name": "main.rs",
          "type": "file",
          "size": 2150,
          "modified": "2024-11-04T14:15:05Z"
        },
        {
          "name": "lib.rs",
          "type": "file",
          "size": 1946,
          "modified": "2024-11-04T14:18:30Z"
        }
      ]
    }
  ]
}
```

## XML Format

```bash
# XML structured output
denarborea --format xml
```

**Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<directory name="." size="1048576" modified="2024-11-04T15:30:22Z">
  <file name="Cargo.toml" size="1234" modified="2024-11-04T15:30:22Z"/>
  <file name="README.md" size="8704" modified="2024-11-04T16:05:15Z"/>
  <directory name="src" size="4096" modified="2024-11-04T14:20:10Z">
    <file name="main.rs" size="2150" modified="2024-11-04T14:15:05Z"/>
    <file name="lib.rs" size="1946" modified="2024-11-04T14:18:30Z"/>
  </directory>
  <directory name="target" size="15728640" modified="2024-11-04T15:45:00Z">
    <directory name="release" size="15728640" modified="2024-11-04T15:45:00Z">
      <file name="denarborea" size="15728640" modified="2024-11-04T15:45:00Z"/>
    </directory>
  </directory>
</directory>
```

## CSV Format

```bash
# Tabular CSV output
denarborea --format csv
```

**Output:**
```csv
path,name,type,size,modified,permissions
"./Cargo.toml","Cargo.toml","file",1234,"2024-11-04T15:30:22Z","rw-r--r--"
"./README.md","README.md","file",8704,"2024-11-04T16:05:15Z","rw-r--r--"
"./src","src","directory",4096,"2024-11-04T14:20:10Z","rwxr-xr-x"
"./src/main.rs","main.rs","file",2150,"2024-11-04T14:15:05Z","rw-r--r--"
"./src/lib.rs","lib.rs","file",1946,"2024-11-04T14:18:30Z","rw-r--r--"
"./target","target","directory",15728640,"2024-11-04T15:45:00Z","rwxr-xr-x"
"./target/release","release","directory",15728640,"2024-11-04T15:45:00Z","rwxr-xr-x"
"./target/release/denarborea","denarborea","file",15728640,"2024-11-04T15:45:00Z","rwxr-xr-x"
```

## Markdown Format

```bash
# Markdown table output
denarborea --format markdown
```

**Output:**
```markdown
| Path | Name | Type | Size | Modified | Permissions |
|------|------|------|------|----------|-------------|
| ./Cargo.toml | Cargo.toml | file | 1.2 KB | 2024-11-04 15:30:22 | rw-r--r-- |
| ./README.md | README.md | file | 8.5 KB | 2024-11-04 16:05:15 | rw-r--r-- |
| ./src | src | directory | 4.0 KB | 2024-11-04 14:20:10 | rwxr-xr-x |
| ./src/main.rs | main.rs | file | 2.1 KB | 2024-11-04 14:15:05 | rw-r--r-- |
| ./src/lib.rs | lib.rs | file | 1.9 KB | 2024-11-04 14:18:30 | rw-r--r-- |
| ./target | target | directory | 15.0 MB | 2024-11-04 15:45:00 | rwxr-xr-x |
| ./target/release | release | directory | 15.0 MB | 2024-11-04 15:45:00 | rwxr-xr-x |
| ./target/release/denarborea | denarborea | file | 15.0 MB | 2024-11-04 15:45:00 | rwxr-xr-x |
```

## Saving to Files

### Save Tree Output
```bash
# Save standard tree to file
denarborea -o project-structure.txt
```

### Save JSON Output
```bash
# Export as JSON for processing
denarborea --format json -o structure.json
```

### Save CSV for Analysis
```bash
# Export as CSV for spreadsheet analysis
denarborea --format csv -o file-inventory.csv
```

### Save Markdown for Documentation
```bash
# Generate markdown table for docs
denarborea --format markdown -o file-structure.md
```

## Format-Specific Options

### JSON with Git Status
```bash
denarborea --format json --git-status
```

**Output:**
```json
{
  "name": ".",
  "type": "directory",
  "git_status": "clean",
  "children": [
    {
      "name": "Cargo.toml",
      "type": "file",
      "size": 1234,
      "git_status": "modified"
    },
    {
      "name": "README.md",
      "type": "file",
      "size": 8704,
      "git_status": "modified"
    }
  ]
}
```

### CSV with All Details
```bash
denarborea --format csv -s -p -t --git-status
```

**Output:**
```csv
path,name,type,size,modified,permissions,git_status
"./Cargo.toml","Cargo.toml","file","1.2 KB","2024-11-04T15:30:22Z","rw-r--r--","modified"
"./README.md","README.md","file","8.5 KB","2024-11-04T16:05:15Z","rw-r--r--","modified"
"./src","src","directory","4.0 KB","2024-11-04T14:20:10Z","rwxr-xr-x","clean"
```

## Use Cases

### Documentation Generation
```bash
# Generate project structure for README
denarborea --format markdown -L 3 -o project-structure.md
```

### Build System Integration
```bash
# Export file list for build tools
denarborea --format json --files-only -o file-manifest.json
```

### Inventory Management
```bash
# Create file inventory with checksums
denarborea --format csv --checksum -o file-inventory.csv
```

### API Integration
```bash
# Generate JSON for web APIs
denarborea --format json --git-status | curl -X POST -H "Content-Type: application/json" -d @- https://api.example.com/structure
```
