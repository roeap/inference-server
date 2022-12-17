mod gen {
    include!("gen/inference.rs");
    include!("gen/inference.model_repository.rs");
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
