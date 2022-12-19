//! Core service managing model repositories and instances
mod inference;
mod repository;

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::ModelServerResult;
use crate::handler::OnnxInferenceHandler;

use dashmap::DashMap;
use futures::stream::BoxStream;
use inference_protocol::{ModelIndex, ModelInferRequest, ModelInferResponse};

/// Trait for handling repositories
#[tonic::async_trait]
pub trait RepositoryHandler: std::fmt::Display + Send + Sync + std::fmt::Debug + 'static {
    /// List all models within the repository
    async fn list(&self) -> BoxStream<'_, ModelServerResult<ModelIndex>>;

    /// Find models based on name
    async fn find(&self, name: String) -> BoxStream<'_, ModelServerResult<ModelIndex>>;

    /// Get the infernece handler for the specified model
    ///
    /// If no version is supplied, the choice of version is implementaion specific.
    async fn get(
        &self,
        name: String,
        version: Option<String>,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>>;
}

/// Trait for running inference with a model
#[tonic::async_trait]
pub trait InferenceHandler: std::fmt::Display + Send + Sync + std::fmt::Debug + 'static {
    /// Evaluate the model to get a prediction
    async fn predict(&self, request: ModelInferRequest) -> ModelServerResult<ModelInferResponse>;
}

/// A service implementation handling model repositories and model instances
#[derive(Clone, Default, Debug)]
pub struct ModelService {
    model_handlers: Arc<DashMap<String, Arc<dyn InferenceHandler>>>,
    repositories: Arc<HashMap<String, Arc<dyn RepositoryHandler>>>,
}

impl ModelService {
    /// Create a new [`ModelService`] instance
    pub fn new(model_handlers: Arc<DashMap<String, Arc<dyn InferenceHandler>>>) -> Self {
        Self {
            model_handlers,
            ..Default::default()
        }
    }
}
