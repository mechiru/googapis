/// Protobuf schema is an API presentation the proto buffer schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSchema {
    /// Descriptor for input message. The descriptor has to be self contained,
    /// including all the nested types, excepted for proto buffer well known types
    /// (https://developers.google.com/protocol-buffers/docs/reference/google.protobuf)
    /// and zetasql public protos
    /// (https://github.com/google/zetasql/tree/master/zetasql/public/proto).
    #[prost(message, optional, tag = "1")]
    pub proto_descriptor: ::core::option::Option<::prost_types::DescriptorProto>,
}
/// Protobuf rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoRows {
    /// A sequence of rows serialized as a Protocol Buffer.
    ///
    /// See https://developers.google.com/protocol-buffers/docs/overview for more
    /// information on deserializing this field.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub serialized_rows: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Schema of a table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
}
/// A field in TableSchema
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFieldSchema {
    /// Required. The field name. The name must contain only letters (a-z, A-Z),
    /// numbers (0-9), or underscores (_), and must start with a letter or
    /// underscore. The maximum length is 128 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The field data type.
    #[prost(enumeration = "table_field_schema::Type", tag = "2")]
    pub r#type: i32,
    /// Optional. The field mode. The default value is NULLABLE.
    #[prost(enumeration = "table_field_schema::Mode", tag = "3")]
    pub mode: i32,
    /// Optional. Describes the nested schema fields if the type property is set to STRUCT.
    #[prost(message, repeated, tag = "4")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
    /// Optional. The field description. The maximum length is 1,024 characters.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TableFieldSchema`.
pub mod table_field_schema {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Illegal value
        Unspecified = 0,
        /// 64K, UTF8
        String = 1,
        /// 64-bit signed
        Int64 = 2,
        /// 64-bit IEEE floating point
        Double = 3,
        /// Aggregate type
        Struct = 4,
        /// 64K, Binary
        Bytes = 5,
        /// 2-valued
        Bool = 6,
        /// 64-bit signed usec since UTC epoch
        Timestamp = 7,
        /// Civil date - Year, Month, Day
        Date = 8,
        /// Civil time - Hour, Minute, Second, Microseconds
        Time = 9,
        /// Combination of civil date and civil time
        Datetime = 10,
        /// Geography object (go/googlesql_geography)
        Geography = 11,
        /// Numeric value (go/googlesql_numeric)
        Numeric = 12,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Illegal value
        Unspecified = 0,
        Nullable = 1,
        Required = 2,
        Repeated = 3,
    }
}
/// Information about a single stream that gets data inside the storage system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "write_stream::Type", tag = "2")]
    pub r#type: i32,
    /// Output only. Create time of the stream.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Commit time of the stream.
    /// If a stream is of `COMMITTED` type, then it will have a commit_time same as
    /// `create_time`. If the stream is of `PENDING` type, commit_time being empty
    /// means it is not committed.
    #[prost(message, optional, tag = "4")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The schema of the destination table. It is only returned in
    /// `CreateWriteStream` response. Caller should generate data that's
    /// compatible with this schema to send in initial `AppendRowsRequest`.
    /// The table schema could go out of date during the life time of the stream.
    #[prost(message, optional, tag = "5")]
    pub table_schema: ::core::option::Option<TableSchema>,
    /// Id set by client to annotate its identity.
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WriteStream`.
pub mod write_stream {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unknown type.
        Unspecified = 0,
        /// Data will commit automatically and appear as soon as the write is
        /// acknowledged.
        Committed = 1,
        /// Data is invisible until the stream is committed.
        Pending = 2,
        /// Data is only visible up to the offset to which it was flushed.
        Buffered = 3,
    }
}
/// Request message for `CreateWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWriteStreamRequest {
    /// Required. Reference to the table to which the stream belongs, in the format
    /// of `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Stream to be created.
    #[prost(message, optional, tag = "2")]
    pub write_stream: ::core::option::Option<WriteStream>,
}
/// Request message for `AppendRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsRequest {
    /// Required. The stream that is the target of the append operation. This value must be
    /// specified for the initial request. If subsequent requests specify the
    /// stream name, it must equal to the value provided in the first request.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// Optional. If present, the write is only performed if the next append offset is same
    /// as the provided value. If not present, the write is performed at the
    /// current end of stream.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<i64>,
    /// Only initial request setting is respected. If true, drop unknown input
    /// fields. Otherwise, the extra fields will cause append to fail. Default
    /// value is false.
    #[prost(bool, tag = "5")]
    pub ignore_unknown_fields: bool,
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[prost(oneof = "append_rows_request::Rows", tags = "4")]
    pub rows: ::core::option::Option<append_rows_request::Rows>,
}
/// Nested message and enum types in `AppendRowsRequest`.
pub mod append_rows_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoData {
        /// Proto schema used to serialize the data.
        #[prost(message, optional, tag = "1")]
        pub writer_schema: ::core::option::Option<super::ProtoSchema>,
        /// Serialized row data in protobuf message format.
        #[prost(message, optional, tag = "2")]
        pub rows: ::core::option::Option<super::ProtoRows>,
    }
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        #[prost(message, tag = "4")]
        ProtoRows(ProtoData),
    }
}
/// Response message for `AppendRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsResponse {
    /// If backend detects a schema update, pass it to user so that user can
    /// use it to input new type of message. It will be empty when there is no
    /// schema updates.
    #[prost(message, optional, tag = "3")]
    pub updated_schema: ::core::option::Option<TableSchema>,
    #[prost(oneof = "append_rows_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<append_rows_response::Response>,
}
/// Nested message and enum types in `AppendRowsResponse`.
pub mod append_rows_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The row offset at which the last append occurred.
        #[prost(int64, tag = "1")]
        Offset(i64),
        /// Error in case of append failure. If set, it means rows are not accepted
        /// into the system. Users can retry within the same connection.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::super::rpc::Status),
    }
}
/// Request message for `GetWriteStreamRequest`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWriteStreamRequest {
    /// Required. Name of the stream to get, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `BatchCommitWriteStreams`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsRequest {
    /// Required. Parent table that all the streams should belong to, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The group of streams that will be committed atomically.
    #[prost(string, repeated, tag = "2")]
    pub write_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for `BatchCommitWriteStreams`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsResponse {
    /// The time at which streams were committed in microseconds granularity.
    #[prost(message, optional, tag = "1")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for invoking `FinalizeWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamRequest {
    /// Required. Name of the stream to finalize, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for `FinalizeWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamResponse {
    /// Number of rows in the finalized stream.
    #[prost(int64, tag = "1")]
    pub row_count: i64,
}
/// Request message for `FlushRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsRequest {
    /// Required. The stream that is the target of the flush operation.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// Ending offset of the flush operation. Rows before this offset(including
    /// this offset) will be flushed.
    #[prost(int64, tag = "2")]
    pub offset: i64,
}
/// Respond message for `FlushRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsResponse {
    /// The rows before this offset (including this offset) are flushed.
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
#[doc = r" Generated client implementations."]
pub mod big_query_write_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " BigQuery Write API."]
    #[doc = ""]
    #[doc = " The Write API can be used to write data to BigQuery."]
    pub struct BigQueryWriteClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigQueryWriteClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a write stream to the given table."]
        pub async fn create_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/CreateWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Appends data to the given stream."]
        #[doc = ""]
        #[doc = " If `offset` is specified, the `offset` is checked against the end of"]
        #[doc = " stream. The server returns `OUT_OF_RANGE` in `AppendRowsResponse` if an"]
        #[doc = " attempt is made to append to an offset beyond the current end of the stream"]
        #[doc = " or `ALREADY_EXISTS` if user provids an `offset` that has already been"]
        #[doc = " written to. User can retry with adjusted offset within the same RPC"]
        #[doc = " stream. If `offset` is not specified, append happens at the end of the"]
        #[doc = " stream."]
        #[doc = ""]
        #[doc = " The response contains the offset at which the append happened. Responses"]
        #[doc = " are received in the same order in which requests are sent. There will be"]
        #[doc = " one response for each successful request. If the `offset` is not set in"]
        #[doc = " response, it means append didn't happen due to some errors. If one request"]
        #[doc = " fails, all the subsequent requests will also fail until a success request"]
        #[doc = " is made again."]
        #[doc = ""]
        #[doc = " If the stream is of `PENDING` type, data will only be available for read"]
        #[doc = " operations after the stream is committed."]
        pub async fn append_rows(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AppendRowsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AppendRowsResponse>>,
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
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/AppendRows",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Gets a write stream."]
        pub async fn get_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/GetWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finalize a write stream so that no new data can be appended to the"]
        #[doc = " stream."]
        pub async fn finalize_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeWriteStreamRequest>,
        ) -> Result<tonic::Response<super::FinalizeWriteStreamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/FinalizeWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Atomically commits a group of `PENDING` streams that belong to the same"]
        #[doc = " `parent` table."]
        #[doc = " Streams must be finalized before commit and cannot be committed multiple"]
        #[doc = " times. Once a stream is committed, data in the stream becomes available"]
        #[doc = " for read operations."]
        pub async fn batch_commit_write_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCommitWriteStreamsRequest>,
        ) -> Result<tonic::Response<super::BatchCommitWriteStreamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/BatchCommitWriteStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Flushes rows to a BUFFERED stream."]
        #[doc = " If users are appending rows to BUFFERED stream, flush operation is"]
        #[doc = " required in order for the rows to become available for reading. A"]
        #[doc = " Flush operation flushes up to any previously flushed offset in a BUFFERED"]
        #[doc = " stream, to the offset specified in the request."]
        pub async fn flush_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::FlushRowsRequest>,
        ) -> Result<tonic::Response<super::FlushRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1alpha2.BigQueryWrite/FlushRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigQueryWriteClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigQueryWriteClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigQueryWriteClient {{ ... }}")
        }
    }
}
