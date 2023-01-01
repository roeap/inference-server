//! A model repository base on storage location
use std::collections::HashMap;
use std::sync::Arc;

use super::{ModelReference, RepositoryHandler};
use crate::error::{Error, ModelServerResult};
use crate::models::{InferenceHandler, OnnxInferenceHandler};

use futures::stream::{iter, BoxStream};
use futures::StreamExt;
use inference_protocol::mlflow::{ModelVersion, RegisteredModel};
use object_store::{path::Path, DynObjectStore};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Repository for handling models availabe in a storage location
#[derive(Debug, Clone)]
pub struct StorageRepository {
    store: Arc<DynObjectStore>,
    models: Arc<HashMap<String, HashMap<String, ModelVersion>>>,
}

impl std::fmt::Display for StorageRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StorageRepository()")
    }
}

impl StorageRepository {
    /// Crete a new [`StorageRepository`] instance
    pub async fn try_new(store: Arc<DynObjectStore>) -> ModelServerResult<Self> {
        let repo_path = Path::from("repo.yaml");
        let data = store.get(&repo_path).await?.bytes().await?;
        let info = serde_yaml::from_slice::<CatalogInfo>(&data)?;
        let mut models = HashMap::new();
        for model in info.models {
            let mut versions = HashMap::new();
            for version in model.latest_versions {
                versions.insert(version.version.clone().unwrap_or_default(), version);
            }
            let model_name = model.name.unwrap_or_default();
            info!(
                "Found {} version(s) for model '{}'.",
                versions.len(),
                model_name
            );
            models.insert(model_name, versions);
        }
        Ok(Self {
            store,
            models: Arc::new(models),
        })
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

    async fn load(&self, name: &String) -> ModelServerResult<()> {
        if self.models.contains_key(name) {
            Ok(())
        } else {
            Err(Error::ModelNotFound(name.clone()))
        }
    }

    async fn contains_model(&self, name: &String) -> bool {
        self.models.contains_key(name)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    async fn get(
        &self,
        name: &String,
        version: &String,
    ) -> ModelServerResult<Arc<dyn InferenceHandler>> {
        let info = self
            .models
            .get(name)
            .ok_or(Error::ModelNotFound(name.clone()))?
            .get(version)
            .ok_or(Error::ModelNotFound(version.clone()))?;
        let path = Path::from(info.source());
        let data = self.store.get(&path).await?.bytes().await?;
        Ok(Arc::new(OnnxInferenceHandler::try_new(data).await?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogInfo {
    models: Vec<RegisteredModel>,
}
