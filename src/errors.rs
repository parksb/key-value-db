use failure::Fail;
use std::io;

/// Errors for KvStore application
#[derive(Fail, Debug)]
pub enum KvStoreError {
    /// io error
    #[fail(display = "io error: {}", _0)]
    Io(#[cause] io::Error),

    /// serde (serialization/deserialization) error
    #[fail(display = "serde error: {}", _0)]
    Serde(#[cause] serde_json::Error),

    /// key not found error
    #[fail(display = "key not found")]
    KeyNotFound,

    /// unknown error
    #[fail(display = "unknown error")]
    Unknown,
}

impl From<io::Error> for KvStoreError {
    fn from(err: io::Error) -> KvStoreError {
        KvStoreError::Io(err)
    }
}

impl From<serde_json::Error> for KvStoreError {
    fn from(err: serde_json::Error) -> KvStoreError {
        KvStoreError::Serde(err)
    }
}

/// Result type override
pub type Result<T> = std::result::Result<T, KvStoreError>;
