//! Core service managing model repositories and instances
mod inference;
mod repository;

use std::collections::HashMap;
use std::sync::Arc;

use crate::models::InferenceHandler;
use crate::repositories::RepositoryHandler;

use dashmap::DashMap;
pub use inference_protocol::inference_service_server::InferenceServiceServer;
pub use inference_protocol::model_repository_service_server::ModelRepositoryServiceServer;

const DEFAULT_REPOSITORY_NAME: &str = "default";

/// A service implementation handling model repositories and model instances
#[derive(Clone, Default, Debug)]
pub struct ModelService {
    instances: Arc<DashMap<String, Arc<dyn InferenceHandler>>>,
    repositories: Arc<HashMap<String, Arc<dyn RepositoryHandler>>>,
}

impl ModelService {
    /// Create a new [`ModelService`] instance
    pub fn new(
        repositories: Option<Arc<HashMap<String, Arc<dyn RepositoryHandler>>>>,
        instances: Option<Arc<DashMap<String, Arc<dyn InferenceHandler>>>>,
    ) -> Self {
        Self {
            instances: instances.unwrap_or_default(),
            repositories: repositories.unwrap_or_default(),
        }
    }

    /// Consume self, to add grpc services that can be added to a server
    pub fn into_services(
        self,
    ) -> (
        InferenceServiceServer<ModelService>,
        ModelRepositoryServiceServer<ModelService>,
    ) {
        let inference = InferenceServiceServer::new(self.clone());
        let repository = ModelRepositoryServiceServer::new(self);
        (inference, repository)
    }

    fn get_repository(&self, name: &String) -> Option<&Arc<dyn RepositoryHandler>> {
        let default = String::from(DEFAULT_REPOSITORY_NAME);
        let repository_name = if name.is_empty() { &default } else { name };
        self.repositories.get(repository_name)
    }
}
