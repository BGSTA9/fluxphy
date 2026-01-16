//! Error types for FluxPhy

use thiserror::Error;

/// Errors that can occur during flux transfer operations
#[derive(Error, Debug)]
pub enum FluxError {
    #[error("Source file not found: {0}")]
    SourceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Disk full: {0}")]
    DiskFull(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Destination is a directory, use --recursive")]
    DestinationIsDirectory,

    #[error("Transfer cancelled by user")]
    TransferCancelled,

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Config file not found")]
    ConfigNotFound,

    #[error("Source is a directory but --recursive was not specified")]
    RecursiveRequired,

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
}

/// Result type alias for FluxPhy operations
pub type FluxResult<T> = Result<T, FluxError>;
