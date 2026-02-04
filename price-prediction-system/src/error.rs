use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Model error: {0}")]
    ModelError(String),
    
    #[error("Dimension mismatch: {0}")]
    DimensionMismatch(String),
}

pub type Result<T> = std::result::Result<T, PredictionError>;
