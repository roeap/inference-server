//! Common error types for the crate
use tonic::Status;
use tract_onnx::prelude::{tract_ndarray::ShapeError, TractError};

/// Shared error type for ModelService
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error occured during model inference
    #[error("Error during model inference: {0}")]
    Inference(#[from] TractError),

    /// Error occuring when handling data conversion
    #[error("Error converting data: {0}")]
    DataConversion(#[from] ShapeError),

    /// Error accesing backend artifact storage
    #[error("Error interacting with objuect store: {0}")]
    Storage(#[from] object_store::Error),
}

/// Shared result type for ModelService
pub type ModelServerResult<T> = std::result::Result<T, Error>;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Inference(err) => Status::internal(err.to_string()),
            Error::DataConversion(err) => Status::failed_precondition(err.to_string()),
            Error::Storage(err) => Status::internal(err.to_string()),
        }
    }
}
