// error.rs
//
// This module defines custom error types for both the Ollama client
// and the project indexer functionality.

use thiserror::Error;

/// Custom error type for the project indexer
///
/// This enum represents all possible errors that can occur during the
/// project indexing process. It includes errors from:
/// - File system operations
/// - Git commands
/// - Ollama API calls
/// - JSON serialization/deserialization
/// - URL parsing
/// - Input validation
///
/// # Examples
///
/// ```rust
/// use projets_indexer::error::OllamaError;
///
/// let error = OllamaError::ValidationError("Invalid input".to_string());
/// println!("Error: {}", error);
/// ```
#[derive(Error, Debug)]
pub enum OllamaError {
    /// Error occurred during an HTTP request
    #[error("Request error: {0}")]
    RequestError(String),

    /// Error occurred while parsing a URL
    #[error("URL error: {0}")]
    UrlError(String),

    /// Error occurred during JSON serialization/deserialization
    #[error("JSON error: {0}")]
    JsonError(String),

    /// Error returned by the Ollama API
    #[error("API error: {message}{}", status_code.map(|code| format!(" (Status code: {code})")).unwrap_or_default())]
    ApiError {
        /// The error message from the API
        message: String,
        /// Optional HTTP status code
        status_code: Option<u16>,
    },

    /// Error occurred during input validation
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Error occurred during I/O operations
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error occurred during directory traversal
    #[error("Directory traversal error: {0}")]
    WalkdirError(#[from] walkdir::Error),

    /// Error occurred during tracing setup
    #[error("Tracing error: {0}")]
    TracingError(String),

    /// Error occurred during task join
    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

/// Type alias for Result using OllamaError
///
/// This type alias simplifies error handling by using our custom error type
/// as the error variant in Result types throughout the codebase.
///
/// # Examples
///
/// ```rust
/// use projets_indexer::error::Result;
///
/// fn my_function() -> Result<()> {
///     // Function implementation
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, OllamaError>;
