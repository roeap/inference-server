use crate::handler::{InferenceHandler, OnnxInferenceHandler};

use inference_protocol::inference_service_server::InferenceService;
use inference_protocol::*;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct InferenceServiceImpl {}

#[tonic::async_trait]
impl InferenceService for InferenceServiceImpl {
    async fn server_live(
        &self,
        _request: Request<ServerLiveRequest>,
    ) -> std::result::Result<Response<ServerLiveResponse>, Status> {
        Ok(Response::new(ServerLiveResponse { live: true }))
    }

    async fn server_ready(
        &self,
        _request: Request<ServerReadyRequest>,
    ) -> std::result::Result<Response<ServerReadyResponse>, Status> {
        Ok(Response::new(ServerReadyResponse { ready: true }))
    }

    async fn model_ready(
        &self,
        request: Request<ModelReadyRequest>,
    ) -> std::result::Result<Response<ModelReadyResponse>, Status> {
        let _ready_request = request.into_inner();
        todo!()
    }

    async fn server_metadata(
        &self,
        _request: Request<ServerMetadataRequest>,
    ) -> std::result::Result<Response<ServerMetadataResponse>, Status> {
        Ok(Response::new(ServerMetadataResponse {
            name: "inference-server-rs".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            ..Default::default()
        }))
    }

    async fn model_metadata(
        &self,
        request: Request<ModelMetadataRequest>,
    ) -> std::result::Result<Response<ModelMetadataResponse>, Status> {
        let _meta_request = request.into_inner();
        todo!()
    }

    async fn model_infer(
        &self,
        request: Request<ModelInferRequest>,
    ) -> std::result::Result<Response<ModelInferResponse>, Status> {
        let handler = OnnxInferenceHandler {};
        let result = handler.predict(request.into_inner()).await.unwrap();
        Ok(Response::new(result))
    }
}
