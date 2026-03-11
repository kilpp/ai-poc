use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Safety violation: {0}")]
    SafetyViolation(String),

    #[error("Budget exhausted: {0}")]
    BudgetExhausted(String),

    #[error("Extraction error: {0}")]
    Extraction(String),
}

pub type Result<T> = std::result::Result<T, AgentError>;
