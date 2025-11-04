# Sorting

Control how files and directories are ordered in the output.

## Sort by Name (Default)

```bash
# Alphabetical sorting (default behavior)
denarborea --sort name
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── examples/
├── src/
└── target/
```

## Sort by Size

```bash
# Sort by file/directory size (largest first)
denarborea --sort size -s
```

**Output:**
```
.
├── target/ [15.2 MB]
├── README.md [8.5 KB]
├── src/ [4.0 KB]
├── Cargo.toml [1.2 KB]
└── examples/ [856 B]
```

## Sort by Modification Time

```bash
# Sort by last modified time (newest first)
denarborea --sort time -t
```

**Output:**
```
.
├── README.md [2024-11-04 16:05:15]
├── target/ [2024-11-04 15:45:00]
├── Cargo.toml [2024-11-04 15:30:22]
├── src/ [2024-11-04 14:20:10]
└── examples/ [2024-11-04 12:15:30]
```

## Sort by Extension

```bash
# Group files by extension
denarborea --sort extension
```

**Output:**
```
.
├── README.md
├── Cargo.toml
├── src/
│   ├── config.rs
│   ├── lib.rs
│   ├── main.rs
│   └── utils.rs
├── examples/
└── target/
```

## Sort by Type

```bash
# Directories first, then files
denarborea --sort type
```

**Output:**
```
.
├── examples/
├── src/
├── target/
├── Cargo.toml
└── README.md
```

## Reverse Sort Order

```bash
# Reverse any sort order
denarborea --sort size -r -s
```

**Output:**
```
.
├── examples/ [856 B]
├── Cargo.toml [1.2 KB]
├── src/ [4.0 KB]
├── README.md [8.5 KB]
└── target/ [15.2 MB]
```

## Advanced Sorting Examples

### Sort by Size with Details
```bash
# Show sizes and sort by them
denarborea --sort size -s -p -t
```

**Output:**
```
.
├── target/ [15.2 MB] [rwxr-xr-x] [2024-11-04 15:45:00]
├── README.md [8.5 KB] [rw-r--r--] [2024-11-04 16:05:15]
├── src/ [4.0 KB] [rwxr-xr-x] [2024-11-04 14:20:10]
├── Cargo.toml [1.2 KB] [rw-r--r--] [2024-11-04 15:30:22]
└── examples/ [856 B] [rwxr-xr-x] [2024-11-04 12:15:30]
```

### Sort by Time with Git Status
```bash
# Sort by modification time and show git status
denarborea --sort time --git-status -t
```

**Output:**
```
.
├── M README.md [2024-11-04 16:05:15]
├──   target/ [2024-11-04 15:45:00]
├── M Cargo.toml [2024-11-04 15:30:22]
├──   src/ [2024-11-04 14:20:10]
└──   examples/ [2024-11-04 12:15:30]
```

### Sort Large Directories
```bash
# Sort by size in a large directory with limits
denarborea /usr/bin --sort size -s --limit 10
```

**Output:**
```
/usr/bin
├── docker [68.2 MB]
├── node [45.1 MB]
├── python3 [32.8 MB]
├── git [28.4 MB]
├── cargo [25.7 MB]
├── rustc [22.3 MB]
├── vim [18.9 MB]
├── bash [15.6 MB]
├── curl [12.4 MB]
└── ... (showing first 10 items)
```

## Sorting Options Summary

| Option | Description | Example |
|--------|-------------|---------|
| `--sort name` | Sort alphabetically (default) | `denarborea --sort name` |
| `--sort size` | Sort by size (largest first) | `denarborea --sort size -s` |
| `--sort time` | Sort by modification time (newest first) | `denarborea --sort time -t` |
| `--sort extension` | Group by file extension | `denarborea --sort extension` |
| `--sort type` | Directories first, then files | `denarborea --sort type` |
| `-r, --reverse` | Reverse sort order | `denarborea --sort size -r` |
