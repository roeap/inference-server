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
mod error;
mod models;
mod repositories;
mod service;

use std::{collections::HashMap, sync::Arc};

use crate::repositories::{RepositoryHandler, StorageRepository};
use crate::service::ModelService;

use object_store::local::LocalFileSystem;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{self, layer::SubscriberExt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(format!(
            "{},h2=off",
            "debug"
        )))
        .with(stdout_log)
        .try_init()?;

    let store = Arc::new(LocalFileSystem::new_with_prefix(
        "inference-server/tests/data",
    )?);
    let repository = StorageRepository::try_new(store).await?;
    let mut repositories: HashMap<String, Arc<dyn RepositoryHandler>> = HashMap::new();
    repositories.insert("default".to_string(), Arc::new(repository));

    let (infer_svc, repo_svc) =
        ModelService::new(Some(Arc::new(repositories)), None).into_services();

    info!("Listening on 0.0.0.0:50051");

    Server::builder()
        .add_service(infer_svc)
        .add_service(repo_svc)
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
