use svg_hush::FError;
use thiserror::Error;
use tokio::task;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    FilterError(#[from] FError),

    #[error("{0}")]
    JoinError(#[from] task::JoinError),

    #[error("{0}")]
    RequestError(#[from] reqwest::Error),

    #[error("{0}")]
    SerializeError(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;
