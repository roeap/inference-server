// @generated
/// Generated server implementations.
pub mod grpc_inference_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GrpcInferenceServiceServer.
    #[async_trait]
    pub trait GrpcInferenceService: Send + Sync + 'static {
        async fn server_live(
            &self,
            request: tonic::Request<super::ServerLiveRequest>,
        ) -> Result<tonic::Response<super::ServerLiveResponse>, tonic::Status>;
        async fn server_ready(
            &self,
            request: tonic::Request<super::ServerReadyRequest>,
        ) -> Result<tonic::Response<super::ServerReadyResponse>, tonic::Status>;
        async fn model_ready(
            &self,
            request: tonic::Request<super::ModelReadyRequest>,
        ) -> Result<tonic::Response<super::ModelReadyResponse>, tonic::Status>;
        async fn server_metadata(
            &self,
            request: tonic::Request<super::ServerMetadataRequest>,
        ) -> Result<tonic::Response<super::ServerMetadataResponse>, tonic::Status>;
        async fn model_metadata(
            &self,
            request: tonic::Request<super::ModelMetadataRequest>,
        ) -> Result<tonic::Response<super::ModelMetadataResponse>, tonic::Status>;
        async fn model_infer(
            &self,
            request: tonic::Request<super::ModelInferRequest>,
        ) -> Result<tonic::Response<super::ModelInferResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GrpcInferenceServiceServer<T: GrpcInferenceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GrpcInferenceService> GrpcInferenceServiceServer<T> {
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
    for GrpcInferenceServiceServer<T>
    where
        T: GrpcInferenceService,
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
                "/inference.GRPCInferenceService/ServerLive" => {
                    #[allow(non_camel_case_types)]
                    struct ServerLiveSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ServerLiveRequest>
                    for ServerLiveSvc<T> {
                        type Response = super::ServerLiveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerLiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).server_live(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerLiveSvc(inner);
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
                "/inference.GRPCInferenceService/ServerReady" => {
                    #[allow(non_camel_case_types)]
                    struct ServerReadySvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ServerReadyRequest>
                    for ServerReadySvc<T> {
                        type Response = super::ServerReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).server_ready(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerReadySvc(inner);
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
                "/inference.GRPCInferenceService/ModelReady" => {
                    #[allow(non_camel_case_types)]
                    struct ModelReadySvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ModelReadyRequest>
                    for ModelReadySvc<T> {
                        type Response = super::ModelReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).model_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModelReadySvc(inner);
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
                "/inference.GRPCInferenceService/ServerMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct ServerMetadataSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ServerMetadataRequest>
                    for ServerMetadataSvc<T> {
                        type Response = super::ServerMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).server_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerMetadataSvc(inner);
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
                "/inference.GRPCInferenceService/ModelMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct ModelMetadataSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ModelMetadataRequest>
                    for ModelMetadataSvc<T> {
                        type Response = super::ModelMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).model_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModelMetadataSvc(inner);
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
                "/inference.GRPCInferenceService/ModelInfer" => {
                    #[allow(non_camel_case_types)]
                    struct ModelInferSvc<T: GrpcInferenceService>(pub Arc<T>);
                    impl<
                        T: GrpcInferenceService,
                    > tonic::server::UnaryService<super::ModelInferRequest>
                    for ModelInferSvc<T> {
                        type Response = super::ModelInferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelInferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).model_infer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModelInferSvc(inner);
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
    impl<T: GrpcInferenceService> Clone for GrpcInferenceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GrpcInferenceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GrpcInferenceService> tonic::server::NamedService
    for GrpcInferenceServiceServer<T> {
        const NAME: &'static str = "inference.GRPCInferenceService";
    }
}
