mod client;
mod gen {
    include!("gen/mlflow.rs");
}

use std::collections::HashMap;

use crate::client::retry::RetryExt;
pub use crate::gen::create_experiment::Response as CreateExperimentResponse;
pub use crate::gen::create_registered_model::Response as CreateRegisteredModelResponse;
pub use crate::gen::delete_experiment::Response as DeleteExperimentResponse;
pub use crate::gen::get_experiment::Response as GetExperimentResponse;
pub use crate::gen::get_experiment_by_name::Response as GetExperimentByNameResponse;
pub use crate::gen::get_registered_model::Response as GetRegisteredModelResponse;
pub use crate::gen::rename_registered_model::Response as RenameRegisteredModelResponse;
pub use crate::gen::restore_experiment::Response as RestoreExperimentResponse;
pub use crate::gen::search_experiments::Response as SearchExperimentsResponse;
pub use crate::gen::update_experiment::Response as UpdateExperimentResponse;
pub use crate::gen::update_registered_model::Response as UpdateRegisteredModelResponse;
pub use crate::gen::{
    CreateExperiment, CreateRegisteredModel, DeleteExperiment, DeleteRegisteredModel,
    ExperimentTag, GetExperiment, GetExperimentByName, GetRegisteredModel, RegisteredModelTag,
    RenameRegisteredModel, RestoreExperiment, SearchExperiments, UpdateRegisteredModel,
};
use bytes::Bytes;
use gen::UpdateExperiment;
use reqwest::{
    header::{HeaderValue, CONTENT_LENGTH},
    Client as ReqwestClient, Method, Response,
};
use serde::Serialize;
use url::Url;

pub use client::{backoff::BackoffConfig, retry::RetryConfig, ClientOptions};

