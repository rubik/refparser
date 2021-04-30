use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}`")]
    Generic(String),

    #[error("parse error:{0}")]
    ParseError(#[from] serde_json::Error),

    #[error("io error:{0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
