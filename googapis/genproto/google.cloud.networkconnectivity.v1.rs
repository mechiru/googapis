/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A hub is a collection of spokes. A single hub can contain spokes from
/// multiple regions. However, all of a hub's spokes must be associated with
/// resources that reside in the same VPC network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hub {
    /// Immutable. The name of the hub. Hub names must be unique. They use the
    /// following form:
    ///     `projects/{project_number}/locations/global/hubs/{hub_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the hub was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the hub was last updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key:value format. For more information about labels, see
    /// [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// An optional description of the hub.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the hub. This value is unique across all hub
    /// resources. If a hub is deleted and another with the same name is created,
    /// the new hub is assigned a different unique_id.
    #[prost(string, tag="8")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this hub.
    #[prost(enumeration="State", tag="9")]
    pub state: i32,
    /// The VPC network associated with this hub's spokes. All of the VPN tunnels,
    /// VLAN attachments, and router appliance instances referenced by this hub's
    /// spokes must belong to this VPC network.
    ///
    /// This field is read-only. Network Connectivity Center automatically
    /// populates it based on the set of spokes attached to the hub.
    #[prost(message, repeated, tag="10")]
    pub routing_vpcs: ::prost::alloc::vec::Vec<RoutingVpc>,
}
/// RoutingVPC contains information about the VPC network that is associated with
/// a hub's spokes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingVpc {
    /// The URI of the VPC network.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
}
/// A spoke represents a connection between your Google Cloud network resources
/// and a non-Google-Cloud network.
///
/// When you create a spoke, you associate it with a hub. You must also identify
/// a value for exactly one of the following fields:
///
/// * linked_vpn_tunnels
/// * linked_interconnect_attachments
/// * linked_router_appliance_instances
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spoke {
    /// Immutable. The name of the spoke. Spoke names must be unique. They use the
    /// following form:
    ///     `projects/{project_number}/locations/{region}/spokes/{spoke_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the spoke was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the spoke was last updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key:value format. For more information about labels, see
    /// [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// An optional description of the spoke.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The name of the hub that this spoke is attached to.
    #[prost(string, tag="6")]
    pub hub: ::prost::alloc::string::String,
    /// VPN tunnels that are associated with the spoke.
    #[prost(message, optional, tag="17")]
    pub linked_vpn_tunnels: ::core::option::Option<LinkedVpnTunnels>,
    /// VLAN attachments that are associated with the spoke.
    #[prost(message, optional, tag="18")]
    pub linked_interconnect_attachments: ::core::option::Option<LinkedInterconnectAttachments>,
    /// Router appliance instances that are associated with the spoke.
    #[prost(message, optional, tag="19")]
    pub linked_router_appliance_instances: ::core::option::Option<LinkedRouterApplianceInstances>,
    /// Output only. The Google-generated UUID for the spoke. This value is unique across all
    /// spoke resources. If a spoke is deleted and another with the same name is
    /// created, the new spoke is assigned a different unique_id.
    #[prost(string, tag="11")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this spoke.
    #[prost(enumeration="State", tag="15")]
    pub state: i32,
}
/// Request for \[HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the results listed in the response.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for \[HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsResponse {
    /// The requested hubs.
    #[prost(message, repeated, tag="1")]
    pub hubs: ::prost::alloc::vec::Vec<Hub>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[HubService.GetHub][google.cloud.networkconnectivity.v1.HubService.GetHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHubRequest {
    /// Required. The name of the hub resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[HubService.CreateHub][google.cloud.networkconnectivity.v1.HubService.CreateHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHubRequest {
    /// Required. The parent resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A unique identifier for the hub.
    #[prost(string, tag="2")]
    pub hub_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new hub.
    #[prost(message, optional, tag="3")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[HubService.UpdateHub][google.cloud.networkconnectivity.v1.HubService.UpdateHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHubRequest {
    /// Optional. In the case of an update to an existing hub, field mask is used to specify
    /// the fields to be overwritten. The fields specified in the update_mask are
    /// relative to the resource, not the full request. A field is overwritten if
    /// it is in the mask. If the user does not provide a mask, then all fields are
    /// overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the hub should be in after the update.
    #[prost(message, optional, tag="2")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.DeleteHub][google.cloud.networkconnectivity.v1.HubService.DeleteHub\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHubRequest {
    /// Required. The name of the hub to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesRequest {
    /// Required. The parent resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the results listed in the response.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response for \[HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesResponse {
    /// The requested spokes.
    #[prost(message, repeated, tag="1")]
    pub spokes: ::prost::alloc::vec::Vec<Spoke>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for \[HubService.GetSpoke][google.cloud.networkconnectivity.v1.HubService.GetSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpokeRequest {
    /// Required. The name of the spoke resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[HubService.CreateSpoke][google.cloud.networkconnectivity.v1.HubService.CreateSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpokeRequest {
    /// Required. The parent resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique id for the spoke to create.
    #[prost(string, tag="2")]
    pub spoke_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new spoke.
    #[prost(message, optional, tag="3")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[HubService.UpdateSpoke][google.cloud.networkconnectivity.v1.HubService.UpdateSpoke\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpokeRequest {
    /// Optional. In the case of an update to an existing spoke, field mask is used to
    /// specify the fields to be overwritten. The fields specified in the
    /// update_mask are relative to the resource, not the full request. A field is
    /// overwritten if it is in the mask. If the user does not provide a mask, then
    /// all fields are overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the spoke should be in after the update.
    #[prost(message, optional, tag="2")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.DeleteSpoke][google.cloud.networkconnectivity.v1.HubService.DeleteSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpokeRequest {
    /// Required. The name of the spoke to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique request ID (optional). If you specify this ID, you can use it
    /// in cases when you need to retry your request. When you need to retry, this
    /// ID lets the server know that it can ignore the request if it has already
    /// been completed. The server guarantees that for at least 60 minutes after
    /// the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A collection of Cloud VPN tunnel resources. These resources should be
/// redundant HA VPN tunnels that all advertise the same prefixes to Google
/// Cloud. Alternatively, in a passive/active configuration, all tunnels
/// should be capable of advertising the same prefixes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedVpnTunnels {
    /// The URIs of linked VPN tunnel resources.
    #[prost(string, repeated, tag="1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. This field is set to false by default, but you must set it
    /// to true. Note that data transfer is available only in supported locations.
    #[prost(bool, tag="2")]
    pub site_to_site_data_transfer: bool,
}
/// A collection of VLAN attachment resources. These resources should
/// be redundant attachments that all advertise the same prefixes to Google
/// Cloud. Alternatively, in active/passive configurations, all attachments
/// should be capable of advertising the same prefixes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedInterconnectAttachments {
    /// The URIs of linked interconnect attachment resources
    #[prost(string, repeated, tag="1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. This field is set to false by default, but you must set it
    /// to true. Note that data transfer is available only in supported locations.
    #[prost(bool, tag="2")]
    pub site_to_site_data_transfer: bool,
}
/// A collection of router appliance instances. If you have multiple router
/// appliance instances connected to the same site, they should all be attached
/// to the same spoke.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedRouterApplianceInstances {
    /// The list of router appliance instances.
    #[prost(message, repeated, tag="1")]
    pub instances: ::prost::alloc::vec::Vec<RouterApplianceInstance>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. This field is set to false by default, but you must set it
    /// to true. Note that data transfer is available only in supported locations.
    #[prost(bool, tag="2")]
    pub site_to_site_data_transfer: bool,
}
/// A router appliance instance is a Compute Engine virtual machine (VM) instance
/// that acts as a BGP speaker. A router appliance instance is specified by the
/// URI of the VM and the internal IP address of one of the VM's network
/// interfaces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterApplianceInstance {
    /// The URI of the VM.
    #[prost(string, tag="1")]
    pub virtual_machine: ::prost::alloc::string::String,
    /// The IP address on the VM to use for peering.
    #[prost(string, tag="3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// The State enum represents the lifecycle stage of a Network Connectivity
/// Center resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// No state information available
    Unspecified = 0,
    /// The resource's create operation is in progress
    Creating = 1,
    /// The resource is active
    Active = 2,
    /// The resource's Delete operation is in progress
    Deleting = 3,
}
/// Generated client implementations.
pub mod hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Network Connectivity Center is a hub-and-spoke abstraction for network
    /// connectivity management in Google Cloud. It reduces operational complexity
    /// through a simple, centralized connectivity management model.
    #[derive(Debug, Clone)]
    pub struct HubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> HubServiceClient<T>
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
        ) -> HubServiceClient<InterceptedService<T, F>>
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
            HubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists hubs in a given project.
        pub async fn list_hubs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHubsRequest>,
        ) -> Result<tonic::Response<super::ListHubsResponse>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/ListHubs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details about the specified hub.
        pub async fn get_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHubRequest>,
        ) -> Result<tonic::Response<super::Hub>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new hub in the specified project.
        pub async fn create_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/CreateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the description and/or labels of the specified hub.
        pub async fn update_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/UpdateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified hub.
        pub async fn delete_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHubRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/DeleteHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the spokes in the specified project and location.
        pub async fn list_spokes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpokesRequest>,
        ) -> Result<tonic::Response<super::ListSpokesResponse>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/ListSpokes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details about the specified spoke.
        pub async fn get_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpokeRequest>,
        ) -> Result<tonic::Response<super::Spoke>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a spoke in the specified project and location.
        pub async fn create_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/CreateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of the specified spoke.
        pub async fn update_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/UpdateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified spoke.
        pub async fn delete_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpokeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networkconnectivity.v1.HubService/DeleteSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
