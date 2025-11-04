# Advanced Examples

Complex usage scenarios and real-world applications of DenArborea.

## Project Analysis

### Rust Project Structure
```bash
# Analyze Rust project excluding build artifacts
denarborea --exclude "target" -e rs,toml --stats --git-status
```

**Output:**
```
.
├── M Cargo.toml
├──   src/
│   ├── M main.rs
│   ├──   lib.rs
│   ├── M config.rs
│   └──   utils.rs
├──   tests/
│   ├──   integration_tests.rs
│   └──   unit_tests.rs
└──   docs/example_files/
    └──   basic_usage.rs

📊 Tree Statistics:
Directories: 3
Files: 7
Total size: 45.2 KB
Average file size: 6.5 KB
Largest file: lib.rs (12.3 KB)
Modified files: 3 (42.9%)

📈 File Types:
.rs: 6 files (85.7%)
.toml: 1 file (14.3%)
```

### Node.js Project Cleanup
```bash
# Find large files excluding dependencies
denarborea --exclude "node_modules" --min-size 1MB --sort size -s
```

**Output:**
```
.
├── dist/bundle.js [5.2 MB]
├── coverage/report.html [2.8 MB]
├── docs/api.pdf [1.4 MB]
└── assets/video.mp4 [1.2 MB]

📊 Summary: 4 large files found (>1MB)
Total size: 10.6 MB
```

## Git Workflow Integration

### Pre-commit Analysis
```bash
# Check what files will be committed
denarborea --git-status --git-ignore -s -t | grep "M \|A "
```

**Output:**
```
├── M src/main.rs [2.1 KB] [2024-11-04 16:05:15]
├── A tests/new_test.rs [1.8 KB] [2024-11-04 16:03:22]
├── M README.md [8.5 KB] [2024-11-04 16:05:15]
```

### Release Preparation
```bash
# Generate file manifest for release
denarborea --format json --git-ignore --files-only -o release-manifest.json
```

### Code Review Helper
```bash
# Show modified files with context
denarborea --git-status -s -p -t -L 2 --exclude "target" --exclude "*.lock"
```

**Output:**
```
.
├── M Cargo.toml [1.2 KB] [rw-r--r--] [2024-11-04 15:30:22]
├── M README.md [8.5 KB] [rw-r--r--] [2024-11-04 16:05:15]
├──   src/ [rwxr-xr-x] [2024-11-04 14:20:10]
│   ├── M config.rs [3.9 KB] [rw-r--r--] [2024-11-04 14:15:05]
│   └── M main.rs [5.0 KB] [rw-r--r--] [2024-11-04 14:25:30]
└──   tests/ [rwxr-xr-x] [2024-11-04 13:45:00]
```

## Documentation Generation

### API Documentation Structure
```bash
# Generate markdown table for documentation
denarborea docs/ --format markdown --files-only -e md
```

**Output:**
```markdown
| Path | Name | Size | Modified |
|------|------|------|----------|
| docs/api.md | api.md | 12.3 KB | 2024-11-04 15:30:22 |
| docs/getting-started.md | getting-started.md | 8.7 KB | 2024-11-04 14:20:10 |
| docs/advanced-usage.md | advanced-usage.md | 15.2 KB | 2024-11-04 16:05:15 |
| docs/troubleshooting.md | troubleshooting.md | 6.1 KB | 2024-11-04 13:45:00 |
```

### Project README Generation
```bash
# Create project structure section for README
denarborea -L 3 --exclude "target" --exclude "node_modules" -o project-structure.txt
```

## Build System Integration

### Makefile Integration
```makefile
# Makefile example
.PHONY: analyze
analyze:
	@echo "Project Analysis:"
	@denarborea --stats --git-status --git-ignore
	@echo "\nLarge Files (>100KB):"
	@denarborea --min-size 100KB --sort size -s --files-only

.PHONY: manifest
manifest:
	@denarborea --format json --git-ignore -o build/file-manifest.json
	@echo "File manifest generated: build/file-manifest.json"
```

### CI/CD Pipeline
```yaml
# GitHub Actions example
- name: Analyze Project Structure
  run: |
    denarborea --stats --format json > structure-report.json
    denarborea --git-status --format csv > git-status.csv
    
- name: Check for Large Files
  run: |
    if denarborea --min-size 10MB --files-only | grep -q .; then
      echo "Warning: Large files detected"
      denarborea --min-size 10MB --sort size -s
      exit 1
    fi
```

## System Administration

### Log File Analysis
```bash
# Find large log files
denarborea /var/log --min-size 100MB --sort size -s -t
```

**Output:**
```
/var/log
├── syslog.1 [245.7 MB] [2024-11-03 23:59:59]
├── kern.log [189.3 MB] [2024-11-04 15:30:22]
├── apache2/access.log [156.8 MB] [2024-11-04 16:05:15]
└── mysql/error.log [123.4 MB] [2024-11-04 14:20:10]
```

### Disk Usage Investigation
```bash
# Find directories consuming most space
denarborea /home --directories-only --sort size -s --limit 10
```

**Output:**
```
/home
├── user1/ [15.2 GB]
├── user2/ [8.7 GB]
├── user3/ [5.4 GB]
├── shared/ [3.2 GB]
├── backup/ [2.1 GB]
├── temp/ [1.8 GB]
├── cache/ [945 MB]
├── logs/ [567 MB]
├── config/ [234 MB]
└── scripts/ [123 MB]
```

