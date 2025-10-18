pub mod utils;
pub mod config;
pub mod git;
pub mod display;

pub use config::{Config, OutputFormat, SortBy};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
