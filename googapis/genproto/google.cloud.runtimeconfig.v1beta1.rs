/// A RuntimeConfig resource is the primary resource in the Cloud RuntimeConfig
/// service. A RuntimeConfig resource consists of metadata and a hierarchy of
/// variables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeConfig {
    /// The resource name of a runtime config. The name must have the format:
    ///
    ///     projects/[PROJECT_ID]/configs/[CONFIG_NAME]
    ///
    /// The `[PROJECT_ID]` must be a valid project ID, and `[CONFIG_NAME]` is an
    /// arbitrary name that matches RFC 1035 segment specification. The length of
    /// `[CONFIG_NAME]` must be less than 64 bytes.
    ///
    /// You pick the RuntimeConfig resource name, but the server will validate that
    /// the name adheres to this format. After you create the resource, you cannot
    /// change the resource's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An optional description of the RuntimeConfig object.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// Describes a single variable within a RuntimeConfig resource.
/// The name denotes the hierarchical variable name. For example,
/// `ports/serving_port` is a valid variable name. The variable value is an
/// opaque string and only leaf variables can have values (that is, variables
/// that do not have any child variables).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    /// The name of the variable resource, in the format:
    ///
    ///     projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]
    ///
    /// The `[PROJECT_ID]` must be a valid project ID, `[CONFIG_NAME]` must be a
    /// valid RuntimeConfig reource and `[VARIABLE_NAME]` follows Unix file system
    /// file path naming.
    ///
    /// The `[VARIABLE_NAME]` can contain ASCII letters, numbers, slashes and
    /// dashes. Slashes are used as path element separators and are not part of the
    /// `[VARIABLE_NAME]` itself, so `[VARIABLE_NAME]` must contain at least one
    /// non-slash character. Multiple slashes are coalesced into single slash
    /// character. Each path segment should follow RFC 1035 segment specification.
    /// The length of a `[VARIABLE_NAME]` must be less than 256 bytes.
    ///
    /// Once you create a variable, you cannot change the variable name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// [Output Only] The time of the last variable update.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// [Ouput only] The current state of the variable. The variable state
    /// indicates the outcome of the `variables().watch` call and is visible
    /// through the `get` and `list` calls.
    #[prost(enumeration = "VariableState", tag = "4")]
    pub state: i32,
    /// The value of the variable. It can be either a binary or a string
    /// value. You must specify one of either `value` or `text`. Specifying both
    /// will cause the server to return an error.
    #[prost(oneof = "variable::Contents", tags = "2, 5")]
    pub contents: ::core::option::Option<variable::Contents>,
}
/// Nested message and enum types in `Variable`.
pub mod variable {
    /// The value of the variable. It can be either a binary or a string
    /// value. You must specify one of either `value` or `text`. Specifying both
    /// will cause the server to return an error.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Contents {
        /// The binary value of the variable. The length of the value must be less
        /// than 4096 bytes. Empty values are also accepted. The value must be
        /// base64 encoded. Only one of `value` or `text` can be set.
        #[prost(bytes, tag = "2")]
        Value(::prost::alloc::vec::Vec<u8>),
        /// The string value of the variable. The length of the value must be less
        /// than 4096 bytes. Empty values are also accepted. For example,
        /// `text: "my text value"`. The string must be valid UTF-8.
        #[prost(string, tag = "5")]
        Text(::prost::alloc::string::String),
    }
}
/// The condition that a Waiter resource is waiting for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndCondition {
    /// The condition oneof holds the available condition types for this
    /// EndCondition. Currently, the only available type is Cardinality.
    #[prost(oneof = "end_condition::Condition", tags = "1")]
    pub condition: ::core::option::Option<end_condition::Condition>,
}
/// Nested message and enum types in `EndCondition`.
pub mod end_condition {
    /// A Cardinality condition for the Waiter resource. A cardinality condition is
    /// met when the number of variables under a specified path prefix reaches a
    /// predefined number. For example, if you set a Cardinality condition where
    /// the `path` is set to `/foo` and the number of paths is set to 2, the
    /// following variables would meet the condition in a RuntimeConfig resource:
    ///
    /// + `/foo/variable1 = "value1"`
    /// + `/foo/variable2 = "value2"`
    /// + `/bar/variable3 = "value3"`
    ///
    /// It would not would not satisify the same condition with the `number` set to
    /// 3, however, because there is only 2 paths that start with `/foo`.
    /// Cardinality conditions are recursive; all subtrees under the specific
    /// path prefix are counted.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cardinality {
        /// The root of the variable subtree to monitor. For example, `/foo`.
        #[prost(string, tag = "1")]
        pub path: ::prost::alloc::string::String,
        /// The number variables under the `path` that must exist to meet this
        /// condition. Defaults to 1 if not specified.
        #[prost(int32, tag = "2")]
        pub number: i32,
    }
    /// The condition oneof holds the available condition types for this
    /// EndCondition. Currently, the only available type is Cardinality.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// The cardinality of the `EndCondition`.
        #[prost(message, tag = "1")]
        Cardinality(Cardinality),
    }
}
/// A Waiter resource waits for some end condition within a RuntimeConfig
/// resource to be met before it returns. For example, assume you have a
/// distributed system where each node writes to a Variable resource indidicating
/// the node's readiness as part of the startup process.
///
/// You then configure a Waiter resource with the success condition set to wait
/// until some number of nodes have checked in. Afterwards, your application
/// runs some arbitrary code after the condition has been met and the waiter
/// returns successfully.
///
/// Once created, a Waiter resource is immutable.
///
/// To learn more about using waiters, read the
/// [Creating a
/// Waiter](/deployment-manager/runtime-configurator/creating-a-waiter)
/// documentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waiter {
    /// The name of the Waiter resource, in the format:
    ///
    ///     projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]
    ///
    /// The `[PROJECT_ID]` must be a valid Google Cloud project ID,
    /// the `[CONFIG_NAME]` must be a valid RuntimeConfig resource, the
    /// `[WAITER_NAME]` must match RFC 1035 segment specification, and the length
    /// of `[WAITER_NAME]` must be less than 64 bytes.
    ///
    /// After you create a Waiter resource, you cannot change the resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// [Required] Specifies the timeout of the waiter in seconds, beginning from
    /// the instant that `waiters().create` method is called. If this time elapses
    /// before the success or failure conditions are met, the waiter fails and sets
    /// the `error` code to `DEADLINE_EXCEEDED`.
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// [Optional] The failure condition of this waiter. If this condition is met,
    /// `done` will be set to `true` and the `error` code will be set to `ABORTED`.
    /// The failure condition takes precedence over the success condition. If both
    /// conditions are met, a failure will be indicated. This value is optional; if
    /// no failure condition is set, the only failure scenario will be a timeout.
    #[prost(message, optional, tag = "3")]
    pub failure: ::core::option::Option<EndCondition>,
    /// [Required] The success condition. If this condition is met, `done` will be
    /// set to `true` and the `error` value will remain unset. The failure
    /// condition takes precedence over the success condition. If both conditions
    /// are met, a failure will be indicated.
    #[prost(message, optional, tag = "4")]
    pub success: ::core::option::Option<EndCondition>,
    /// [Output Only] The instant at which this Waiter resource was created. Adding
    /// the value of `timeout` to this instant yields the timeout deadline for the
    /// waiter.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// [Output Only] If the value is `false`, it means the waiter is still waiting
    /// for one of its conditions to be met.
    ///
    /// If true, the waiter has finished. If the waiter finished due to a timeout
    /// or failure, `error` will be set.
    #[prost(bool, tag = "6")]
    pub done: bool,
    /// [Output Only] If the waiter ended due to a failure or timeout, this value
    /// will be set.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// The `VariableState` describes the last known state of the variable and is
