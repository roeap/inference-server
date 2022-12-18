// @generated
/// Generated client implementations.
pub mod databricks_mlflow_artifacts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DatabricksMlflowArtifactsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabricksMlflowArtifactsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatabricksMlflowArtifactsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatabricksMlflowArtifactsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DatabricksMlflowArtifactsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_credentials_for_read(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCredentialsForRead>,
        ) -> Result<
            tonic::Response<super::get_credentials_for_read::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.DatabricksMlflowArtifactsService/getCredentialsForRead",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_credentials_for_write(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCredentialsForWrite>,
        ) -> Result<
            tonic::Response<super::get_credentials_for_write::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.DatabricksMlflowArtifactsService/getCredentialsForWrite",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod databricks_mlflow_artifacts_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DatabricksMlflowArtifactsServiceServer.
    #[async_trait]
    pub trait DatabricksMlflowArtifactsService: Send + Sync + 'static {
        async fn get_credentials_for_read(
            &self,
            request: tonic::Request<super::GetCredentialsForRead>,
        ) -> Result<
            tonic::Response<super::get_credentials_for_read::Response>,
            tonic::Status,
        >;
        async fn get_credentials_for_write(
            &self,
            request: tonic::Request<super::GetCredentialsForWrite>,
        ) -> Result<
            tonic::Response<super::get_credentials_for_write::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DatabricksMlflowArtifactsServiceServer<
        T: DatabricksMlflowArtifactsService,
    > {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DatabricksMlflowArtifactsService> DatabricksMlflowArtifactsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DatabricksMlflowArtifactsServiceServer<T>
    where
        T: DatabricksMlflowArtifactsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mlflow.DatabricksMlflowArtifactsService/getCredentialsForRead" => {
                    #[allow(non_camel_case_types)]
                    struct getCredentialsForReadSvc<T: DatabricksMlflowArtifactsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatabricksMlflowArtifactsService,
                    > tonic::server::UnaryService<super::GetCredentialsForRead>
                    for getCredentialsForReadSvc<T> {
                        type Response = super::get_credentials_for_read::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCredentialsForRead>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_credentials_for_read(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getCredentialsForReadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.DatabricksMlflowArtifactsService/getCredentialsForWrite" => {
                    #[allow(non_camel_case_types)]
                    struct getCredentialsForWriteSvc<
                        T: DatabricksMlflowArtifactsService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatabricksMlflowArtifactsService,
                    > tonic::server::UnaryService<super::GetCredentialsForWrite>
                    for getCredentialsForWriteSvc<T> {
                        type Response = super::get_credentials_for_write::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCredentialsForWrite>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_credentials_for_write(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getCredentialsForWriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: DatabricksMlflowArtifactsService> Clone
    for DatabricksMlflowArtifactsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DatabricksMlflowArtifactsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatabricksMlflowArtifactsService> tonic::server::NamedService
    for DatabricksMlflowArtifactsServiceServer<T> {
        const NAME: &'static str = "mlflow.DatabricksMlflowArtifactsService";
    }
}
/// Generated client implementations.
pub mod model_registry_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ModelRegistryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModelRegistryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ModelRegistryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ModelRegistryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ModelRegistryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn create_registered_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRegisteredModel>,
        ) -> Result<
            tonic::Response<super::create_registered_model::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/createRegisteredModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn rename_registered_model(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameRegisteredModel>,
        ) -> Result<
            tonic::Response<super::rename_registered_model::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/renameRegisteredModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_registered_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRegisteredModel>,
        ) -> Result<
            tonic::Response<super::update_registered_model::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/updateRegisteredModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_registered_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegisteredModel>,
        ) -> Result<
            tonic::Response<super::delete_registered_model::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/deleteRegisteredModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_registered_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRegisteredModel>,
        ) -> Result<
            tonic::Response<super::get_registered_model::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/getRegisteredModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn search_registered_models(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRegisteredModels>,
        ) -> Result<
            tonic::Response<super::search_registered_models::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/searchRegisteredModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_registered_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRegisteredModels>,
        ) -> Result<
            tonic::Response<super::list_registered_models::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/listRegisteredModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_latest_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestVersions>,
        ) -> Result<
            tonic::Response<super::get_latest_versions::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/getLatestVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModelVersion>,
        ) -> Result<
            tonic::Response<super::create_model_version::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/createModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModelVersion>,
        ) -> Result<
            tonic::Response<super::update_model_version::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/updateModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transition_model_version_stage(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionModelVersionStage>,
        ) -> Result<
            tonic::Response<super::transition_model_version_stage::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/transitionModelVersionStage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelVersion>,
        ) -> Result<
            tonic::Response<super::delete_model_version::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/deleteModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_model_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelVersion>,
        ) -> Result<tonic::Response<super::get_model_version::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/getModelVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn search_model_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchModelVersions>,
        ) -> Result<
            tonic::Response<super::search_model_versions::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/searchModelVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_model_version_download_uri(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelVersionDownloadUri>,
        ) -> Result<
            tonic::Response<super::get_model_version_download_uri::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/getModelVersionDownloadUri",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_registered_model_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRegisteredModelTag>,
        ) -> Result<
            tonic::Response<super::set_registered_model_tag::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/setRegisteredModelTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_model_version_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModelVersionTag>,
        ) -> Result<
            tonic::Response<super::set_model_version_tag::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/setModelVersionTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_registered_model_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegisteredModelTag>,
        ) -> Result<
            tonic::Response<super::delete_registered_model_tag::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/deleteRegisteredModelTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_model_version_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelVersionTag>,
        ) -> Result<
            tonic::Response<super::delete_model_version_tag::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.ModelRegistryService/deleteModelVersionTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod model_registry_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ModelRegistryServiceServer.
    #[async_trait]
    pub trait ModelRegistryService: Send + Sync + 'static {
        async fn create_registered_model(
            &self,
            request: tonic::Request<super::CreateRegisteredModel>,
        ) -> Result<
            tonic::Response<super::create_registered_model::Response>,
            tonic::Status,
        >;
        async fn rename_registered_model(
            &self,
            request: tonic::Request<super::RenameRegisteredModel>,
        ) -> Result<
            tonic::Response<super::rename_registered_model::Response>,
            tonic::Status,
        >;
        async fn update_registered_model(
            &self,
            request: tonic::Request<super::UpdateRegisteredModel>,
        ) -> Result<
            tonic::Response<super::update_registered_model::Response>,
            tonic::Status,
        >;
        async fn delete_registered_model(
            &self,
            request: tonic::Request<super::DeleteRegisteredModel>,
        ) -> Result<
            tonic::Response<super::delete_registered_model::Response>,
            tonic::Status,
        >;
        async fn get_registered_model(
            &self,
            request: tonic::Request<super::GetRegisteredModel>,
        ) -> Result<
            tonic::Response<super::get_registered_model::Response>,
            tonic::Status,
        >;
        async fn search_registered_models(
            &self,
            request: tonic::Request<super::SearchRegisteredModels>,
        ) -> Result<
            tonic::Response<super::search_registered_models::Response>,
            tonic::Status,
        >;
        async fn list_registered_models(
            &self,
            request: tonic::Request<super::ListRegisteredModels>,
        ) -> Result<
            tonic::Response<super::list_registered_models::Response>,
            tonic::Status,
        >;
        async fn get_latest_versions(
            &self,
            request: tonic::Request<super::GetLatestVersions>,
        ) -> Result<
            tonic::Response<super::get_latest_versions::Response>,
            tonic::Status,
        >;
        async fn create_model_version(
            &self,
            request: tonic::Request<super::CreateModelVersion>,
        ) -> Result<
            tonic::Response<super::create_model_version::Response>,
            tonic::Status,
        >;
        async fn update_model_version(
            &self,
            request: tonic::Request<super::UpdateModelVersion>,
        ) -> Result<
            tonic::Response<super::update_model_version::Response>,
            tonic::Status,
        >;
        async fn transition_model_version_stage(
            &self,
            request: tonic::Request<super::TransitionModelVersionStage>,
        ) -> Result<
            tonic::Response<super::transition_model_version_stage::Response>,
            tonic::Status,
        >;
        async fn delete_model_version(
            &self,
            request: tonic::Request<super::DeleteModelVersion>,
        ) -> Result<
            tonic::Response<super::delete_model_version::Response>,
            tonic::Status,
        >;
        async fn get_model_version(
            &self,
            request: tonic::Request<super::GetModelVersion>,
        ) -> Result<tonic::Response<super::get_model_version::Response>, tonic::Status>;
        async fn search_model_versions(
            &self,
            request: tonic::Request<super::SearchModelVersions>,
        ) -> Result<
            tonic::Response<super::search_model_versions::Response>,
            tonic::Status,
        >;
        async fn get_model_version_download_uri(
            &self,
            request: tonic::Request<super::GetModelVersionDownloadUri>,
        ) -> Result<
            tonic::Response<super::get_model_version_download_uri::Response>,
            tonic::Status,
        >;
        async fn set_registered_model_tag(
            &self,
            request: tonic::Request<super::SetRegisteredModelTag>,
        ) -> Result<
            tonic::Response<super::set_registered_model_tag::Response>,
            tonic::Status,
        >;
        async fn set_model_version_tag(
            &self,
            request: tonic::Request<super::SetModelVersionTag>,
        ) -> Result<
            tonic::Response<super::set_model_version_tag::Response>,
            tonic::Status,
        >;
        async fn delete_registered_model_tag(
            &self,
            request: tonic::Request<super::DeleteRegisteredModelTag>,
        ) -> Result<
            tonic::Response<super::delete_registered_model_tag::Response>,
            tonic::Status,
        >;
        async fn delete_model_version_tag(
            &self,
            request: tonic::Request<super::DeleteModelVersionTag>,
        ) -> Result<
            tonic::Response<super::delete_model_version_tag::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ModelRegistryServiceServer<T: ModelRegistryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ModelRegistryService> ModelRegistryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ModelRegistryServiceServer<T>
    where
        T: ModelRegistryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mlflow.ModelRegistryService/createRegisteredModel" => {
                    #[allow(non_camel_case_types)]
                    struct createRegisteredModelSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::CreateRegisteredModel>
                    for createRegisteredModelSvc<T> {
                        type Response = super::create_registered_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRegisteredModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_registered_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createRegisteredModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/renameRegisteredModel" => {
                    #[allow(non_camel_case_types)]
                    struct renameRegisteredModelSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::RenameRegisteredModel>
                    for renameRegisteredModelSvc<T> {
                        type Response = super::rename_registered_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenameRegisteredModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).rename_registered_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = renameRegisteredModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/updateRegisteredModel" => {
                    #[allow(non_camel_case_types)]
                    struct updateRegisteredModelSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::UpdateRegisteredModel>
                    for updateRegisteredModelSvc<T> {
                        type Response = super::update_registered_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRegisteredModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_registered_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateRegisteredModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/deleteRegisteredModel" => {
                    #[allow(non_camel_case_types)]
                    struct deleteRegisteredModelSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::DeleteRegisteredModel>
                    for deleteRegisteredModelSvc<T> {
                        type Response = super::delete_registered_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRegisteredModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_registered_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteRegisteredModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/getRegisteredModel" => {
                    #[allow(non_camel_case_types)]
                    struct getRegisteredModelSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::GetRegisteredModel>
                    for getRegisteredModelSvc<T> {
                        type Response = super::get_registered_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRegisteredModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_registered_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getRegisteredModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/searchRegisteredModels" => {
                    #[allow(non_camel_case_types)]
                    struct searchRegisteredModelsSvc<T: ModelRegistryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::SearchRegisteredModels>
                    for searchRegisteredModelsSvc<T> {
                        type Response = super::search_registered_models::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRegisteredModels>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).search_registered_models(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = searchRegisteredModelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/listRegisteredModels" => {
                    #[allow(non_camel_case_types)]
                    struct listRegisteredModelsSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::ListRegisteredModels>
                    for listRegisteredModelsSvc<T> {
                        type Response = super::list_registered_models::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRegisteredModels>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_registered_models(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listRegisteredModelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/getLatestVersions" => {
                    #[allow(non_camel_case_types)]
                    struct getLatestVersionsSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::GetLatestVersions>
                    for getLatestVersionsSvc<T> {
                        type Response = super::get_latest_versions::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLatestVersions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_latest_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getLatestVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/createModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct createModelVersionSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::CreateModelVersion>
                    for createModelVersionSvc<T> {
                        type Response = super::create_model_version::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateModelVersion>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createModelVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/updateModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct updateModelVersionSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::UpdateModelVersion>
                    for updateModelVersionSvc<T> {
                        type Response = super::update_model_version::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateModelVersion>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateModelVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/transitionModelVersionStage" => {
                    #[allow(non_camel_case_types)]
                    struct transitionModelVersionStageSvc<T: ModelRegistryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::TransitionModelVersionStage>
                    for transitionModelVersionStageSvc<T> {
                        type Response = super::transition_model_version_stage::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionModelVersionStage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transition_model_version_stage(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = transitionModelVersionStageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/deleteModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct deleteModelVersionSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::DeleteModelVersion>
                    for deleteModelVersionSvc<T> {
                        type Response = super::delete_model_version::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelVersion>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteModelVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/getModelVersion" => {
                    #[allow(non_camel_case_types)]
                    struct getModelVersionSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::GetModelVersion>
                    for getModelVersionSvc<T> {
                        type Response = super::get_model_version::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelVersion>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_model_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getModelVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/searchModelVersions" => {
                    #[allow(non_camel_case_types)]
                    struct searchModelVersionsSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::SearchModelVersions>
                    for searchModelVersionsSvc<T> {
                        type Response = super::search_model_versions::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchModelVersions>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).search_model_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = searchModelVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/getModelVersionDownloadUri" => {
                    #[allow(non_camel_case_types)]
                    struct getModelVersionDownloadUriSvc<T: ModelRegistryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::GetModelVersionDownloadUri>
                    for getModelVersionDownloadUriSvc<T> {
                        type Response = super::get_model_version_download_uri::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelVersionDownloadUri>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_model_version_download_uri(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getModelVersionDownloadUriSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/setRegisteredModelTag" => {
                    #[allow(non_camel_case_types)]
                    struct setRegisteredModelTagSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::SetRegisteredModelTag>
                    for setRegisteredModelTagSvc<T> {
                        type Response = super::set_registered_model_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRegisteredModelTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_registered_model_tag(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = setRegisteredModelTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/setModelVersionTag" => {
                    #[allow(non_camel_case_types)]
                    struct setModelVersionTagSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::SetModelVersionTag>
                    for setModelVersionTagSvc<T> {
                        type Response = super::set_model_version_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetModelVersionTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_model_version_tag(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = setModelVersionTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/deleteRegisteredModelTag" => {
                    #[allow(non_camel_case_types)]
                    struct deleteRegisteredModelTagSvc<T: ModelRegistryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::DeleteRegisteredModelTag>
                    for deleteRegisteredModelTagSvc<T> {
                        type Response = super::delete_registered_model_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRegisteredModelTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_registered_model_tag(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteRegisteredModelTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.ModelRegistryService/deleteModelVersionTag" => {
                    #[allow(non_camel_case_types)]
                    struct deleteModelVersionTagSvc<T: ModelRegistryService>(pub Arc<T>);
                    impl<
                        T: ModelRegistryService,
                    > tonic::server::UnaryService<super::DeleteModelVersionTag>
                    for deleteModelVersionTagSvc<T> {
                        type Response = super::delete_model_version_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelVersionTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_model_version_tag(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteModelVersionTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ModelRegistryService> Clone for ModelRegistryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ModelRegistryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ModelRegistryService> tonic::server::NamedService
    for ModelRegistryServiceServer<T> {
        const NAME: &'static str = "mlflow.ModelRegistryService";
    }
}
/// Generated client implementations.
pub mod mlflow_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MlflowServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MlflowServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MlflowServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MlflowServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MlflowServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_experiment_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExperimentByName>,
        ) -> Result<
            tonic::Response<super::get_experiment_by_name::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/getExperimentByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExperiment>,
        ) -> Result<tonic::Response<super::create_experiment::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/createExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_experiments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExperiments>,
        ) -> Result<tonic::Response<super::list_experiments::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/listExperiments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExperiment>,
        ) -> Result<tonic::Response<super::get_experiment::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/getExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExperiment>,
        ) -> Result<tonic::Response<super::delete_experiment::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/deleteExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restore_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreExperiment>,
        ) -> Result<
            tonic::Response<super::restore_experiment::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/restoreExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExperiment>,
        ) -> Result<tonic::Response<super::update_experiment::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/updateExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_run(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRun>,
        ) -> Result<tonic::Response<super::create_run::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/createRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_run(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRun>,
        ) -> Result<tonic::Response<super::update_run::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/updateRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_run(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRun>,
        ) -> Result<tonic::Response<super::delete_run::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/deleteRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restore_run(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreRun>,
        ) -> Result<tonic::Response<super::restore_run::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/restoreRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::LogMetric>,
        ) -> Result<tonic::Response<super::log_metric::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/logMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_param(
            &mut self,
            request: impl tonic::IntoRequest<super::LogParam>,
        ) -> Result<tonic::Response<super::log_param::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/logParam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_experiment_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::SetExperimentTag>,
        ) -> Result<
            tonic::Response<super::set_experiment_tag::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/setExperimentTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTag>,
        ) -> Result<tonic::Response<super::set_tag::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/setTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTag>,
        ) -> Result<tonic::Response<super::delete_tag::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/deleteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRun>,
        ) -> Result<tonic::Response<super::get_run::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/getRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn search_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRuns>,
        ) -> Result<tonic::Response<super::search_runs::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/searchRuns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtifacts>,
        ) -> Result<tonic::Response<super::list_artifacts::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/listArtifacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_metric_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricHistory>,
        ) -> Result<
            tonic::Response<super::get_metric_history::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/getMetricHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::LogBatch>,
        ) -> Result<tonic::Response<super::log_batch::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/logBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_model(
            &mut self,
            request: impl tonic::IntoRequest<super::LogModel>,
        ) -> Result<tonic::Response<super::log_model::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mlflow.MlflowService/logModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod mlflow_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MlflowServiceServer.
    #[async_trait]
    pub trait MlflowService: Send + Sync + 'static {
        async fn get_experiment_by_name(
            &self,
            request: tonic::Request<super::GetExperimentByName>,
        ) -> Result<
            tonic::Response<super::get_experiment_by_name::Response>,
            tonic::Status,
        >;
        async fn create_experiment(
            &self,
            request: tonic::Request<super::CreateExperiment>,
        ) -> Result<tonic::Response<super::create_experiment::Response>, tonic::Status>;
        async fn list_experiments(
            &self,
            request: tonic::Request<super::ListExperiments>,
        ) -> Result<tonic::Response<super::list_experiments::Response>, tonic::Status>;
        async fn get_experiment(
            &self,
            request: tonic::Request<super::GetExperiment>,
        ) -> Result<tonic::Response<super::get_experiment::Response>, tonic::Status>;
        async fn delete_experiment(
            &self,
            request: tonic::Request<super::DeleteExperiment>,
        ) -> Result<tonic::Response<super::delete_experiment::Response>, tonic::Status>;
        async fn restore_experiment(
            &self,
            request: tonic::Request<super::RestoreExperiment>,
        ) -> Result<tonic::Response<super::restore_experiment::Response>, tonic::Status>;
        async fn update_experiment(
            &self,
            request: tonic::Request<super::UpdateExperiment>,
        ) -> Result<tonic::Response<super::update_experiment::Response>, tonic::Status>;
        async fn create_run(
            &self,
            request: tonic::Request<super::CreateRun>,
        ) -> Result<tonic::Response<super::create_run::Response>, tonic::Status>;
        async fn update_run(
            &self,
            request: tonic::Request<super::UpdateRun>,
        ) -> Result<tonic::Response<super::update_run::Response>, tonic::Status>;
        async fn delete_run(
            &self,
            request: tonic::Request<super::DeleteRun>,
        ) -> Result<tonic::Response<super::delete_run::Response>, tonic::Status>;
        async fn restore_run(
            &self,
            request: tonic::Request<super::RestoreRun>,
        ) -> Result<tonic::Response<super::restore_run::Response>, tonic::Status>;
        async fn log_metric(
            &self,
            request: tonic::Request<super::LogMetric>,
        ) -> Result<tonic::Response<super::log_metric::Response>, tonic::Status>;
        async fn log_param(
            &self,
            request: tonic::Request<super::LogParam>,
        ) -> Result<tonic::Response<super::log_param::Response>, tonic::Status>;
        async fn set_experiment_tag(
            &self,
            request: tonic::Request<super::SetExperimentTag>,
        ) -> Result<tonic::Response<super::set_experiment_tag::Response>, tonic::Status>;
        async fn set_tag(
            &self,
            request: tonic::Request<super::SetTag>,
        ) -> Result<tonic::Response<super::set_tag::Response>, tonic::Status>;
        async fn delete_tag(
            &self,
            request: tonic::Request<super::DeleteTag>,
        ) -> Result<tonic::Response<super::delete_tag::Response>, tonic::Status>;
        async fn get_run(
            &self,
            request: tonic::Request<super::GetRun>,
        ) -> Result<tonic::Response<super::get_run::Response>, tonic::Status>;
        async fn search_runs(
            &self,
            request: tonic::Request<super::SearchRuns>,
        ) -> Result<tonic::Response<super::search_runs::Response>, tonic::Status>;
        async fn list_artifacts(
            &self,
            request: tonic::Request<super::ListArtifacts>,
        ) -> Result<tonic::Response<super::list_artifacts::Response>, tonic::Status>;
        async fn get_metric_history(
            &self,
            request: tonic::Request<super::GetMetricHistory>,
        ) -> Result<tonic::Response<super::get_metric_history::Response>, tonic::Status>;
        async fn log_batch(
            &self,
            request: tonic::Request<super::LogBatch>,
        ) -> Result<tonic::Response<super::log_batch::Response>, tonic::Status>;
        async fn log_model(
            &self,
            request: tonic::Request<super::LogModel>,
        ) -> Result<tonic::Response<super::log_model::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MlflowServiceServer<T: MlflowService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MlflowService> MlflowServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MlflowServiceServer<T>
    where
        T: MlflowService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mlflow.MlflowService/getExperimentByName" => {
                    #[allow(non_camel_case_types)]
                    struct getExperimentByNameSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::GetExperimentByName>
                    for getExperimentByNameSvc<T> {
                        type Response = super::get_experiment_by_name::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExperimentByName>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_experiment_by_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getExperimentByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/createExperiment" => {
                    #[allow(non_camel_case_types)]
                    struct createExperimentSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::CreateExperiment>
                    for createExperimentSvc<T> {
                        type Response = super::create_experiment::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateExperiment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_experiment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/listExperiments" => {
                    #[allow(non_camel_case_types)]
                    struct listExperimentsSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::ListExperiments>
                    for listExperimentsSvc<T> {
                        type Response = super::list_experiments::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListExperiments>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_experiments(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listExperimentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/getExperiment" => {
                    #[allow(non_camel_case_types)]
                    struct getExperimentSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::GetExperiment>
                    for getExperimentSvc<T> {
                        type Response = super::get_experiment::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExperiment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_experiment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/deleteExperiment" => {
                    #[allow(non_camel_case_types)]
                    struct deleteExperimentSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::DeleteExperiment>
                    for deleteExperimentSvc<T> {
                        type Response = super::delete_experiment::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteExperiment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_experiment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/restoreExperiment" => {
                    #[allow(non_camel_case_types)]
                    struct restoreExperimentSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::RestoreExperiment>
                    for restoreExperimentSvc<T> {
                        type Response = super::restore_experiment::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreExperiment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).restore_experiment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = restoreExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/updateExperiment" => {
                    #[allow(non_camel_case_types)]
                    struct updateExperimentSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::UpdateExperiment>
                    for updateExperimentSvc<T> {
                        type Response = super::update_experiment::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateExperiment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_experiment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateExperimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/createRun" => {
                    #[allow(non_camel_case_types)]
                    struct createRunSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::CreateRun>
                    for createRunSvc<T> {
                        type Response = super::create_run::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRun>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createRunSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/updateRun" => {
                    #[allow(non_camel_case_types)]
                    struct updateRunSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::UpdateRun>
                    for updateRunSvc<T> {
                        type Response = super::update_run::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRun>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateRunSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/deleteRun" => {
                    #[allow(non_camel_case_types)]
                    struct deleteRunSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::DeleteRun>
                    for deleteRunSvc<T> {
                        type Response = super::delete_run::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRun>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteRunSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/restoreRun" => {
                    #[allow(non_camel_case_types)]
                    struct restoreRunSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::RestoreRun>
                    for restoreRunSvc<T> {
                        type Response = super::restore_run::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreRun>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).restore_run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = restoreRunSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/logMetric" => {
                    #[allow(non_camel_case_types)]
                    struct logMetricSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::LogMetric>
                    for logMetricSvc<T> {
                        type Response = super::log_metric::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogMetric>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log_metric(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = logMetricSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/logParam" => {
                    #[allow(non_camel_case_types)]
                    struct logParamSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::LogParam>
                    for logParamSvc<T> {
                        type Response = super::log_param::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogParam>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log_param(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = logParamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/setExperimentTag" => {
                    #[allow(non_camel_case_types)]
                    struct setExperimentTagSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::SetExperimentTag>
                    for setExperimentTagSvc<T> {
                        type Response = super::set_experiment_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetExperimentTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_experiment_tag(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = setExperimentTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/setTag" => {
                    #[allow(non_camel_case_types)]
                    struct setTagSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::SetTag>
                    for setTagSvc<T> {
                        type Response = super::set_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = setTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/deleteTag" => {
                    #[allow(non_camel_case_types)]
                    struct deleteTagSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::DeleteTag>
                    for deleteTagSvc<T> {
                        type Response = super::delete_tag::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTag>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteTagSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/getRun" => {
                    #[allow(non_camel_case_types)]
                    struct getRunSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::GetRun>
                    for getRunSvc<T> {
                        type Response = super::get_run::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRun>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_run(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getRunSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/searchRuns" => {
                    #[allow(non_camel_case_types)]
                    struct searchRunsSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::SearchRuns>
                    for searchRunsSvc<T> {
                        type Response = super::search_runs::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRuns>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_runs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = searchRunsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/listArtifacts" => {
                    #[allow(non_camel_case_types)]
                    struct listArtifactsSvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::ListArtifacts>
                    for listArtifactsSvc<T> {
                        type Response = super::list_artifacts::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtifacts>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_artifacts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listArtifactsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/getMetricHistory" => {
                    #[allow(non_camel_case_types)]
                    struct getMetricHistorySvc<T: MlflowService>(pub Arc<T>);
                    impl<
                        T: MlflowService,
                    > tonic::server::UnaryService<super::GetMetricHistory>
                    for getMetricHistorySvc<T> {
                        type Response = super::get_metric_history::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMetricHistory>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_metric_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getMetricHistorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/logBatch" => {
                    #[allow(non_camel_case_types)]
                    struct logBatchSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::LogBatch>
                    for logBatchSvc<T> {
                        type Response = super::log_batch::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogBatch>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log_batch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = logBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.MlflowService/logModel" => {
                    #[allow(non_camel_case_types)]
                    struct logModelSvc<T: MlflowService>(pub Arc<T>);
                    impl<T: MlflowService> tonic::server::UnaryService<super::LogModel>
                    for logModelSvc<T> {
                        type Response = super::log_model::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogModel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = logModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MlflowService> Clone for MlflowServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MlflowService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MlflowService> tonic::server::NamedService for MlflowServiceServer<T> {
        const NAME: &'static str = "mlflow.MlflowService";
    }
}
