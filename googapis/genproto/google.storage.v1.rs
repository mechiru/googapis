/// A bucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    /// Access controls on the bucket.
    #[prost(message, repeated, tag="1")]
    pub acl: ::prost::alloc::vec::Vec<BucketAccessControl>,
    /// Default access controls to apply to new objects when no ACL is provided.
    #[prost(message, repeated, tag="2")]
    pub default_object_acl: ::prost::alloc::vec::Vec<ObjectAccessControl>,
    /// The bucket's lifecycle configuration. See
    /// \[<https://developers.google.com/storage/docs/lifecycle\]Lifecycle> Management]
    /// for more information.
    #[prost(message, optional, tag="3")]
    pub lifecycle: ::core::option::Option<bucket::Lifecycle>,
    /// The creation time of the bucket in
    /// \[<https://tools.ietf.org/html/rfc3339\][RFC> 3339] format.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="4")]
    pub time_created: ::core::option::Option<::prost_types::Timestamp>,
    /// The ID of the bucket. For buckets, the `id` and `name` properties are the
    /// same.
    /// Attempting to update this field after the bucket is created will result in
    /// an error.
    #[prost(string, tag="5")]
    pub id: ::prost::alloc::string::String,
    /// The name of the bucket.
    /// Attempting to update this field after the bucket is created will result in
    /// an error.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
    /// The project number of the project the bucket belongs to.
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag="7")]
    pub project_number: i64,
    /// The metadata generation of this bucket.
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag="8")]
    pub metageneration: i64,
    /// The bucket's \[<https://www.w3.org/TR/cors/\][Cross-Origin> Resource Sharing]
    /// (CORS) configuration.
    #[prost(message, repeated, tag="9")]
    pub cors: ::prost::alloc::vec::Vec<bucket::Cors>,
    /// The location of the bucket. Object data for objects in the bucket resides
    /// in physical storage within this region.  Defaults to `US`. See the
    /// \[<https://developers.google.com/storage/docs/concepts-techniques#specifyinglocations"\][developer's>
    /// guide] for the authoritative list. Attempting to update this field after
    /// the bucket is created will result in an error.
    #[prost(string, tag="10")]
    pub location: ::prost::alloc::string::String,
    /// The bucket's default storage class, used whenever no storageClass is
    /// specified for a newly-created object. This defines how objects in the
    /// bucket are stored and determines the SLA and the cost of storage.
    /// If this value is not specified when the bucket is created, it will default
    /// to `STANDARD`. For more information, see
    /// <https://developers.google.com/storage/docs/storage-classes.>
    #[prost(string, tag="11")]
    pub storage_class: ::prost::alloc::string::String,
    /// HTTP 1.1 \[<https://tools.ietf.org/html/rfc7232#section-2.3"\]Entity> tag]
    /// for the bucket.
    /// Attempting to set this field will result in an error.
    #[prost(string, tag="12")]
    pub etag: ::prost::alloc::string::String,
    /// The modification time of the bucket.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="13")]
    pub updated: ::core::option::Option<::prost_types::Timestamp>,
    /// The default value for event-based hold on newly created objects in this
    /// bucket.  Event-based hold is a way to retain objects indefinitely until an
    /// event occurs, signified by the
    /// hold's release. After being released, such objects will be subject to
    /// bucket-level retention (if any).  One sample use case of this flag is for
    /// banks to hold loan documents for at least 3 years after loan is paid in
    /// full. Here, bucket-level retention is 3 years and the event is loan being
    /// paid in full. In this example, these objects will be held intact for any
    /// number of years until the event has occurred (event-based hold on the
    /// object is released) and then 3 more years after that. That means retention
    /// duration of the objects begins from the moment event-based hold
    /// transitioned from true to false.  Objects under event-based hold cannot be
    /// deleted, overwritten or archived until the hold is removed.
    #[prost(bool, tag="14")]
    pub default_event_based_hold: bool,
    /// User-provided labels, in key/value pairs.
    #[prost(map="string, string", tag="15")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The bucket's website configuration, controlling how the service behaves
    /// when accessing bucket contents as a web site. See the
    /// \[<https://cloud.google.com/storage/docs/static-website\][Static> Website
    /// Examples] for more information.
    #[prost(message, optional, tag="16")]
    pub website: ::core::option::Option<bucket::Website>,
    /// The bucket's versioning configuration.
    #[prost(message, optional, tag="17")]
    pub versioning: ::core::option::Option<bucket::Versioning>,
    /// The bucket's logging configuration, which defines the destination bucket
    /// and optional name prefix for the current bucket's logs.
    #[prost(message, optional, tag="18")]
    pub logging: ::core::option::Option<bucket::Logging>,
    /// The owner of the bucket. This is always the project team's owner group.
    #[prost(message, optional, tag="19")]
    pub owner: ::core::option::Option<Owner>,
    /// Encryption configuration for a bucket.
    #[prost(message, optional, tag="20")]
    pub encryption: ::core::option::Option<bucket::Encryption>,
    /// The bucket's billing configuration.
    #[prost(message, optional, tag="21")]
    pub billing: ::core::option::Option<bucket::Billing>,
    /// The bucket's retention policy. The retention policy enforces a minimum
    /// retention time for all objects contained in the bucket, based on their
    /// creation time. Any attempt to overwrite or delete objects younger than the
    /// retention period will result in a PERMISSION_DENIED error.  An unlocked
    /// retention policy can be modified or removed from the bucket via a
    /// storage.buckets.update operation. A locked retention policy cannot be
    /// removed or shortened in duration for the lifetime of the bucket.
    /// Attempting to remove or decrease period of a locked retention policy will
    /// result in a PERMISSION_DENIED error.
    #[prost(message, optional, tag="22")]
    pub retention_policy: ::core::option::Option<bucket::RetentionPolicy>,
    /// The location type of the bucket (region, dual-region, multi-region, etc).
    #[prost(string, tag="23")]
    pub location_type: ::prost::alloc::string::String,
    /// The bucket's IAM configuration.
    #[prost(message, optional, tag="24")]
    pub iam_configuration: ::core::option::Option<bucket::IamConfiguration>,
    /// The zone or zones from which the bucket is intended to use zonal quota.
    /// Requests for data from outside the specified affinities are still allowed
    /// but won’t be able to use zonal quota. The values are case-insensitive.
    /// Attempting to update this field after bucket is created will result in an
    /// error.
    #[prost(string, repeated, tag="25")]
    pub zone_affinity: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Bucket`.
pub mod bucket {
    /// Billing properties of a bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Billing {
        /// When set to true, Requester Pays is enabled for this bucket.
        #[prost(bool, tag="1")]
        pub requester_pays: bool,
    }
    /// Cross-Origin Response sharing (CORS) properties for a bucket.
    /// For more on GCS and CORS, see
    /// <https://cloud.google.com/storage/docs/cross-origin.>
    /// For more on CORS in general, see <https://tools.ietf.org/html/rfc6454.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cors {
        /// The list of Origins eligible to receive CORS response headers. See
        /// \[<https://tools.ietf.org/html/rfc6454\][RFC> 6454] for more on origins.
        /// Note: "*" is permitted in the list of origins, and means "any Origin".
        #[prost(string, repeated, tag="1")]
        pub origin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of HTTP methods on which to include CORS response headers,
        /// (`GET`, `OPTIONS`, `POST`, etc) Note: "*" is permitted in the list of
        /// methods, and means "any method".
        #[prost(string, repeated, tag="2")]
        pub method: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of HTTP headers other than the
        /// \[<https://www.w3.org/TR/cors/#simple-response-header\][simple> response
        /// headers] to give permission for the user-agent to share across domains.
        #[prost(string, repeated, tag="3")]
        pub response_header: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The value, in seconds, to return in the
        /// \[<https://www.w3.org/TR/cors/#access-control-max-age-response-header\][Access-Control-Max-Age>
        /// header] used in preflight responses.
        #[prost(int32, tag="4")]
        pub max_age_seconds: i32,
    }
    /// Encryption properties of a bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Encryption {
        /// A Cloud KMS key that will be used to encrypt objects inserted into this
        /// bucket, if no encryption method is specified.
        #[prost(string, tag="1")]
        pub default_kms_key_name: ::prost::alloc::string::String,
    }
    /// Bucket restriction options currently enforced on the bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamConfiguration {
        #[prost(message, optional, tag="1")]
        pub uniform_bucket_level_access: ::core::option::Option<iam_configuration::UniformBucketLevelAccess>,
    }
    /// Nested message and enum types in `IamConfiguration`.
    pub mod iam_configuration {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UniformBucketLevelAccess {
            /// If set, access checks only use bucket-level IAM policies or above.
            #[prost(bool, tag="1")]
            pub enabled: bool,
            /// The deadline time for changing
            /// <code>iamConfiguration.uniformBucketLevelAccess.enabled</code> from
            /// true to false in \[<https://tools.ietf.org/html/rfc3339\][RFC> 3339]. After
            /// the deadline is passed the field is immutable.
            #[prost(message, optional, tag="2")]
            pub locked_time: ::core::option::Option<::prost_types::Timestamp>,
        }
    }
    /// Lifecycle properties of a bucket.
    /// For more information, see <https://cloud.google.com/storage/docs/lifecycle.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Lifecycle {
        /// A lifecycle management rule, which is made of an action to take and the
        /// condition(s) under which the action will be taken.
        #[prost(message, repeated, tag="1")]
        pub rule: ::prost::alloc::vec::Vec<lifecycle::Rule>,
    }
    /// Nested message and enum types in `Lifecycle`.
    pub mod lifecycle {
        /// A lifecycle Rule, combining an action to take on an object and a
        /// condition which will trigger that action.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Rule {
            /// The action to take.
            #[prost(message, optional, tag="1")]
            pub action: ::core::option::Option<rule::Action>,
            /// The condition(s) under which the action will be taken.
            #[prost(message, optional, tag="2")]
            pub condition: ::core::option::Option<rule::Condition>,
        }
        /// Nested message and enum types in `Rule`.
        pub mod rule {
            /// An action to take on an object.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Action {
                /// Type of the action. Currently, only `Delete` and
                /// `SetStorageClass` are supported.
                #[prost(string, tag="1")]
                pub r#type: ::prost::alloc::string::String,
                /// Target storage class. Required iff the type of the action is
                /// SetStorageClass.
                #[prost(string, tag="2")]
                pub storage_class: ::prost::alloc::string::String,
            }
            /// A condition of an object which triggers some action.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Condition {
                /// Age of an object (in days). This condition is satisfied when an
                /// object reaches the specified age.
                #[prost(int32, tag="1")]
                pub age: i32,
                /// A date in [RFC 3339]\[1\] format with only the date part (for
                /// instance, "2013-01-15"). This condition is satisfied when an
                /// object is created before midnight of the specified date in UTC.
                /// \[1\]: <https://tools.ietf.org/html/rfc3339>
                #[prost(message, optional, tag="2")]
                pub created_before: ::core::option::Option<::prost_types::Timestamp>,
                /// Relevant only for versioned objects. If the value is
                /// `true`, this condition matches live objects; if the value
                /// is `false`, it matches archived objects.
                #[prost(message, optional, tag="3")]
                pub is_live: ::core::option::Option<bool>,
                /// Relevant only for versioned objects. If the value is N, this
                /// condition is satisfied when there are at least N versions (including
                /// the live version) newer than this version of the object.
                #[prost(int32, tag="4")]
                pub num_newer_versions: i32,
                /// Objects having any of the storage classes specified by this condition
                /// will be matched. Values include `MULTI_REGIONAL`, `REGIONAL`,
                /// `NEARLINE`, `COLDLINE`, `STANDARD`, and
                /// `DURABLE_REDUCED_AVAILABILITY`.
                #[prost(string, repeated, tag="5")]
                pub matches_storage_class: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// A regular expression that satisfies the RE2 syntax. This condition is
                /// satisfied when the name of the object matches the RE2 pattern.  Note:
                /// This feature is currently in the "Early Access" launch stage and is
                /// only available to a whitelisted set of users; that means that this
                /// feature may be changed in backward-incompatible ways and that it is
                /// not guaranteed to be released.
                #[prost(string, tag="6")]
                pub matches_pattern: ::prost::alloc::string::String,
            }
        }
    }
    /// Logging-related properties of a bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Logging {
        /// The destination bucket where the current bucket's logs should be placed.
        #[prost(string, tag="1")]
        pub log_bucket: ::prost::alloc::string::String,
        /// A prefix for log object names.
        #[prost(string, tag="2")]
        pub log_object_prefix: ::prost::alloc::string::String,
    }
    /// Retention policy properties of a bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetentionPolicy {
        /// Server-determined value that indicates the time from which policy was
        /// enforced and effective. This value is in
        /// \[<https://tools.ietf.org/html/rfc3339\][RFC> 3339] format.
        #[prost(message, optional, tag="1")]
        pub effective_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Once locked, an object retention policy cannot be modified.
        #[prost(bool, tag="2")]
        pub is_locked: bool,
        /// The duration in seconds that objects need to be retained. Retention
        /// duration must be greater than zero and less than 100 years. Note that
        /// enforcement of retention periods less than a day is not guaranteed. Such
        /// periods should only be used for testing purposes.
        #[prost(int64, tag="3")]
        pub retention_period: i64,
    }
    /// Properties of a bucket related to versioning.
    /// For more on GCS versioning, see
    /// <https://cloud.google.com/storage/docs/object-versioning.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Versioning {
        /// While set to true, versioning is fully enabled for this bucket.
        #[prost(bool, tag="1")]
        pub enabled: bool,
    }
    /// Properties of a bucket related to accessing the contents as a static
    /// website. For more on hosting a static website via GCS, see
    /// <https://cloud.google.com/storage/docs/hosting-static-website.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Website {
        /// If the requested object path is missing, the service will ensure the path
        /// has a trailing '/', append this suffix, and attempt to retrieve the
        /// resulting object. This allows the creation of `index.html`
        /// objects to represent directory pages.
        #[prost(string, tag="1")]
        pub main_page_suffix: ::prost::alloc::string::String,
        /// If the requested object path is missing, and any
        /// `mainPageSuffix` object is missing, if applicable, the service
        /// will return the named object from this bucket as the content for a
        /// \[<https://tools.ietf.org/html/rfc7231#section-6.5.4\][404> Not Found]
        /// result.
        #[prost(string, tag="2")]
        pub not_found_page: ::prost::alloc::string::String,
    }
}
/// An access-control entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketAccessControl {
    /// The access permission for the entity.
    #[prost(string, tag="1")]
    pub role: ::prost::alloc::string::String,
    /// HTTP 1.1 \["<https://tools.ietf.org/html/rfc7232#section-2.3\][Entity> tag]
    /// for the access-control entry.
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
    /// The ID of the access-control entry.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// The name of the bucket.
    #[prost(string, tag="4")]
    pub bucket: ::prost::alloc::string::String,
    /// The entity holding the permission, in one of the following forms:
    /// * `user-{userid}`
    /// * `user-{email}`
    /// * `group-{groupid}`
    /// * `group-{email}`
    /// * `domain-{domain}`
    /// * `project-{team-projectid}`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    /// Examples:
    /// * The user `liz@example.com` would be `user-liz@example.com`.
    /// * The group `example@googlegroups.com` would be
    /// `group-example@googlegroups.com`
    /// * All members of the Google Apps for Business domain `example.com` would be
    /// `domain-example.com`
    #[prost(string, tag="6")]
    pub entity: ::prost::alloc::string::String,
    /// The ID for the entity, if any.
    #[prost(string, tag="7")]
    pub entity_id: ::prost::alloc::string::String,
    /// The email address associated with the entity, if any.
    #[prost(string, tag="8")]
    pub email: ::prost::alloc::string::String,
    /// The domain associated with the entity, if any.
    #[prost(string, tag="9")]
    pub domain: ::prost::alloc::string::String,
    /// The project team associated with the entity, if any.
    #[prost(message, optional, tag="10")]
    pub project_team: ::core::option::Option<ProjectTeam>,
}
/// The response to a call to BucketAccessControls.ListBucketAccessControls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketAccessControlsResponse {
    /// The list of items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<BucketAccessControl>,
}
/// The result of a call to Buckets.ListBuckets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsResponse {
    /// The list of items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Bucket>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// An notification channel used to watch for resource changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// A UUID or similar unique string that identifies this channel.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// An opaque ID that identifies the resource being watched on this channel.
    /// Stable across different API versions.
    #[prost(string, tag="2")]
    pub resource_id: ::prost::alloc::string::String,
    /// A version-specific identifier for the watched resource.
    #[prost(string, tag="3")]
    pub resource_uri: ::prost::alloc::string::String,
    /// An arbitrary string delivered to the target address with each notification
    /// delivered over this channel. Optional.
    #[prost(string, tag="4")]
    pub token: ::prost::alloc::string::String,
    /// Date and time of notification channel expiration. Optional.
    #[prost(message, optional, tag="5")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of delivery mechanism used for this channel.
    #[prost(string, tag="6")]
    pub r#type: ::prost::alloc::string::String,
    /// The address where notifications are delivered for this channel.
    #[prost(string, tag="7")]
    pub address: ::prost::alloc::string::String,
    /// Additional parameters controlling delivery channel behavior. Optional.
    #[prost(map="string, string", tag="8")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    #[prost(bool, tag="9")]
    pub payload: bool,
}
/// The result of a call to Channels.ListChannels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    /// The list of notification channels for a bucket.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<list_channels_response::Items>,
}
/// Nested message and enum types in `ListChannelsResponse`.
pub mod list_channels_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Items {
        /// User-specified name for a channel. Needed to unsubscribe.
        #[prost(string, tag="1")]
        pub channel_id: ::prost::alloc::string::String,
        /// Opaque value generated by GCS representing a bucket. Needed to
        /// unsubscribe.
        #[prost(string, tag="2")]
        pub resource_id: ::prost::alloc::string::String,
        /// Url used to identify where notifications are sent to.
        #[prost(string, tag="3")]
        pub push_url: ::prost::alloc::string::String,
        /// Email address of the subscriber.
        #[prost(string, tag="4")]
        pub subscriber_email: ::prost::alloc::string::String,
        /// Time when the channel was created.
        #[prost(message, optional, tag="5")]
        pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Message used to convey content being read or written, along with its
/// checksum.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChecksummedData {
    /// The data.
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// CRC32C digest of the contents.
    #[prost(message, optional, tag="2")]
    pub crc32c: ::core::option::Option<u32>,
}
/// Message used for storing full (not subrange) object checksums.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectChecksums {
    /// CRC32C digest of the object data. Computed by the GCS service for
    /// all written objects, and validated by the GCS service against
    /// client-supplied values if present in an InsertObjectRequest.
    #[prost(message, optional, tag="1")]
    pub crc32c: ::core::option::Option<u32>,
    /// Hex-encoded MD5 hash of the object data (hexdigest). Whether/how this
    /// checksum is provided and validated is service-dependent.
    #[prost(string, tag="2")]
    pub md5_hash: ::prost::alloc::string::String,
}
/// A collection of enums used in multiple places throughout the API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonEnums {
}
/// Nested message and enum types in `CommonEnums`.
pub mod common_enums {
    /// A set of properties to return in a response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Projection {
        /// No specified projection.
        Unspecified = 0,
        /// Omit `owner`, `acl`, and `defaultObjectAcl` properties.
        NoAcl = 1,
        /// Include all properties.
        Full = 2,
    }
    /// Predefined or "canned" aliases for sets of specific bucket ACL entries.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PredefinedBucketAcl {
        /// No predefined ACL.
        Unspecified = 0,
        /// Project team owners get `OWNER` access, and
        /// `allAuthenticatedUsers` get `READER` access.
        BucketAclAuthenticatedRead = 1,
        /// Project team owners get `OWNER` access.
        BucketAclPrivate = 2,
        /// Project team members get access according to their roles.
        BucketAclProjectPrivate = 3,
        /// Project team owners get `OWNER` access, and
        /// `allUsers` get `READER` access.
        BucketAclPublicRead = 4,
        /// Project team owners get `OWNER` access, and
        /// `allUsers` get `WRITER` access.
        BucketAclPublicReadWrite = 5,
    }
    /// Predefined or "canned" aliases for sets of specific object ACL entries.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PredefinedObjectAcl {
        /// No predefined ACL.
        Unspecified = 0,
        /// Object owner gets `OWNER` access, and
        /// `allAuthenticatedUsers` get `READER` access.
        ObjectAclAuthenticatedRead = 1,
        /// Object owner gets `OWNER` access, and project team owners get
        /// `OWNER` access.
        ObjectAclBucketOwnerFullControl = 2,
        /// Object owner gets `OWNER` access, and project team owners get
        /// `READER` access.
        ObjectAclBucketOwnerRead = 3,
        /// Object owner gets `OWNER` access.
        ObjectAclPrivate = 4,
        /// Object owner gets `OWNER` access, and project team members get
        /// access according to their roles.
        ObjectAclProjectPrivate = 5,
        /// Object owner gets `OWNER` access, and `allUsers`
        /// get `READER` access.
        ObjectAclPublicRead = 6,
    }
}
/// Specifies a requested range of bytes to download.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentRange {
    /// The starting offset of the object data.
    #[prost(int64, tag="1")]
    pub start: i64,
    /// The ending offset of the object data.
    #[prost(int64, tag="2")]
    pub end: i64,
    /// The complete length of the object data.
    #[prost(int64, tag="3")]
    pub complete_length: i64,
}
/// Hmac Key Metadata, which includes all information other than the secret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HmacKeyMetadata {
    /// Resource name ID of the key in the format <projectId>/<accessId>.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Globally unique id for keys.
    #[prost(string, tag="2")]
    pub access_id: ::prost::alloc::string::String,
    /// The project ID that the hmac key is contained in.
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
    /// Email of the service account the key authenticates as.
    #[prost(string, tag="4")]
    pub service_account_email: ::prost::alloc::string::String,
    /// State of the key. One of ACTIVE, INACTIVE, or DELETED.
    #[prost(string, tag="5")]
    pub state: ::prost::alloc::string::String,
    /// The creation time of the HMAC key in RFC 3339 format.
    #[prost(message, optional, tag="6")]
    pub time_created: ::core::option::Option<::prost_types::Timestamp>,
    /// The last modification time of the HMAC key metadata in RFC 3339 format.
    #[prost(message, optional, tag="7")]
    pub updated: ::core::option::Option<::prost_types::Timestamp>,
    /// Tag updated with each key update.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
}
/// A subscription to receive Google PubSub notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// The Cloud PubSub topic to which this subscription publishes. Formatted as:
    /// '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    /// If present, only send notifications about listed event types. If empty,
    /// sent notifications for all event types.
    #[prost(string, repeated, tag="2")]
    pub event_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An optional list of additional attributes to attach to each Cloud PubSub
    /// message published for this notification subscription.
    #[prost(map="string, string", tag="3")]
    pub custom_attributes: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// HTTP 1.1 \[<https://tools.ietf.org/html/rfc7232#section-2.3\][Entity> tag]
    /// for this subscription notification.
    #[prost(string, tag="4")]
    pub etag: ::prost::alloc::string::String,
    /// If present, only apply this notification configuration to object names that
    /// begin with this prefix.
    #[prost(string, tag="5")]
    pub object_name_prefix: ::prost::alloc::string::String,
    /// The desired content of the Payload.
    #[prost(string, tag="6")]
    pub payload_format: ::prost::alloc::string::String,
    /// The ID of the notification.
    #[prost(string, tag="7")]
    pub id: ::prost::alloc::string::String,
}
/// The result of a call to Notifications.ListNotifications
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsResponse {
    /// The list of items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Notification>,
}
/// An object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// Content-Encoding of the object data, matching
    /// \[<https://tools.ietf.org/html/rfc7231#section-3.1.2.2\][RFC> 7231 §3.1.2.2]
    #[prost(string, tag="1")]
    pub content_encoding: ::prost::alloc::string::String,
    /// Content-Disposition of the object data, matching
    /// \[<https://tools.ietf.org/html/rfc6266\][RFC> 6266].
    #[prost(string, tag="2")]
    pub content_disposition: ::prost::alloc::string::String,
    /// Cache-Control directive for the object data, matching
    /// \[<https://tools.ietf.org/html/rfc7234#section-5.2"\][RFC> 7234 §5.2].
    /// If omitted, and the object is accessible to all anonymous users, the
    /// default will be `public, max-age=3600`.
    #[prost(string, tag="3")]
    pub cache_control: ::prost::alloc::string::String,
    /// Access controls on the object.
    #[prost(message, repeated, tag="4")]
    pub acl: ::prost::alloc::vec::Vec<ObjectAccessControl>,
    /// Content-Language of the object data, matching
    /// \[<https://tools.ietf.org/html/rfc7231#section-3.1.3.2\][RFC> 7231 §3.1.3.2].
    #[prost(string, tag="5")]
    pub content_language: ::prost::alloc::string::String,
    /// The version of the metadata for this object at this generation. Used for
    /// preconditions and for detecting changes in metadata. A metageneration
    /// number is only meaningful in the context of a particular generation of a
    /// particular object.
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag="6")]
    pub metageneration: i64,
    /// The deletion time of the object. Will be returned if and only if this
    /// version of the object has been deleted.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="7")]
    pub time_deleted: ::core::option::Option<::prost_types::Timestamp>,
    /// Content-Type of the object data, matching
    /// \[<https://tools.ietf.org/html/rfc7231#section-3.1.1.5\][RFC> 7231 §3.1.1.5].
    /// If an object is stored without a Content-Type, it is served as
    /// `application/octet-stream`.
    #[prost(string, tag="8")]
    pub content_type: ::prost::alloc::string::String,
    /// Content-Length of the object data in bytes, matching
    /// \[<https://tools.ietf.org/html/rfc7230#section-3.3.2\][RFC> 7230 §3.3.2].
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag="9")]
    pub size: i64,
    /// The creation time of the object.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="10")]
    pub time_created: ::core::option::Option<::prost_types::Timestamp>,
    /// CRC32c checksum. For more information about using the CRC32c
    /// checksum, see
    /// \[<https://cloud.google.com/storage/docs/hashes-etags#_JSONAPI\][Hashes> and
    /// ETags: Best Practices]. This is a server determined value and should not be
    /// supplied by the user when sending an Object. The server will ignore any
    /// value provided. Users should instead use the object_checksums field on the
    /// InsertObjectRequest when uploading an object.
    #[prost(message, optional, tag="11")]
    pub crc32c: ::core::option::Option<u32>,
    /// Number of underlying components that make up this object. Components are
    /// accumulated by compose operations.
    /// Attempting to set this field will result in an error.
    #[prost(int32, tag="12")]
    pub component_count: i32,
    /// MD5 hash of the data; encoded using base64 as per
    /// \[<https://tools.ietf.org/html/rfc4648#section-4\][RFC> 4648 §4]. For more
    /// information about using the MD5 hash, see
    /// \[<https://cloud.google.com/storage/docs/hashes-etags#_JSONAPI\][Hashes> and
    /// ETags: Best Practices]. This is a server determined value and should not be
    /// supplied by the user when sending an Object. The server will ignore any
    /// value provided. Users should instead use the object_checksums field on the
    /// InsertObjectRequest when uploading an object.
    #[prost(string, tag="13")]
    pub md5_hash: ::prost::alloc::string::String,
    /// HTTP 1.1 Entity tag for the object. See
    /// \[<https://tools.ietf.org/html/rfc7232#section-2.3\][RFC> 7232 §2.3].
    /// Attempting to set this field will result in an error.
    #[prost(string, tag="14")]
    pub etag: ::prost::alloc::string::String,
    /// The modification time of the object metadata.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="15")]
    pub updated: ::core::option::Option<::prost_types::Timestamp>,
    /// Storage class of the object.
    #[prost(string, tag="16")]
    pub storage_class: ::prost::alloc::string::String,
    /// Cloud KMS Key used to encrypt this object, if the object is encrypted by
    /// such a key.
    #[prost(string, tag="17")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// The time at which the object's storage class was last changed. When the
    /// object is initially created, it will be set to time_created.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="18")]
    pub time_storage_class_updated: ::core::option::Option<::prost_types::Timestamp>,
    /// Whether an object is under temporary hold. While this flag is set to true,
    /// the object is protected against deletion and overwrites.  A common use case
    /// of this flag is regulatory investigations where objects need to be retained
    /// while the investigation is ongoing. Note that unlike event-based hold,
    /// temporary hold does not impact retention expiration time of an object.
    #[prost(bool, tag="19")]
    pub temporary_hold: bool,
    /// A server-determined value that specifies the earliest time that the
    /// object's retention period expires. This value is in
    /// \[<https://tools.ietf.org/html/rfc3339\][RFC> 3339] format.
    /// Note 1: This field is not provided for objects with an active event-based
    /// hold, since retention expiration is unknown until the hold is removed.
    /// Note 2: This value can be provided even when temporary hold is set (so that
    /// the user can reason about policy without having to first unset the
    /// temporary hold).
    #[prost(message, optional, tag="20")]
    pub retention_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-provided metadata, in key/value pairs.
    #[prost(map="string, string", tag="21")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Whether an object is under event-based hold. Event-based hold is a way to
    /// retain objects until an event occurs, which is signified by the
    /// hold's release (i.e. this value is set to false). After being released (set
    /// to false), such objects will be subject to bucket-level retention (if any).
    /// One sample use case of this flag is for banks to hold loan documents for at
    /// least 3 years after loan is paid in full. Here, bucket-level retention is 3
    /// years and the event is the loan being paid in full. In this example, these
    /// objects will be held intact for any number of years until the event has
    /// occurred (event-based hold on the object is released) and then 3 more years
    /// after that. That means retention duration of the objects begins from the
    /// moment event-based hold transitioned from true to false.
    #[prost(message, optional, tag="29")]
    pub event_based_hold: ::core::option::Option<bool>,
    /// The name of the object.
    /// Attempting to update this field after the object is created will result in
    /// an error.
    #[prost(string, tag="23")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the object, including the bucket name, object name, and
    /// generation number.
    /// Attempting to update this field after the object is created will result in
    /// an error.
    #[prost(string, tag="24")]
    pub id: ::prost::alloc::string::String,
    /// The name of the bucket containing this object.
    /// Attempting to update this field after the object is created will result in
    /// an error.
    #[prost(string, tag="25")]
    pub bucket: ::prost::alloc::string::String,
    /// The content generation of this object. Used for object versioning.
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag="26")]
    pub generation: i64,
    /// The owner of the object. This will always be the uploader of the object.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag="27")]
    pub owner: ::core::option::Option<Owner>,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by
    /// such a key.
    #[prost(message, optional, tag="28")]
    pub customer_encryption: ::core::option::Option<object::CustomerEncryption>,
}
/// Nested message and enum types in `Object`.
pub mod object {
    /// Describes the customer-specified mechanism used to store the data at rest.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomerEncryption {
        /// The encryption algorithm.
        #[prost(string, tag="1")]
        pub encryption_algorithm: ::prost::alloc::string::String,
        /// SHA256 hash value of the encryption key.
        #[prost(string, tag="2")]
        pub key_sha256: ::prost::alloc::string::String,
    }
}
/// An access-control entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectAccessControl {
    /// The access permission for the entity.
    #[prost(string, tag="1")]
    pub role: ::prost::alloc::string::String,
    /// HTTP 1.1 Entity tag for the access-control entry.
    /// See \[<https://tools.ietf.org/html/rfc7232#section-2.3\][RFC> 7232 §2.3].
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
    /// The ID of the access-control entry.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// The name of the bucket.
    #[prost(string, tag="4")]
    pub bucket: ::prost::alloc::string::String,
    /// The name of the object, if applied to an object.
    #[prost(string, tag="5")]
    pub object: ::prost::alloc::string::String,
    /// The content generation of the object, if applied to an object.
    #[prost(int64, tag="6")]
    pub generation: i64,
    /// The entity holding the permission, in one of the following forms:
    /// * `user-{userid}`
    /// * `user-{email}`
    /// * `group-{groupid}`
    /// * `group-{email}`
    /// * `domain-{domain}`
    /// * `project-{team-projectid}`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    /// Examples:
    /// * The user `liz@example.com` would be `user-liz@example.com`.
    /// * The group `example@googlegroups.com` would be
    /// `group-example@googlegroups.com`.
    /// * All members of the Google Apps for Business domain `example.com` would be
    /// `domain-example.com`.
    #[prost(string, tag="7")]
    pub entity: ::prost::alloc::string::String,
    /// The ID for the entity, if any.
    #[prost(string, tag="8")]
    pub entity_id: ::prost::alloc::string::String,
    /// The email address associated with the entity, if any.
    #[prost(string, tag="9")]
    pub email: ::prost::alloc::string::String,
    /// The domain associated with the entity, if any.
    #[prost(string, tag="10")]
    pub domain: ::prost::alloc::string::String,
    /// The project team associated with the entity, if any.
    #[prost(message, optional, tag="11")]
    pub project_team: ::core::option::Option<ProjectTeam>,
}
/// The result of a call to ObjectAccessControls.ListObjectAccessControls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectAccessControlsResponse {
    /// The list of items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<ObjectAccessControl>,
}
/// The result of a call to Objects.ListObjects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsResponse {
    /// The list of prefixes of objects matching-but-not-listed up to and including
    /// the requested delimiter.
    #[prost(string, repeated, tag="1")]
    pub prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of items.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Object>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents the Viewers, Editors, or Owners of a given project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectTeam {
    /// The project number.
    #[prost(string, tag="1")]
    pub project_number: ::prost::alloc::string::String,
    /// The team.
    #[prost(string, tag="2")]
    pub team: ::prost::alloc::string::String,
}
/// A subscription to receive Google PubSub notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// The ID of the notification.
    #[prost(string, tag="1")]
    pub email_address: ::prost::alloc::string::String,
}
/// The owner of a specific resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    /// The entity, in the form `user-`*userId*.
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    /// The ID for the entity.
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
/// Request message for DeleteBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for InsertBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertBucketAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Properties of the new bucket access control being inserted.
    #[prost(message, optional, tag="3")]
    pub bucket_access_control: ::core::option::Option<BucketAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketAccessControlsRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request for PatchBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchBucketAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// The BucketAccessControl for updating.
    #[prost(message, optional, tag="4")]
    pub bucket_access_control: ::core::option::Option<BucketAccessControl>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`.
    ///
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request for UpdateBucketAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBucketAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// The BucketAccessControl for updating.
    #[prost(message, optional, tag="4")]
    pub bucket_access_control: ::core::option::Option<BucketAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for DeleteBucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// If set, only deletes the bucket if its metageneration matches this value.
    #[prost(message, optional, tag="2")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// If set, only deletes the bucket if its metageneration does not match this
    /// value.
    #[prost(message, optional, tag="3")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetBucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration matches the given value.
    #[prost(message, optional, tag="2")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration does not match the given value.
    #[prost(message, optional, tag="3")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Set of properties to return. Defaults to `NO_ACL`.
    #[prost(enumeration="common_enums::Projection", tag="4")]
    pub projection: i32,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for InsertBucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertBucketRequest {
    /// Apply a predefined set of access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedBucketAcl", tag="1")]
    pub predefined_acl: i32,
    /// Apply a predefined set of default object access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="2")]
    pub predefined_default_object_acl: i32,
    /// Required. A valid API project identifier.
    #[prost(string, tag="3")]
    pub project: ::prost::alloc::string::String,
    /// Set of properties to return. Defaults to `NO_ACL`, unless the
    /// bucket resource specifies `acl` or `defaultObjectAcl`
    /// properties, when it defaults to `FULL`.
    #[prost(enumeration="common_enums::Projection", tag="4")]
    pub projection: i32,
    /// Properties of the new bucket being inserted, including its name.
    #[prost(message, optional, tag="6")]
    pub bucket: ::core::option::Option<Bucket>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="7")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListChannels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListBuckets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsRequest {
    /// Maximum number of buckets to return in a single response. The service will
    /// use this parameter or 1,000 items, whichever is smaller.
    #[prost(int32, tag="1")]
    pub max_results: i32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter results to buckets whose names begin with this prefix.
    #[prost(string, tag="3")]
    pub prefix: ::prost::alloc::string::String,
    /// Required. A valid API project identifier.
    #[prost(string, tag="4")]
    pub project: ::prost::alloc::string::String,
    /// Set of properties to return. Defaults to `NO_ACL`.
    #[prost(enumeration="common_enums::Projection", tag="5")]
    pub projection: i32,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="7")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for LockRetentionPolicy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockRetentionPolicyRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Makes the operation conditional on whether bucket's current metageneration
    /// matches the given value. Must be positive.
    #[prost(int64, tag="2")]
    pub if_metageneration_match: i64,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request for PatchBucket method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchBucketRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration matches the given value.
    #[prost(message, optional, tag="2")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration does not match the given value.
    #[prost(message, optional, tag="3")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Apply a predefined set of access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedBucketAcl", tag="4")]
    pub predefined_acl: i32,
    /// Apply a predefined set of default object access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="5")]
    pub predefined_default_object_acl: i32,
    /// Set of properties to return. Defaults to `FULL`.
    #[prost(enumeration="common_enums::Projection", tag="6")]
    pub projection: i32,
    /// The Bucket metadata for updating.
    #[prost(message, optional, tag="8")]
    pub metadata: ::core::option::Option<Bucket>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`. Note: not recommended. If a new
    /// field is introduced at a later time, an older client updating with the `*`
    /// may accidentally reset the new field's value.
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="9")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="10")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request for UpdateBucket method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBucketRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration matches the given value.
    #[prost(message, optional, tag="2")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the return of the bucket metadata conditional on whether the bucket's
    /// current metageneration does not match the given value.
    #[prost(message, optional, tag="3")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Apply a predefined set of access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedBucketAcl", tag="4")]
    pub predefined_acl: i32,
    /// Apply a predefined set of default object access controls to this bucket.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="5")]
    pub predefined_default_object_acl: i32,
    /// Set of properties to return. Defaults to `FULL`.
    #[prost(enumeration="common_enums::Projection", tag="6")]
    pub projection: i32,
    /// The Bucket metadata for updating.
    #[prost(message, optional, tag="8")]
    pub metadata: ::core::option::Option<Bucket>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="9")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for StopChannel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopChannelRequest {
    /// The channel to be stopped.
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="2")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for DeleteDefaultObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDefaultObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetDefaultObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for InsertDefaultObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertDefaultObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Properties of the object access control being inserted.
    #[prost(message, optional, tag="3")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListDefaultObjectAccessControls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDefaultObjectAccessControlsRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// If present, only return default ACL listing if the bucket's current
    /// metageneration matches this value.
    #[prost(message, optional, tag="2")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// If present, only return default ACL listing if the bucket's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="3")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for PatchDefaultObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchDefaultObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// The ObjectAccessControl for updating.
    #[prost(message, optional, tag="4")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`. Note: not recommended. If a new
    /// field is introduced at a later time, an older client updating with the `*`
    /// may accidentally reset the new field's value.
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for UpdateDefaultObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDefaultObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// The ObjectAccessControl for updating.
    #[prost(message, optional, tag="4")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for DeleteNotification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationRequest {
    /// Required. The parent bucket of the notification.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. ID of the notification to delete.
    #[prost(string, tag="2")]
    pub notification: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetNotification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationRequest {
    /// Required. The parent bucket of the notification.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Notification ID.
    /// Required.
    #[prost(string, tag="2")]
    pub notification: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for InsertNotification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertNotificationRequest {
    /// Required. The parent bucket of the notification.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Properties of the notification to be inserted.
    #[prost(message, optional, tag="3")]
    pub notification: ::core::option::Option<Notification>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListNotifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    /// Required. Name of a Google Cloud Storage bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for DeleteObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for InsertObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// Properties of the object access control to be inserted.
    #[prost(message, optional, tag="5")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for ListObjectAccessControls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectAccessControlsRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for PatchObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// Required. Name of the object.
    /// Required.
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// The ObjectAccessControl for updating.
    #[prost(message, optional, tag="5")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`. Note: not recommended. If a new
    /// field is introduced at a later time, an older client updating with the `*`
    /// may accidentally reset the new field's value.
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="7")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for UpdateObjectAccessControl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectAccessControlRequest {
    /// Required. Name of a bucket.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The entity holding the permission. Can be one of:
    /// * `user-`*userId*
    /// * `user-`*emailAddress*
    /// * `group-`*groupId*
    /// * `group-`*emailAddress*
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
    /// Required. Name of the object.
    /// Required.
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// The ObjectAccessControl for updating.
    #[prost(message, optional, tag="6")]
    pub object_access_control: ::core::option::Option<ObjectAccessControl>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="7")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`. Note: not recommended. If a new
    /// field is introduced at a later time, an older client updating with the `*`
    /// may accidentally reset the new field's value.
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="8")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ComposeObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposeObjectRequest {
    /// Required. Name of the bucket containing the source objects. The destination object is
    /// stored in this bucket.
    #[prost(string, tag="1")]
    pub destination_bucket: ::prost::alloc::string::String,
    /// Required. Name of the new object.
    #[prost(string, tag="2")]
    pub destination_object: ::prost::alloc::string::String,
    /// Apply a predefined set of access controls to the destination object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="3")]
    pub destination_predefined_acl: i32,
    /// Properties of the resulting object.
    #[prost(message, optional, tag="11")]
    pub destination: ::core::option::Option<Object>,
    /// The list of source objects that will be concatenated into a single object.
    #[prost(message, repeated, tag="12")]
    pub source_objects: ::prost::alloc::vec::Vec<compose_object_request::SourceObjects>,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="5")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Resource name of the Cloud KMS key, of the form
    /// `projects/my-project/locations/my-location/keyRings/my-kr/cryptoKeys/my-key`,
    /// that will be used to encrypt the object. Overrides the object
    /// metadata's `kms_key_name` value, if any.
    #[prost(string, tag="6")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="9")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="10")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Nested message and enum types in `ComposeObjectRequest`.
pub mod compose_object_request {
    /// Description of a source object for a composition request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourceObjects {
        /// The source object's name. All source objects must reside in the same
        /// bucket.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// The generation of this object to use as the source.
        #[prost(int64, tag="2")]
        pub generation: i64,
        /// Conditions that must be met for this operation to execute.
        #[prost(message, optional, tag="3")]
        pub object_preconditions: ::core::option::Option<source_objects::ObjectPreconditions>,
    }
    /// Nested message and enum types in `SourceObjects`.
    pub mod source_objects {
        /// Preconditions for a source object of a composition request.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ObjectPreconditions {
            /// Only perform the composition if the generation of the source object
            /// that would be used matches this value.  If this value and a generation
            /// are both specified, they must be the same value or the call will fail.
            #[prost(message, optional, tag="1")]
            pub if_generation_match: ::core::option::Option<i64>,
        }
    }
}
/// Request message for CopyObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyObjectRequest {
    /// Required. Name of the bucket in which to store the new object. Overrides the provided
    /// object
    /// metadata's `bucket` value, if any.
    #[prost(string, tag="1")]
    pub destination_bucket: ::prost::alloc::string::String,
    /// Required. Name of the new object.
    /// Required when the object metadata is not otherwise provided. Overrides the
    /// object metadata's `name` value, if any.
    #[prost(string, tag="2")]
    pub destination_object: ::prost::alloc::string::String,
    /// Apply a predefined set of access controls to the destination object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="3")]
    pub destination_predefined_acl: i32,
    /// Makes the operation conditional on whether the destination object's current
    /// generation matches the given value. Setting to 0 makes the operation
    /// succeed only if there are no live versions of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the destination object's current
    /// generation does not match the given value. If no live object exists, the
    /// precondition fails. Setting to 0 makes the operation succeed only if there
    /// is a live version of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the destination object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="6")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the destination object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// generation matches the given value.
    #[prost(message, optional, tag="8")]
    pub if_source_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// generation does not match the given value.
    #[prost(message, optional, tag="9")]
    pub if_source_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="10")]
    pub if_source_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="11")]
    pub if_source_metageneration_not_match: ::core::option::Option<i64>,
    /// Set of properties to return. Defaults to `NO_ACL`, unless the
    /// object resource specifies the `acl` property, when it defaults
    /// to `full`.
    #[prost(enumeration="common_enums::Projection", tag="12")]
    pub projection: i32,
    /// Required. Name of the bucket in which to find the source object.
    #[prost(string, tag="13")]
    pub source_bucket: ::prost::alloc::string::String,
    /// Required. Name of the source object.
    #[prost(string, tag="14")]
    pub source_object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of the source object (as opposed to
    /// the latest version, the default).
    #[prost(int64, tag="15")]
    pub source_generation: i64,
    /// Properties of the resulting object. If not set, duplicate properties of
    /// source object.
    #[prost(message, optional, tag="17")]
    pub destination: ::core::option::Option<Object>,
    /// Resource name of the Cloud KMS key, of the form
    /// `projects/my-project/locations/my-location/keyRings/my-kr/cryptoKeys/my-key`,
    /// that will be used to encrypt the object. Overrides the object
    /// metadata's `kms_key_name` value, if any.
    #[prost(string, tag="20")]
    pub destination_kms_key_name: ::prost::alloc::string::String,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="18")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="19")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Message for deleting an object.
/// Either `bucket` and `object` *or* `upload_id` **must** be set (but not both).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// Required. Name of the bucket in which the object resides.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. The name of the object to delete (when not using a resumable write).
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// The resumable upload_id of the object to delete (when using a
    /// resumable write). This should be copied from the `upload_id` field of
    /// `StartResumableWriteResponse`.
    #[prost(string, tag="3")]
    pub upload_id: ::prost::alloc::string::String,
    /// If present, permanently deletes a specific revision of this object (as
    /// opposed to the latest version, the default).
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="6")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="8")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="10")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="11")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetObjectMedia.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectMediaRequest {
    /// The name of the bucket containing the object to read.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// The name of the object to read.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed
    /// to the latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// The offset for the first byte to return in the read, relative to the start
    /// of the object.
    ///
    /// A negative `read_offset` value will be interpreted as the number of bytes
    /// back from the end of the object to be returned. For example, if an object's
    /// length is 15 bytes, a GetObjectMediaRequest with `read_offset` = -5 and
    /// `read_limit` = 3 would return bytes 10 through 12 of the object.
    #[prost(int64, tag="4")]
    pub read_offset: i64,
    /// The maximum number of `data` bytes the server is allowed to return in the
    /// sum of all `Object` messages. A `read_limit` of zero indicates that there
    /// is no limit, and a negative `read_limit` will cause an error.
    ///
    /// If the stream returns fewer bytes than allowed by the `read_limit` and no
    /// error occurred, the stream includes all data from the `read_offset` to the
    /// end of the resource.
    #[prost(int64, tag="5")]
    pub read_limit: i64,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="6")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="7")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="8")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="9")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="11")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="12")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRequest {
    /// Required. Name of the bucket in which the object resides.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="6")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Set of properties to return. Defaults to `NO_ACL`.
    #[prost(enumeration="common_enums::Projection", tag="8")]
    pub projection: i32,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="10")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="11")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Response message for GetObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectMediaResponse {
    /// A portion of the data for the object. The service **may** leave `data`
    /// empty for any given `ReadResponse`. This enables the service to inform the
    /// client that the request is still live while it is running an operation to
    /// generate more data.
    #[prost(message, optional, tag="1")]
    pub checksummed_data: ::core::option::Option<ChecksummedData>,
    /// The checksums of the complete object. The client should compute one of
    /// these checksums over the downloaded object and compare it against the value
    /// provided here.
    #[prost(message, optional, tag="2")]
    pub object_checksums: ::core::option::Option<ObjectChecksums>,
    /// If read_offset and or read_limit was specified on the
    /// GetObjectMediaRequest, ContentRange will be populated on the first
    /// GetObjectMediaResponse message of the read stream.
    #[prost(message, optional, tag="3")]
    pub content_range: ::core::option::Option<ContentRange>,
    /// Metadata of the object whose media is being returned.
    /// Only populated in the first response in the stream.
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<Object>,
}
/// Describes an attempt to insert an object, possibly over multiple requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertObjectSpec {
    /// Destination object, including its name and its metadata.
    #[prost(message, optional, tag="1")]
    pub resource: ::core::option::Option<Object>,
    /// Apply a predefined set of access controls to this object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="2")]
    pub predefined_acl: i32,
    /// Makes the operation conditional on whether the object's current
    /// generation matches the given value. Setting to 0 makes the operation
    /// succeed only if there are no live versions of the object.
    #[prost(message, optional, tag="3")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// generation does not match the given value. If no live object exists, the
    /// precondition fails. Setting to 0 makes the operation succeed only if
    /// there is a live version of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="5")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="6")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Set of properties to return. Defaults to `NO_ACL`, unless the
    /// object resource specifies the `acl` property, when it defaults
    /// to `full`.
    #[prost(enumeration="common_enums::Projection", tag="7")]
    pub projection: i32,
}
/// Message for writing an object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertObjectRequest {
    /// Required. The offset from the beginning of the object at which the data should be
    /// written.
    ///
    /// In the first `InsertObjectRequest` of a `InsertObject()` action, it
    /// indicates the initial offset for the `Insert()` call. The value **must** be
    /// equal to the `committed_size` that a call to `QueryWriteStatus()` would
    /// return (0 if this is the first write to the object).
    ///
    /// On subsequent calls, this value **must** be no larger than the sum of the
    /// first `write_offset` and the sizes of all `data` chunks sent previously on
    /// this stream.
    ///
    /// An incorrect value will cause an error.
    #[prost(int64, tag="3")]
    pub write_offset: i64,
    /// Checksums for the complete object. If the checksums computed by the service
    /// don't match the specifified checksums the call will fail. May only be
    /// provided in the first or last request (either with first_message, or
    /// finish_write set).
    #[prost(message, optional, tag="6")]
    pub object_checksums: ::core::option::Option<ObjectChecksums>,
    /// If `true`, this indicates that the write is complete. Sending any
    /// `InsertObjectRequest`s subsequent to one in which `finish_write` is `true`
    /// will cause an error.
    /// For a non-resumable write (where the upload_id was not set in the first
    /// message), it is an error not to set this field in the final message of the
    /// stream.
    #[prost(bool, tag="7")]
    pub finish_write: bool,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="8")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="9")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
    /// The first message of each stream should set one of the following.
    #[prost(oneof="insert_object_request::FirstMessage", tags="1, 2")]
    pub first_message: ::core::option::Option<insert_object_request::FirstMessage>,
    /// A portion of the data for the object.
    #[prost(oneof="insert_object_request::Data", tags="4, 5")]
    pub data: ::core::option::Option<insert_object_request::Data>,
}
/// Nested message and enum types in `InsertObjectRequest`.
pub mod insert_object_request {
    /// The first message of each stream should set one of the following.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FirstMessage {
        /// For resumable uploads. This should be the `upload_id` returned from a
        /// call to `StartResumableWriteResponse`.
        #[prost(string, tag="1")]
        UploadId(::prost::alloc::string::String),
        /// For non-resumable uploads. Describes the overall upload, including the
        /// destination bucket and object name, preconditions, etc.
        #[prost(message, tag="2")]
        InsertObjectSpec(super::InsertObjectSpec),
    }
    /// A portion of the data for the object.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The data to insert. If a crc32c checksum is provided that doesn't match
        /// the checksum computed by the service, the request will fail.
        #[prost(message, tag="4")]
        ChecksummedData(super::ChecksummedData),
        /// A reference to an existing object. This can be used to support
        /// several use cases:
        ///   - Writing a sequence of data buffers supports the basic use case of
        ///     uploading a complete object, chunk by chunk.
        ///   - Writing a sequence of references to existing objects allows an
        ///     object to be composed from a collection of objects, which can be
        ///     used to support parallel object writes.
        ///   - Writing a single reference with a given offset and size can be used
        ///     to create an object from a slice of an existing object.
        ///   - Writing an object referencing a object slice (created as noted
        ///     above) followed by a data buffer followed by another object
        ///     slice can be used to support delta upload functionality.
        #[prost(message, tag="5")]
        Reference(super::GetObjectMediaRequest),
    }
}
/// Request message for ListObjects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsRequest {
    /// Required. Name of the bucket in which to look for objects.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Returns results in a directory-like mode. `items` will contain
    /// only objects whose names, aside from the `prefix`, do not
    /// contain `delimiter`. Objects whose names, aside from the
    /// `prefix`, contain `delimiter` will have their name,
    /// truncated after the `delimiter`, returned in
    /// `prefixes`. Duplicate `prefixes` are omitted.
    #[prost(string, tag="2")]
    pub delimiter: ::prost::alloc::string::String,
    /// If true, objects that end in exactly one instance of `delimiter`
    /// will have their metadata included in `items` in addition to
    /// `prefixes`.
    #[prost(bool, tag="3")]
    pub include_trailing_delimiter: bool,
    /// Maximum number of `items` plus `prefixes` to return
    /// in a single page of responses. As duplicate `prefixes` are
    /// omitted, fewer total results may be returned than requested. The service
    /// will use this parameter or 1,000 items, whichever is smaller.
    #[prost(int32, tag="4")]
    pub max_results: i32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter results to objects whose names begin with this prefix.
    #[prost(string, tag="6")]
    pub prefix: ::prost::alloc::string::String,
    /// Set of properties to return. Defaults to `NO_ACL`.
    #[prost(enumeration="common_enums::Projection", tag="7")]
    pub projection: i32,
    /// If `true`, lists all versions of an object as distinct results.
    /// The default is `false`. For more information, see
    /// [Object
    /// Versioning](<https://cloud.google.com/storage/docs/object-versioning>).
    #[prost(bool, tag="9")]
    pub versions: bool,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="10")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request object for `ByteStream.QueryWriteStatus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusRequest {
    /// Required. The name of the resume token for the object whose write status is being
    /// requested.
    #[prost(string, tag="1")]
    pub upload_id: ::prost::alloc::string::String,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="2")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Response object for `ByteStream.QueryWriteStatus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusResponse {
    /// The number of bytes that have been processed for the given object.
    #[prost(int64, tag="1")]
    pub committed_size: i64,
    /// `complete` is `true` only if the client has sent a `InsertObjectRequest`
    /// with `finish_write` set to true, and the server has processed that request.
    #[prost(bool, tag="2")]
    pub complete: bool,
}
/// Request message for RewriteObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriteObjectRequest {
    /// Required. Name of the bucket in which to store the new object. Overrides the provided
    /// object metadata's `bucket` value, if any.
    #[prost(string, tag="1")]
    pub destination_bucket: ::prost::alloc::string::String,
    /// Required. Name of the new object.
    /// Required when the object metadata is not otherwise provided. Overrides the
    /// object metadata's `name` value, if any.
    #[prost(string, tag="2")]
    pub destination_object: ::prost::alloc::string::String,
    /// Resource name of the Cloud KMS key, of the form
    /// `projects/my-project/locations/my-location/keyRings/my-kr/cryptoKeys/my-key`,
    /// that will be used to encrypt the object. Overrides the object
    /// metadata's `kms_key_name` value, if any.
    #[prost(string, tag="3")]
    pub destination_kms_key_name: ::prost::alloc::string::String,
    /// Apply a predefined set of access controls to the destination object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="4")]
    pub destination_predefined_acl: i32,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="6")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the destination object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the destination object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="8")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// generation matches the given value.
    #[prost(message, optional, tag="9")]
    pub if_source_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// generation does not match the given value.
    #[prost(message, optional, tag="10")]
    pub if_source_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="11")]
    pub if_source_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the source object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="12")]
    pub if_source_metageneration_not_match: ::core::option::Option<i64>,
    /// The maximum number of bytes that will be rewritten per rewrite request.
    /// Most callers
    /// shouldn't need to specify this parameter - it is primarily in place to
    /// support testing. If specified the value must be an integral multiple of
    /// 1 MiB (1048576). Also, this only applies to requests where the source and
    /// destination span locations and/or storage classes. Finally, this value must
    /// not change across rewrite calls else you'll get an error that the
    /// `rewriteToken` is invalid.
    #[prost(int64, tag="13")]
    pub max_bytes_rewritten_per_call: i64,
    /// Set of properties to return. Defaults to `NO_ACL`, unless the
    /// object resource specifies the `acl` property, when it defaults
    /// to `full`.
    #[prost(enumeration="common_enums::Projection", tag="14")]
    pub projection: i32,
    /// Include this field (from the previous rewrite response) on each rewrite
    /// request after the first one, until the rewrite response 'done' flag is
    /// true. Calls that provide a rewriteToken can omit all other request fields,
    /// but if included those fields must match the values provided in the first
    /// rewrite request.
    #[prost(string, tag="15")]
    pub rewrite_token: ::prost::alloc::string::String,
    /// Required. Name of the bucket in which to find the source object.
    #[prost(string, tag="16")]
    pub source_bucket: ::prost::alloc::string::String,
    /// Required. Name of the source object.
    #[prost(string, tag="17")]
    pub source_object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of the source object (as opposed to
    /// the latest version, the default).
    #[prost(int64, tag="18")]
    pub source_generation: i64,
    /// Properties of the destination, post-rewrite object.
    #[prost(message, optional, tag="20")]
    pub object: ::core::option::Option<Object>,
    /// The algorithm used to encrypt the source object, if any.
    #[prost(string, tag="21")]
    pub copy_source_encryption_algorithm: ::prost::alloc::string::String,
    /// The encryption key used to encrypt the source object, if any.
    #[prost(string, tag="22")]
    pub copy_source_encryption_key: ::prost::alloc::string::String,
    /// The SHA-256 hash of the key used to encrypt the source object, if any.
    #[prost(string, tag="23")]
    pub copy_source_encryption_key_sha256: ::prost::alloc::string::String,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="24")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="25")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// A rewrite response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriteResponse {
    /// The total bytes written so far, which can be used to provide a waiting user
    /// with a progress indicator. This property is always present in the response.
    #[prost(int64, tag="1")]
    pub total_bytes_rewritten: i64,
    /// The total size of the object being copied in bytes. This property is always
    /// present in the response.
    #[prost(int64, tag="2")]
    pub object_size: i64,
    /// `true` if the copy is finished; otherwise, `false` if
    /// the copy is in progress. This property is always present in the response.
    #[prost(bool, tag="3")]
    pub done: bool,
    /// A token to use in subsequent requests to continue copying data. This token
    /// is present in the response only when there is more data to copy.
    #[prost(string, tag="4")]
    pub rewrite_token: ::prost::alloc::string::String,
    /// A resource containing the metadata for the copied-to object. This property
    /// is present in the response only when copying completes.
    #[prost(message, optional, tag="5")]
    pub resource: ::core::option::Option<Object>,
}
/// Request message StartResumableWrite.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResumableWriteRequest {
    /// The destination bucket, object, and metadata, as well as any preconditions.
    #[prost(message, optional, tag="1")]
    pub insert_object_spec: ::core::option::Option<InsertObjectSpec>,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="3")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="4")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Response object for ByteStream.StartResumableWrite.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResumableWriteResponse {
    /// The upload_id of the newly started resumable write operation. This
    /// value should be copied into the `InsertObjectRequest.upload_id` field.
    #[prost(string, tag="1")]
    pub upload_id: ::prost::alloc::string::String,
}
/// Request message for PatchObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchObjectRequest {
    /// Required. Name of the bucket in which the object resides.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="6")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Apply a predefined set of access controls to this object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="8")]
    pub predefined_acl: i32,
    /// Set of properties to return. Defaults to `FULL`.
    #[prost(enumeration="common_enums::Projection", tag="9")]
    pub projection: i32,
    /// The Object metadata for updating.
    #[prost(message, optional, tag="11")]
    pub metadata: ::core::option::Option<Object>,
    /// List of fields to be updated.
    ///
    /// To specify ALL fields, equivalent to the JSON API's "update" function,
    /// specify a single field with the value `*`. Note: not recommended. If a new
    /// field is introduced at a later time, an older client updating with the `*`
    /// may accidentally reset the new field's value.
    ///
    /// Not specifying any fields is an error.
    /// Not specifying a field while setting that field to a non-default value is
    /// an error.
    #[prost(message, optional, tag="12")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="13")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="14")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for UpdateObject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    /// Required. Name of the bucket in which the object resides.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// If present, selects a specific revision of this object (as opposed to the
    /// latest version, the default).
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// Makes the operation conditional on whether the object's current generation
    /// matches the given value. Setting to 0 makes the operation succeed only if
    /// there are no live versions of the object.
    #[prost(message, optional, tag="4")]
    pub if_generation_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current generation
    /// does not match the given value. If no live object exists, the precondition
    /// fails. Setting to 0 makes the operation succeed only if there is a live
    /// version of the object.
    #[prost(message, optional, tag="5")]
    pub if_generation_not_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration matches the given value.
    #[prost(message, optional, tag="6")]
    pub if_metageneration_match: ::core::option::Option<i64>,
    /// Makes the operation conditional on whether the object's current
    /// metageneration does not match the given value.
    #[prost(message, optional, tag="7")]
    pub if_metageneration_not_match: ::core::option::Option<i64>,
    /// Apply a predefined set of access controls to this object.
    #[prost(enumeration="common_enums::PredefinedObjectAcl", tag="8")]
    pub predefined_acl: i32,
    /// Set of properties to return. Defaults to `FULL`.
    #[prost(enumeration="common_enums::Projection", tag="9")]
    pub projection: i32,
    /// The Object metadata for updating.
    #[prost(message, optional, tag="11")]
    pub metadata: ::core::option::Option<Object>,
    /// A set of parameters common to Storage API requests concerning an object.
    #[prost(message, optional, tag="12")]
    pub common_object_request_params: ::core::option::Option<CommonObjectRequestParams>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="13")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for WatchAllObjects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchAllObjectsRequest {
    /// Name of the bucket in which to look for objects.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// If `true`, lists all versions of an object as distinct results.
    /// The default is `false`. For more information, see
    /// [Object
    /// Versioning](<https://cloud.google.com/storage/docs/object-versioning>).
    #[prost(bool, tag="2")]
    pub versions: bool,
    /// Returns results in a directory-like mode. `items` will contain
    /// only objects whose names, aside from the `prefix`, do not
    /// contain `delimiter`. Objects whose names, aside from the
    /// `prefix`, contain `delimiter` will have their name,
    /// truncated after the `delimiter`, returned in
    /// `prefixes`. Duplicate `prefixes` are omitted.
    #[prost(string, tag="3")]
    pub delimiter: ::prost::alloc::string::String,
    /// Maximum number of `items` plus `prefixes` to return
    /// in a single page of responses. As duplicate `prefixes` are
    /// omitted, fewer total results may be returned than requested. The service
    /// will use this parameter or 1,000 items, whichever is smaller.
    #[prost(int32, tag="4")]
    pub max_results: i32,
    /// Filter results to objects whose names begin with this prefix.
    #[prost(string, tag="5")]
    pub prefix: ::prost::alloc::string::String,
    /// If true, objects that end in exactly one instance of `delimiter`
    /// will have their metadata included in `items` in addition to
    /// `prefixes`.
    #[prost(bool, tag="6")]
    pub include_trailing_delimiter: bool,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// Set of properties to return. Defaults to `NO_ACL`.
    #[prost(enumeration="common_enums::Projection", tag="8")]
    pub projection: i32,
    /// Properties of the channel to be inserted.
    #[prost(message, optional, tag="10")]
    pub channel: ::core::option::Option<Channel>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="11")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request message for GetProjectServiceAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectServiceAccountRequest {
    /// Required. Project ID.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHmacKeyRequest {
    /// Required. The project that the HMAC-owning service account lives in.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The service account to create the HMAC for.
    #[prost(string, tag="2")]
    pub service_account_email: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Create hmac response.  The only time the secret for an HMAC will be returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHmacKeyResponse {
    /// Key metadata.
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<HmacKeyMetadata>,
    /// HMAC key secret material.
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
}
/// Request object to delete a given HMAC key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHmacKeyRequest {
    /// Required. The identifying key for the HMAC to delete.
    #[prost(string, tag="1")]
    pub access_id: ::prost::alloc::string::String,
    /// Required. The project id the HMAC key lies in.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request object to get metadata on a given HMAC key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHmacKeyRequest {
    /// Required. The identifying key for the HMAC to delete.
    #[prost(string, tag="1")]
    pub access_id: ::prost::alloc::string::String,
    /// Required. The project id the HMAC key lies in.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="3")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Request to fetch a list of HMAC keys under a given project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHmacKeysRequest {
    /// Required. The project id to list HMAC keys for.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// An optional filter to only return HMAC keys for one service account.
    #[prost(string, tag="2")]
    pub service_account_email: ::prost::alloc::string::String,
    /// An optional bool to return deleted keys that have not been wiped out yet.
    #[prost(bool, tag="3")]
    pub show_deleted_keys: bool,
    /// The maximum number of keys to return.
    #[prost(int32, tag="4")]
    pub max_results: i32,
    /// A previously returned token from ListHmacKeysResponse to get the next page.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="6")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Hmac key list response with next page information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHmacKeysResponse {
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag="1")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The list of items.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<HmacKeyMetadata>,
}
/// Request object to update an HMAC key state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHmacKeyRequest {
    /// Required. The id of the HMAC key.
    #[prost(string, tag="1")]
    pub access_id: ::prost::alloc::string::String,
    /// Required. The project id the HMAC's service account lies in.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The service account owner of the HMAC key.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<HmacKeyMetadata>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="5")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// A wrapper around the IAM get policy request to support our
/// common_request_params.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIamPolicyRequest {
    /// The request sent to IAM.
    #[prost(message, optional, tag="1")]
    pub iam_request: ::core::option::Option<super::super::iam::v1::GetIamPolicyRequest>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="2")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// A wrapper around the IAM set policy request to support our
/// common_request_params.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIamPolicyRequest {
    /// The request sent to IAM.
    #[prost(message, optional, tag="1")]
    pub iam_request: ::core::option::Option<super::super::iam::v1::SetIamPolicyRequest>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="2")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// A wrapper around the IAM test iam permissions request to support our
/// common_request_params.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestIamPermissionsRequest {
    /// The request sent to IAM.
    #[prost(message, optional, tag="1")]
    pub iam_request: ::core::option::Option<super::super::iam::v1::TestIamPermissionsRequest>,
    /// A set of parameters common to all Storage API requests.
    #[prost(message, optional, tag="2")]
    pub common_request_params: ::core::option::Option<CommonRequestParams>,
}
/// Parameters that can be passed to any object request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonObjectRequestParams {
    /// Encryption algorithm used with Customer-Supplied Encryption Keys feature.
    #[prost(string, tag="1")]
    pub encryption_algorithm: ::prost::alloc::string::String,
    /// Encryption key used with Customer-Supplied Encryption Keys feature.
    #[prost(string, tag="2")]
    pub encryption_key: ::prost::alloc::string::String,
    /// SHA256 hash of encryption key used with Customer-Supplied Encryption Keys
    /// feature.
    #[prost(string, tag="3")]
    pub encryption_key_sha256: ::prost::alloc::string::String,
}
/// Parameters that can be passed to any request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonRequestParams {
    /// Required. Required when using buckets with Requestor Pays feature enabled.
    #[prost(string, tag="1")]
    pub user_project: ::prost::alloc::string::String,
    /// Lets you enforce per-user quotas from a server-side application even in
    /// cases when the user's IP address is unknown. This can occur, for example,
    /// with applications that run cron jobs on App Engine on a user's behalf.
    /// You can choose any arbitrary string that uniquely identifies a user, but it
    /// is limited to 40 characters.
    /// Overrides user_ip if both are provided.
    #[prost(string, tag="2")]
    pub quota_user: ::prost::alloc::string::String,
    /// Subset of fields to include in the response.
    #[prost(message, optional, tag="4")]
    pub fields: ::core::option::Option<::prost_types::FieldMask>,
}
/// Shared constants.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConstants {
}
/// Nested message and enum types in `ServiceConstants`.
pub mod service_constants {
    /// A collection of constant values meaningful to the Storage API.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Values {
        /// Unused. Proto3 requires first enum to be 0.
        Unspecified = 0,
        /// The maximum size chunk that can will be returned in a single
        /// ReadRequest.
        /// 2 MiB.
        MaxReadChunkBytes = 2097152,
        /// The maximum size of an object in MB - whether written in a single stream
        /// or composed from multiple other objects.
        /// 5 TiB.
        MaxObjectSizeMb = 5242880,
        /// The maximum length field name that can be sent in a single
        /// custom metadata field.
        /// 1 KiB.
        MaxCustomMetadataFieldNameBytes = 1024,
        /// The maximum length field value that can be sent in a single
        /// custom_metadata field.
        /// 4 KiB.
        MaxCustomMetadataFieldValueBytes = 4096,
        /// The maximum total bytes that can be populated into all field names and
        /// values of the custom_metadata for one object.
        /// 8 KiB.
        MaxCustomMetadataTotalSizeBytes = 8192,
        /// The maximum total bytes that can be populated into all bucket metadata
        /// fields.
        /// 20 KiB.
        MaxBucketMetadataTotalSizeBytes = 20480,
        /// The maximum number of NotificationConfigurations that can be registered
        /// for a given bucket.
        MaxNotificationConfigsPerBucket = 100,
        /// The maximum number of custom attributes per NotificationConfig.
        MaxNotificationCustomAttributes = 5,
        /// The maximum length of a custom attribute key included in
        /// NotificationConfig.
        MaxNotificationCustomAttributeKeyLength = 256,
        /// The maximum number of key/value entries per bucket label.
        MaxLabelsEntriesCount = 64,
        /// The maximum character length of the key or value in a bucket
        /// label map.
        MaxLabelsKeyValueLength = 63,
        /// The maximum byte size of the key or value in a bucket label
        /// map.
        MaxLabelsKeyValueBytes = 128,
        /// The maximum number of object IDs that can be included in a
        /// DeleteObjectsRequest.
        MaxObjectIdsPerDeleteObjectsRequest = 1000,
        /// The maximum number of days for which a token returned by the
        /// GetListObjectsSplitPoints RPC is valid.
        SplitTokenMaxValidDays = 14,
    }
}
/// Generated client implementations.
pub mod storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Manages Google Cloud Storage resources.
    #[derive(Debug, Clone)]
    pub struct StorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StorageClient<T>
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
        ) -> StorageClient<InterceptedService<T, F>>
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
            StorageClient::new(InterceptedService::new(inner, interceptor))
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
        /// Permanently deletes the ACL entry for the specified entity on the specified
        /// bucket.
        pub async fn delete_bucket_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketAccessControlRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteBucketAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the ACL entry for the specified entity on the specified bucket.
        pub async fn get_bucket_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketAccessControlRequest>,
        ) -> Result<tonic::Response<super::BucketAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetBucketAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ACL entry on the specified bucket.
        pub async fn insert_bucket_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertBucketAccessControlRequest>,
        ) -> Result<tonic::Response<super::BucketAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertBucketAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves ACL entries on the specified bucket.
        pub async fn list_bucket_access_controls(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBucketAccessControlsRequest>,
        ) -> Result<
            tonic::Response<super::ListBucketAccessControlsResponse>,
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
                "/google.storage.v1.Storage/ListBucketAccessControls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an ACL entry on the specified bucket. Equivalent to
        /// PatchBucketAccessControl, but all unspecified fields will be
        /// reset to their default values.
        pub async fn update_bucket_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBucketAccessControlRequest>,
        ) -> Result<tonic::Response<super::BucketAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateBucketAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an ACL entry on the specified bucket.
        pub async fn patch_bucket_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchBucketAccessControlRequest>,
        ) -> Result<tonic::Response<super::BucketAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/PatchBucketAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes an empty bucket.
        pub async fn delete_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for the specified bucket.
        pub async fn get_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketRequest>,
        ) -> Result<tonic::Response<super::Bucket>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new bucket.
        pub async fn insert_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertBucketRequest>,
        ) -> Result<tonic::Response<super::Bucket>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List active object change notification channels for this bucket.
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelsRequest>,
        ) -> Result<tonic::Response<super::ListChannelsResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/ListChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a list of buckets for a given project.
        pub async fn list_buckets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBucketsRequest>,
        ) -> Result<tonic::Response<super::ListBucketsResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/ListBuckets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Locks retention policy on a bucket.
        pub async fn lock_bucket_retention_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::LockRetentionPolicyRequest>,
        ) -> Result<tonic::Response<super::Bucket>, tonic::Status> {
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
                "/google.storage.v1.Storage/LockBucketRetentionPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM policy for the specified bucket.
        pub async fn get_bucket_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIamPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.storage.v1.Storage/GetBucketIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an IAM policy for the specified bucket.
        pub async fn set_bucket_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetIamPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.storage.v1.Storage/SetBucketIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Tests a set of permissions on the given bucket to see which, if
        /// any, are held by the caller.
        pub async fn test_bucket_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.storage.v1.Storage/TestBucketIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a bucket. Changes to the bucket will be readable immediately after
        /// writing, but configuration changes may take time to propagate.
        pub async fn patch_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchBucketRequest>,
        ) -> Result<tonic::Response<super::Bucket>, tonic::Status> {
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
                "/google.storage.v1.Storage/PatchBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a bucket. Equivalent to PatchBucket, but always replaces all
        /// mutatable fields of the bucket with new values, reverting all
        /// unspecified fields to their default values.
        /// Like PatchBucket, Changes to the bucket will be readable immediately after
        /// writing, but configuration changes may take time to propagate.
        pub async fn update_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBucketRequest>,
        ) -> Result<tonic::Response<super::Bucket>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Halts "Object Change Notification" push messagages.
        /// See https://cloud.google.com/storage/docs/object-change-notification
        /// Note: this is not related to the newer "Notifications" resource, which
        /// are stopped using DeleteNotification.
        pub async fn stop_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::StopChannelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/StopChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes the default object ACL entry for the specified entity
        /// on the specified bucket.
        pub async fn delete_default_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteDefaultObjectAccessControlRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteDefaultObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the default object ACL entry for the specified entity on the
        /// specified bucket.
        pub async fn get_default_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultObjectAccessControlRequest>,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetDefaultObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new default object ACL entry on the specified bucket.
        pub async fn insert_default_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<
                super::InsertDefaultObjectAccessControlRequest,
            >,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertDefaultObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves default object ACL entries on the specified bucket.
        pub async fn list_default_object_access_controls(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDefaultObjectAccessControlsRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListObjectAccessControlsResponse>,
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
                "/google.storage.v1.Storage/ListDefaultObjectAccessControls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a default object ACL entry on the specified bucket.
        pub async fn patch_default_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PatchDefaultObjectAccessControlRequest,
            >,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/PatchDefaultObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a default object ACL entry on the specified bucket. Equivalent to
        /// PatchDefaultObjectAccessControl, but modifies all unspecified fields to
        /// their default values.
        pub async fn update_default_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateDefaultObjectAccessControlRequest,
            >,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateDefaultObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes a notification subscription.
        /// Note: Older, "Object Change Notification" push subscriptions should be
        /// deleted using StopChannel instead.
        pub async fn delete_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// View a notification configuration.
        pub async fn get_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationRequest>,
        ) -> Result<tonic::Response<super::Notification>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a notification subscription for a given bucket.
        /// These notifications, when triggered, publish messages to the specified
        /// Cloud Pub/Sub topics.
        /// See https://cloud.google.com/storage/docs/pubsub-notifications.
        pub async fn insert_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertNotificationRequest>,
        ) -> Result<tonic::Response<super::Notification>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a list of notification subscriptions for a given bucket.
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationsResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/ListNotifications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Permanently deletes the ACL entry for the specified entity on the specified
        /// object.
        pub async fn delete_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectAccessControlRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the ACL entry for the specified entity on the specified object.
        pub async fn get_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectAccessControlRequest>,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ACL entry on the specified object.
        pub async fn insert_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertObjectAccessControlRequest>,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves ACL entries on the specified object.
        pub async fn list_object_access_controls(
            &mut self,
            request: impl tonic::IntoRequest<super::ListObjectAccessControlsRequest>,
        ) -> Result<
            tonic::Response<super::ListObjectAccessControlsResponse>,
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
                "/google.storage.v1.Storage/ListObjectAccessControls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches an ACL entry on the specified object.  Patch is similar to update,
        /// but only applies or appends the specified fields in the
        /// object_access_control object.  Other fields are unaffected.
        pub async fn patch_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchObjectAccessControlRequest>,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/PatchObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an ACL entry on the specified object.
        pub async fn update_object_access_control(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectAccessControlRequest>,
        ) -> Result<tonic::Response<super::ObjectAccessControl>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateObjectAccessControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Concatenates a list of existing objects into a new object in the same
        /// bucket.
        pub async fn compose_object(
            &mut self,
            request: impl tonic::IntoRequest<super::ComposeObjectRequest>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/ComposeObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Copies a source object to a destination object. Optionally overrides
        /// metadata.
        pub async fn copy_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CopyObjectRequest>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/CopyObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an object and its metadata. Deletions are permanent if versioning
        /// is not enabled for the bucket, or if the `generation` parameter
        /// is used.
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves an object's metadata.
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reads an object's data.
        pub async fn get_object_media(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectMediaRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetObjectMediaResponse>>,
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
                "/google.storage.v1.Storage/GetObjectMedia",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Stores a new object and metadata.
        ///
        /// An object can be written either in a single message stream or in a
        /// resumable sequence of message streams. To write using a single stream,
        /// the client should include in the first message of the stream an
        /// `InsertObjectSpec` describing the destination bucket, object, and any
        /// preconditions. Additionally, the final message must set 'finish_write' to
        /// true, or else it is an error.
        ///
        /// For a resumable write, the client should instead call
        /// `StartResumableWrite()` and provide that method an `InsertObjectSpec.`
        /// They should then attach the returned `upload_id` to the first message of
        /// each following call to `Insert`. If there is an error or the connection is
        /// broken during the resumable `Insert()`, the client should check the status
        /// of the `Insert()` by calling `QueryWriteStatus()` and continue writing from
        /// the returned `committed_size`. This may be less than the amount of data the
        /// client previously sent.
        ///
        /// The service will not view the object as complete until the client has
        /// sent an `Insert` with `finish_write` set to `true`. Sending any
        /// requests on a stream after sending a request with `finish_write` set to
        /// `true` will cause an error. The client **should** check the
        /// `Object` it receives to determine how much data the service was
        /// able to commit and whether the service views the object as complete.
        pub async fn insert_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::InsertObjectRequest,
            >,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/InsertObject",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Retrieves a list of objects matching the criteria.
        pub async fn list_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListObjectsRequest>,
        ) -> Result<tonic::Response<super::ListObjectsResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/ListObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Rewrites a source object to a destination object. Optionally overrides
        /// metadata.
        pub async fn rewrite_object(
            &mut self,
            request: impl tonic::IntoRequest<super::RewriteObjectRequest>,
        ) -> Result<tonic::Response<super::RewriteResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/RewriteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a resumable write. How long the write operation remains valid, and
        /// what happens when the write operation becomes invalid, are
        /// service-dependent.
        pub async fn start_resumable_write(
            &mut self,
            request: impl tonic::IntoRequest<super::StartResumableWriteRequest>,
        ) -> Result<tonic::Response<super::StartResumableWriteResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/StartResumableWrite",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Determines the `committed_size` for an object that is being written, which
        /// can then be used as the `write_offset` for the next `Write()` call.
        ///
        /// If the object does not exist (i.e., the object has been deleted, or the
        /// first `Write()` has not yet reached the service), this method returns the
        /// error `NOT_FOUND`.
        ///
        /// The client **may** call `QueryWriteStatus()` at any time to determine how
        /// much data has been processed for this object. This is useful if the
        /// client is buffering data and needs to know which data can be safely
        /// evicted. For any sequence of `QueryWriteStatus()` calls for a given
        /// object name, the sequence of returned `committed_size` values will be
        /// non-decreasing.
        pub async fn query_write_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWriteStatusRequest>,
        ) -> Result<tonic::Response<super::QueryWriteStatusResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/QueryWriteStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an object's metadata.
        pub async fn patch_object(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchObjectRequest>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/PatchObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an object's metadata. Equivalent to PatchObject, but always
        /// replaces all mutatable fields of the bucket with new values, reverting all
        /// unspecified fields to their default values.
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM policy for the specified object.
        pub async fn get_object_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIamPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.storage.v1.Storage/GetObjectIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an IAM policy for the specified object.
        pub async fn set_object_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetIamPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.storage.v1.Storage/SetObjectIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Tests a set of permissions on the given object to see which, if
        /// any, are held by the caller.
        pub async fn test_object_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.storage.v1.Storage/TestObjectIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Watch for changes on all objects in a bucket.
        pub async fn watch_all_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchAllObjectsRequest>,
        ) -> Result<tonic::Response<super::Channel>, tonic::Status> {
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
                "/google.storage.v1.Storage/WatchAllObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the name of a project's Google Cloud Storage service account.
        pub async fn get_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new HMAC key for the given service account.
        pub async fn create_hmac_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHmacKeyRequest>,
        ) -> Result<tonic::Response<super::CreateHmacKeyResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/CreateHmacKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a given HMAC key.  Key must be in an INACTIVE state.
        pub async fn delete_hmac_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHmacKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.storage.v1.Storage/DeleteHmacKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an existing HMAC key metadata for the given id.
        pub async fn get_hmac_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHmacKeyRequest>,
        ) -> Result<tonic::Response<super::HmacKeyMetadata>, tonic::Status> {
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
                "/google.storage.v1.Storage/GetHmacKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists HMAC keys under a given project with the additional filters provided.
        pub async fn list_hmac_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHmacKeysRequest>,
        ) -> Result<tonic::Response<super::ListHmacKeysResponse>, tonic::Status> {
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
                "/google.storage.v1.Storage/ListHmacKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a given HMAC key state between ACTIVE and INACTIVE.
        pub async fn update_hmac_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHmacKeyRequest>,
        ) -> Result<tonic::Response<super::HmacKeyMetadata>, tonic::Status> {
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
                "/google.storage.v1.Storage/UpdateHmacKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
