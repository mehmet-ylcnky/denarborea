# Basic Usage

This guide covers the fundamental commands and options for using DenArborea.

## Getting Started

### Show Current Directory
```bash
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

### Show Specific Directory
```bash
denarborea /path/to/directory
```

### Limit Depth
```bash
# Limit to 2 levels deep
denarborea -L 2
```

**Output:**
```
.
├── Cargo.toml
├── README.md
├── src/
└── target/
```

### Show Hidden Files
```bash
denarborea -a
```

**Output:**
```
.
├── .git/
├── .gitignore
├── Cargo.toml
├── README.md
└── src/
```

## Common Options

| Option | Description | Example |
|--------|-------------|---------|
| `-L, --level <NUM>` | Maximum depth to display | `denarborea -L 3` |
| `-a, --all` | Show hidden files | `denarborea -a` |
| `-d, --directories-only` | Show only directories | `denarborea -d` |
| `-f, --files-only` | Show only files | `denarborea -f` |
| `--help` | Show help information | `denarborea --help` |
| `--version` | Show version | `denarborea --version` |
