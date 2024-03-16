#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("{0}")]
    // Message(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
