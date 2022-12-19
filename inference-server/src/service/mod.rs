//! Core service managing model repositories and instances
mod inference;
mod repository;

use std::collections::HashMap;
use std::sync::Arc;

use crate::models::InferenceHandler;
use crate::repositories::RepositoryHandler;

use dashmap::DashMap;

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
}
