use super::ModelService;
use crate::error::Error;

use inference_protocol::model_repository_service_server::ModelRepositoryService;
use inference_protocol::*;
use tonic::{Request, Response, Status};
use tracing::info;

#[tonic::async_trait]
impl ModelRepositoryService for ModelService {
    async fn repository_index(
        &self,
        _request: Request<RepositoryIndexRequest>,
    ) -> Result<Response<RepositoryIndexResponse>, Status> {
        todo!()
    }
    async fn repository_model_load(
        &self,
        request: Request<RepositoryModelLoadRequest>,
    ) -> Result<Response<RepositoryModelLoadResponse>, Status> {
        let load_request = request.into_inner();

        info!(
            "loading model '{}' from repository '{}'",
            load_request.model_name, load_request.repository_name
        );

        let repository = self
            .get_repository(&load_request.repository_name)
            .ok_or(Error::RepositoryNotFound(load_request.repository_name))?;
        let model = repository
            .get(load_request.model_name.clone(), Default::default())
            .await?;
        self.instances.insert(load_request.model_name, model);

        Ok(Response::new(RepositoryModelLoadResponse {}))
    }
    async fn repository_model_unload(
        &self,
        _request: Request<RepositoryModelUnloadRequest>,
    ) -> Result<Response<RepositoryModelUnloadResponse>, Status> {
        todo!()
    }
}
