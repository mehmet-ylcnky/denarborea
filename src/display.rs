use crate::{Config, OutputFormat, Result, git::GitInfo, utils};
use colored::*;
use crossterm::style::Stylize;
use humansize::{DECIMAL, format_size};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub is_executable: bool,
    pub is_symlink: bool,
    pub modified_time: Option<SystemTime>,
    pub permissions: Option<u32>,
    pub checksum: Option<String>,
    pub file_count:Option<usize>,
    pub dir_count: Option<usize>,
}

impl FileInfo {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let name = if let Some(file_name) = path.file_name() {
            file_name.to_string_lossy().to_string()
        } else {
            path.to_string_lossy().to_string() 
        };

        let modified_time = metadata.modified().ok();
        let permissions = Self::get_permissions(&metadata);
        
        let (file_count, dir_count) = if metadata.is_dir() {
            let (files, dirs) = utils::count_files_in_dir(path);
            (Some(files), Some(dirs))
        } else {
            (None, None)
        };

        Ok(Self {
            path: path.to_path_buf(),
            name,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_executable: Self::is_executable(&metadata),
            is_symlink: metadata.file_type().is_symlink(),
            modified_time,
            permissions,
            checksum: None, //will be calculated on demand
            file_count,
            dir_count,
        })
    }

    #[cfg(unix)]
    fn is_executable(metadata: &fs::Metadata) -> bool {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    fn is_executable(metadata: &fs::Metadata) -> bool {
        false
    }

    #[cfg(unix)]
    fn get_permissions(metadata: &fs::Metadata) -> Option<u32> {
        use std::os::unix::fs::PermissionsExt;
        Some(metadata.permissions().mode())
    }

    #[cfg(not(unix))]
    fn get_permissions(metadata: &fs::Metadata) -> Option<u32> {
        None
    }

    pub fn calculate_checksum(&mut self) -> Option<&String> {
        if self.checksum.is_none() && !self.is_dir {
            if let Ok(hash) = utils::calculate_md5(&self.path) {
                self.checksum = Some(hash);
            }
        }
        self.checksum.as_ref()
    }
}

pub struct TreeDisplay {
    config: Config,
    git_info: Option<GitInfo>,
}

