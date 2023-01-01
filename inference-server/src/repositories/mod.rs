//! Model repository implementations
use std::sync::Arc;

use crate::error::ModelServerResult;
use crate::models::InferenceHandler;

use futures::stream::BoxStream;

pub mod storage;

pub use self::storage::StorageRepository;

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

    /// Load meta data for all model versions into memory
    async fn load(&self, name: &String) -> ModelServerResult<()>;

    /// Check wether model with given name is defined / available in the repository
    async fn contains_model(&self, name: &String) -> bool;

    /// Get the infernece handler for the specified model
    ///
    /// If no version is supplied, the choice of version is implementaion specific.
    async fn get(
        &self,
        name: &String,
        version: &String,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>>;
}
