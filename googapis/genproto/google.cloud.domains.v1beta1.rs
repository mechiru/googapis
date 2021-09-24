/// The `Registration` resource facilitates managing and configuring domain name
/// registrations.
///
///
/// To create a new `Registration` resource, find a suitable domain name by
/// calling the `SearchDomains` method with a query to see available domain name
/// options. After choosing a name, call `RetrieveRegisterParameters` to
/// ensure availability and obtain information like pricing, which is needed to
/// build a call to `RegisterDomain`.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registration {
    /// Output only. Name of the `Registration` resource, in the format
    /// `projects/*/locations/*/registrations/<domain_name>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format.
    #[prost(string, tag = "2")]
    pub domain_name: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the `Registration` resource.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The expiration timestamp of the `Registration`.
    #[prost(message, optional, tag = "6")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the `Registration`
    #[prost(enumeration = "registration::State", tag = "7")]
    pub state: i32,
    /// Output only. The set of issues with the `Registration` that require attention.
    #[prost(
        enumeration = "registration::Issue",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub issues: ::prost::alloc::vec::Vec<i32>,
    /// Set of labels associated with the `Registration`.
    #[prost(map = "string, string", tag = "9")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Settings for management of the `Registration`, including renewal, billing,
    /// and transfer. You cannot update these with the `UpdateRegistration`
    /// method. To update these settings, use the `ConfigureManagementSettings`
    /// method.
    #[prost(message, optional, tag = "10")]
    pub management_settings: ::core::option::Option<ManagementSettings>,
    /// Settings controlling the DNS configuration of the `Registration`. You
    /// cannot update these with the `UpdateRegistration` method. To update these
    /// settings, use the `ConfigureDnsSettings` method.
    #[prost(message, optional, tag = "11")]
    pub dns_settings: ::core::option::Option<DnsSettings>,
    /// Required. Settings for contact information linked to the `Registration`. You cannot
    /// update these with the `UpdateRegistration` method. To update these
    /// settings, use the `ConfigureContactSettings` method.
    #[prost(message, optional, tag = "12")]
    pub contact_settings: ::core::option::Option<ContactSettings>,
    /// Output only. Pending contact settings for the `Registration`. Updates to the
    /// `contact_settings` field that change its `registrant_contact` or `privacy`
    /// fields require email confirmation by the `registrant_contact`
    /// before taking effect. This field is set only if there are pending updates
    /// to the `contact_settings` that have not yet been confirmed. To confirm the
    /// changes, the `registrant_contact` must follow the instructions in the
    /// email they receive.
    #[prost(message, optional, tag = "13")]
    pub pending_contact_settings: ::core::option::Option<ContactSettings>,
    /// Output only. Set of options for the `contact_settings.privacy` field that this
    /// `Registration` supports.
    #[prost(enumeration = "ContactPrivacy", repeated, packed = "false", tag = "14")]
    pub supported_privacy: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Registration`.
pub mod registration {
    /// Possible states of a `Registration`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is undefined.
        Unspecified = 0,
        /// The domain is being registered.
        RegistrationPending = 1,
        /// The domain registration failed. You can delete resources in this state
        /// to allow registration to be retried.
        RegistrationFailed = 2,
        /// The domain is registered and operational. The domain renews automatically
        /// as long as it remains in this state.
        Active = 6,
        /// The domain is suspended and inoperative. For more details, see the
        /// `issues` field.
        Suspended = 7,
        /// The domain has been exported from Cloud Domains to
        /// [Google Domains](https://domains.google/). You can no longer update it
        /// with this API, and information shown about it may be stale. Without further action, domains in this
        /// state expire at their `expire_time`. You can delete the resource
        /// after the `expire_time` has passed.
        Exported = 8,
    }
    /// Possible issues with a `Registration` that require attention.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Issue {
        /// The issue is undefined.
        Unspecified = 0,
        /// Contact the Cloud Support team to resolve a problem with this domain.
        ContactSupport = 1,
        /// [ICANN](https://icann.org/) requires verification of the email address
        /// in the `Registration`'s `contact_settings.registrant_contact` field. To
        /// verify the email address, follow the
        /// instructions in the email the `registrant_contact` receives following
        /// registration. If you do not complete email verification within
        /// 14 days of registration, the domain is suspended. To resend the
        /// verification email, call ConfigureContactSettings and provide the current
        /// `registrant_contact.email`.
        UnverifiedEmail = 2,
    }
}
/// Defines renewal, billing, and transfer settings for a `Registration`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagementSettings {
    /// Output only. The renewal method for this `Registration`.
    #[prost(enumeration = "management_settings::RenewalMethod", tag = "3")]
    pub renewal_method: i32,
    /// Controls whether the domain can be transferred to another registrar.
    #[prost(enumeration = "TransferLockState", tag = "4")]
    pub transfer_lock_state: i32,
}
/// Nested message and enum types in `ManagementSettings`.
pub mod management_settings {
    /// Defines how the `Registration` is renewed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RenewalMethod {
        /// The renewal method is undefined.
        Unspecified = 0,
        /// The domain is automatically renewed each year .
        ///
        /// To disable automatic renewals, export the domain by calling
        /// `ExportRegistration` .
        AutomaticRenewal = 1,
        /// The domain must be explicitly renewed each year before its
        /// `expire_time`. This option is only available when the `Registration`
        /// is in state `EXPORTED`.
        ///
        /// To manage the domain's current billing and
        /// renewal settings, go to [Google Domains](https://domains.google/).
        ManualRenewal = 2,
    }
}
/// Defines the DNS configuration of a `Registration`, including name servers,
/// DNSSEC, and glue records.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsSettings {
    /// The list of glue records for this `Registration`. Commonly empty.
    #[prost(message, repeated, tag = "4")]
    pub glue_records: ::prost::alloc::vec::Vec<dns_settings::GlueRecord>,
    /// The DNS provider of the registration.
    #[prost(oneof = "dns_settings::DnsProvider", tags = "1, 2")]
    pub dns_provider: ::core::option::Option<dns_settings::DnsProvider>,
}
/// Nested message and enum types in `DnsSettings`.
pub mod dns_settings {
    /// Configuration for an arbitrary DNS provider.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomDns {
        /// Required. A list of name servers that store the DNS zone for this domain. Each name
        /// server is a domain name, with Unicode domain names expressed in
        /// Punycode format.
        #[prost(string, repeated, tag = "1")]
        pub name_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of DS records for this domain, which are used to enable DNSSEC.
        /// The domain's DNS provider can provide the values to set here. If this
        /// field is empty, DNSSEC is disabled.
        #[prost(message, repeated, tag = "2")]
        pub ds_records: ::prost::alloc::vec::Vec<DsRecord>,
    }
    /// Configuration for using the free DNS zone provided by Google Domains as a
    /// `Registration`'s `dns_provider`. You cannot configure the DNS zone itself
    /// using the API. To configure the DNS zone, go to
    /// [Google Domains](https://domains.google/).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoogleDomainsDns {
        /// Output only. A list of name servers that store the DNS zone for this domain. Each name
        /// server is a domain name, with Unicode domain names expressed in
        /// Punycode format. This field is automatically populated with the name
        /// servers assigned to the Google Domains DNS zone.
        #[prost(string, repeated, tag = "1")]
        pub name_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Required. The state of DS records for this domain. Used to enable or disable
        /// automatic DNSSEC.
        #[prost(enumeration = "DsState", tag = "2")]
        pub ds_state: i32,
        /// Output only. The list of DS records published for this domain. The list is
        /// automatically populated when `ds_state` is `DS_RECORDS_PUBLISHED`,
        /// otherwise it remains empty.
        #[prost(message, repeated, tag = "3")]
        pub ds_records: ::prost::alloc::vec::Vec<DsRecord>,
    }
    /// Defines a Delegation Signer (DS) record, which is needed to enable DNSSEC
    /// for a domain. It contains a digest (hash) of a DNSKEY record that must be
    /// present in the domain's DNS zone.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DsRecord {
        /// The key tag of the record. Must be set in range 0 -- 65535.
        #[prost(int32, tag = "1")]
        pub key_tag: i32,
        /// The algorithm used to generate the referenced DNSKEY.
        #[prost(enumeration = "ds_record::Algorithm", tag = "2")]
        pub algorithm: i32,
        /// The hash function used to generate the digest of the referenced DNSKEY.
        #[prost(enumeration = "ds_record::DigestType", tag = "3")]
        pub digest_type: i32,
        /// The digest generated from the referenced DNSKEY.
        #[prost(string, tag = "4")]
        pub digest: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `DsRecord`.
    pub mod ds_record {
        /// List of algorithms used to create a DNSKEY. Certain
        /// algorithms are not supported for particular domains.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Algorithm {
            /// The algorithm is unspecified.
            Unspecified = 0,
            /// DSA/SHA1. Not recommended for new deployments.
            Dsa = 3,
            /// ECC. Not recommended for new deployments.
            Ecc = 4,
            /// RSA/SHA-1. Not recommended for new deployments.
            Rsasha1 = 5,
            /// DSA-NSEC3-SHA1. Not recommended for new deployments.
            Dsansec3sha1 = 6,
            /// RSA/SHA1-NSEC3-SHA1. Not recommended for new deployments.
            Rsasha1nsec3sha1 = 7,
            /// RSA/SHA-256.
            Rsasha256 = 8,
            /// RSA/SHA-512.
            Rsasha512 = 10,
            /// GOST R 34.10-2001.
            Eccgost = 12,
            /// ECDSA Curve P-256 with SHA-256.
            Ecdsap256sha256 = 13,
            /// ECDSA Curve P-384 with SHA-384.
            Ecdsap384sha384 = 14,
            /// Ed25519.
            Ed25519 = 15,
            /// Ed448.
            Ed448 = 16,
        }
        /// List of hash functions that may have been used to generate a digest of a
        /// DNSKEY.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum DigestType {
            /// The DigestType is unspecified.
            Unspecified = 0,
            /// SHA-1. Not recommended for new deployments.
            Sha1 = 1,
            /// SHA-256.
            Sha256 = 2,
            /// GOST R 34.11-94.
            Gost3411 = 3,
            /// SHA-384.
            Sha384 = 4,
        }
    }
    /// Defines a host on your domain that is a DNS name server for your domain
    /// and/or other domains. Glue records are a way of making the IP address of a
    /// name server known, even when it serves DNS queries for its parent domain.
    /// For example, when `ns.example.com` is a name server for `example.com`, the
    /// host `ns.example.com` must have a glue record to break the circular DNS
    /// reference.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GlueRecord {
        /// Required. Domain name of the host in Punycode format.
        #[prost(string, tag = "1")]
        pub host_name: ::prost::alloc::string::String,
        /// List of IPv4 addresses corresponding to this host in the standard decimal
        /// format (e.g. `198.51.100.1`). At least one of `ipv4_address` and
        /// `ipv6_address` must be set.
        #[prost(string, repeated, tag = "2")]
        pub ipv4_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of IPv6 addresses corresponding to this host in the standard
        /// hexadecimal format (e.g. `2001:db8::`). At least one of
        /// `ipv4_address` and `ipv6_address` must be set.
        #[prost(string, repeated, tag = "3")]
        pub ipv6_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The publication state of DS records for a `Registration`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DsState {
        /// DS state is unspecified.
        Unspecified = 0,
        /// DNSSEC is disabled for this domain. No DS records for this domain are
        /// published in the parent DNS zone.
        DsRecordsUnpublished = 1,
        /// DNSSEC is enabled for this domain. Appropriate DS records for this domain
        /// are published in the parent DNS zone. This option is valid only if the
        /// DNS zone referenced in the `Registration`'s `dns_provider` field is
        /// already DNSSEC-signed.
        DsRecordsPublished = 2,
    }
    /// The DNS provider of the registration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DnsProvider {
        /// An arbitrary DNS provider identified by its name servers.
        #[prost(message, tag = "1")]
        CustomDns(CustomDns),
        /// The free DNS zone provided by
        /// [Google Domains](https://domains.google/).
        #[prost(message, tag = "2")]
        GoogleDomainsDns(GoogleDomainsDns),
    }
}
/// Defines the contact information associated with a `Registration`.
///
/// [ICANN](https://icann.org/) requires all domain names to have associated
/// contact information. The `registrant_contact` is considered the
/// domain's legal owner, and often the other contacts are identical.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactSettings {
    /// Required. Privacy setting for the contacts associated with the `Registration`.
    #[prost(enumeration = "ContactPrivacy", tag = "1")]
    pub privacy: i32,
    /// Required. The registrant contact for the `Registration`.
    ///
    /// *Caution: Anyone with access to this email address, phone number,
    /// and/or postal address can take control of the domain.*
    ///
    /// *Warning: For new `Registration`s, the registrant will receive an email
    /// confirmation that they must complete within 14 days to avoid domain
    /// suspension.*
    #[prost(message, optional, tag = "2")]
    pub registrant_contact: ::core::option::Option<contact_settings::Contact>,
    /// Required. The administrative contact for the `Registration`.
    #[prost(message, optional, tag = "3")]
    pub admin_contact: ::core::option::Option<contact_settings::Contact>,
    /// Required. The technical contact for the `Registration`.
    #[prost(message, optional, tag = "4")]
    pub technical_contact: ::core::option::Option<contact_settings::Contact>,
}
/// Nested message and enum types in `ContactSettings`.
pub mod contact_settings {
    /// Details required for a contact associated with a `Registration`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contact {
        /// Required. Postal address of the contact.
        #[prost(message, optional, tag = "1")]
        pub postal_address:
            ::core::option::Option<super::super::super::super::r#type::PostalAddress>,
        /// Required. Email address of the contact.
        #[prost(string, tag = "2")]
        pub email: ::prost::alloc::string::String,
        /// Required. Phone number of the contact in international format. For example,
        /// `"+1-800-555-0123"`.
        #[prost(string, tag = "3")]
        pub phone_number: ::prost::alloc::string::String,
        /// Fax number of the contact in international format. For example,
        /// `"+1-800-555-0123"`.
        #[prost(string, tag = "4")]
        pub fax_number: ::prost::alloc::string::String,
    }
}
/// Request for the `SearchDomains` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDomainsRequest {
    /// Required. String used to search for available domain names.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Required. The location. Must be in the format `projects/*/locations/*`.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
}
/// Response for the `SearchDomains` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDomainsResponse {
    /// Results of the domain name search.
    #[prost(message, repeated, tag = "1")]
    pub register_parameters: ::prost::alloc::vec::Vec<RegisterParameters>,
}
/// Request for the `RetrieveRegisterParameters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveRegisterParametersRequest {
    /// Required. The domain name. Unicode domain names must be expressed in Punycode format.
    #[prost(string, tag = "1")]
    pub domain_name: ::prost::alloc::string::String,
    /// Required. The location. Must be in the format `projects/*/locations/*`.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
}
/// Response for the `RetrieveRegisterParameters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveRegisterParametersResponse {
    /// Parameters to use when calling the `RegisterDomain` method.
    #[prost(message, optional, tag = "1")]
    pub register_parameters: ::core::option::Option<RegisterParameters>,
}
/// Request for the `RegisterDomain` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDomainRequest {
    /// Required. The parent resource of the `Registration`. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The complete `Registration` resource to be created.
    #[prost(message, optional, tag = "2")]
    pub registration: ::core::option::Option<Registration>,
    /// The list of domain notices that you acknowledge. Call
    /// `RetrieveRegisterParameters` to see the notices that need acknowledgement.
    #[prost(enumeration = "DomainNotice", repeated, tag = "3")]
    pub domain_notices: ::prost::alloc::vec::Vec<i32>,
    /// The list of contact notices that the caller acknowledges. The notices
    /// required here depend on the values specified in
    /// `registration.contact_settings`.
    #[prost(enumeration = "ContactNotice", repeated, tag = "4")]
    pub contact_notices: ::prost::alloc::vec::Vec<i32>,
    /// Required. Yearly price to register or renew the domain.
    /// The value that should be put here can be obtained from
    /// RetrieveRegisterParameters or SearchDomains calls.
    #[prost(message, optional, tag = "5")]
    pub yearly_price: ::core::option::Option<super::super::super::r#type::Money>,
    /// When true, only validation will be performed, without actually registering
    /// the domain. Follows:
    /// https://cloud.google.com/apis/design/design_patterns#request_validation
    #[prost(bool, tag = "6")]
    pub validate_only: bool,
}
/// Request for the `ListRegistrations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegistrationsRequest {
    /// Required. The project and location from which to list `Registration`s, specified in
    /// the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// When set to the `next_page_token` from a prior response, provides the next
    /// page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter expression to restrict the `Registration`s returned.
    ///
    /// The expression must specify the field name, a comparison operator, and the
    /// value that you want to use for filtering. The value must be a string, a
    /// number, a boolean, or an enum value. The comparison operator should be one
    /// of =, !=, >, <, >=, <=, or : for prefix or wildcard matches.
    ///
    /// For example, to filter to a specific domain name, use an expression like
    /// `domainName="example.com"`. You can also check for the existence of a
    /// field; for example, to find domains using custom DNS settings, use an
    /// expression like `dnsSettings.customDns:*`.
    ///
    /// You can also create compound filters by combining expressions with the
    /// `AND` and `OR` operators. For example, to find domains that are suspended
    /// or have specific issues flagged, use an expression like
    /// `(state=SUSPENDED) OR (issue:*)`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListRegistrations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegistrationsResponse {
    /// A list of `Registration`s.
    #[prost(message, repeated, tag = "1")]
    pub registrations: ::prost::alloc::vec::Vec<Registration>,
    /// When present, there are more results to retrieve. Set `page_token` to this
    /// value on a subsequent call to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `GetRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegistrationRequest {
    /// Required. The name of the `Registration` to get, in the format
    /// `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `UpdateRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistrationRequest {
    /// Fields of the `Registration` to update.
    #[prost(message, optional, tag = "1")]
    pub registration: ::core::option::Option<Registration>,
    /// Required. The field mask describing which fields to update as a comma-separated list.
    /// For example, if only the labels are being updated, the `update_mask` would
    /// be `"labels"`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `ConfigureManagementSettings` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureManagementSettingsRequest {
    /// Required. The name of the `Registration` whose management settings are being updated,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub registration: ::prost::alloc::string::String,
    /// Fields of the `ManagementSettings` to update.
    #[prost(message, optional, tag = "2")]
    pub management_settings: ::core::option::Option<ManagementSettings>,
    /// Required. The field mask describing which fields to update as a comma-separated list.
    /// For example, if only the transfer lock is being updated, the `update_mask`
    /// would be `"transfer_lock_state"`.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `ConfigureDnsSettings` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureDnsSettingsRequest {
    /// Required. The name of the `Registration` whose DNS settings are being updated,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub registration: ::prost::alloc::string::String,
    /// Fields of the `DnsSettings` to update.
    #[prost(message, optional, tag = "2")]
    pub dns_settings: ::core::option::Option<DnsSettings>,
    /// Required. The field mask describing which fields to update as a comma-separated list.
    /// For example, if only the name servers are being updated for an existing
    /// Custom DNS configuration, the `update_mask` would be
    /// `"custom_dns.name_servers"`.
    ///
    /// When changing the DNS provider from one type to another, pass the new
    /// provider's field name as part of the field mask. For example, when changing
    /// from a Google Domains DNS configuration to a Custom DNS configuration, the
    /// `update_mask` would be `"custom_dns"`. //
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Validate the request without actually updating the DNS settings.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request for the `ConfigureContactSettings` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureContactSettingsRequest {
    /// Required. The name of the `Registration` whose contact settings are being updated,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub registration: ::prost::alloc::string::String,
    /// Fields of the `ContactSettings` to update.
    #[prost(message, optional, tag = "2")]
    pub contact_settings: ::core::option::Option<ContactSettings>,
    /// Required. The field mask describing which fields to update as a comma-separated list.
    /// For example, if only the registrant contact is being updated, the
    /// `update_mask` would be `"registrant_contact"`.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The list of contact notices that the caller acknowledges. The notices
    /// required here depend on the values specified in `contact_settings`.
    #[prost(enumeration = "ContactNotice", repeated, tag = "4")]
    pub contact_notices: ::prost::alloc::vec::Vec<i32>,
    /// Validate the request without actually updating the contact settings.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request for the `ExportRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportRegistrationRequest {
    /// Required. The name of the `Registration` to export,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `DeleteRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegistrationRequest {
    /// Required. The name of the `Registration` to delete,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `RetrieveAuthorizationCode` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveAuthorizationCodeRequest {
    /// Required. The name of the `Registration` whose authorization code is being retrieved,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub registration: ::prost::alloc::string::String,
}
/// Request for the `ResetAuthorizationCode` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetAuthorizationCodeRequest {
    /// Required. The name of the `Registration` whose authorization code is being reset,
    /// in the format `projects/*/locations/*/registrations/*`.
    #[prost(string, tag = "1")]
    pub registration: ::prost::alloc::string::String,
}
/// Parameters required to register a new domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterParameters {
    /// The domain name. Unicode domain names are expressed in Punycode format.
    #[prost(string, tag = "1")]
    pub domain_name: ::prost::alloc::string::String,
    /// Indicates whether the domain is available for registration. This value is
    /// accurate when obtained by calling `RetrieveRegisterParameters`, but is
    /// approximate when obtained by calling `SearchDomains`.
    #[prost(enumeration = "register_parameters::Availability", tag = "2")]
    pub availability: i32,
    /// Contact privacy options that the domain supports.
    #[prost(enumeration = "ContactPrivacy", repeated, tag = "3")]
    pub supported_privacy: ::prost::alloc::vec::Vec<i32>,
    /// Notices about special properties of the domain.
    #[prost(enumeration = "DomainNotice", repeated, tag = "4")]
    pub domain_notices: ::prost::alloc::vec::Vec<i32>,
    /// Price to register or renew the domain for one year.
    #[prost(message, optional, tag = "5")]
    pub yearly_price: ::core::option::Option<super::super::super::r#type::Money>,
}
/// Nested message and enum types in `RegisterParameters`.
pub mod register_parameters {
    /// Possible availability states of a domain name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Availability {
        /// The availability is unspecified.
        Unspecified = 0,
        /// The domain is available for registration.
        Available = 1,
        /// The domain is not available for registration. Generally this means it is
        /// already registered to another party.
        Unavailable = 2,
        /// The domain is not currently supported by Cloud Domains, but may
        /// be available elsewhere.
        Unsupported = 3,
        /// Cloud Domains is unable to determine domain availability, generally
        /// due to system maintenance at the domain name registry.
        Unknown = 4,
    }
}
/// Defines an authorization code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationCode {
    /// The Authorization Code in ASCII. It can be used to transfer the domain
    /// to or from another registrar.
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation. Output only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// API version used to start the operation.
    #[prost(string, tag = "6")]
    pub api_version: ::prost::alloc::string::String,
}
/// Defines a set of possible contact privacy settings for a `Registration`.
///
/// [ICANN](https://icann.org/) maintains the WHOIS database, a publicly
/// accessible mapping from domain name to contact information, and requires that
/// each domain name have an entry. Choose from these options to control how much
/// information in your `ContactSettings` is published.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContactPrivacy {
    /// The contact privacy settings are undefined.
    Unspecified = 0,
    /// All the data from `ContactSettings` is publicly available. When setting
    /// this option, you must also provide a
    /// `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the
    /// request.
    PublicContactData = 1,
    /// None of the data from `ContactSettings` is publicly available. Instead,
    /// proxy contact data is published for your domain. Email sent to the proxy
    /// email address is forwarded to the registrant's email address. Cloud Domains
    /// provides this privacy proxy service at no additional cost.
    PrivateContactData = 2,
    /// Some data from `ContactSettings` is publicly available. The actual
    /// information redacted depends on the domain. For details, see [the
    /// registration privacy
    /// article](https://support.google.com/domains/answer/3251242).
    RedactedContactData = 3,
}
/// Notices about special properties of certain domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainNotice {
    /// The notice is undefined.
    Unspecified = 0,
    /// Indicates that the domain is preloaded on the HTTP Strict Transport
    /// Security list in browsers. Serving a website on such domain requires
    /// an SSL certificate. For details, see
    /// [how to get an SSL
    /// certificate](https://support.google.com/domains/answer/7638036).
    HstsPreloaded = 1,
}
/// Notices related to contact information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContactNotice {
    /// The notice is undefined.
    Unspecified = 0,
    /// Required when setting the `privacy` field of `ContactSettings` to
    /// `PUBLIC_CONTACT_DATA`, which exposes contact data publicly.
    PublicContactDataAcknowledgement = 1,
}
/// Possible states of a `Registration`'s transfer lock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransferLockState {
    /// The state is unspecified.
    Unspecified = 0,
    /// The domain is unlocked and can be transferred to another registrar.
    Unlocked = 1,
    /// The domain is locked and cannot be transferred to another registrar.
    Locked = 2,
}
#[doc = r" Generated client implementations."]
pub mod domains_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Cloud Domains API enables management and configuration of domain names."]
    #[derive(Debug, Clone)]
    pub struct DomainsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DomainsClient<T>
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
        ) -> DomainsClient<InterceptedService<T, F>>
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
            DomainsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Searches for available domain names similar to the provided query."]
        #[doc = ""]
        #[doc = " Availability results from this method are approximate; call"]
        #[doc = " `RetrieveRegisterParameters` on a domain before registering to confirm"]
        #[doc = " availability."]
        pub async fn search_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchDomainsRequest>,
        ) -> Result<tonic::Response<super::SearchDomainsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/SearchDomains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets parameters needed to register a new domain name, including price and"]
        #[doc = " up-to-date availability. Use the returned values to call `RegisterDomain`."]
        pub async fn retrieve_register_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveRegisterParametersRequest>,
        ) -> Result<tonic::Response<super::RetrieveRegisterParametersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/RetrieveRegisterParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers a new domain name and creates a corresponding `Registration`"]
        #[doc = " resource."]
        #[doc = ""]
        #[doc = " Call `RetrieveRegisterParameters` first to check availability of the domain"]
        #[doc = " name and determine parameters like price that are needed to build a call to"]
        #[doc = " this method."]
        #[doc = ""]
        #[doc = " A successful call creates a `Registration` resource in state"]
        #[doc = " `REGISTRATION_PENDING`, which resolves to `ACTIVE` within 1-2"]
        #[doc = " minutes, indicating that the domain was successfully registered. If the"]
        #[doc = " resource ends up in state `REGISTRATION_FAILED`, it indicates that the"]
        #[doc = " domain was not registered successfully, and you can safely delete the"]
        #[doc = " resource and retry registration."]
        pub async fn register_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterDomainRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/RegisterDomain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the `Registration` resources in a project."]
        pub async fn list_registrations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRegistrationsRequest>,
        ) -> Result<tonic::Response<super::ListRegistrationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/ListRegistrations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a `Registration` resource."]
        pub async fn get_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRegistrationRequest>,
        ) -> Result<tonic::Response<super::Registration>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/GetRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates select fields of a `Registration` resource, notably `labels`. To"]
        #[doc = " update other fields, use the appropriate custom update method:"]
        #[doc = ""]
        #[doc = " * To update management settings, see `ConfigureManagementSettings`"]
        #[doc = " * To update DNS configuration, see `ConfigureDnsSettings`"]
        #[doc = " * To update contact information, see `ConfigureContactSettings`"]
        pub async fn update_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRegistrationRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/UpdateRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a `Registration`'s management settings."]
        pub async fn configure_management_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureManagementSettingsRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/ConfigureManagementSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a `Registration`'s DNS settings."]
        pub async fn configure_dns_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureDnsSettingsRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/ConfigureDnsSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a `Registration`'s contact settings. Some changes require"]
        #[doc = " confirmation by the domain's registrant contact ."]
        pub async fn configure_contact_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureContactSettingsRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/ConfigureContactSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports a `Registration` that you no longer want to use with"]
        #[doc = " Cloud Domains. You can continue to use the domain in"]
        #[doc = " [Google Domains](https://domains.google/) until it expires."]
        #[doc = ""]
        #[doc = " If the export is successful:"]
        #[doc = ""]
        #[doc = " * The resource's `state` becomes `EXPORTED`, meaning that it is no longer"]
        #[doc = " managed by Cloud Domains"]
        #[doc = " * Because individual users can own domains in Google Domains, the calling"]
        #[doc = " user becomes the domain's sole owner. Permissions for the domain are"]
        #[doc = " subsequently managed in Google Domains."]
        #[doc = " * Without further action, the domain does not renew automatically."]
        #[doc = " The new owner can set up billing in Google Domains to renew the domain"]
        #[doc = " if needed."]
        pub async fn export_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportRegistrationRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/ExportRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a `Registration` resource."]
        #[doc = ""]
        #[doc = " This method only works on resources in one of the following states:"]
        #[doc = ""]
        #[doc = " * `state` is `EXPORTED` with `expire_time` in the past"]
        #[doc = " * `state` is `REGISTRATION_FAILED`"]
        pub async fn delete_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegistrationRequest>,
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
                "/google.cloud.domains.v1beta1.Domains/DeleteRegistration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the authorization code of the `Registration` for the purpose of"]
        #[doc = " transferring the domain to another registrar."]
        #[doc = ""]
        #[doc = " You can call this method only after 60 days have elapsed since the initial"]
        #[doc = " domain registration."]
        pub async fn retrieve_authorization_code(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveAuthorizationCodeRequest>,
        ) -> Result<tonic::Response<super::AuthorizationCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/RetrieveAuthorizationCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets the authorization code of the `Registration` to a new random string."]
        #[doc = ""]
        #[doc = " You can call this method only after 60 days have elapsed since the initial"]
        #[doc = " domain registration."]
        pub async fn reset_authorization_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetAuthorizationCodeRequest>,
        ) -> Result<tonic::Response<super::AuthorizationCode>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.domains.v1beta1.Domains/ResetAuthorizationCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