/// used during a `variables().watch` call to distinguish the state of the
/// variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableState {
    /// Default variable state.
    Unspecified = 0,
    /// The variable was updated, while `variables().watch` was executing.
    Updated = 1,
    /// The variable was deleted, while `variables().watch` was executing.
    Deleted = 2,
}
/// Request for the `ListConfigs()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfigsRequest {
    /// The [project
    /// ID](https://support.google.com/cloud/answer/6158840?hl=en&ref_topic=6158848)
    /// for this request, in the format `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Specifies the number of results to return per page. If there are fewer
    /// elements than the specified number, returns all elements.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Specifies a page token to use. Set `pageToken` to a `nextPageToken`
    /// returned by a previous list request to get the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListConfigs()` returns the following response. The order of returned
/// objects is arbitrary; that is, it is not ordered in any particular way.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfigsResponse {
    /// A list of the configurations in the project. The order of returned
    /// objects is arbitrary; that is, it is not ordered in any particular way.
    #[prost(message, repeated, tag = "1")]
    pub configs: ::prost::alloc::vec::Vec<RuntimeConfig>,
    /// This token allows you to get the next page of results for list requests.
    /// If the number of results is larger than `pageSize`, use the `nextPageToken`
    /// as a value for the query parameter `pageToken` in the next list request.
    /// Subsequent list requests will have their own `nextPageToken` to continue
    /// paging through the results
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Gets a RuntimeConfig resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
    /// The name of the RuntimeConfig resource to retrieve, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Creates a RuntimeConfig resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConfigRequest {
    /// The [project
    /// ID](https://support.google.com/cloud/answer/6158840?hl=en&ref_topic=6158848)
    /// for this request, in the format `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The RuntimeConfig to create.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<RuntimeConfig>,
    /// An optional but recommended unique `request_id`. If the server
    /// receives two `create()` requests  with the same
    /// `request_id`, then the second request will be ignored and the
    /// first resource created and stored in the backend is returned.
    /// Empty `request_id` fields are ignored.
    ///
    /// It is responsibility of the client to ensure uniqueness of the
    /// `request_id` strings.
    ///
    /// `request_id` strings are limited to 64 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `UpdateConfig()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigRequest {
    /// The name of the RuntimeConfig resource to update, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The config resource to update.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<RuntimeConfig>,
}
/// Request for the `DeleteConfig()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConfigRequest {
    /// The RuntimeConfig resource to delete, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListVariables()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVariablesRequest {
    /// The path to the RuntimeConfig resource for which you want to list
    /// variables. The configuration must exist beforehand; the path must by in the
    /// format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Filters variables by matching the specified filter. For example:
    ///
    /// `projects/example-project/config/[CONFIG_NAME]/variables/example-variable`.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the number of results to return per page. If there are fewer
    /// elements than the specified number, returns all elements.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Specifies a page token to use. Set `pageToken` to a `nextPageToken`
    /// returned by a previous list request to get the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// The flag indicates whether the user wants to return values of variables.
    /// If true, then only those variables that user has IAM GetVariable permission
    /// will be returned along with their values.
    #[prost(bool, tag = "5")]
    pub return_values: bool,
}
/// Response for the `ListVariables()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVariablesResponse {
    /// A list of variables and their values. The order of returned variable
    /// objects is arbitrary.
    #[prost(message, repeated, tag = "1")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    /// This token allows you to get the next page of results for list requests.
    /// If the number of results is larger than `pageSize`, use the `nextPageToken`
    /// as a value for the query parameter `pageToken` in the next list request.
    /// Subsequent list requests will have their own `nextPageToken` to continue
    /// paging through the results
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `WatchVariable()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchVariableRequest {
    /// The name of the variable to watch, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If specified, checks the current timestamp of the variable and if the
    /// current timestamp is newer than `newerThan` timestamp, the method returns
    /// immediately.
    ///
    /// If not specified or the variable has an older timestamp, the watcher waits
    /// for a the value to change before returning.
    #[prost(message, optional, tag = "4")]
    pub newer_than: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `GetVariable()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVariableRequest {
    /// The name of the variable to return, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIBLE_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateVariable()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVariableRequest {
    /// The path to the RutimeConfig resource that this variable should belong to.
    /// The configuration must exist beforehand; the path must by in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The variable to create.
    #[prost(message, optional, tag = "2")]
    pub variable: ::core::option::Option<Variable>,
    /// An optional but recommended unique `request_id`. If the server
    /// receives two `create()` requests  with the same
    /// `request_id`, then the second request will be ignored and the
    /// first resource created and stored in the backend is returned.
    /// Empty `request_id` fields are ignored.
    ///
    /// It is responsibility of the client to ensure uniqueness of the
    /// `request_id` strings.
    ///
    /// `request_id` strings are limited to 64 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for the `UpdateVariable()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVariableRequest {
    /// The name of the variable to update, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The variable to update.
    #[prost(message, optional, tag = "2")]
    pub variable: ::core::option::Option<Variable>,
}
/// Request for the `DeleteVariable()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVariableRequest {
    /// The name of the variable to delete, in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` to recursively delete multiple variables with the same
    /// prefix.
    #[prost(bool, tag = "2")]
    pub recursive: bool,
}
/// Request for the `ListWaiters()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWaitersRequest {
    /// The path to the configuration for which you want to get a list of waiters.
    /// The configuration must exist beforehand; the path must by in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Specifies the number of results to return per page. If there are fewer
    /// elements than the specified number, returns all elements.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Specifies a page token to use. Set `pageToken` to a `nextPageToken`
    /// returned by a previous list request to get the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the `ListWaiters()` method.