/// A specialized `Result` for mlflow-client related errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Error in http client: {0}")]
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Error parsing url: {0}")]
    Url(#[from] url::ParseError),

    #[error("service did not respond: {0}")]
    Retry(#[from] crate::client::retry::Error),

    #[error("Error serializaing request/response data: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Missing required configuration: {0}")]
    MissingConfig(String),

    #[error("MError during service request: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Default)]
pub struct MlFlowClientBuilder {
    host: Option<String>,
    retry_config: RetryConfig,
    client_options: ClientOptions,
}

impl MlFlowClientBuilder {
    /// Create a new [`MlFlowClientBuilder`] with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Load values confgured in teh environment
    ///
    /// - `MLFLOW_TRACKING_URI`: URI for mlflow tracking server
    pub fn from_env() -> Self {
        let mut builder = Self::default();

        if let Ok(host) = std::env::var("MLFLOW_TRACKING_URI") {
            builder.host = Some(host);
        }

        builder
    }

    /// Set the retry configuration
    pub fn with_service_url(mut self, service_url: impl Into<String>) -> Self {
        self.host = Some(service_url.into());
        self
    }

    /// Sets what protocol is allowed. If `allow_http` is :
    /// * false (default):  Only HTTPS are allowed
    /// * true:  HTTP and HTTPS are allowed
    pub fn with_allow_http(mut self, allow_http: bool) -> Self {
        self.client_options = self.client_options.with_allow_http(allow_http);
        self
    }

    /// Set the retry configuration
    pub fn with_retry(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Set the proxy_url to be used by the underlying client
    pub fn with_proxy_url(mut self, proxy_url: impl Into<String>) -> Self {
        self.client_options = self.client_options.with_proxy_url(proxy_url);
        self
    }

    /// Sets the client options, overriding any already set
    pub fn with_client_options(mut self, options: ClientOptions) -> Self {
        self.client_options = options;
        self
    }

    pub fn build(self) -> Result<MlflowClient> {
        let Self {
            retry_config,
            host,
            client_options,
        } = self;

        let host = host.ok_or(Error::MissingConfig("host".into()))?;
        let host_url = Url::parse(&host)?;

        let config = MlflowConfig {
            retry_config,
            service: host_url,
            client_options,
        };

        Ok(MlflowClient::try_new(config)?)
    }
}

/// Configuration for [`MlflowClient`]
#[derive(Debug, Clone)]
pub struct MlflowConfig {
    // pub credentials: CredentialProvider,
    pub retry_config: RetryConfig,
    pub service: Url,
    pub client_options: ClientOptions,
}

#[derive(Debug, Clone)]
pub struct MlflowClient {
    config: MlflowConfig,
    client: ReqwestClient,
}

impl MlflowClient {
    /// Create a new instance of [`MlflowClient`]
    pub fn try_new(config: MlflowConfig) -> Result<Self> {
        let client = config.client_options.client()?;
        Ok(Self { config, client })
    }

    async fn request<T: Serialize + std::fmt::Debug + ?Sized + Sync>(
        &self,
        method: Method,
        path: impl AsRef<str>,
        body: &T,
    ) -> Result<Response> {
        let url = self
            .config
            .service
            .join("api/2.0/mlflow/")?
            .join(path.as_ref())?;
        let bytes = Bytes::from(serde_json::to_vec(body)?);
        let builder = self
            .client
            .request(method, url)
            .header(CONTENT_LENGTH, HeaderValue::from(bytes.len()))
            .body(bytes);
        Ok(builder
            // .with_azure_authorization(&credential, &self.config.account)
            .send_retry(&self.config.retry_config)
            .await?)
    }

    pub async fn create_experiment(
        &self,
        name: impl Into<String>,
        artifact_location: Option<impl Into<String>>,
        tags: Option<HashMap<String, String>>,
    ) -> Result<CreateExperimentResponse> {
        let payload = CreateExperiment {
            name: Some(name.into()),
            artifact_location: artifact_location.map(|d| d.into()),
            tags: tags
                .map(|t| {
                    t.iter()
                        .map(|(key, value)| ExperimentTag {
                            key: Some(key.clone()),
                            value: Some(value.clone()),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };
        let response = self
            .request(Method::POST, "experiments/create", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn get_experiment(
        &self,
        experiment_id: impl Into<String>,
    ) -> Result<GetExperimentResponse> {
        let payload = GetExperiment {
            experiment_id: Some(experiment_id.into()),
        };
        let response = self
            .request(Method::GET, "experiments/get", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn get_experiment_by_name(
        &self,
        experiment_name: impl Into<String>,
    ) -> Result<GetExperimentByNameResponse> {
        let payload = GetExperimentByName {
            experiment_name: Some(experiment_name.into()),
        };
        let response = self
            .request(Method::GET, "experiments/get-by-name", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn delete_experiment(&self, experiment_id: impl Into<String>) -> Result<()> {
        let payload = DeleteExperiment {
            experiment_id: Some(experiment_id.into()),
        };
        // TODO docs clearly say POST, but make sure its not DELETE afetr all
        self.request(Method::POST, "experiments/delete", &payload)
            .await?;
        Ok(())
    }

    pub async fn restore_experiment(&self, experiment_id: impl Into<String>) -> Result<()> {
        let payload = RestoreExperiment {
            experiment_id: Some(experiment_id.into()),
        };
        self.request(Method::POST, "experiments/restore", &payload)
            .await?;
        Ok(())
    }

    pub async fn update_experiment(
        &self,
        experiment_id: impl Into<String>,
        new_name: impl Into<String>,
    ) -> Result<()> {
        let payload = UpdateExperiment {
            experiment_id: Some(experiment_id.into()),
            new_name: Some(new_name.into()),
        };
        self.request(Method::POST, "experiments/update", &payload)
            .await?;
        Ok(())
    }

    pub async fn create_registered_model(
        &self,
        name: impl Into<String>,
        description: Option<impl Into<String>>,
        tags: Option<HashMap<String, String>>,
    ) -> Result<CreateRegisteredModelResponse> {
        let payload = CreateRegisteredModel {
            name: Some(name.into()),
            description: description.map(|d| d.into()),
            tags: tags
                .map(|t| {
                    t.iter()
                        .map(|(key, value)| RegisteredModelTag {
                            key: Some(key.clone()),
                            value: Some(value.clone()),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };
        let response = self
            .request(Method::POST, "registered-models/create", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn get_registered_model(
        &self,
        name: impl Into<String>,
    ) -> Result<GetRegisteredModelResponse> {
        let payload = GetRegisteredModel {
            name: Some(name.into()),
        };
        let response = self
            .request(Method::GET, "registered-models/get", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn rename_registered_model(
        &self,
        name: impl Into<String>,
        new_name: impl Into<String>,
    ) -> Result<RenameRegisteredModelResponse> {
        let payload = RenameRegisteredModel {
            name: Some(name.into()),
            new_name: Some(new_name.into()),
        };
        let response = self
            .request(Method::POST, "registered-models/rename", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn update_registered_model(
        &self,
        name: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> Result<UpdateRegisteredModelResponse> {
        let payload = UpdateRegisteredModel {
            name: Some(name.into()),
            description: description.map(|d| d.into()),
        };
        let response = self
            .request(Method::PATCH, "registered-models/rename", &payload)
            .await?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    pub async fn delete_registered_model(&self, name: impl Into<String>) -> Result<()> {
        let payload = DeleteRegisteredModel {
            name: Some(name.into()),
        };
        self.request(Method::DELETE, "registered-models/delete", &payload)
            .await?;
        Ok(())
    }
}
