use std::collections::HashMap;
use std::path::Path;

#[cfg(feature = "git")]
use git2::{Repository, Status};

#[derive(Debug, Clone, Copy)]
pub enum GitStatus {
    Untracked,
    Modified,
    Added,
    Deleted,
    Renamed,
    Ignored,
    Clean,
}

impl GitStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            GitStatus::Untracked => "??",
            GitStatus::Modified => "M ",
            GitStatus::Added => "A ",
            GitStatus::Deleted => "D ",
            GitStatus::Renamed => "R ",
            GitStatus::Ignored => "I ",
            GitStatus::Clean => "  ",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            GitStatus::Untracked => colored::Color::Red,
            GitStatus::Modified => colored::Color::Yellow,
            GitStatus::Added => colored::Color::Green,
            GitStatus::Deleted => colored::Color::Red,
            GitStatus::Renamed => colored::Color::Blue,
            GitStatus::Ignored => colored::Color::BrightBlack,
            GitStatus::Clean => colored::Color::White,
        }
    }
}

#[derive(Clone)]
pub struct GitInfo {
    #[cfg(feature = "git")]
    repo: Option<std::sync::Arc<Repository>>,
    status_cache: HashMap<std::path::PathBuf, GitStatus>,
}

impl GitInfo {
    pub fn new(root_path: &Path) -> Self {
        #[cfg(feature = "git")]
        {
            let repo = Repository::discover(root_path).ok();
            let mut git_info = Self {
                repo: repo.map(std::sync::Arc::new),
                status_cache: HashMap::new(),
            };

            get_info.load_status();
            git_info
        }

        #[cfg(not(feature = "git"))]
        {
            Self {
                status_cache: HashMap::new(),
            }
        }
    }
}