# Display Options

Configure how DenArborea displays file and directory information.

## File Sizes

```bash
# Show file sizes in human-readable format
denarborea -s
```

**Output:**
```
.
├── Cargo.toml [1.2 KB]
├── README.md [8.5 KB]
├── src/ [4.0 KB]
│   ├── main.rs [2.1 KB]
│   └── lib.rs [1.9 KB]
└── target/ [15.2 MB]
```

## Permissions

```bash
# Show Unix-style permissions
denarborea -p
```

**Output:**
```
.
├── Cargo.toml [rw-r--r--]
├── README.md [rw-r--r--]
├── src/ [rwxr-xr-x]
│   ├── main.rs [rw-r--r--]
│   └── lib.rs [rw-r--r--]
└── target/ [rwxr-xr-x]
```

## Timestamps

```bash
# Show last modified time
denarborea -t
```

**Output:**
```
.
├── Cargo.toml [2024-11-04 15:30:22]
├── README.md [2024-11-04 16:05:15]
├── src/ [2024-11-04 14:20:10]
│   ├── main.rs [2024-11-04 14:15:05]
│   └── lib.rs [2024-11-04 14:18:30]
└── target/ [2024-11-04 15:45:00]
```

## File/Directory Counts

```bash
# Show counts for directories
denarborea -c
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── src/ [2 files]
│   ├── main.rs
│   └── lib.rs
└── target/ [156 files, 12 directories]
```

## Combined Options

```bash
# Show everything: sizes, permissions, timestamps, counts
denarborea -s -p -t -c
```

**Output:**
```
.
├── Cargo.toml [1.2 KB] [rw-r--r--] [2024-11-04 15:30:22]
├── README.md [8.5 KB] [rw-r--r--] [2024-11-04 16:05:15]
├── src/ [4.0 KB] [rwxr-xr-x] [2024-11-04 14:20:10] [2 files]
│   ├── main.rs [2.1 KB] [rw-r--r--] [2024-11-04 14:15:05]
│   └── lib.rs [1.9 KB] [rw-r--r--] [2024-11-04 14:18:30]
└── target/ [15.2 MB] [rwxr-xr-x] [2024-11-04 15:45:00] [156 files, 12 directories]
```

## Statistics Summary

```bash
# Show comprehensive statistics
denarborea --stats
```

**Output:**
```
.
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── lib.rs

📊 Tree Statistics:
Directories: 2
Files: 3
Total size: 11.8 KB
Average file size: 3.9 KB
Largest file: README.md (8.5 KB)
Smallest file: Cargo.toml (1.2 KB)

📈 File Types:
.rs: 2 files (66.7%)
.toml: 1 file (33.3%)
.md: 1 file (33.3%)
```

## Checksums

```bash
# Calculate and display MD5 checksums
denarborea --checksum
```

**Output:**
```
.
├── Cargo.toml [MD5: a1b2c3d4e5f6...]
├── README.md [MD5: f6e5d4c3b2a1...]
└── src/
    ├── main.rs [MD5: 1a2b3c4d5e6f...]
    └── lib.rs [MD5: 6f5e4d3c2b1a...]
```

## Full Paths

```bash
# Show absolute paths instead of relative
denarborea --full-path
```

**Output:**
```
/home/user/project
├── /home/user/project/Cargo.toml
├── /home/user/project/README.md
└── /home/user/project/src
    ├── /home/user/project/src/main.rs
    └── /home/user/project/src/lib.rs
```
