/// The message used by the client to register interest in an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// The `target` value **must** be a valid URL path pointing to an entity
    /// to watch. Note that the service name **must** be
    /// removed from the target field (e.g., the target field must say
    /// "/foo/bar", not "myservice.googleapis.com/foo/bar"). A client is
    /// also allowed to pass system-specific parameters in the URL that
    /// are only obeyed by some implementations. Some parameters will be
    /// implementation-specific. However, some have predefined meaning
    /// and are listed here:
    ///
    ///  * recursive = true|false \[default=false\]
    ///    If set to true, indicates that the client wants to watch all elements
    ///    of entities in the subtree rooted at the entity's name in `target`. For
    ///    descendants that are not the immediate children of the target, the
    ///    `Change.element` will contain slashes.
    ///
    ///    Note that some namespaces and entities will not support recursive
    ///    watching. When watching such an entity, a client must not set recursive
    ///    to true. Otherwise, it will receive an `UNIMPLEMENTED` error.
    ///
    /// Normal URL encoding must be used inside `target`.  For example, if a query
    /// parameter name or value, or the non-query parameter portion of `target`
    /// contains a special character, it must be %-encoded.  We recommend that
    /// clients and servers use their runtime's URL library to produce and consume
    /// target values.
    #[prost(string, tag="1")]
    pub target: ::prost::alloc::string::String,
    /// The `resume_marker` specifies how much of the existing underlying state is
    /// delivered to the client when the watch request is received by the
    /// system. The client can set this marker in one of the following ways to get
    /// different semantics:
    ///
    /// *   Parameter is not specified or has the value "".
    ///     Semantics: Fetch initial state.
    ///     The client wants the entity's initial state to be delivered. See the
    ///     description in "Initial State".
    ///
    /// *   Parameter is set to the string "now" (UTF-8 encoding).
    ///     Semantics: Fetch new changes only.
    ///     The client just wants to get the changes received by the system after
    ///     the watch point. The system may deliver changes from before the watch
    ///     point as well.
    ///
    /// *   Parameter is set to a value received in an earlier
    ///     `Change.resume_marker` field while watching the same entity.
    ///     Semantics: Resume from a specific point.
    ///     The client wants to receive the changes from a specific point; this
    ///     value must correspond to a value received in the `Change.resume_marker`
    ///     field. The system may deliver changes from before the `resume_marker`
    ///     as well. If the system cannot resume the stream from this point (e.g.,
    ///     if it is too far behind in the stream), it can raise the
    ///     `FAILED_PRECONDITION` error.
    ///
    /// An implementation MUST support an unspecified parameter and the
    /// empty string "" marker (initial state fetching) and the "now" marker.
    /// It need not support resuming from a specific point.
    #[prost(bytes="vec", tag="2")]
    pub resume_marker: ::prost::alloc::vec::Vec<u8>,
}
/// A batch of Change messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeBatch {
    /// A list of Change messages.
    #[prost(message, repeated, tag="1")]
    pub changes: ::prost::alloc::vec::Vec<Change>,
}
/// A Change indicates the most recent state of an element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    /// Name of the element, interpreted relative to the entity's actual
    /// name. "" refers to the entity itself. The element name is a valid
    /// UTF-8 string.
    #[prost(string, tag="1")]
    pub element: ::prost::alloc::string::String,
    /// The state of the `element`.
    #[prost(enumeration="change::State", tag="2")]
    pub state: i32,
    /// The actual change data. This field is present only when `state() == EXISTS`
    /// or `state() == ERROR`. Please see
    /// \[google.protobuf.Any][google.protobuf.Any\] about how to use the Any type.
    #[prost(message, optional, tag="6")]
    pub data: ::core::option::Option<::prost_types::Any>,
    /// If present, provides a compact representation of all the messages that have
    /// been received by the caller for the given entity, e.g., it could be a
    /// sequence number or a multi-part timestamp/version vector. This marker can
    /// be provided in the Request message, allowing the caller to resume the
    /// stream watching at a specific point without fetching the initial state.
    #[prost(bytes="vec", tag="4")]
    pub resume_marker: ::prost::alloc::vec::Vec<u8>,
    /// If true, this Change is followed by more Changes that are in the same group
    /// as this Change.
    #[prost(bool, tag="5")]
    pub continued: bool,
}
/// Nested message and enum types in `Change`.
pub mod change {
    /// A reported value can be in one of the following states:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The element exists and its full value is included in data.
        Exists = 0,
        /// The element does not exist.
        DoesNotExist = 1,
        /// Element may or may not exist. Used only for initial state delivery when
        /// the client is not interested in fetching the initial state. See the
        /// "Initial State" section above.
        InitialStateSkipped = 2,
        /// The element may exist, but some error has occurred. More information is
        /// available in the data field - the value is a serialized Status
        /// proto (from \[google.rpc.Status][\])
        Error = 3,
    }
}
/// Generated client implementations.
pub mod watcher_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The service that a client uses to connect to the watcher system.
    /// The errors returned by the service are in the canonical error space,
    /// see [google.rpc.Code][].
    #[derive(Debug, Clone)]
    pub struct WatcherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WatcherClient<T>
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
        ) -> WatcherClient<InterceptedService<T, F>>
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
            WatcherClient::new(InterceptedService::new(inner, interceptor))
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
        /// Start a streaming RPC to get watch information from the server.
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ChangeBatch>>,
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
                "/google.watcher.v1.Watcher/Watch",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
