mod client;

use crate::client::retry::RetryExt;

use bytes::{Buf, Bytes};
use reqwest::{
    header::{HeaderValue, CONTENT_LENGTH},
    Client as ReqwestClient, Method, Response, StatusCode,
};
use serde::Serialize;
use url::Url;

pub use client::{backoff::BackoffConfig, retry::RetryConfig, ClientOptions};

/// A specialized `Result` for object store-related errors
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

    async fn post_request<T: Serialize + std::fmt::Debug + ?Sized + Sync>(
        &self,
        path: impl AsRef<str>,
        bytes: Option<Bytes>,
        query: &T,
    ) -> Result<Response> {
        let url = self.config.service.join(path.as_ref())?;

        let mut builder = self.client.request(Method::POST, url).query(query);

        if let Some(bytes) = bytes {
            builder = builder
                .header(CONTENT_LENGTH, HeaderValue::from(bytes.len()))
                .body(bytes)
        } else {
            builder = builder.header(CONTENT_LENGTH, HeaderValue::from_static("0"));
        }

        let response = builder
            // .with_azure_authorization(&credential, &self.config.account)
            .send_retry(&self.config.retry_config)
            .await?;

        Ok(response)
    }
}
