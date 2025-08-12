pub mod utils;
pub mod config;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
