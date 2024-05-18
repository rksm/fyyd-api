use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("client creation error: {0}")]
    ClientCreation(#[source] reqwest::Error),

    #[error("request error: {0}")]
    Request(#[source] reqwest::Error),

    // #[error("deserialization error={cause} (payload={payload}))")]
    #[error("deserialization error={cause})")]
    Deserialization {
        #[source]
        cause: serde_json::Error,
        payload: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
