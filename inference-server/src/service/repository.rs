use super::ModelService;
use inference_protocol::model_repository_service_server::ModelRepositoryService;
use inference_protocol::*;
use tonic::{Request, Response, Status};

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
        _request: Request<RepositoryModelLoadRequest>,
    ) -> Result<Response<RepositoryModelLoadResponse>, Status> {
        todo!()
    }
    async fn repository_model_unload(
        &self,
        _request: Request<RepositoryModelUnloadRequest>,
    ) -> Result<Response<RepositoryModelUnloadResponse>, Status> {
        todo!()
    }
}