## Data Analysis

### CSV File Inspection
```bash
# Analyze CSV structure and content
denarborea --view data/sales.csv --max-lines 5
```

**Output:**
```
📊 CSV File: data/sales.csv
────────────────────────────────────────────────────────────────────────────────
│ date       │ product    │ quantity │ price    │ total     │
├────────────┼────────────┼──────────┼──────────┼───────────┤
│ 2024-01-01 │ Widget A   │ 10       │ 25.99    │ 259.90    │
│ 2024-01-01 │ Widget B   │ 5        │ 45.50    │ 227.50    │
│ 2024-01-02 │ Widget A   │ 15       │ 25.99    │ 389.85    │
│ 2024-01-02 │ Widget C   │ 8        │ 12.75    │ 102.00    │
│ 2024-01-03 │ Widget B   │ 12       │ 45.50    │ 546.00    │

📈 Summary: 5 columns, 5 rows shown (of 10,000 total)
```

### Parquet Data Exploration
```bash
# Explore parquet file structure
denarborea --view analytics/user_events.parquet --max-lines 3
```

**Output:**
```
📊 Parquet File: analytics/user_events.parquet
────────────────────────────────────────────────────────────────────────────────
Version: 2
Number of rows: 1,000,000
Number of row groups: 10

Schema:
  1: user_id (int64)
  2: event_type (string)
  3: timestamp (timestamp)
  4: properties (json)

Data:
│ user_id │ event_type │ timestamp           │ properties        │
├─────────┼────────────┼─────────────────────┼───────────────────┤
│ 12345   │ page_view  │ 2024-11-04 10:30:15│ {"page": "/home"} │
│ 12346   │ click      │ 2024-11-04 10:30:22│ {"button": "cta"} │
│ 12347   │ purchase   │ 2024-11-04 10:31:05│ {"amount": 99.99} │

📈 Summary: 4 columns, 3 rows shown (limited to 3 rows)
```

## Security Auditing

### Find Sensitive Files
```bash
# Look for potential sensitive files
denarborea --pattern ".*\.(key|pem|p12|jks)$" -s -t
```

**Output:**
```
.
├── config/ssl/server.key [2.1 KB] [2024-11-04 10:15:30]
├── certs/client.pem [1.8 KB] [2024-11-04 09:45:22]
└── secrets/keystore.jks [4.5 KB] [2024-11-04 08:30:15]

⚠️  Warning: 3 potential sensitive files found
```

### Permission Audit
```bash
# Check file permissions in sensitive directories
denarborea /etc --files-only -p --pattern ".*\.(conf|cfg|ini)$"
```

**Output:**
```
/etc
├── ssh/sshd_config [rw-r--r--]
├── nginx/nginx.conf [rw-r--r--]
├── mysql/my.cnf [rw-r-----]
├── ssl/openssl.cnf [rw-r--r--]
└── app/config.ini [rw-------]
```

## Performance Monitoring

### Build Artifact Analysis
```bash
# Monitor build output sizes
denarborea target/release --sort size -s --files-only
```

**Output:**
```
target/release
├── denarborea [15.2 MB]
├── libdenarborea.so [8.7 MB]
├── denarborea.debug [25.4 MB]
└── deps/ [156 files, 45.8 MB total]

📊 Build Summary:
Total artifacts: 159 files
Total size: 95.1 MB
Largest binary: denarborea.debug (25.4 MB)
```

### Dependency Size Analysis
```bash
# Analyze dependency sizes in different package managers
denarborea node_modules --directories-only --sort size -s --limit 10
```

**Output:**
```
node_modules
├── @babel/core/ [12.3 MB]
├── webpack/ [8.7 MB]
├── typescript/ [6.4 MB]
├── react/ [4.2 MB]
├── lodash/ [3.8 MB]
├── moment/ [2.9 MB]
├── axios/ [1.7 MB]
├── express/ [1.4 MB]
├── jest/ [1.2 MB]
└── eslint/ [945 KB]

📊 Top 10 dependencies: 43.5 MB (of 156.7 MB total)
```

## Automation Scripts

### Cleanup Script
```bash
#!/bin/bash
# cleanup.sh - Remove large temporary files

echo "Finding large temporary files..."
denarborea /tmp --min-size 100MB --sort size -s --files-only > large_temps.txt

if [ -s large_temps.txt ]; then
    echo "Large temporary files found:"
    cat large_temps.txt
    echo "Remove these files? (y/N)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        # Extract file paths and remove them
        awk '{print $2}' large_temps.txt | xargs rm -f
        echo "Files removed."
    fi
else
    echo "No large temporary files found."
fi
```

### Monitoring Script
```bash
#!/bin/bash
# monitor.sh - Monitor project changes

echo "Project Status Report - $(date)"
echo "=================================="

echo -e "\n📊 Project Statistics:"
denarborea --stats --git-ignore

echo -e "\n🔄 Git Status:"
denarborea --git-status --git-ignore -s | head -20

echo -e "\n📈 Largest Files:"
denarborea --sort size -s --files-only --limit 5 --git-ignore

echo -e "\n⚠️  Large Files (>1MB):"
denarborea --min-size 1MB --sort size -s --files-only --git-ignore
```
