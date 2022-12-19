//! Model repository implementations
use std::sync::Arc;

use crate::error::ModelServerResult;
use crate::models::{InferenceHandler, OnnxInferenceHandler};

use futures::stream::{iter, BoxStream};
use futures::StreamExt;
use object_store::{path::Path, DynObjectStore};

/// A reference to a model within a repository
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct ModelReference {
    /// The name of the model
    pub name: String,
    /// String uniquely identifying the version of the model
    pub version: String,
}

/// Trait for handling repositories
#[tonic::async_trait]
pub trait RepositoryHandler: std::fmt::Display + Send + Sync + std::fmt::Debug + 'static {
    /// List all models within the repository
    async fn list(&self) -> BoxStream<'_, ModelServerResult<ModelReference>>;

    /// Find models based on name
    async fn find(&self, name: &String) -> BoxStream<'_, ModelServerResult<ModelReference>>;

    /// Get the infernece handler for the specified model
    ///
    /// If no version is supplied, the choice of version is implementaion specific.
    async fn get(
        &self,
        name: String,
        version: String,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>>;
}

/// Repository for handling models availabe in a storage location
#[derive(Debug, Clone)]
pub struct StorageRepository {
    store: Arc<DynObjectStore>,
}

impl std::fmt::Display for StorageRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StorageRepository()")
    }
}

impl StorageRepository {
    /// Crete a new [`StorageRepository`] instance
    pub fn new(store: Arc<DynObjectStore>) -> Self {
        Self { store }
    }
}

#[tonic::async_trait]
impl RepositoryHandler for StorageRepository {
    async fn list(&self) -> BoxStream<'_, ModelServerResult<ModelReference>> {
        iter(vec![
            Ok(ModelReference {
                name: "onnx".into(),
                ..Default::default()
            }),
            Ok(ModelReference {
                name: "sample".into(),
                ..Default::default()
            }),
        ])
        .boxed()
    }

    async fn find(&self, _name: &String) -> BoxStream<'_, ModelServerResult<ModelReference>> {
        todo!()
    }

    async fn get(
        &self,
        _name: String,
        _version: String,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>> {
        let data = self
            .store
            .get(&Path::from("model.onnx"))
            .await?
            .bytes()
            .await?;
        Ok(Arc::new(OnnxInferenceHandler::try_new(data).await?))
    }
}
