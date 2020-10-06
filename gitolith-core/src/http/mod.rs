pub mod router;
pub mod handlers;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
