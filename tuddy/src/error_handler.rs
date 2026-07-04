use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Database Error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Service Error: {0}")]
    Service(String),

    #[error("Utility Error: {0}")]
    Utility(String),

    #[error("Config Error: {0}")]
    Config(String),
}
