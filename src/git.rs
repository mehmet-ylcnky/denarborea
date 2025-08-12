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