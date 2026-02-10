//! Error types for sherpa-ncnn

use thiserror::Error;

/// Error type for sherpa-ncnn operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create recognizer: {0}")]
    RecognizerCreation(String),

    #[error("Failed to create stream: {0}")]
    StreamCreation(String),

    #[error("Failed to create VAD: {0}")]
    VadCreation(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Model file not found: {0}")]
    ModelNotFound(String),

    #[error("Null pointer returned from C API")]
    NullPointer,

    #[error("Invalid UTF-8 in result")]
    InvalidUtf8,
}

/// Result type for sherpa-ncnn operations
pub type Result<T> = std::result::Result<T, Error>;
