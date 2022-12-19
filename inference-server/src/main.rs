#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]
//! A high performance inference server compliant eith V2 inference API
pub mod error;
pub mod models;
pub mod repositories;
pub mod service;

use std::{collections::HashMap, sync::Arc};

use repositories::{RepositoryHandler, StorageRepository};

use inference_protocol::inference_service_server::InferenceServiceServer;
use inference_protocol::model_repository_service_server::ModelRepositoryServiceServer;
use service::ModelService;
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

    let mut repositories: HashMap<String, Arc<dyn RepositoryHandler>> = HashMap::new();
    repositories.insert("default".to_string(), Arc::new(StorageRepository {}));

    let service = ModelService::new(Some(Arc::new(repositories)), None);
    let repo_svc = ModelRepositoryServiceServer::new(service.clone());
    let infer_svc = InferenceServiceServer::new(service);

    info!("Listening on 0.0.0.0:50051");

    Server::builder()
        .add_service(infer_svc)
        .add_service(repo_svc)
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
