mod client;

pub use client::{backoff::BackoffConfig, retry::RetryConfig, ClientOptions};

/// A specialized `Result` for object store-related errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Error in http client: {0}")]
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),
}
