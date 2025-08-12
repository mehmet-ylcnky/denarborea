use serde::{Deserialize, Serialize};
use clap::ValueEnum;

use crate::config;

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum SortBy {
    #[value(name = "name")]
    Name,
    #[value(name = "size")]
    Size,
    #[value(name = "time")]
    Time,
    #[value(name = "extension")]
    Extension,
    #[value(name = "type")]
    Type
}

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum OutputFormat {
    #[value(name = "tree")]
    Tree,
    #[value(name = "json")]
    Json,
    #[value(name = "xml")]
    Xml,
    #[value(name = "csv")]
    Csv,
    #[value(name = "markdown")]
    Markdown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    pub show_size: bool,
    pub show_permissions: bool,
    pub show_time: bool,
    pub show_count: bool,
    pub use_colors: bool,
    pub sort_by: SortBy,
    pub reverse_sort: bool,
    pub filter_extension: Option<String>,
    pub directories_only: bool,
    pub files_only: bool,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Option<String>,
    pub git_ignore: bool,
    pub git_status: bool,
    pub limit: Option<usize>,
    pub output_format: OutputFormat,
    pub follow_links: bool,
    pub full_path: bool,
    pub show_checksum: bool,
    pub show_stats: bool,
    pub interactive: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_depth: None,
            show_hidden: false,
            show_size: false,
            show_permissions: false,
            show_time: false,
            show_count: false,
            use_colors: true,
            sort_by: SortBy::Name,
            reverse_sort: false,
            filter_extension: None,
            directories_only: false,
            files_only: false,
            min_size: None,
            max_size: None,
            exclude_patterns: Vec::new(),
            include_patterns: None,
            git_ignore: false,
            git_status: false,
            limit: None,
            output_format: OutputFormat::Tree,
            follow_links: false,
            full_path: false,
            show_checksum: false,
            show_stats: false,
            interactive: false,
        }
    }
}

impl Config  {
    pub fn load_from_file() -> create::Result<Option<Self>> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.json("tree-visualizer").join("config.toml");
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(Some(config));
            }
        }
        Ok(None)
    }

    pub fn save_to_file(&self) -> create::Result<()> {
        if let Some(config_dir) = dirs::config_dir() {
            let tree_config_dir = config_dir.join("tree-visualier");
            std::fs::create_dir_all(&tree_config_dir)?;

            let config_path = tree_config_dir.join("config.toml");
            let content = toml::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }
        Ok(())
    }

    pub fn get_extension(&self) -> Vec<String> {
        match &self.filter_extension {
            Some(ext_str) => ext_str
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn matches_size_filter(&self, size:u64) -> bool {
        if let Some(min_size) = self.min_size {
            if size < min_size {
                return false;
            }
        }

        if let Some(max_size) = self.max_size {
            if size > max_size {
                return false;
            }
        }

        true
    }
}