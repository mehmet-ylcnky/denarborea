# Git Integration

Integrate with Git repositories to show file status and respect .gitignore rules.

## Git Status Indicators

```bash
# Show git status for each file
denarborea --git-status
```

**Output:**
```
.
├── M Cargo.lock
├── M Cargo.toml
├── M README.md
├──   src/
│   ├── M config.rs
│   ├──   display.rs
│   └── M main.rs
└── I target/
```

### Status Indicators Explained

| Indicator | Meaning | Description |
|-----------|---------|-------------|
| `M ` | Modified | File has been modified |
| `A ` | Added | New file staged for commit |
| `D ` | Deleted | File has been deleted |
| `??` | Untracked | File not tracked by git |
| `R ` | Renamed | File has been renamed |
| `I ` | Ignored | File is ignored by .gitignore |
| `  ` | Clean | File is unchanged |

## Git Ignore Integration

```bash
# Respect .gitignore rules (hide ignored files)
denarborea --git-ignore
```

**Output:**
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src/
    ├── config.rs
    ├── display.rs
    └── main.rs
```

## Combined Git Features

```bash
# Show status and hide ignored files
denarborea --git-status --git-ignore
```

**Output:**
```
.
├── M Cargo.lock
├── M Cargo.toml
├── M README.md
└── src/
    ├── M config.rs
    ├──   display.rs
    └── M main.rs
```

## Git Status with File Details

### With Sizes
```bash
denarborea --git-status --git-ignore -s
```

**Output:**
```
.
├── M Cargo.lock [39.36 kB]
├── M Cargo.toml [528 B]
├── M README.md [8.78 kB]
└── src/
    ├── M config.rs [3.89 kB]
    ├──   display.rs [10.75 kB]
    └── M main.rs [4.99 kB]
```

### With Timestamps
```bash
denarborea --git-status -t
```

**Output:**
```
.
├── M Cargo.lock [2024-11-04 15:30:22]
├── M Cargo.toml [2024-11-04 15:30:22]
├── M README.md [2024-11-04 16:05:15]
└── src/
    ├── M config.rs [2024-11-04 14:15:05]
    ├──   display.rs [2024-11-04 13:20:10]
    └── M main.rs [2024-11-04 14:25:30]
```

## Practical Git Workflows

### Find Modified Files
```bash
# Show only modified files with details
denarborea --git-status --git-ignore -s -L 2 | grep "M "
```

**Output:**
```
├── M Cargo.lock [39.36 kB]
├── M Cargo.toml [528 B]
├── M README.md [8.78 kB]
    ├── M config.rs [3.89 kB]
    └── M main.rs [4.99 kB]
```

### Analyze Modified Files with Statistics
```bash
denarborea src --git-status -s --stats
```

**Output:**
```
src
├── M config.rs [3.89 kB]
├──   display.rs [10.75 kB]
├── M main.rs [4.99 kB]
└──   tree.rs [18.64 kB]

📊 Tree Statistics:
Directories: 0
Files: 4
Total size: 38.27 kB
Average file size: 9.57 kB
Largest file: tree.rs (18.64 kB)
Modified files: 2 (50%)
```

### Check Specific Directory Status
```bash
# Focus on source directory changes
denarborea src --git-status --git-ignore -s -p
```

**Output:**
```
src
├── M config.rs [3.89 kB] [rw-r--r--]
├──   display.rs [10.75 kB] [rw-r--r--]
├── M main.rs [4.99 kB] [rw-r--r--]
└──   tree.rs [18.64 kB] [rw-r--r--]
```

## Git Repository Detection

DenArborea automatically detects Git repositories by looking for:
- `.git` directory in current or parent directories
- Git configuration files
- Git index files

### Non-Git Directories
```bash
# In a non-git directory, git flags have no effect
denarborea /tmp --git-status
```

**Output:**
```
/tmp
├── file1.txt
├── file2.txt
└── directory/
    └── file3.txt
```

### Submodules and Nested Repositories
```bash
# Git status works in submodules too
denarborea vendor/library --git-status
```

**Output:**
```
vendor/library
├──   README.md
├──   src/
│   ├──   lib.rs
│   └── M main.rs
└── ?? new_feature.rs
```

## Performance Considerations

Git integration adds minimal overhead:
- Status checking: ~5-10ms per 100 files
- .gitignore parsing: ~1-2ms per directory
- Repository detection: ~1ms per invocation

For large repositories (>10k files), consider:
- Using `--limit` to reduce output
- Focusing on specific directories
- Combining with filtering options