impl TreeDisplay {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            git_info: None,
        }
    }

    pub fn with_git_info(mut self, git_info: GitInfo) -> Self {
        self.git_info = Some(git_info);
        self
    }

    pub fn format_path(&self, path: &Path) ->Result<String> {
        let display_path = if self.config.full_path {
            path.canonicalize()?.display().to_string()
        } else {
            path.display().to_string()
        };

        if self.config.use_colors {
            Ok(display_path.blue().bold().to_string())
        } else {
            Ok(display_path)
        }
    }

    pub fn format_file_info(&self, info: &FileInfo) -> Result<String> {
        match self.config.output_format {
            OutputFormat::Tree => self.format_tree_line(info),
            OutputFormat::Json => self.format_json_line(info),
            OutputFormat::Csv => self.format_csv_line(info),
            OutputFormat::Markdown => self.format_markdown_line(info),
            OutputFormat::Xml => self.format_xml_line(info),
        }
    }

    fn format_tree_line(&self, info: &FileInfo) -> Result<String> {
        let mut output = String::new();

        // git status indicator
        if self.config.git_status {
            if let Some(ref git_info) = self.git_info {
                let abs_path = info
                    .path
                    .canonicalize()
                    .unwrap_or_else(|_| info.path.clone());

                let status = git_info.get_status(&abs_path);

                let status_str = if self.config.use_colors {
                    status.symbol().color(status.color()).to_string()
                } else {
                    status.symbol().to_string()
                };

                output.push_str(&format!("{}", status_str));
            }
        }

        let formatted_name = if self.config.use_colors {
            self.colorize_name(info)
        } else {
            info.name.clone()
        };

        output.push_str(&formatted_name);

        // add file/directory counts
        if self.config.show_count && info.is_dir {
            if let (Some(files), Some(dirs)) = (info.file_count, info.dir_count) {
                let count_str = format!("({} files, {} dirs)", files, dirs);
                if self.config.use_colors {
                    output.push_str(&count_str.dimmed().to_string());
                } else {
                    output.push_str(&count_str);
                }
            }
        }

        if self.config.show_size && !info.is_dir {
            let size_str = format!("[{}]", format_size(info.size, DECIMAL));
            if self.config.use_colors {
                output.push_str(&size_str.dimmed().to_string());
            } else {
                output.push_str(&size_str);
            }
        }

        if self.config.show_permissions {
            if let Some(mode) = info.permissions {
                let perm_str = format!("({})", utils::format_permissions(mode));
                if self.config.use_colors {
                    output.push_str(&perm_str.cyan().to_string());
                } else {
                    output.push_str(&perm_str);
                }
            }
        }

        // Add time if requested
        if self.config.show_time {
            if let Some(time) = info.modified_time {
                let time_str = format!("[{}]", utils::format_time(time));
                if self.config.use_colors {
                    output.push_str(&time_str.dimmed().to_string());
                } else {
                    output.push_str(&time_str);
                }
            }
        }

        // Add checksum if requested
        if self.config.show_checksum && !info.is_dir {
            if let Some(checksum) = &info.checksum {
                let checksum_str = format!("<{}>", &checksum[..8]);
                if self.config.use_colors {
                    output.push_str(&checksum_str.magenta().to_string());
                } else {
                    output.push_str(&checksum_str);
                }
            }
        }

        // Add symlink if requested
        if info.is_symlink {
            if let Ok(target) = fs::read_link(&info.path) {
                let symlink_str = format!("-> {}", target.display());
                if self.config.use_colors {
                    output.push_str(&symlink_str.cyan().to_string());
                } else {
                    output.push_str(&symlink_str);
                }
            }
        }
        Ok(output)
    }

    fn format_json_line(&self, info: &FileInfo) -> Result<String> {
        use serde_json::json;

        let json_obj = json!({
            "name": info.name,
            "path": info.path,
            "size": info.size,
            "is_dir": info.is_dir,
            "is_executable": info.is_executable,
            "is_symlink": info.is_symlink,
            "modified_time": info.modified_time.map(|t| utils::format_time(t)),
            "permissions": info.permissions.map(|p| utils::format_permissions(p)),
            "checksum": info.checksum,
            "file_count": info.file_count,
            "dir_count": info.dir_count,
        });

        Ok(json_obj.to_string())
    }

    fn format_csv_line(&self, info: &FileInfo) -> Result<String> {
        let path = info.path.display().to_string();
        let size = info.size.to_string();
        let is_dir = info.is_dir.to_string();
        let modified = info
            .modified_time
            .map(|t| utils::format_time(t))
            .unwrap_or_default();
        let permissions = info
            .permissions
            .map(|p| utils::format_permissions(p))
            .unwrap_or_default();

        Ok(format!(
            "{},{},{},{},{},{}",
            info.name, path, size, is_dir, modified, permissions
        ))
    }

    fn format_markdown_line(&self, info: &FileInfo) -> Result<String> {
        let icon = if info.is_dir {"--"} else {"xx"};
        let size = if info.is_dir {
            "-".to_string()
        } else {
            format_size(info.size, DECIMAL)
        };
        let modified = info
            .modified_time
            .map(|t| utils::format_time(t))
            .unwrap_or_default();

        Ok(format!(
            "| {} {} | {} | {} |",
            icon, info.name, size, modified
        ))
    }

    fn format_xml_line(&self, info: &FileInfo) -> Result<String> {
        let modified = info
            .modified_time
            .map(|t| utils::format_time(t))
            .unwrap_or_default();

        Ok(format!(
            "<file name=\"{}\" path=\"{}\" size=\"{}\" is_dir=\"{}\" modified=\"{}\" />",
            info.name,
            info.path.display(),
            info.size,
            info.is_dir,
            modified
        ))
    }

    fn colorize_name(&self, info: &FileInfo) -> String {
        if info.is_dir {
            info.name.blue().to_string()
        } else if info.is_executable {
            info.name.green().bold().to_string()
        } else if info.is_symlink {
            info.name.cyan().to_string()
        } else {
            match info.path.extension().and_then(|s| s.to_str()) {
                Some("rs") => info.name.red().to_string(),
                Some("py") => info.name.yellow().to_string(),
                Some("js") | Some("ts") => info.name.bright_yellow().to_string(),
                Some("html") | Some("css") => info.name.magenta().to_string(),
                Some("md") => info.name.bright_blue().to_string(),
                Some("json") | Some("yaml") | Some("yml") | Some("toml") => info.name.bright_green().to_string(),
                Some("jpg") | Some("png") | Some("gif") | Some("svg") => {
                    info.name.bright_magenta().to_string()
                }
                Some("exe") | Some("bin") => info.name.green().bold().to_string(),
                Some("zip") | Some("tar") | Some("gz") | Some("7z") => {
                    info.name.red().bold().to_string()
                }
                _ => info.name.normal().to_string()
            }
        }
    }
}

