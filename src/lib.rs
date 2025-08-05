pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
