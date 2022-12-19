//! Model repository implementations
use std::sync::Arc;

use crate::error::ModelServerResult;
use crate::models::{InferenceHandler, OnnxInferenceHandler};

use futures::stream::BoxStream;
use inference_protocol::ModelIndex;

/// Trait for handling repositories
#[tonic::async_trait]
pub trait RepositoryHandler: std::fmt::Display + Send + Sync + std::fmt::Debug + 'static {
    /// List all models within the repository
    async fn list(&self) -> BoxStream<'_, ModelServerResult<ModelIndex>>;

    /// Find models based on name
    async fn find(&self, name: &String) -> BoxStream<'_, ModelServerResult<ModelIndex>>;

    /// Get the infernece handler for the specified model
    ///
    /// If no version is supplied, the choice of version is implementaion specific.
    async fn get(
        &self,
        name: &String,
        version: Option<String>,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>>;
}

#[derive(Debug, Clone)]
pub struct StorageRepository {}

impl std::fmt::Display for StorageRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StorageRepository()")
    }
}

#[tonic::async_trait]
impl RepositoryHandler for StorageRepository {
    async fn list(&self) -> BoxStream<'_, ModelServerResult<ModelIndex>> {
        todo!()
    }

    async fn find(&self, _name: &String) -> BoxStream<'_, ModelServerResult<ModelIndex>> {
        todo!()
    }

    async fn get(
        &self,
        _name: &String,
        _version: Option<String>,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>> {
        Ok(Arc::new(
            OnnxInferenceHandler::try_new("inference-server/tests/data/model.onnx").await?,
        ))
    }
}
