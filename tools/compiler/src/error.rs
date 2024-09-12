use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerializeError(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;
