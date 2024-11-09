use thiserror::Error;


#[derive(Error,Debug)]
pub enum LabError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),

    #[error("Failed to load .env file: {0}")]
    DotEnvError(#[from] dotenv::Error),

    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Tool execution failed: {0}")]
    ToolError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

}