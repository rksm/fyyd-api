use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("client creation error: {0}")]
    ClientCreation(#[source] reqwest::Error),

    #[error("request error: {0}")]
    Request(#[source] reqwest::Error),

    #[error("deserialization error: {0}")]
    Deserialization(#[source] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
