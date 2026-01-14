use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image error: {0}")]
    ImageError(String),
    
    #[error("Conversion failed: {0}")]
    #[allow(dead_code)]
    ConversionFailed(String),
}
