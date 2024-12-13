// This file is @generated by prost-build.
/// MonoVertexMetrics is used to provide information about the mono vertex including processing rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonoVertexMetrics {
    #[prost(string, tag = "1")]
    pub mono_vertex: ::prost::alloc::string::String,
    /// Processing rate in the past period of time, 1m, 5m, 15m, default
    #[prost(map = "string, message", tag = "2")]
    pub processing_rates: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        f64,
    >,
    /// Pending in the past period of time, 1m, 5m, 15m, default
    #[prost(map = "string, message", tag = "3")]
    pub pendings: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonoVertexMetricsResponse {
    #[prost(message, optional, tag = "1")]
    pub metrics: ::core::option::Option<MonoVertexMetrics>,
}
/// MonoVertexStatus is used to provide information about the mono vertex status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonoVertexStatus {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonoVertexStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<MonoVertexStatus>,
}
/// Generated client implementations.
pub mod mono_vertex_daemon_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// MonoVertexDaemonService is a grpc service that is used to provide APIs for giving any MonoVertex information.
    #[derive(Debug, Clone)]
    pub struct MonoVertexDaemonServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonoVertexDaemonServiceClient<tonic::transport::Channel> {
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
    impl<T> MonoVertexDaemonServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> MonoVertexDaemonServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MonoVertexDaemonServiceClient::new(
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
        pub async fn get_mono_vertex_metrics(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetMonoVertexMetricsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mvtxdaemon.MonoVertexDaemonService/GetMonoVertexMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mvtxdaemon.MonoVertexDaemonService",
                        "GetMonoVertexMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_mono_vertex_status(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetMonoVertexStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mvtxdaemon.MonoVertexDaemonService/GetMonoVertexStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mvtxdaemon.MonoVertexDaemonService",
                        "GetMonoVertexStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
