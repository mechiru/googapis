/// Request message for the Parse method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseRequest {
    /// Required. Source text in CEL syntax.
    #[prost(string, tag="1")]
    pub cel_source: ::prost::alloc::string::String,
    /// Tag for version of CEL syntax, for future use.
    #[prost(string, tag="2")]
    pub syntax_version: ::prost::alloc::string::String,
    /// File or resource for source text, used in \[SourceInfo][google.api.SourceInfo\].
    #[prost(string, tag="3")]
    pub source_location: ::prost::alloc::string::String,
    /// Prevent macro expansion.  See "Macros" in Language Defiinition.
    #[prost(bool, tag="4")]
    pub disable_macros: bool,
}
/// Response message for the Parse method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseResponse {
    /// The parsed representation, or unset if parsing failed.
    #[prost(message, optional, tag="1")]
    pub parsed_expr: ::core::option::Option<super::super::v1alpha1::ParsedExpr>,
    /// Any number of issues with \[StatusDetails][\] as the details.
    #[prost(message, repeated, tag="2")]
    pub issues: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
}
/// Request message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    /// Required. The parsed representation of the CEL program.
    #[prost(message, optional, tag="1")]
    pub parsed_expr: ::core::option::Option<super::super::v1alpha1::ParsedExpr>,
    /// Declarations of types for external variables and functions.
    /// Required if program uses external variables or functions
    /// not in the default environment.
    #[prost(message, repeated, tag="2")]
    pub type_env: ::prost::alloc::vec::Vec<super::super::v1alpha1::Decl>,
    /// The protocol buffer context.  See "Name Resolution" in the
    /// Language Definition.
    #[prost(string, tag="3")]
    pub container: ::prost::alloc::string::String,
    /// If true, use only the declarations in \[type_env][google.api.expr.conformance.v1alpha1.CheckRequest.type_env\].  If false (default),
    /// add declarations for the standard definitions to the type environment.  See
    /// "Standard Definitions" in the Language Definition.
    #[prost(bool, tag="4")]
    pub no_std_env: bool,
}
/// Response message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    /// The annotated representation, or unset if checking failed.
    #[prost(message, optional, tag="1")]
    pub checked_expr: ::core::option::Option<super::super::v1alpha1::CheckedExpr>,
    /// Any number of issues with \[StatusDetails][\] as the details.
    #[prost(message, repeated, tag="2")]
    pub issues: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
}
/// Request message for the Eval method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvalRequest {
    /// Bindings for the external variables.  The types SHOULD be compatible
    /// with the type environment in \[CheckRequest][google.api.expr.conformance.v1alpha1.CheckRequest\], if checked.
    #[prost(map="string, message", tag="3")]
    pub bindings: ::std::collections::HashMap<::prost::alloc::string::String, super::super::v1alpha1::ExprValue>,
    /// SHOULD be the same container as used in \[CheckRequest][google.api.expr.conformance.v1alpha1.CheckRequest\], if checked.
    #[prost(string, tag="4")]
    pub container: ::prost::alloc::string::String,
    /// Required. Either the parsed or annotated representation of the CEL program.
    #[prost(oneof="eval_request::ExprKind", tags="1, 2")]
    pub expr_kind: ::core::option::Option<eval_request::ExprKind>,
}
/// Nested message and enum types in `EvalRequest`.
pub mod eval_request {
    /// Required. Either the parsed or annotated representation of the CEL program.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprKind {
        /// Evaluate based on the parsed representation.
        #[prost(message, tag="1")]
        ParsedExpr(super::super::super::v1alpha1::ParsedExpr),
        /// Evaluate based on the checked representation.
        #[prost(message, tag="2")]
        CheckedExpr(super::super::super::v1alpha1::CheckedExpr),
    }
}
/// Response message for the Eval method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvalResponse {
    /// The execution result, or unset if execution couldn't start.
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::v1alpha1::ExprValue>,
    /// Any number of issues with \[StatusDetails][\] as the details.
    /// Note that CEL execution errors are reified into \[ExprValue][\].
    /// Nevertheless, we'll allow out-of-band issues to be raised,
    /// which also makes the replies more regular.
    #[prost(message, repeated, tag="2")]
    pub issues: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
}
/// Warnings or errors in service execution are represented by
/// \[google.rpc.Status][google.rpc.Status\] messages, with the following message
/// in the details field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueDetails {
    /// The severity of the issue.
    #[prost(enumeration="issue_details::Severity", tag="1")]
    pub severity: i32,
    /// Position in the source, if known.
    #[prost(message, optional, tag="2")]
    pub position: ::core::option::Option<super::super::v1alpha1::SourcePosition>,
    /// Expression ID from \[Expr][\], 0 if unknown.
    #[prost(int64, tag="3")]
    pub id: i64,
}
/// Nested message and enum types in `IssueDetails`.
pub mod issue_details {
    /// Severities of issues.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// An unspecified severity.
        Unspecified = 0,
        /// Deprecation issue for statements and method that may no longer be
        /// supported or maintained.
        Deprecation = 1,
        /// Warnings such as: unused variables.
        Warning = 2,
        /// Errors such as: unmatched curly braces or variable redefinition.
        Error = 3,
    }
}
/// Generated client implementations.
pub mod conformance_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Access a CEL implementation from another process or machine.
    /// A CEL implementation is decomposed as a parser, a static checker,
    /// and an evaluator.  Every CEL implementation is expected to provide
    /// a server for this API.  The API will be used for conformance testing
    /// and other utilities.
    #[derive(Debug, Clone)]
    pub struct ConformanceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConformanceServiceClient<T>
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
        ) -> ConformanceServiceClient<InterceptedService<T, F>>
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
            ConformanceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Transforms CEL source text into a parsed representation.
        pub async fn parse(
            &mut self,
            request: impl tonic::IntoRequest<super::ParseRequest>,
        ) -> Result<tonic::Response<super::ParseResponse>, tonic::Status> {
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
                "/google.api.expr.conformance.v1alpha1.ConformanceService/Parse",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Runs static checks on a parsed CEL representation and return
        /// an annotated representation, or a set of issues.
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> Result<tonic::Response<super::CheckResponse>, tonic::Status> {
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
                "/google.api.expr.conformance.v1alpha1.ConformanceService/Check",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Evaluates a parsed or annotation CEL representation given
        /// values of external bindings.
        pub async fn eval(
            &mut self,
            request: impl tonic::IntoRequest<super::EvalRequest>,
        ) -> Result<tonic::Response<super::EvalResponse>, tonic::Status> {
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
                "/google.api.expr.conformance.v1alpha1.ConformanceService/Eval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
