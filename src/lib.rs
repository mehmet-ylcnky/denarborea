pub mod config;
pub mod display;
pub mod git;
pub mod stats;
pub mod tree;
pub mod utils;
pub mod viewer;

pub use config::{Config, OutputFormat, SortBy};
pub use display::{FileInfo, TreeDisplay};
pub use stats::TreeStats;
pub use tree::TreeVisualizer;
pub use viewer::{FileViewer, ViewerFormat, ViewerStrategy};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
