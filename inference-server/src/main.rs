pub mod error;
pub mod handler;
pub mod inference;
pub mod repository;

use inference::InferenceServiceImpl;
use inference_protocol::inference_service_server::InferenceServiceServer;
use inference_protocol::model_repository_service_server::ModelRepositoryServiceServer;
use repository::ModelRepositoryServiceImpl;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{self, layer::SubscriberExt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(format!(
            "{},h2=off,sqlparser=off,datafusion_optimizer=off,datafusion::physical_plan::planner=off",
            "debug"
        )))
        .with(stdout_log)
        .try_init()?;

    let infer_svc = InferenceServiceServer::new(InferenceServiceImpl {});
    let repo_svc = ModelRepositoryServiceServer::new(ModelRepositoryServiceImpl {});

    info!("Listening on 0.0.0.0:50051");

    Server::builder()
        .add_service(infer_svc)
        .add_service(repo_svc)
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
