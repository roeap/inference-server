use tract_onnx::{prelude::TractError, tract_hir::tract_ndarray::ShapeError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error during model inference: {0}")]
    Inference(#[from] TractError),

    #[error("Error converting data: {0}")]
    DataConversion(#[from] ShapeError),
}

pub type Result<T> = std::result::Result<T, Error>;
