// @generated
/// Generated server implementations.
pub mod model_repository_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ModelRepositoryServiceServer.
    #[async_trait]
    pub trait ModelRepositoryService: Send + Sync + 'static {
        async fn repository_index(
            &self,
            request: tonic::Request<super::RepositoryIndexRequest>,
        ) -> Result<tonic::Response<super::RepositoryIndexResponse>, tonic::Status>;
        async fn repository_model_load(
            &self,
            request: tonic::Request<super::RepositoryModelLoadRequest>,
        ) -> Result<tonic::Response<super::RepositoryModelLoadResponse>, tonic::Status>;
        async fn repository_model_unload(
            &self,
            request: tonic::Request<super::RepositoryModelUnloadRequest>,
        ) -> Result<
            tonic::Response<super::RepositoryModelUnloadResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ModelRepositoryServiceServer<T: ModelRepositoryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ModelRepositoryService> ModelRepositoryServiceServer<T> {
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
    for ModelRepositoryServiceServer<T>
    where
        T: ModelRepositoryService,
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
                "/inference.model_repository.ModelRepositoryService/RepositoryIndex" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryIndexSvc<T: ModelRepositoryService>(pub Arc<T>);
                    impl<
                        T: ModelRepositoryService,
                    > tonic::server::UnaryService<super::RepositoryIndexRequest>
                    for RepositoryIndexSvc<T> {
                        type Response = super::RepositoryIndexResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryIndexRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).repository_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RepositoryIndexSvc(inner);
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
                "/inference.model_repository.ModelRepositoryService/RepositoryModelLoad" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryModelLoadSvc<T: ModelRepositoryService>(pub Arc<T>);
                    impl<
                        T: ModelRepositoryService,
                    > tonic::server::UnaryService<super::RepositoryModelLoadRequest>
                    for RepositoryModelLoadSvc<T> {
                        type Response = super::RepositoryModelLoadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryModelLoadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).repository_model_load(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RepositoryModelLoadSvc(inner);
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
                "/inference.model_repository.ModelRepositoryService/RepositoryModelUnload" => {
                    #[allow(non_camel_case_types)]
                    struct RepositoryModelUnloadSvc<T: ModelRepositoryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ModelRepositoryService,
                    > tonic::server::UnaryService<super::RepositoryModelUnloadRequest>
                    for RepositoryModelUnloadSvc<T> {
                        type Response = super::RepositoryModelUnloadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepositoryModelUnloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).repository_model_unload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RepositoryModelUnloadSvc(inner);
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
    impl<T: ModelRepositoryService> Clone for ModelRepositoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ModelRepositoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ModelRepositoryService> tonic::server::NamedService
    for ModelRepositoryServiceServer<T> {
        const NAME: &'static str = "inference.model_repository.ModelRepositoryService";
    }
}
