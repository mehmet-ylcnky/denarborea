use clap::{Parser};
use std::path::PathBuf;
use denarborea::{Config, OutputFormat, Result, SortBy, TreeVisualizer, utils::parse_size};

#[derive(Parser)]
#[command(name = "denarborea")]
#[command(about = "Directory visualizer for Dena")]
#[command(version = "0.1.0")]
#[command(long_about = "
    A fast and beautiful directory visualizer for Dena
")]
struct Cli {
    ///Directory to visualize
    #[arg(default_value = ".", help = "Path to the directory to visualize")]
    path: PathBuf,

    /// Maximum depth to visualize
    #[arg(short = 'L', long, help = "Limit the depth of recursion")]
    max_depth: Option<usize>,

    /// Show hidden files and directories
    #[arg(short = 'a', long, help = "Show hidden files and directories")]
    all: bool,

    ///Show file sizes
    #[arg(short = 's', long, help = "Show file sizes in human readable format")]
    size: bool,

    /// Show file permissions
    #[arg(short = 'p', long, help = "Show file permissions")]
    permissions: bool,

    /// Show last modified tie
    #[arg(short = 't', long, help = "Show last modified time")]
    time: bool,

    /// Show file counts for directories
    #[arg(short = 'c', long, help = "Show file/directory counts")]
    count: bool,

    /// Disable colors
    #[arg(long, help = "Disable colored output")]
    no_color: bool,

    /// Sort method
    #[arg(
        long,
        value_enum,
        default_value = "name",
        help = "Sort files and directories"
    )]
    sort: SortBy,

    /// Filter by file extension (comma-separated for multiple)
    #[arg(short = 'e', long, help = "Filter by file extension (e.g., rs, py, js)")]
    extension: Option<String>,

    /// Show only directories
    #[arg(short = 'd', long, help = "Show only directories")]
    directories_only: bool,

    /// Show only files
    #[arg(short = 'f', long, help = "Show only files")]
    files_only: bool,

    /// Minimum file size filter
    #[arg(long, help = "Show only files larger than specified size (e.g., 1MB, 500KB")]
    min_size: Option<String>,

    /// Maximum file size filter
    #[arg(long, help = "Show only files smaller than specified size (e.g., 10MB, 2GB")]
    max_size: Option<String>,

    /// Éxclude directories/files matching pattern
    #[arg(long, help = "Exclude path matching pattern (can be used multiple times")]
    exclude: Vec<String>,

    /// Include directories/files matching pattern
    #[arg(long, help = "Include path matching pattern (can be used multiple times")]
    include: Vec<String>,

    /// Respect .gitignore files
    #[arg(long, help = "Respect .gitignore files and global git config")]
    git_ignore: bool,

    /// Show git status
    #[arg(long, help = "Shot Git status indicators (requires git repo)")]
    git_status: bool,

    /// Limit number of files to show
    #[arg(long, help = "Limit the number of files displayed")]
    limit: Option<usize>,

    /// Output format
    #[arg(long, value_enum, default_value = "tree", help = "Output format")]
    format: OutputFormat,

    /// Output to file instead of stdout
    #[arg(short = 'o', long, help = "Output to file")]
    output: Option<PathBuf>,

    /// Follow Symlinks
    #[arg(long, help = "Follow symbolic links")]
    follow_links: bool,

    /// Show full paths instead of relative
    #[arg(long, help = "Show full absolute paths")]
    full_path: bool,

    /// Show file checksums (MD5)
    #[arg(long, help = "Show MD5 checksums for files")]
    checksum: bool,

    /// Reverse sort order
    #[arg(short = 'r', long, help = "Reverse the sort order")]
    reverse: bool,

    /// Show summar statistics
    #[arg(long, help = "Show summary statistics at the end")]
    stats:bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config{
        max_depth: cli.max_depth,
        show_hidden: cli.all,
        show_size: cli.size,
        show_permissions: cli.permissions,
        show_time: cli.time,
        show_count: cli.count,
        use_colors: !cli.no_color,
        sort_by: cli.sort,
        reverse_sort: cli.reverse,
        filter_extension: cli.extension,
        directories_only: cli.directories_only,
        files_only: cli.files_only,
        min_size: parse_size(&cli.min_size)?,
        max_size: parse_size(&cli.max_size)?,
        exclude_patterns: cli.exclude,
        include_patterns: if cli.include.is_empty() { None } else { Some(cli.include.join(",")) },
        git_ignore: cli.git_ignore,
        git_status: cli.git_status,
        limit: cli.limit,
        output_format: cli.format,
        follow_links: cli.follow_links,
        full_path: cli.full_path,
        show_checksum: cli.checksum,
        show_stats: cli.stats,
        interactive: false,
    };

    let mut visualizer = TreeVisualizer::new(config);

    if let Some(output_file) = cli.output {
        visualizer.visualize_to_file(&cli.path, &output_file)?;
    } else {
        visualizer.visualize(&cli.path)?;
    }
    
    Ok(())
}