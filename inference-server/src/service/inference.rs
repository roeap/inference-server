use std::sync::Arc;

use super::ModelService;
use crate::error::{Error, ModelServerResult};
use crate::models::InferenceHandler;

use dashmap::mapref::one::Ref;
use inference_protocol::inference_service_server::InferenceService;
use inference_protocol::*;
use tonic::{Request, Response, Status};

fn model_key(name: &String, version: &String) -> String {
    format!("{name}::{version}")
}

impl ModelService {
    async fn get_model_instance(
        &self,
        name: &String,
        version: &String,
    ) -> ModelServerResult<Ref<'_, String, Arc<dyn InferenceHandler>>> {
        if let Some(inst) = self.instances.get(&model_key(name, version)) {
            return Ok(inst);
        }
        for (_, repo) in self.repositories.iter() {
            if repo.contains_model(name).await {
                let handler = repo.get(name, version).await?;
                self.instances.insert(model_key(name, version), handler);
                return Ok(self.instances.get(&model_key(name, version)).unwrap());
            }
        }
        Err(Error::ModelNotFound(name.clone()))
    }
}

#[tonic::async_trait]
impl InferenceService for ModelService {
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
        let infer_request = request.into_inner();
        let handler = self
            .get_model_instance(&infer_request.model_name, &infer_request.model_version)
            .await?;
        let result = handler.predict(infer_request).await?;
        Ok(Response::new(result))
    }
}
