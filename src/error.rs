use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("SerdeJson")]
    SerdeJson(#[from] serde_json::Error),

    #[error("I/O")]
    Io(#[from] std::io::Error),
}
