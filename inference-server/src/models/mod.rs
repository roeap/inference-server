//! Model runtimes
pub mod onnx;

use crate::error::ModelServerResult;

use inference_protocol::{ModelInferRequest, ModelInferResponse};

pub use onnx::OnnxInferenceHandler;

/// Trait for running inference with a model
#[tonic::async_trait]
pub trait InferenceHandler: std::fmt::Display + Send + Sync + std::fmt::Debug {
    /// Evaluate the model to get a prediction
    async fn predict(&self, request: ModelInferRequest) -> ModelServerResult<ModelInferResponse>;
}
