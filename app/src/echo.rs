/// EchoRequest is the request for echo.
#[derive(Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// EchoResponse is the response for echo.
#[derive(Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// KeyValueRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// KeyValueResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod echo_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Echo is the echo service.
    #[derive(Debug, Clone)]
    pub struct EchoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EchoClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EchoClient<T>
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
        ) -> EchoClient<InterceptedService<T, F>>
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
            EchoClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// UnaryEcho is unary echo.
        pub async fn unary_echo(
            &mut self,
            request: impl tonic::IntoRequest<super::EchoRequest>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/echo.Echo/UnaryEcho");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("echo.Echo", "UnaryEcho"));
            self.inner.unary(req, path, codec).await
        }
        /// ServerStreamingEcho is server side streaming.
        pub async fn server_streaming_echo(
            &mut self,
            request: impl tonic::IntoRequest<super::EchoRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::EchoResponse>>,
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
                "/echo.Echo/ServerStreamingEcho",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("echo.Echo", "ServerStreamingEcho"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// ClientStreamingEcho is client side streaming.
        pub async fn client_streaming_echo(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EchoRequest>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status> {
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
                "/echo.Echo/ClientStreamingEcho",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("echo.Echo", "ClientStreamingEcho"));
            self.inner.client_streaming(req, path, codec).await
        }
        /// BidirectionalStreamingEcho is bidi streaming.
        pub async fn bidirectional_streaming_echo(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EchoRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::EchoResponse>>,
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
                "/echo.Echo/BidirectionalStreamingEcho",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("echo.Echo", "BidirectionalStreamingEcho"));
            self.inner.streaming(req, path, codec).await
        }
        /// KeyValue store - set value, return RecordId or nil
        pub async fn set_value(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeyValueResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/echo.Echo/SetValue");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("echo.Echo", "SetValue"));
            self.inner.unary(req, path, codec).await
        }
        /// KeyValue store - get value
        pub async fn get_value(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeyValueResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/echo.Echo/GetValue");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("echo.Echo", "GetValue"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod echo_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EchoServer.
    #[async_trait]
    pub trait Echo: Send + Sync + 'static {
        /// UnaryEcho is unary echo.
        async fn unary_echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        /// Server streaming response type for the ServerStreamingEcho method.
        type ServerStreamingEchoStream: futures_core::Stream<
                Item = std::result::Result<super::EchoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// ServerStreamingEcho is server side streaming.
        async fn server_streaming_echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ServerStreamingEchoStream>,
            tonic::Status,
        >;
        /// ClientStreamingEcho is client side streaming.
        async fn client_streaming_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        /// Server streaming response type for the BidirectionalStreamingEcho method.
        type BidirectionalStreamingEchoStream: futures_core::Stream<
                Item = std::result::Result<super::EchoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// BidirectionalStreamingEcho is bidi streaming.
        async fn bidirectional_streaming_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::BidirectionalStreamingEchoStream>,
            tonic::Status,
        >;
        /// KeyValue store - set value, return RecordId or nil
        async fn set_value(
            &self,
            request: tonic::Request<super::KeyValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeyValueResponse>,
            tonic::Status,
        >;
        /// KeyValue store - get value
        async fn get_value(
            &self,
            request: tonic::Request<super::KeyValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeyValueResponse>,
            tonic::Status,
        >;
    }
    /// Echo is the echo service.
    #[derive(Debug)]
    pub struct EchoServer<T: Echo> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Echo> EchoServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EchoServer<T>
    where
        T: Echo,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/echo.Echo/UnaryEcho" => {
                    #[allow(non_camel_case_types)]
                    struct UnaryEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::UnaryService<super::EchoRequest>
                    for UnaryEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).unary_echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnaryEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/ServerStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ServerStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<
                        T: Echo,
                    > tonic::server::ServerStreamingService<super::EchoRequest>
                    for ServerStreamingEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::ServerStreamingEchoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).server_streaming_echo(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/ClientStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<
                        T: Echo,
                    > tonic::server::ClientStreamingService<super::EchoRequest>
                    for ClientStreamingEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).client_streaming_echo(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClientStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/BidirectionalStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct BidirectionalStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::StreamingService<super::EchoRequest>
                    for BidirectionalStreamingEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::BidirectionalStreamingEchoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).bidirectional_streaming_echo(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BidirectionalStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/SetValue" => {
                    #[allow(non_camel_case_types)]
                    struct SetValueSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::UnaryService<super::KeyValueRequest>
                    for SetValueSvc<T> {
                        type Response = super::KeyValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set_value(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/GetValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetValueSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::UnaryService<super::KeyValueRequest>
                    for GetValueSvc<T> {
                        type Response = super::KeyValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_value(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: Echo> Clone for EchoServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Echo> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Echo> tonic::server::NamedService for EchoServer<T> {
        const NAME: &'static str = "echo.Echo";
    }
}
