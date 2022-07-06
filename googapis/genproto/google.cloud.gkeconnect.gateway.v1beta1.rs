/// Generated client implementations.
pub mod gateway_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Gateway service is a public API which works as a Kubernetes resource model
    /// proxy between end users and registered Kubernetes clusters. Each RPC in this
    /// service matches with an HTTP verb. End user will initiate kubectl commands
    /// against the Gateway service, and Gateway service will forward user requests
    /// to clusters.
    #[derive(Debug, Clone)]
    pub struct GatewayServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GatewayServiceClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayServiceClient<InterceptedService<T, F>>
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
            GatewayServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// GetResource performs an HTTP GET request on the Kubernetes API Server.
        pub async fn get_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::api::HttpBody,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::api::HttpBody>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayService/GetResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PostResource performs an HTTP POST on the Kubernetes API Server.
        pub async fn post_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::api::HttpBody,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::api::HttpBody>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayService/PostResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteResource performs an HTTP DELETE on the Kubernetes API Server.
        pub async fn delete_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::api::HttpBody,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::api::HttpBody>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayService/DeleteResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PutResource performs an HTTP PUT on the Kubernetes API Server.
        pub async fn put_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::api::HttpBody,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::api::HttpBody>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayService/PutResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PatchResource performs an HTTP PATCH on the Kubernetes API Server.
        pub async fn patch_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::api::HttpBody,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::api::HttpBody>,
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
                "/google.cloud.gkeconnect.gateway.v1beta1.GatewayService/PatchResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
