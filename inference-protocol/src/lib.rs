mod gen {
    include!("gen/inference.rs");
    include!("gen/inference.model_repository.rs");
    pub mod mlflow {
        include!("gen/mlflow.rs");
    }
}

pub mod model_repository_service_server {
    use super::gen;
    pub use gen::model_repository_service_server::ModelRepositoryService;
    pub use gen::model_repository_service_server::ModelRepositoryServiceServer;
}

pub mod model_repository_service_client {
    use super::gen;
    pub use gen::model_repository_service_client::ModelRepositoryServiceClient;
}

pub mod inference_service_server {
    use super::gen;
    pub use gen::grpc_inference_service_server::GrpcInferenceService as InferenceService;
    pub use gen::grpc_inference_service_server::GrpcInferenceServiceServer as InferenceServiceServer;
}

pub mod inference_service_client {
    use super::gen;
    pub use gen::grpc_inference_service_client::GrpcInferenceServiceClient as InferenceServiceClient;
}

pub mod mlflow {
    use super::gen;
    pub use gen::mlflow::{ModelVersion, RegisteredModel};
}

// model repository
pub use gen::repository_index_response::ModelIndex;
pub use gen::RepositoryIndexRequest;
pub use gen::RepositoryIndexResponse;
pub use gen::RepositoryModelLoadRequest;
pub use gen::RepositoryModelLoadResponse;
pub use gen::RepositoryModelUnloadRequest;
pub use gen::RepositoryModelUnloadResponse;

// model inference - status
pub use gen::ModelReadyRequest;
pub use gen::ModelReadyResponse;
pub use gen::ServerLiveRequest;
pub use gen::ServerLiveResponse;
pub use gen::ServerReadyRequest;
pub use gen::ServerReadyResponse;

// model inference - metadata
pub use gen::model_metadata_response::TensorMetadata;
pub use gen::ModelMetadataRequest;
pub use gen::ModelMetadataResponse;
pub use gen::ServerMetadataRequest;
pub use gen::ServerMetadataResponse;

// model inference - inference
pub use gen::infer_parameter::ParameterChoice;
pub use gen::model_infer_request::InferInputTensor;
pub use gen::model_infer_request::InferRequestedOutputTensor;
pub use gen::model_infer_response::InferOutputTensor;
pub use gen::InferParameter;
pub use gen::InferTensorContents;
pub use gen::ModelInferRequest;
pub use gen::ModelInferResponse;

#[cfg(feature = "onnx")]
mod tract;

#[cfg(feature = "onnx")]
pub use crate::tract::*;
