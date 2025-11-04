# File Viewer

View and analyze file contents directly from the command line with syntax highlighting and structured formatting.

## Supported Formats

- **JSON** - Pretty-printed with syntax highlighting
- **CSV** - Formatted tables with column alignment
- **YAML** - Structured display with proper indentation
- **TOML** - Configuration file formatting
- **Text** - Syntax-highlighted code and plain text
- **Binary** - Hex dump with file type detection
- **Parquet** - Schema and tabular data display

## Basic File Viewing

### JSON Files
```bash
denarborea --view examples/config.json
```

**Output:**
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

### CSV Files
```bash
denarborea --view examples/data.csv
```

**Output:**
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

### YAML Files
```bash
denarborea --view examples/docker-compose.yml
```

**Output:**
```
📄 YAML File: examples/docker-compose.yml
────────────────────────────────────────────────────────────
version: '3.8'
services:
  web:
    image: nginx:alpine
    ports:
    - 80:80
    volumes:
    - ./html:/usr/share/nginx/html
    environment:
    - NGINX_HOST=localhost
    - NGINX_PORT=80
  database:
    image: postgres:13
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
    volumes:
    - postgres_data:/var/lib/postgresql/data
    ports:
    - 5432:5432
volumes:
  postgres_data: null

📊 Summary: Object with 3 keys
```

### Text Files
```bash
denarborea --view examples/README.txt
```

**Output:**
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

## Binary Files

### Hex Dump Display
```bash
denarborea --view examples/sample.bin
```

**Output:**
```
🔢 Binary File: examples/sample.bin (42 bytes)
────────────────────────────────────────────────────────────────────────────────
File Type: Unknown Binary

Hex Dump:
00000000  50 4e 47 89 50 4e 47 0d  0a 1a 0a 00 00 00 0d 48  |PNG.PNG........H|
00000010  65 6c 6c 6f 00 57 6f 72  6c 64 00 42 69 6e 61 72  |ello.World.Binar|
00000020  79 00 44 61 74 61 ff fe  fd fc                    |y.Data....|
```

### File Type Detection
```bash
denarborea --view image.png --viewer-format binary
```

**Output:**
```
🔢 Binary File: image.png (1.2 MB)
────────────────────────────────────────────────────────────────────────────────
File Type: PNG Image

Hex Dump:
00000000  89 50 4e 47 0d 0a 1a 0a  00 00 00 0d 49 48 44 52  |.PNG........IHDR|
00000010  00 00 03 20 00 00 02 58  08 06 00 00 00 4c 8d 87  |... ...X.....L..|
00000020  3c 00 00 00 19 74 45 58  74 53 6f 66 74 77 61 72  |<....tEXtSoftwar|
```

## Parquet Files

### Schema and Data Display
```bash
denarborea --view examples/employees.parquet
```

**Output:**
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

## Content Limits

### Limit Lines
```bash
# Show only first 10 lines
denarborea --view large-file.txt --max-lines 10
```

**Output:**
```
📄 large-file.txt (Plain Text)
────────────────────────────────────────────────────────────
   1 │ Line 1 content
   2 │ Line 2 content
   3 │ Line 3 content
   4 │ Line 4 content
   5 │ Line 5 content
   6 │ Line 6 content
   7 │ Line 7 content
   8 │ Line 8 content
   9 │ Line 9 content
  10 │ Line 10 content

... (showing first 10 lines of 1000 total)
```

### Limit Bytes
```bash
# Show only first 1KB
denarborea --view large-file.txt --max-bytes 1024
```

**Output:**
```
📄 large-file.txt (Plain Text)
────────────────────────────────────────────────────────────
   1 │ Content truncated to first 1024 bytes...
   2 │ [Additional content hidden]

... (showing first 1024 bytes of 50KB total)
```

### Parquet Row Limits
```bash
# Show only first 3 rows of parquet data
denarborea --view employees.parquet --max-lines 3
```

**Output:**
```
📊 Parquet File: employees.parquet
────────────────────────────────────────────────────────────────────────────────
Schema: [schema details...]

Data:
│ id    │ name    │ age │ salary   │ active │
├───────┼─────────┼─────┼──────────┼────────┤
│ 1     │ Alice   │ 25  │ 50000.00 │ true   │
│ 2     │ Bob     │ 30  │ 60000.00 │ true   │
│ 3     │ Charlie │ 35  │ 70000.00 │ false  │

📈 Summary: 5 columns, 3 rows shown (limited to 3 rows)
```

## Custom Delimiters

### CSV with Semicolon
```bash
denarborea --view data.csv --delimiter ";"
```

### CSV with Tab
```bash
denarborea --view data.tsv --delimiter $'\t'
```

### CSV with Pipe
```bash
denarborea --view data.txt --delimiter "|"
```

## Format Override

### Force JSON Interpretation
```bash
# View .txt file as JSON
denarborea --view data.txt --viewer-format json
```

### Force Binary View
```bash
# View any file as binary
denarborea --view document.pdf --viewer-format binary
```

### Auto-Detection
```bash
# Let DenArborea detect format automatically
denarborea --view unknown-file --viewer-format auto
```

## Error Handling

### Malformed JSON
```bash
denarborea --view broken.json
```

**Output:**
```
📋 JSON File: broken.json
────────────────────────────────────────────────────────────
Error parsing JSON: EOF while parsing an object at line 1 column 19

Raw content:
{"incomplete": true
```

### Unreadable Files
```bash
denarborea --view /root/secret.txt
```

**Output:**
```
Error viewing file /root/secret.txt: Permission denied (os error 13)
```

### Binary Files as Text
```bash
denarborea --view image.png --viewer-format text
```

**Output:**
```
📄 image.png (Plain Text)
────────────────────────────────────────────────────────────
   1 │ <binary content detected - use --viewer-format binary for hex dump>
```

## Advanced Usage

### Combine with Tree View
```bash
# View directory structure and file content
denarborea examples/ && denarborea --view examples/config.json
```

### Pipe to Other Tools
```bash
# Extract JSON and process with jq
denarborea --view config.json --viewer-format json | jq '.features'
```

### Batch File Analysis
```bash
# View multiple files
for file in examples/*.json; do
  echo "=== $file ==="
  denarborea --view "$file"
done
```