/// Order of returned waiter objects is arbitrary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWaitersResponse {
    /// Found waiters in the project.
    #[prost(message, repeated, tag = "1")]
    pub waiters: ::prost::alloc::vec::Vec<Waiter>,
    /// This token allows you to get the next page of results for list requests.
    /// If the number of results is larger than `pageSize`, use the `nextPageToken`
    /// as a value for the query parameter `pageToken` in the next list request.
    /// Subsequent list requests will have their own `nextPageToken` to continue
    /// paging through the results
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `GetWaiter()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWaiterRequest {
    /// The fully-qualified name of the Waiter resource object to retrieve, in the
    /// format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `CreateWaiter()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWaiterRequest {
    /// The path to the configuration that will own the waiter.
    /// The configuration must exist beforehand; the path must by in the format:
    ///
    /// `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The Waiter resource to create.
    #[prost(message, optional, tag = "2")]
    pub waiter: ::core::option::Option<Waiter>,
    /// An optional but recommended unique `request_id`. If the server
    /// receives two `create()` requests  with the same
    /// `request_id`, then the second request will be ignored and the
    /// first resource created and stored in the backend is returned.
    /// Empty `request_id` fields are ignored.
    ///
    /// It is responsibility of the client to ensure uniqueness of the
    /// `request_id` strings.
    ///
    /// `request_id` strings are limited to 64 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for the `DeleteWaiter()` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWaiterRequest {
    /// The Waiter resource to delete, in the format:
    ///
    ///  `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod runtime_config_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " RuntimeConfig API represents configuration objects and operations on those"]
    #[doc = " configuration objects."]
    #[doc = " RuntimeConfig objects consist of Variables logically grouped in the those"]
    #[doc = " objects."]
    #[doc = " Variables are simple key-value pairs. Variables can be watched for changes or"]
    #[doc = " deletions. Variable key can be hieararchical, e.g. ports/serving_port,"]
    #[doc = " ports/monitoring_port, etc. Variable names can be hierarchical. No variable"]
    #[doc = " name can be prefix of another."]
    #[doc = " Config objects represent logical containers for variables, e.g. flags,"]
    #[doc = " passwords, etc."]
    #[derive(Debug, Clone)]
    pub struct RuntimeConfigManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RuntimeConfigManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RuntimeConfigManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RuntimeConfigManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists all the RuntimeConfig resources within project."]
        pub async fn list_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConfigsRequest>,
        ) -> Result<tonic::Response<super::ListConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/ListConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a RuntimeConfig resource."]
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::RuntimeConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/GetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new RuntimeConfig resource. The configuration name must be"]
        #[doc = " unique within project."]
        pub async fn create_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConfigRequest>,
        ) -> Result<tonic::Response<super::RuntimeConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/CreateConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a RuntimeConfig resource. The configuration must exist beforehand."]
        pub async fn update_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConfigRequest>,
        ) -> Result<tonic::Response<super::RuntimeConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/UpdateConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a RuntimeConfig resource."]
        pub async fn delete_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/DeleteConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists variables within given a configuration, matching any provided"]
        #[doc = " filters. This only lists variable names, not the values, unless"]
        #[doc = " `return_values` is true, in which case only variables that user has IAM"]
        #[doc = " permission to GetVariable will be returned."]
        pub async fn list_variables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVariablesRequest>,
        ) -> Result<tonic::Response<super::ListVariablesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/ListVariables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a single variable."]
        pub async fn get_variable(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVariableRequest>,
        ) -> Result<tonic::Response<super::Variable>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/GetVariable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Watches a specific variable and waits for a change in the variable's value."]
        #[doc = " When there is a change, this method returns the new value or times out."]
        #[doc = ""]
        #[doc = " If a variable is deleted while being watched, the `variableState` state is"]
        #[doc = " set to `DELETED` and the method returns the last known variable `value`."]
        #[doc = ""]
        #[doc = " If you set the deadline for watching to a larger value than internal"]
        #[doc = " timeout (60 seconds), the current variable value is returned and the"]
        #[doc = " `variableState` will be `VARIABLE_STATE_UNSPECIFIED`."]
        #[doc = ""]
        #[doc = " To learn more about creating a watcher, read the"]
        #[doc = " [Watching a Variable for"]
        #[doc = " Changes](/deployment-manager/runtime-configurator/watching-a-variable)"]
        #[doc = " documentation."]
        pub async fn watch_variable(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchVariableRequest>,
        ) -> Result<tonic::Response<super::Variable>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/WatchVariable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a variable within the given configuration. You cannot create"]
        #[doc = " a variable with a name that is a prefix of an existing variable name, or a"]
        #[doc = " name that has an existing variable name as a prefix."]
        #[doc = ""]
        #[doc = " To learn more about creating a variable, read the"]
        #[doc = " [Setting and Getting"]
        #[doc = " Data](/deployment-manager/runtime-configurator/set-and-get-variables)"]
        #[doc = " documentation."]
        pub async fn create_variable(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVariableRequest>,
        ) -> Result<tonic::Response<super::Variable>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/CreateVariable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing variable with a new value."]
        pub async fn update_variable(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVariableRequest>,
        ) -> Result<tonic::Response<super::Variable>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/UpdateVariable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a variable or multiple variables."]
        #[doc = ""]
        #[doc = " If you specify a variable name, then that variable is deleted. If you"]
        #[doc = " specify a prefix and `recursive` is true, then all variables with that"]
        #[doc = " prefix are deleted. You must set a `recursive` to true if you delete"]
        #[doc = " variables by prefix."]
        pub async fn delete_variable(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVariableRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/DeleteVariable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List waiters within the given configuration."]
        pub async fn list_waiters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWaitersRequest>,
        ) -> Result<tonic::Response<super::ListWaitersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/ListWaiters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a single waiter."]
        pub async fn get_waiter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWaiterRequest>,
        ) -> Result<tonic::Response<super::Waiter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/GetWaiter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Waiter resource. This operation returns a long-running Operation"]
        #[doc = " resource which can be polled for completion. However, a waiter with the"]
        #[doc = " given name will exist (and can be retrieved) prior to the operation"]
        #[doc = " completing. If the operation fails, the failed Waiter resource will"]
        #[doc = " still exist and must be deleted prior to subsequent creation attempts."]
        pub async fn create_waiter(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWaiterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/CreateWaiter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the waiter with the specified name."]
        pub async fn delete_waiter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWaiterRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.runtimeconfig.v1beta1.RuntimeConfigManager/DeleteWaiter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
