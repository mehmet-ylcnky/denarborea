# Filtering

Filter files and directories based on various criteria.

## Extension Filtering

### Single Extension
```bash
# Show only Rust files
denarborea -e rs
```

**Output:**
```
.
└── src/
    ├── main.rs
    ├── lib.rs
    └── config.rs
```

### Multiple Extensions
```bash
# Show Rust, TOML, and Markdown files
denarborea -e rs,toml,md
```

**Output:**
```
.
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── lib.rs
    └── config.rs
```

## File Type Filtering

### Directories Only
```bash
denarborea -d
```

**Output:**
```
.
├── src/
├── target/
├── docs/
└── docs/example_files/
```

### Files Only
```bash
denarborea -f
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── .gitignore
└── LICENSE
```

## Size Filtering

### Minimum Size
```bash
# Files larger than 1MB
denarborea --min-size 1MB
```

**Output:**
```
.
└── target/
    └── release/
        └── denarborea [2.3 MB]
```

### Maximum Size
```bash
# Files smaller than 100KB
denarborea --max-size 100KB
```

**Output:**
```
.
├── Cargo.toml [1.2 KB]
├── README.md [8.5 KB]
├── .gitignore [156 B]
└── src/
    ├── main.rs [2.1 KB]
    └── lib.rs [1.9 KB]
```

### Size Range
```bash
# Files between 1KB and 10KB
denarborea --min-size 1KB --max-size 10KB
```

**Output:**
```
.
├── Cargo.toml [1.2 KB]
├── README.md [8.5 KB]
└── src/
    ├── main.rs [2.1 KB]
    └── lib.rs [1.9 KB]
```

## Pattern Matching

### Exclude Patterns
```bash
# Exclude build directories and dependencies
denarborea --exclude "target" --exclude "node_modules"
```

**Output:**
```
.
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── lib.rs
```

### Include Patterns
```bash
# Only show source directories
denarborea --include "src"
```

**Output:**
```
.
└── src/
    ├── main.rs
    ├── lib.rs
    └── config.rs
```

### Regex Patterns
```bash
# Files matching regex pattern
denarborea --pattern ".*\.rs$"
```

**Output:**
```
.
└── src/
    ├── main.rs
    ├── lib.rs
    └── config.rs
```

## Result Limiting

### Limit Number of Items
```bash
# Show only first 10 items
denarborea --limit 10
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── .gitignore
├── LICENSE
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   └── utils.rs
└── ... (showing first 10 items)
```

## Combined Filtering

```bash
# Complex filtering: Rust files larger than 1KB, excluding tests
denarborea -e rs --min-size 1KB --exclude "test"
```

**Output:**
```
.
└── src/
    ├── main.rs [2.1 KB]
    ├── lib.rs [1.9 KB]
    └── config.rs [3.2 KB]
```

## Hidden Files

### Show Hidden Files
```bash
# Include dotfiles and hidden directories
denarborea -a
```

**Output:**
```
.
├── .git/
├── .gitignore
├── .github/
├── Cargo.toml
├── README.md
└── src/
```

### Hide Hidden Files (Default)
```bash
# Default behavior - hidden files are excluded
denarborea
```

**Output:**
```
.
├── Cargo.toml
├── README.md
└── src/
```
