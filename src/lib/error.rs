// Import the `thiserror` crate for defining custom error types
use thiserror::Error;

// Define a custom error enum for handling different types of errors in the application
#[derive(Error, Debug)]
pub enum CustomError {
    // Error variant for when the maximum number of retries is reached
    #[error("Max retries reached")]
    MaxRetriesReached,
    
    // Error variant for handling errors from the reqwest HTTP client library
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    // Error variant for handling timeout errors from the tokio asynchronous runtime
    #[error("Timeout error: {0}")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
}
