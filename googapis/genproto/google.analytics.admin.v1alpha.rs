/// A resource message representing a Google Analytics account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Output only. Resource name of this account.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when this account was originally created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when account payload fields were last updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Human-readable display name for this account.
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// Country of business. Must be a Unicode CLDR region code.
    #[prost(string, tag="5")]
    pub region_code: ::prost::alloc::string::String,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted
    /// accounts are excluded from List results unless specifically requested.
    #[prost(bool, tag="6")]
    pub deleted: bool,
}
/// A resource message representing a Google Analytics GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Output only. Resource name of this property.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the entity was originally created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when entity payload fields were last updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this property.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Industry associated with this property
    /// Example: AUTOMOTIVE, FOOD_AND_DRINK
    #[prost(enumeration="IndustryCategory", tag="6")]
    pub industry_category: i32,
    /// Required. Reporting Time Zone, used as the day boundary for reports, regardless of
    /// where the data originates. If the time zone honors DST, Analytics will
    /// automatically adjust for the changes.
    ///
    /// NOTE: Changing the time zone only affects data going forward, and is not
    /// applied retroactively.
    ///
    /// Format: <https://www.iana.org/time-zones>
    /// Example: "America/Los_Angeles"
    #[prost(string, tag="7")]
    pub time_zone: ::prost::alloc::string::String,
    /// The currency type used in reports involving monetary values.
    ///
    ///
    /// Format: <https://en.wikipedia.org/wiki/ISO_4217>
    /// Examples: "USD", "EUR", "JPY"
    #[prost(string, tag="8")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The Google Analytics service level that applies to this property.
    #[prost(enumeration="ServiceLevel", tag="10")]
    pub service_level: i32,
    /// Output only. If set, the time at which this property was trashed. If not set, then this
    /// property is not currently in the trash can.
    #[prost(message, optional, tag="11")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, the time at which this trashed property will be permanently
    /// deleted. If not set, then this property is not currently in the trash can
    /// and is not slated to be deleted.
    #[prost(message, optional, tag="12")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A resource message representing a Google Analytics Android app stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidAppDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/1000/androidAppDataStreams/2000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding Android app in Firebase, if any.
    /// This ID can change if the Android app is deleted and recreated.
    #[prost(string, tag="2")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The package name for the app being measured.
    /// Example: "com.example.myandroidapp"
    #[prost(string, tag="5")]
    pub package_name: ::prost::alloc::string::String,
    /// Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag="6")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a Google Analytics IOS app stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosAppDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/1000/iosAppDataStreams/2000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding iOS app in Firebase, if any.
    /// This ID can change if the iOS app is deleted and recreated.
    #[prost(string, tag="2")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Immutable. The Apple App Store Bundle ID for the app
    /// Example: "com.example.myiosapp"
    #[prost(string, tag="5")]
    pub bundle_id: ::prost::alloc::string::String,
    /// Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag="6")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a Google Analytics web stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/1000/webDataStreams/2000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Analytics "Measurement ID", without the "G-" prefix.
    /// Example: "G-1A2BCD345E" would just be "1A2BCD345E"
    #[prost(string, tag="2")]
    pub measurement_id: ::prost::alloc::string::String,
    /// Output only. ID of the corresponding web app in Firebase, if any.
    /// This ID can change if the web app is deleted and recreated.
    #[prost(string, tag="3")]
    pub firebase_app_id: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Domain name of the web app being measured, or empty.
    /// Example: "<http://www.google.com",> "<https://www.google.com">
    #[prost(string, tag="6")]
    pub default_uri: ::prost::alloc::string::String,
    /// Required. Human-readable display name for the Data Stream.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag="7")]
    pub display_name: ::prost::alloc::string::String,
}
/// A resource message representing a user's permissions on an Account or
/// Property resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLink {
    /// Output only. Example format: properties/1234/userLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Email address of the user to link
    #[prost(string, tag="2")]
    pub email_address: ::prost::alloc::string::String,
    /// Roles directly assigned to this user for this account or property.
    ///
    /// Valid values:
    /// predefinedRoles/read
    /// predefinedRoles/collaborate
    /// predefinedRoles/edit
    /// predefinedRoles/manage-users
    ///
    /// Excludes roles that are inherited from a higher-level entity, group,
    /// or organization admin role.
    ///
    /// A UserLink that is updated to have an empty list of direct_roles will be
    /// deleted.
    #[prost(string, repeated, tag="3")]
    pub direct_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Read-only resource used to summarize a principal's effective roles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLink {
    /// Example format: properties/1234/userLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Email address of the linked user
    #[prost(string, tag="2")]
    pub email_address: ::prost::alloc::string::String,
    /// Roles directly assigned to this user for this entity.
    ///
    /// Format: predefinedRoles/read
    ///
    /// Excludes roles that are inherited from an account (if this is for a
    /// property), group, or organization admin role.
    #[prost(string, repeated, tag="3")]
    pub direct_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Union of all permissions a user has at this account or property (includes
    /// direct permissions, group-inherited permissions, etc.).
    ///
    /// Format: predefinedRoles/read
    #[prost(string, repeated, tag="4")]
    pub effective_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Singleton resource under a WebDataStream, configuring measurement of
/// additional site interactions and content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedMeasurementSettings {
    /// Output only. Resource name of this Data Stream.
    /// Format:
    /// properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings
    /// Example: "properties/1000/webDataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates whether Enhanced Measurement Settings will be used to
    /// automatically measure interactions and content on this web stream.
    ///
    /// Changing this value does not affect the settings themselves, but determines
    /// whether they are respected.
    #[prost(bool, tag="2")]
    pub stream_enabled: bool,
    /// Output only. If enabled, capture a page view event each time a page loads or the
    /// website changes the browser history state.
    #[prost(bool, tag="3")]
    pub page_views_enabled: bool,
    /// If enabled, capture scroll events each time a visitor gets to the bottom of
    /// a page.
    #[prost(bool, tag="4")]
    pub scrolls_enabled: bool,
    /// If enabled, capture an outbound click event each time a visitor clicks a
    /// link that leads them away from your domain.
    #[prost(bool, tag="5")]
    pub outbound_clicks_enabled: bool,
    /// If enabled, capture a view search results event each time a visitor
    /// performs a search on your site (based on a query parameter).
    #[prost(bool, tag="7")]
    pub site_search_enabled: bool,
    /// If enabled, capture video play, progress, and complete events as visitors
    /// view embedded videos on your site.
    #[prost(bool, tag="9")]
    pub video_engagement_enabled: bool,
    /// If enabled, capture a file download event each time a link is clicked with
    /// a common document, compressed file, application, video, or audio extension.
    #[prost(bool, tag="10")]
    pub file_downloads_enabled: bool,
    /// Output only. If enabled, capture a page view event each time a page loads.
    #[prost(bool, tag="12")]
    pub page_loads_enabled: bool,
    /// If enabled, capture a page view event each time the website changes the
    /// browser history state.
    #[prost(bool, tag="13")]
    pub page_changes_enabled: bool,
    /// Required. URL query parameters to interpret as site search parameters.
    /// Max length is 1024 characters. Must not be empty.
    #[prost(string, tag="16")]
    pub search_query_parameter: ::prost::alloc::string::String,
    /// Additional URL query parameters.
    /// Max length is 1024 characters.
    #[prost(string, tag="17")]
    pub uri_query_parameter: ::prost::alloc::string::String,
}
/// A link between an GA4 property and a Firebase project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirebaseLink {
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink, you may
    /// provide this resource name using either a project number or project ID.
    /// Once this resource has been created, returned FirebaseLinks will always
    /// have a project_name that contains a project number.
    ///
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag="2")]
    pub project: ::prost::alloc::string::String,
    /// Output only. Time when this FirebaseLink was originally created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Read-only resource with the tag for sending data from a website to a
/// WebDataStream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSiteTag {
    /// Output only. Resource name for this GlobalSiteTag resource.
    /// Format: properties/{propertyId}/globalSiteTag
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. JavaScript code snippet to be pasted as the first item into the head tag of
    /// every webpage to measure.
    #[prost(string, tag="2")]
    pub snippet: ::prost::alloc::string::String,
}
/// A link between an GA4 property and a Google Ads account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLink {
    /// Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId}
    ///
    /// Note: googleAdsLinkId is not the Google Ads customer ID.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Google Ads customer ID.
    #[prost(string, tag="3")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[prost(bool, tag="4")]
    pub can_manage_clients: bool,
    /// Enable personalized advertising features with this integration.
    /// Automatically publish my Google Analytics audience lists and Google
    /// Analytics remarketing events/parameters to the linked Google Ads account.
    /// If this field is not set on create/update, it will be defaulted to true.
    #[prost(message, optional, tag="5")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Output only. Time when this link was originally created.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this link was last updated.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user that created the link.
    /// An empty string will be returned if the email address can't be retrieved.
    #[prost(string, tag="9")]
    pub creator_email_address: ::prost::alloc::string::String,
}
/// A resource message representing data sharing settings of a Google Analytics
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSharingSettings {
    /// Output only. Resource name.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Allows Google support to access the data in order to help troubleshoot
    /// issues.
    #[prost(bool, tag="2")]
    pub sharing_with_google_support_enabled: bool,
    /// Allows Google sales teams that are assigned to the customer to access the
    /// data in order to suggest configuration changes to improve results.
    /// Sales team restrictions still apply when enabled.
    #[prost(bool, tag="3")]
    pub sharing_with_google_assigned_sales_enabled: bool,
    /// Allows any of Google sales to access the data in order to suggest
    /// configuration changes to improve results.
    #[prost(bool, tag="4")]
    pub sharing_with_google_any_sales_enabled: bool,
    /// Allows Google to use the data to improve other Google products or services.
    #[prost(bool, tag="5")]
    pub sharing_with_google_products_enabled: bool,
    /// Allows Google to share the data anonymously in aggregate form with others.
    #[prost(bool, tag="6")]
    pub sharing_with_others_enabled: bool,
}
/// A virtual resource representing an overview of an account and
/// all its child GA4 properties.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSummary {
    /// Resource name for this account summary.
    /// Format: accountSummaries/{account_id}
    /// Example: "accountSummaries/1000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Resource name of account referred to by this account summary
    /// Format: accounts/{account_id}
    /// Example: "accounts/1000"
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    /// Display name for the account referred to in this account summary.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// List of summaries for child accounts of this account.
    #[prost(message, repeated, tag="4")]
    pub property_summaries: ::prost::alloc::vec::Vec<PropertySummary>,
}
/// A virtual resource representing metadata for an GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertySummary {
    /// Resource name of property referred to by this property summary
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag="1")]
    pub property: ::prost::alloc::string::String,
    /// Display name for the property referred to in this account summary.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
/// A secret value used for sending hits to Measurement Protocol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementProtocolSecret {
    /// Output only. Resource name of this secret. This secret may be a child of any type of
    /// stream.
    /// Format:
    /// properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this secret.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The measurement protocol secret value. Pass this value to the api_secret
    /// field of the Measurement Protocol API when sending hits to this
    /// secret's parent property.
    #[prost(string, tag="3")]
    pub secret_value: ::prost::alloc::string::String,
}
/// A set of changes within a Google Analytics account or its child properties
/// that resulted from the same cause. Common causes would be updates made in the
/// Google Analytics UI, changes from customer support, or automatic Google
/// Analytics system changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryEvent {
    /// ID of this change history event. This ID is unique across Google Analytics.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Time when change was made.
    #[prost(message, optional, tag="2")]
    pub change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of actor that made this change.
    #[prost(enumeration="ActorType", tag="3")]
    pub actor_type: i32,
    /// Email address of the Google account that made the change. This will be a
    /// valid email address if the actor field is set to USER, and empty otherwise.
    /// Google accounts that have been deleted will cause an error.
    #[prost(string, tag="4")]
    pub user_actor_email: ::prost::alloc::string::String,
    /// If true, then the list of changes returned was filtered, and does not
    /// represent all changes that occurred in this event.
    #[prost(bool, tag="5")]
    pub changes_filtered: bool,
    /// A list of changes made in this change history event that fit the filters
    /// specified in SearchChangeHistoryEventsRequest.
    #[prost(message, repeated, tag="6")]
    pub changes: ::prost::alloc::vec::Vec<ChangeHistoryChange>,
}
/// A description of a change to a single Google Analytics resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryChange {
    /// Resource name of the resource whose changes are described by this entry.
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    /// The type of action that changed this resource.
    #[prost(enumeration="ActionType", tag="2")]
    pub action: i32,
    /// Resource contents from before the change was made. If this resource was
    /// created in this change, this field will be missing.
    #[prost(message, optional, tag="3")]
    pub resource_before_change: ::core::option::Option<change_history_change::ChangeHistoryResource>,
    /// Resource contents from after the change was made. If this resource was
    /// deleted in this change, this field will be missing.
    #[prost(message, optional, tag="4")]
    pub resource_after_change: ::core::option::Option<change_history_change::ChangeHistoryResource>,
}
/// Nested message and enum types in `ChangeHistoryChange`.
pub mod change_history_change {
    /// A snapshot of a resource as before or after the result of a change in
    /// change history.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeHistoryResource {
        #[prost(oneof="change_history_resource::Resource", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15")]
        pub resource: ::core::option::Option<change_history_resource::Resource>,
    }
    /// Nested message and enum types in `ChangeHistoryResource`.
    pub mod change_history_resource {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Resource {
            /// A snapshot of an Account resource in change history.
            #[prost(message, tag="1")]
            Account(super::super::Account),
            /// A snapshot of a Property resource in change history.
            #[prost(message, tag="2")]
            Property(super::super::Property),
            /// A snapshot of a WebDataStream resource in change history.
            #[prost(message, tag="3")]
            WebDataStream(super::super::WebDataStream),
            /// A snapshot of an AndroidAppDataStream resource in change history.
            #[prost(message, tag="4")]
            AndroidAppDataStream(super::super::AndroidAppDataStream),
            /// A snapshot of an IosAppDataStream resource in change history.
            #[prost(message, tag="5")]
            IosAppDataStream(super::super::IosAppDataStream),
            /// A snapshot of a FirebaseLink resource in change history.
            #[prost(message, tag="6")]
            FirebaseLink(super::super::FirebaseLink),
            /// A snapshot of a GoogleAdsLink resource in change history.
            #[prost(message, tag="7")]
            GoogleAdsLink(super::super::GoogleAdsLink),
            /// A snapshot of a GoogleSignalsSettings resource in change history.
            #[prost(message, tag="8")]
            GoogleSignalsSettings(super::super::GoogleSignalsSettings),
            /// A snapshot of a DisplayVideo360AdvertiserLink resource in change
            /// history.
            #[prost(message, tag="9")]
            DisplayVideo360AdvertiserLink(super::super::DisplayVideo360AdvertiserLink),
            /// A snapshot of a DisplayVideo360AdvertiserLinkProposal resource in
            /// change history.
            #[prost(message, tag="10")]
            DisplayVideo360AdvertiserLinkProposal(super::super::DisplayVideo360AdvertiserLinkProposal),
            /// A snapshot of a ConversionEvent resource in change history.
            #[prost(message, tag="11")]
            ConversionEvent(super::super::ConversionEvent),
            /// A snapshot of a MeasurementProtocolSecret resource in change history.
            #[prost(message, tag="12")]
            MeasurementProtocolSecret(super::super::MeasurementProtocolSecret),
            /// A snapshot of a CustomDimension resource in change history.
            #[prost(message, tag="13")]
            CustomDimension(super::super::CustomDimension),
            /// A snapshot of a CustomMetric resource in change history.
            #[prost(message, tag="14")]
            CustomMetric(super::super::CustomMetric),
            /// A snapshot of a data retention settings resource in change history.
            #[prost(message, tag="15")]
            DataRetentionSettings(super::super::DataRetentionSettings),
        }
    }
}
/// A link between a GA4 property and a Display & Video 360 advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayVideo360AdvertiserLink {
    /// Output only. The resource name for this DisplayVideo360AdvertiserLink resource.
    /// Format: properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId}
    ///
    /// Note: linkId is not the Display & Video 360 Advertiser ID
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[prost(string, tag="2")]
    pub advertiser_id: ::prost::alloc::string::String,
    /// Output only. The display name of the Display & Video 360 Advertiser.
    #[prost(string, tag="3")]
    pub advertiser_display_name: ::prost::alloc::string::String,
    /// Enables personalized advertising features with this integration.
    /// If this field is not set on create/update, it will be defaulted to true.
    #[prost(message, optional, tag="4")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of campaign data from Display & Video 360 into the GA4
    /// property. After link creation, this can only be updated from the Display &
    /// Video 360 product.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag="5")]
    pub campaign_data_sharing_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360 into the GA4
    /// property. This can only be enabled if campaign_data_import_enabled is
    /// enabled. After link creation, this can only be updated from the Display &
    /// Video 360 product.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag="6")]
    pub cost_data_sharing_enabled: ::core::option::Option<bool>,
}
/// A proposal for a link between an GA4 property and a Display & Video 360
/// advertiser.
///
/// A proposal is converted to a DisplayVideo360AdvertiserLink once approved.
/// Google Analytics admins approve inbound proposals while Display & Video 360
/// admins approve outbound proposals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayVideo360AdvertiserLinkProposal {
    /// Output only. The resource name for this DisplayVideo360AdvertiserLinkProposal resource.
    /// Format:
    /// properties/{propertyId}/displayVideo360AdvertiserLinkProposals/{proposalId}
    ///
    /// Note: proposalId is not the Display & Video 360 Advertiser ID
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[prost(string, tag="2")]
    pub advertiser_id: ::prost::alloc::string::String,
    /// Output only. The status information for this link proposal.
    #[prost(message, optional, tag="3")]
    pub link_proposal_status_details: ::core::option::Option<LinkProposalStatusDetails>,
    /// Output only. The display name of the Display & Video Advertiser.
    /// Only populated for proposals that originated from Display & Video 360.
    #[prost(string, tag="4")]
    pub advertiser_display_name: ::prost::alloc::string::String,
    /// Input only. On a proposal being sent to Display & Video 360, this field must be set to
    /// the email address of an admin on the target advertiser. This is used to
    /// verify that the Google Analytics admin is aware of at least one admin on
    /// the Display & Video 360 Advertiser. This does not restrict approval of the
    /// proposal to a single user. Any admin on the Display & Video 360 Advertiser
    /// may approve the proposal.
    #[prost(string, tag="5")]
    pub validation_email: ::prost::alloc::string::String,
    /// Immutable. Enables personalized advertising features with this integration.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag="6")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of campaign data from Display & Video 360.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag="7")]
    pub campaign_data_sharing_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360.
    /// This can only be enabled if campaign_data_import_enabled is enabled.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag="8")]
    pub cost_data_sharing_enabled: ::core::option::Option<bool>,
}
/// Status information for a link proposal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkProposalStatusDetails {
    /// Output only. The source of this proposal.
    #[prost(enumeration="LinkProposalInitiatingProduct", tag="1")]
    pub link_proposal_initiating_product: i32,
    /// Output only. The email address of the user that proposed this linkage.
    #[prost(string, tag="2")]
    pub requestor_email: ::prost::alloc::string::String,
    /// Output only. The state of this proposal.
    #[prost(enumeration="LinkProposalState", tag="3")]
    pub link_proposal_state: i32,
}
/// A conversion event in a Google Analytics property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionEvent {
    /// Output only. Resource name of this conversion event.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The event name for this conversion event.
    /// Examples: 'click', 'purchase'
    #[prost(string, tag="2")]
    pub event_name: ::prost::alloc::string::String,
    /// Output only. Time when this conversion event was created in the property.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, this event can currently be deleted via DeleteConversionEvent.
    #[prost(bool, tag="4")]
    pub deletable: bool,
    /// Output only. If set to true, this conversion event refers to a custom event.  If set to
    /// false, this conversion event refers to a default event in GA. Default
    /// events typically have special meaning in GA. Default events are usually
    /// created for you by the GA system, but in some cases can be created by
    /// property admins. Custom events count towards the maximum number of
    /// custom conversion events that may be created per property.
    #[prost(bool, tag="5")]
    pub custom: bool,
}
/// Settings values for Google Signals.  This is a singleton resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleSignalsSettings {
    /// Output only. Resource name of this setting.
    /// Format: properties/{property_id}/googleSignalsSettings
    /// Example: "properties/1000/googleSignalsSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Status of this setting.
    #[prost(enumeration="GoogleSignalsState", tag="3")]
    pub state: i32,
    /// Output only. Terms of Service acceptance.
    #[prost(enumeration="GoogleSignalsConsent", tag="4")]
    pub consent: i32,
}
/// A definition for a CustomDimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomDimension {
    /// Output only. Resource name for this CustomDimension resource.
    /// Format: properties/{property}/customDimensions/{customDimension}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging parameter name for this custom dimension.
    ///
    /// If this is a user-scoped dimension, then this is the user property name.
    /// If this is an event-scoped dimension, then this is the event parameter
    /// name.
    ///
    /// May only contain alphanumeric and underscore characters, starting with a
    /// letter. Max length of 24 characters for user-scoped dimensions, 40
    /// characters for event-scoped dimensions.
    #[prost(string, tag="2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom dimension as shown in the Analytics UI.
    /// Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension. Max length of 150 characters.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. The scope of this dimension.
    #[prost(enumeration="custom_dimension::DimensionScope", tag="5")]
    pub scope: i32,
    /// Optional. If set to true, sets this dimension as NPA and excludes it from ads
    /// personalization.
    ///
    /// This is currently only supported by user-scoped custom dimensions.
    #[prost(bool, tag="6")]
    pub disallow_ads_personalization: bool,
}
/// Nested message and enum types in `CustomDimension`.
pub mod custom_dimension {
    /// Valid values for the scope of this dimension.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DimensionScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Dimension scoped to an event.
        Event = 1,
        /// Dimension scoped to a user.
        User = 2,
    }
}
/// A definition for a custom metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetric {
    /// Output only. Resource name for this CustomMetric resource.
    /// Format: properties/{property}/customMetrics/{customMetric}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging name for this custom metric.
    ///
    /// If this is an event-scoped metric, then this is the event parameter
    /// name.
    ///
    /// May only contain alphanumeric and underscore charactes, starting with a
    /// letter. Max length of 40 characters for event-scoped metrics.
    #[prost(string, tag="2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom metric as shown in the Analytics UI.
    /// Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension.
    /// Max length of 150 characters.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type for the custom metric's value.
    #[prost(enumeration="custom_metric::MeasurementUnit", tag="5")]
    pub measurement_unit: i32,
    /// Required. Immutable. The scope of this custom metric.
    #[prost(enumeration="custom_metric::MetricScope", tag="6")]
    pub scope: i32,
}
/// Nested message and enum types in `CustomMetric`.
pub mod custom_metric {
    /// Possible types of representing the custom metric's value.
    ///
    /// Currency representation may change in the future, requiring a breaking API
    /// change.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MeasurementUnit {
        /// MeasurementUnit unspecified or missing.
        Unspecified = 0,
        /// This metric uses default units.
        Standard = 1,
        /// This metric measures a currency.
        Currency = 2,
        /// This metric measures feet.
        Feet = 3,
        /// This metric measures meters.
        Meters = 4,
        /// This metric measures kilometers.
        Kilometers = 5,
        /// This metric measures miles.
        Miles = 6,
        /// This metric measures milliseconds.
        Milliseconds = 7,
        /// This metric measures seconds.
        Seconds = 8,
        /// This metric measures minutes.
        Minutes = 9,
        /// This metric measures hours.
        Hours = 10,
    }
    /// The scope of this metric.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MetricScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Metric scoped to an event.
        Event = 1,
    }
}
/// Settings values for data retention. This is a singleton resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRetentionSettings {
    /// Output only. Resource name for this DataRetentionSetting resource.
    /// Format: properties/{property}/dataRetentionSettings
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The length of time that event-level data is retained.
    #[prost(enumeration="data_retention_settings::RetentionDuration", tag="2")]
    pub event_data_retention: i32,
    /// If true, reset the retention period for the user identifier with every
    /// event from that user.
    #[prost(bool, tag="3")]
    pub reset_user_data_on_new_activity: bool,
}
/// Nested message and enum types in `DataRetentionSettings`.
pub mod data_retention_settings {
    /// Valid values for the data retention duration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RetentionDuration {
        /// Data retention time duration is not specified.
        Unspecified = 0,
        /// The data retention time duration is 2 months.
        TwoMonths = 1,
        /// The data retention time duration is 14 months.
        FourteenMonths = 3,
        /// The data retention time duration is 26 months.
        /// Available to 360 properties only.
        TwentySixMonths = 4,
        /// The data retention time duration is 38 months.
        /// Available to 360 properties only.
        ThirtyEightMonths = 5,
        /// The data retention time duration is 50 months.
        /// Available to 360 properties only.
        FiftyMonths = 6,
    }
}
/// The category selected for this property, used for industry benchmarking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndustryCategory {
    /// Industry category unspecified
    Unspecified = 0,
    /// Automotive
    Automotive = 1,
    /// Business and industrial markets
    BusinessAndIndustrialMarkets = 2,
    /// Finance
    Finance = 3,
    /// Healthcare
    Healthcare = 4,
    /// Technology
    Technology = 5,
    /// Travel
    Travel = 6,
    /// Other
    Other = 7,
    /// Arts and entertainment
    ArtsAndEntertainment = 8,
    /// Beauty and fitness
    BeautyAndFitness = 9,
    /// Books and literature
    BooksAndLiterature = 10,
    /// Food and drink
    FoodAndDrink = 11,
    /// Games
    Games = 12,
    /// Hobbies and leisure
    HobbiesAndLeisure = 13,
    /// Home and garden
    HomeAndGarden = 14,
    /// Internet and telecom
    InternetAndTelecom = 15,
    /// Law and government
    LawAndGovernment = 16,
    /// News
    News = 17,
    /// Online communities
    OnlineCommunities = 18,
    /// People and society
    PeopleAndSociety = 19,
    /// Pets and animals
    PetsAndAnimals = 20,
    /// Real estate
    RealEstate = 21,
    /// Reference
    Reference = 22,
    /// Science
    Science = 23,
    /// Sports
    Sports = 24,
    /// Jobs and education
    JobsAndEducation = 25,
    /// Shopping
    Shopping = 26,
}
/// Various levels of service for Google Analytics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceLevel {
    /// Service level not specified or invalid.
    Unspecified = 0,
    /// The standard version of Google Analytics.
    GoogleAnalyticsStandard = 1,
    /// The paid, premium version of Google Analytics.
    GoogleAnalytics360 = 2,
}
/// Different kinds of actors that can make changes to Google Analytics
/// resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActorType {
    /// Unknown or unspecified actor type.
    Unspecified = 0,
    /// Changes made by the user specified in actor_email.
    User = 1,
    /// Changes made by the Google Analytics system.
    System = 2,
    /// Changes made by Google Analytics support team staff.
    Support = 3,
}
/// Types of actions that may change a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    /// Action type unknown or not specified.
    Unspecified = 0,
    /// Resource was created in this change.
    Created = 1,
    /// Resource was updated in this change.
    Updated = 2,
    /// Resource was deleted in this change.
    Deleted = 3,
}
/// Types of resources whose changes may be returned from change history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeHistoryResourceType {
    /// Resource type unknown or not specified.
    Unspecified = 0,
    /// Account resource
    Account = 1,
    /// Property resource
    Property = 2,
    /// WebDataStream resource
    WebDataStream = 3,
    /// AndroidAppDataStream resource
    AndroidAppDataStream = 4,
    /// IosAppDataStream resource
    IosAppDataStream = 5,
    /// FirebaseLink resource
    FirebaseLink = 6,
    /// GoogleAdsLink resource
    GoogleAdsLink = 7,
    /// GoogleSignalsSettings resource
    GoogleSignalsSettings = 8,
    /// ConversionEvent resource
    ConversionEvent = 9,
    /// MeasurementProtocolSecret resource
    MeasurementProtocolSecret = 10,
    /// CustomDimension resource
    CustomDimension = 11,
    /// CustomMetric resource
    CustomMetric = 12,
    /// DataRetentionSettings resource
    DataRetentionSettings = 13,
}
/// Status of the Google Signals settings (i.e., whether this feature has been
/// enabled for the property).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoogleSignalsState {
    /// Google Signals status defaults to GOOGLE_SIGNALS_STATE_UNSPECIFIED to
    /// represent that the user has not made an explicit choice.
    Unspecified = 0,
    /// Google Signals is enabled.
    GoogleSignalsEnabled = 1,
    /// Google Signals is disabled.
    GoogleSignalsDisabled = 2,
}
/// Consent field of the Google Signals settings (i.e., whether the user has
/// consented to the Google Signals terms of service.)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoogleSignalsConsent {
    /// Google Signals consent value defaults to
    /// GOOGLE_SIGNALS_CONSENT_UNSPECIFIED.  This will be treated as
    /// GOOGLE_SIGNALS_CONSENT_NOT_CONSENTED.
    Unspecified = 0,
    /// Terms of service have been accepted
    Consented = 2,
    /// Terms of service have not been accepted
    NotConsented = 1,
}
/// An indication of which product the user initiated a link proposal from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkProposalInitiatingProduct {
    /// Unspecified product.
    Unspecified = 0,
    /// This proposal was created by a user from Google Analytics.
    GoogleAnalytics = 1,
    /// This proposal was created by a user from a linked product (not Google
    /// Analytics).
    LinkedProduct = 2,
}
/// The state of a link proposal resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkProposalState {
    /// Unspecified state
    Unspecified = 0,
    /// This proposal is awaiting review from a Google Analytics user. This
    /// proposal will automatically expire after some time.
    AwaitingReviewFromGoogleAnalytics = 1,
    /// This proposal is awaiting review from a user of a linked product. This
    /// proposal will automatically expire after some time.
    AwaitingReviewFromLinkedProduct = 2,
    /// This proposal has been withdrawn by an admin on the initiating product.
    /// This proposal will be automatically deleted after some time.
    Withdrawn = 3,
    /// This proposal has been declined by an admin on the receiving product. This
    /// proposal will be automatically deleted after some time.
    Declined = 4,
    /// This proposal expired due to lack of response from an admin on the
    /// receiving product. This proposal will be automatically deleted after some
    /// time.
    Expired = 5,
    /// This proposal has become obsolete because a link was directly created to
    /// the same external product resource that this proposal specifies. This
    /// proposal will be automatically deleted after some time.
    Obsolete = 6,
}
/// Request message for GetAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the account to lookup.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsRequest {
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccounts` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccounts` must
    /// match the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Accounts in the
    /// results. Accounts can be inspected to determine whether they are deleted or
    /// not.
    #[prost(bool, tag="3")]
    pub show_deleted: bool,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// Required. The name of the Account to soft-delete.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// Required. The account to update.
    /// The account's `name` field is used to identify the account.
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<Account>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketRequest {
    /// The account to create.
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<Account>,
    /// Redirect URI where the user will be sent after accepting Terms of Service.
    /// Must be configured in Developers Console as a Redirect URI
    #[prost(string, tag="2")]
    pub redirect_uri: ::prost::alloc::string::String,
}
/// Response message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[prost(string, tag="1")]
    pub account_ticket_id: ::prost::alloc::string::String,
}
/// Request message for GetProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyRequest {
    /// Required. The name of the property to lookup.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesRequest {
    /// Required. An expression for filtering the results of the request.
    /// Fields eligible for filtering are:
    /// `parent:`(The resource name of the parent account) or
    /// `firebase_project:`(The id or number of the linked firebase project).
    /// Some examples of filters:
    ///
    /// ```
    /// | Filter                      | Description                               |
    /// |-----------------------------|-------------------------------------------|
    /// | parent:accounts/123         | The account with account id: 123.         |
    /// | firebase_project:project-id | The firebase project with id: project-id. |
    /// | firebase_project:123        | The firebase project with number: 123.    |
    /// ```
    #[prost(string, tag="1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListProperties` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Properties in the
    /// results. Properties can be inspected to determine whether they are deleted
    /// or not.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePropertyRequest {
    /// Required. The property to update.
    /// The property's `name` field is used to identify the property to be
    /// updated.
    #[prost(message, optional, tag="1")]
    pub property: ::core::option::Option<Property>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePropertyRequest {
    /// Required. The property to create.
    /// Note: the supplied property must specify its parent.
    #[prost(message, optional, tag="1")]
    pub property: ::core::option::Option<Property>,
}
/// Request message for DeleteProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePropertyRequest {
    /// Required. The name of the Property to soft-delete.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all provided values for the 'names' field must match
    /// this field.
    /// Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the user links to retrieve.
    /// A maximum of 1000 user links can be retrieved in a batch.
    /// Format: accounts/{accountId}/userLinks/{userLinkId}
    #[prost(string, repeated, tag="2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for BatchGetUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetUserLinksResponse {
    /// The requested user links.
    #[prost(message, repeated, tag="1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of user links to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 200 user links will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListUserLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListUserLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserLinksResponse {
    /// List of UserLinks. These will be ordered stably, but in an arbitrary order.
    #[prost(message, repeated, tag="1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of user links to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 1000 user links will be returned.
    /// The maximum value is 5000; values above 5000 will be coerced to 5000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `AuditUserLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `AuditUserLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for AuditUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditUserLinksResponse {
    /// List of AuditUserLinks. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag="1")]
    pub user_links: ::prost::alloc::vec::Vec<AuditUserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateUserLink RPC.
///
/// Users can have multiple email addresses associated with their Google
/// account, and one of these email addresses is the "primary" email address.
/// Any of the email addresses associated with a Google account may be used
/// for a new UserLink, but the returned UserLink will always contain the
/// "primary" email address. As a result, the input and output email address
/// for this request may differ.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserLinkRequest {
    /// Required. Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If set, then email the new user notifying them that they've been granted
    /// permissions to the resource.
    #[prost(bool, tag="2")]
    pub notify_new_user: bool,
    /// Required. The user link to create.
    #[prost(message, optional, tag="3")]
    pub user_link: ::core::option::Option<UserLink>,
}
/// Request message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksRequest {
    /// Required. The account or property that all user links in the request are for.
    /// This field is required. The parent field in the CreateUserLinkRequest
    /// messages must either be empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If set, then email the new users notifying them that they've been granted
    /// permissions to the resource. Regardless of whether this is set or not,
    /// notify_new_user field inside each individual request is ignored.
    #[prost(bool, tag="2")]
    pub notify_new_users: bool,
    /// Required. The requests specifying the user links to create.
    /// A maximum of 1000 user links can be created in a batch.
    #[prost(message, repeated, tag="3")]
    pub requests: ::prost::alloc::vec::Vec<CreateUserLinkRequest>,
}
/// Response message for BatchCreateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateUserLinksResponse {
    /// The user links created.
    #[prost(message, repeated, tag="1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for UpdateUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserLinkRequest {
    /// Required. The user link to update.
    #[prost(message, optional, tag="1")]
    pub user_link: ::core::option::Option<UserLink>,
}
/// Request message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent field in the UpdateUserLinkRequest messages must either be
    /// empty or match this field.
    /// Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag="2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateUserLinkRequest>,
}
/// Response message for BatchUpdateUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateUserLinksResponse {
    /// The user links updated.
    #[prost(message, repeated, tag="1")]
    pub user_links: ::prost::alloc::vec::Vec<UserLink>,
}
/// Request message for DeleteUserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserLinkRequest {
    /// Required. Example format: accounts/1234/userLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchDeleteUserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteUserLinksRequest {
    /// Required. The account or property that all user links in the request are
    /// for. The parent of all values for user link names to delete must match this
    /// field.
    /// Example format: accounts/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the user links to update.
    /// A maximum of 1000 user links can be updated in a batch.
    #[prost(message, repeated, tag="2")]
    pub requests: ::prost::alloc::vec::Vec<DeleteUserLinkRequest>,
}
/// Request message for GetWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWebDataStreamRequest {
    /// Required. The name of the web data stream to lookup.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebDataStreamRequest {
    /// Required. The name of the web data stream to delete.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}
    /// Example: "properties/123/webDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWebDataStreamRequest {
    /// Required. The web stream to update.
    /// The `name` field is used to identify the web stream to be updated.
    #[prost(message, optional, tag="1")]
    pub web_data_stream: ::core::option::Option<WebDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateWebDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebDataStreamRequest {
    /// Required. The web stream to create.
    #[prost(message, optional, tag="1")]
    pub web_data_stream: ::core::option::Option<WebDataStream>,
    /// Required. The parent resource where this web data stream will be created.
    /// Format: properties/123
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of web streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWebDataStreams` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListWebDataStreams` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListWebDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub web_data_streams: ::prost::alloc::vec::Vec<WebDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to lookup.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIosAppDataStreamRequest {
    /// Required. The name of the iOS app data stream to delete.
    /// Format: properties/{property_id}/iosAppDataStreams/{stream_id}
    /// Example: "properties/123/iosAppDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateIosAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIosAppDataStreamRequest {
    /// Required. The iOS app stream to update.
    /// The `name` field is used to identify the iOS app stream to be updated.
    #[prost(message, optional, tag="1")]
    pub ios_app_data_stream: ::core::option::Option<IosAppDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to list results of app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListIosAppDataStreams`
    /// call. Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListIosAppDataStreams`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListIosAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIosAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub ios_app_data_streams: ::prost::alloc::vec::Vec<IosAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to lookup.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAndroidAppDataStreamRequest {
    /// Required. The name of the android app data stream to delete.
    /// Format: properties/{property_id}/androidAppDataStreams/{stream_id}
    /// Example: "properties/123/androidAppDataStreams/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAndroidAppDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAndroidAppDataStreamRequest {
    /// Required. The android app stream to update.
    /// The `name` field is used to identify the android app stream to be updated.
    #[prost(message, optional, tag="1")]
    pub android_app_data_stream: ::core::option::Option<AndroidAppDataStream>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListAndroidAppDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsRequest {
    /// Required. The name of the parent property.
    /// For example, to limit results to app streams under the property with Id
    /// 123: "properties/123"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    ///
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous call. Provide this to
    /// retrieve the subsequent page.
    /// When paginating, all other parameters provided to
    /// `ListAndroidAppDataStreams` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for ListAndroidDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndroidAppDataStreamsResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub android_app_data_streams: ::prost::alloc::vec::Vec<AndroidAppDataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnhancedMeasurementSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings
    /// Example: "properties/1000/webDataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnhancedMeasurementSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag="1")]
    pub enhanced_measurement_settings: ::core::option::Option<EnhancedMeasurementSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Firebase link to create.
    #[prost(message, optional, tag="2")]
    pub firebase_link: ::core::option::Option<FirebaseLink>,
}
/// Request message for DeleteFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id}
    /// Example: properties/1234/firebaseLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFirebaseLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[prost(message, repeated, tag="1")]
    pub firebase_links: ::prost::alloc::vec::Vec<FirebaseLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    /// Currently, Google Analytics supports only one FirebaseLink per property,
    /// so this will never be populated.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetGlobalSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGlobalSiteTagRequest {
    /// Required. The name of the site tag to lookup.
    /// Note that site tags are singletons and do not have unique IDs.
    /// Format: properties/{property_id}/webDataStreams/{stream_id}/globalSiteTag
    /// Example: "properties/123/webDataStreams/456/globalSiteTag"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The GoogleAdsLink to create.
    #[prost(message, optional, tag="2")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
}
/// Request message for UpdateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleAdsLinkRequest {
    /// The GoogleAdsLink to update
    #[prost(message, optional, tag="1")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteGoogleAdsLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234/googleAdsLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListGoogleAdsLinks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListGoogleAdsLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[prost(message, repeated, tag="1")]
    pub google_ads_links: ::prost::alloc::vec::Vec<GoogleAdsLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataSharingSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSharingSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesRequest {
    /// The maximum number of AccountSummary resources to return. The service may
    /// return fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccountSummaries` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccountSummaries`
    /// must match the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesResponse {
    /// Account summaries of all accounts the caller has access to.
    #[prost(message, repeated, tag="1")]
    pub account_summaries: ::prost::alloc::vec::Vec<AccountSummary>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for SearchChangeHistoryEvents RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsRequest {
    /// Required. The account resource for which to return change history resources.
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Resource name for a child property. If set, only return changes
    /// made to this property or its child resources.
    #[prost(string, tag="2")]
    pub property: ::prost::alloc::string::String,
    /// Optional. If set, only return changes if they are for a resource that matches at
    /// least one of these types.
    #[prost(enumeration="ChangeHistoryResourceType", repeated, packed="false", tag="3")]
    pub resource_type: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes that match one or more of these types of
    /// actions.
    #[prost(enumeration="ActionType", repeated, packed="false", tag="4")]
    pub action: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes if they are made by a user in this list.
    #[prost(string, repeated, tag="5")]
    pub actor_email: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If set, only return changes made after this time (inclusive).
    #[prost(message, optional, tag="6")]
    pub earliest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. If set, only return changes made before this time (inclusive).
    #[prost(message, optional, tag="7")]
    pub latest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The maximum number of ChangeHistoryEvent items to return.
    /// The service may return fewer than this value, even if there are additional
    /// pages. If unspecified, at most 50 items will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="8")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `SearchChangeHistoryEvents` call.
    /// Provide this to retrieve the subsequent page. When paginating, all other
    /// parameters provided to `SearchChangeHistoryEvents` must match the call that
    /// provided the page token.
    #[prost(string, tag="9")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for SearchAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag="1")]
    pub change_history_events: ::prost::alloc::vec::Vec<ChangeHistoryEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetMeasurementProtocolSecret RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeasurementProtocolSecretRequest {
    /// Required. The name of the measurement protocol secret to lookup.
    /// Format:
    /// properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    /// Note: Any type of stream (WebDataStream, IosAppDataStream,
    /// AndroidAppDataStream) may be a parent.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMeasurementProtocolSecretRequest {
    /// Required. The parent resource where this secret will be created.
    /// Any type of stream (WebDataStream, IosAppDataStream, AndroidAppDataStream)
    /// may be a parent.
    /// Format: properties/{property}/webDataStreams/{webDataStream}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The measurement protocol secret to create.
    #[prost(message, optional, tag="2")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
}
/// Request message for DeleteMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMeasurementProtocolSecretRequest {
    /// Required. The name of the MeasurementProtocolSecret to delete.
    /// Format:
    /// properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    /// Note: Any type of stream (WebDataStream, IosAppDataStream,
    /// AndroidAppDataStream) may be a parent.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMeasurementProtocolSecretRequest {
    /// Required. The measurement protocol secret to update.
    #[prost(message, optional, tag="1")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsRequest {
    /// Required. The resource name of the parent stream.
    /// Any type of stream (WebDataStream, IosAppDataStream, AndroidAppDataStream)
    /// may be a parent.
    /// Format:
    /// properties/{property}/webDataStreams/{webDataStream}/measurementProtocolSecrets
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 10 resources will be returned.
    /// The maximum value is 10. Higher values will be coerced to the maximum.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMeasurementProtocolSecrets`
    /// call. Provide this to retrieve the subsequent page. When paginating, all
    /// other parameters provided to `ListMeasurementProtocolSecrets` must match
    /// the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsResponse {
    /// A list of secrets for the parent stream specified in the request.
    #[prost(message, repeated, tag="1")]
    pub measurement_protocol_secrets: ::prost::alloc::vec::Vec<MeasurementProtocolSecret>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetGoogleSignalsSettings RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleSignalsSettingsRequest {
    /// Required. The name of the google signals settings to retrieve.
    /// Format: properties/{property}/googleSignalsSettings
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateGoogleSignalsSettings RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleSignalsSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag="1")]
    pub google_signals_settings: ::core::option::Option<GoogleSignalsSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversionEventRequest {
    /// Required. The conversion event to create.
    #[prost(message, optional, tag="1")]
    pub conversion_event: ::core::option::Option<ConversionEvent>,
    /// Required. The resource name of the parent property where this conversion event will
    /// be created. Format: properties/123
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for GetConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversionEventRequest {
    /// Required. The resource name of the conversion event to retrieve.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversionEventRequest {
    /// Required. The resource name of the conversion event to delete.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListConversionEvents RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsRequest {
    /// Required. The resource name of the parent property.
    /// Example: 'properties/123'
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConversionEvents` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListConversionEvents`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListConversionEvents RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsResponse {
    /// The requested conversion events
    #[prost(message, repeated, tag="1")]
    pub conversion_events: ::prost::alloc::vec::Vec<ConversionEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDisplayVideo360AdvertiserLinkRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLink to get.
    /// Example format: properties/1234/displayVideo360AdvertiserLink/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDisplayVideo360AdvertiserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDisplayVideo360AdvertiserLinks`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListDisplayVideo360AdvertiserLinks` must match the call that provided the
    /// page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDisplayVideo360AdvertiserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinksResponse {
    /// List of DisplayVideo360AdvertiserLinks.
    #[prost(message, repeated, tag="1")]
    pub display_video_360_advertiser_links: ::prost::alloc::vec::Vec<DisplayVideo360AdvertiserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisplayVideo360AdvertiserLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DisplayVideo360AdvertiserLink to create.
    #[prost(message, optional, tag="2")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
}
/// Request message for DeleteDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDisplayVideo360AdvertiserLinkRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLink to delete.
    /// Example format: properties/1234/displayVideo360AdvertiserLinks/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDisplayVideo360AdvertiserLinkRequest {
    /// The DisplayVideo360AdvertiserLink to update
    #[prost(message, optional, tag="1")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
    /// Required. The list of fields to be updated. Omitted fields will not be updated.
    /// To replace the entire entity, use one path with the string "*" to match
    /// all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to get.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDisplayVideo360AdvertiserLinkProposals RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinkProposalsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// `ListDisplayVideo360AdvertiserLinkProposals` call. Provide this to retrieve
    /// the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListDisplayVideo360AdvertiserLinkProposals` must match the call that
    /// provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDisplayVideo360AdvertiserLinkProposals RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinkProposalsResponse {
    /// List of DisplayVideo360AdvertiserLinkProposals.
    #[prost(message, repeated, tag="1")]
    pub display_video_360_advertiser_link_proposals: ::prost::alloc::vec::Vec<DisplayVideo360AdvertiserLinkProposal>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DisplayVideo360AdvertiserLinkProposal to create.
    #[prost(message, optional, tag="2")]
    pub display_video_360_advertiser_link_proposal: ::core::option::Option<DisplayVideo360AdvertiserLinkProposal>,
}
/// Request message for DeleteDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to delete.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to approve.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDisplayVideo360AdvertiserLinkProposalResponse {
    /// The DisplayVideo360AdvertiserLink created as a result of approving the
    /// proposal.
    #[prost(message, optional, tag="1")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
}
/// Request message for CancelDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to cancel.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomDimensionRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomDimension to create.
    #[prost(message, optional, tag="2")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
}
/// Request message for UpdateCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomDimensionRequest {
    /// The CustomDimension to update
    #[prost(message, optional, tag="1")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
    /// Required. The list of fields to be updated. Omitted fields will not be updated.
    /// To replace the entire entity, use one path with the string "*" to match
    /// all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomDimensions RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomDimensions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomDimensions`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomDimensions RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsResponse {
    /// List of CustomDimensions.
    #[prost(message, repeated, tag="1")]
    pub custom_dimensions: ::prost::alloc::vec::Vec<CustomDimension>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomDimensionRequest {
    /// Required. The name of the CustomDimension to archive.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDimensionRequest {
    /// Required. The name of the CustomDimension to get.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomMetricRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomMetric to create.
    #[prost(message, optional, tag="2")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
}
/// Request message for UpdateCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomMetricRequest {
    /// The CustomMetric to update
    #[prost(message, optional, tag="1")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
    /// Required. The list of fields to be updated. Omitted fields will not be updated.
    /// To replace the entire entity, use one path with the string "*" to match
    /// all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomMetrics` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomMetrics` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsResponse {
    /// List of CustomMetrics.
    #[prost(message, repeated, tag="1")]
    pub custom_metrics: ::prost::alloc::vec::Vec<CustomMetric>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomMetricRequest {
    /// Required. The name of the CustomMetric to archive.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomMetricRequest {
    /// Required. The name of the CustomMetric to get.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetDataRetentionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRetentionSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property}/dataRetentionSettings
    /// Example: "properties/1000/dataRetentionSettings"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataRetentionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRetentionSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag="1")]
    pub data_retention_settings: ::core::option::Option<DataRetentionSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake case
    /// (e.g., "field_to_update"). Omitted fields will not be updated. To replace
    /// the entire entity, use one path with the string "*" to match all fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod analytics_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Service Interface for the Analytics Admin API (GA4).
    #[derive(Debug, Clone)]
    pub struct AnalyticsAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AnalyticsAdminServiceClient<T>
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
        ) -> AnalyticsAdminServiceClient<InterceptedService<T, F>>
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
            AnalyticsAdminServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lookup for a single Account.
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all accounts accessible by the caller.
        ///
        /// Note that these accounts might not currently have GA4 properties.
        /// Soft-deleted (ie: "trashed") accounts are excluded by default.
        /// Returns an empty list if no relevant accounts are found.
        pub async fn list_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks target Account as soft-deleted (ie: "trashed") and returns it.
        ///
        /// This API does not have a method to restore soft-deleted accounts.
        /// However, they can be restored using the Trash Can UI.
        ///
        /// If the accounts are not restored before the expiration time, the account
        /// and all child resources (eg: Properties, GoogleAdsLinks, Streams,
        /// UserLinks) will be permanently purged.
        /// https://support.google.com/analytics/answer/6154772
        ///
        /// Returns an error if the target is not found.
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an account.
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Requests a ticket for creating an account.
        pub async fn provision_account_ticket(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionAccountTicketRequest>,
        ) -> Result<
            tonic::Response<super::ProvisionAccountTicketResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ProvisionAccountTicket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns summaries of all accounts accessible by the caller.
        pub async fn list_account_summaries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountSummariesRequest>,
        ) -> Result<
            tonic::Response<super::ListAccountSummariesResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccountSummaries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single "GA4" Property.
        pub async fn get_property(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns child Properties under the specified parent Account.
        ///
        /// Only "GA4" properties will be returned.
        /// Properties will be excluded if the caller does not have access.
        /// Soft-deleted (ie: "trashed") properties are excluded by default.
        /// Returns an empty list if no relevant properties are found.
        pub async fn list_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPropertiesRequest>,
        ) -> Result<tonic::Response<super::ListPropertiesResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an "GA4" property with the specified location and attributes.
        pub async fn create_property(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks target Property as soft-deleted (ie: "trashed") and returns it.
        ///
        /// This API does not have a method to restore soft-deleted properties.
        /// However, they can be restored using the Trash Can UI.
        ///
        /// If the properties are not restored before the expiration time, the Property
        /// and all child resources (eg: GoogleAdsLinks, Streams, UserLinks)
        /// will be permanently purged.
        /// https://support.google.com/analytics/answer/6154772
        ///
        /// Returns an error if the target is not found, or is not an GA4 Property.
        pub async fn delete_property(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a property.
        pub async fn update_property(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information about a user's link to an account or property.
        pub async fn get_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information about multiple users' links to an account or property.
        pub async fn batch_get_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetUserLinksRequest>,
        ) -> Result<tonic::Response<super::BatchGetUserLinksResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchGetUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all user links on an account or property.
        pub async fn list_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserLinksRequest>,
        ) -> Result<tonic::Response<super::ListUserLinksResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all user links on an account or property, including implicit ones
        /// that come from effective permissions granted by groups or organization
        /// admin roles.
        ///
        /// If a returned user link does not have direct permissions, they cannot
        /// be removed from the account or property directly with the DeleteUserLink
        /// command. They have to be removed from the group/etc that gives them
        /// permissions, which is currently only usable/discoverable in the GA or GMP
        /// UIs.
        pub async fn audit_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::AuditUserLinksRequest>,
        ) -> Result<tonic::Response<super::AuditUserLinksResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/AuditUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a user link on an account or property.
        ///
        /// If the user with the specified email already has permissions on the
        /// account or property, then the user's existing permissions will be unioned
        /// with the permissions specified in the new UserLink.
        pub async fn create_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates information about multiple users' links to an account or property.
        ///
        /// This method is transactional. If any UserLink cannot be created, none of
        /// the UserLinks will be created.
        pub async fn batch_create_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateUserLinksRequest>,
        ) -> Result<
            tonic::Response<super::BatchCreateUserLinksResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchCreateUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a user link on an account or property.
        pub async fn update_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserLinkRequest>,
        ) -> Result<tonic::Response<super::UserLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates information about multiple users' links to an account or property.
        pub async fn batch_update_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateUserLinksRequest>,
        ) -> Result<
            tonic::Response<super::BatchUpdateUserLinksResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchUpdateUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a user link on an account or property.
        pub async fn delete_user_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserLinkRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteUserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes information about multiple users' links to an account or property.
        pub async fn batch_delete_user_links(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteUserLinksRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchDeleteUserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single WebDataStream
        pub async fn get_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a web stream on a property.
        pub async fn delete_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWebDataStreamRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a web stream on a property.
        pub async fn update_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a web stream with the specified location and attributes.
        pub async fn create_web_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebDataStreamRequest>,
        ) -> Result<tonic::Response<super::WebDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateWebDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns child web data streams under the specified parent property.
        ///
        /// Web data streams will be excluded if the caller does not have access.
        /// Returns an empty list if no relevant web data streams are found.
        pub async fn list_web_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListWebDataStreamsResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListWebDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single IosAppDataStream
        pub async fn get_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an iOS app stream on a property.
        pub async fn delete_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIosAppDataStreamRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an iOS app stream on a property.
        pub async fn update_ios_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIosAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::IosAppDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateIosAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns child iOS app data streams under the specified parent property.
        ///
        /// iOS app data streams will be excluded if the caller does not have access.
        /// Returns an empty list if no relevant iOS app data streams are found.
        pub async fn list_ios_app_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIosAppDataStreamsRequest>,
        ) -> Result<
            tonic::Response<super::ListIosAppDataStreamsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListIosAppDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single AndroidAppDataStream
        pub async fn get_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an android app stream on a property.
        pub async fn delete_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAndroidAppDataStreamRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an android app stream on a property.
        pub async fn update_android_app_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAndroidAppDataStreamRequest>,
        ) -> Result<tonic::Response<super::AndroidAppDataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAndroidAppDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns child android app streams under the specified parent property.
        ///
        /// Android app streams will be excluded if the caller does not have access.
        /// Returns an empty list if no relevant android app streams are found.
        pub async fn list_android_app_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAndroidAppDataStreamsRequest>,
        ) -> Result<
            tonic::Response<super::ListAndroidAppDataStreamsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAndroidAppDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the singleton enhanced measurement settings for this web stream.
        /// Note that the stream must enable enhanced measurement for these settings to
        /// take effect.
        pub async fn get_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetEnhancedMeasurementSettingsRequest,
            >,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetEnhancedMeasurementSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the singleton enhanced measurement settings for this web stream.
        /// Note that the stream must enable enhanced measurement for these settings to
        /// take effect.
        pub async fn update_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateEnhancedMeasurementSettingsRequest,
            >,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateEnhancedMeasurementSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a FirebaseLink.
        ///
        /// Properties can have at most one FirebaseLink.
        pub async fn create_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a FirebaseLink on a property
        pub async fn delete_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFirebaseLinkRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists FirebaseLinks on a property.
        /// Properties can have at most one FirebaseLink.
        pub async fn list_firebase_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFirebaseLinksRequest>,
        ) -> Result<tonic::Response<super::ListFirebaseLinksResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListFirebaseLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the Site Tag for the specified web stream.
        /// Site Tags are immutable singletons.
        pub async fn get_global_site_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGlobalSiteTagRequest>,
        ) -> Result<tonic::Response<super::GlobalSiteTag>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetGlobalSiteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a GoogleAdsLink.
        pub async fn create_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a GoogleAdsLink on a property
        pub async fn update_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a GoogleAdsLink on a property
        pub async fn delete_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGoogleAdsLinkRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists GoogleAdsLinks on a property.
        pub async fn list_google_ads_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGoogleAdsLinksRequest>,
        ) -> Result<tonic::Response<super::ListGoogleAdsLinksResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListGoogleAdsLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get data sharing settings on an account.
        /// Data sharing settings are singletons.
        pub async fn get_data_sharing_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSharingSettingsRequest>,
        ) -> Result<tonic::Response<super::DataSharingSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataSharingSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single "GA4" MeasurementProtocolSecret.
        pub async fn get_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeasurementProtocolSecretRequest>,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetMeasurementProtocolSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns child MeasurementProtocolSecrets under the specified parent
        /// Property.
        pub async fn list_measurement_protocol_secrets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListMeasurementProtocolSecretsRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListMeasurementProtocolSecretsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListMeasurementProtocolSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a measurement protocol secret.
        pub async fn create_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateMeasurementProtocolSecretRequest,
            >,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateMeasurementProtocolSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes target MeasurementProtocolSecret.
        pub async fn delete_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteMeasurementProtocolSecretRequest,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteMeasurementProtocolSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a measurement protocol secret.
        pub async fn update_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateMeasurementProtocolSecretRequest,
            >,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateMeasurementProtocolSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches through all changes to an account or its children given the
        /// specified set of filters.
        pub async fn search_change_history_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchChangeHistoryEventsRequest>,
        ) -> Result<
            tonic::Response<super::SearchChangeHistoryEventsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/SearchChangeHistoryEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for Google Signals settings for a property.
        pub async fn get_google_signals_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleSignalsSettingsRequest>,
        ) -> Result<tonic::Response<super::GoogleSignalsSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetGoogleSignalsSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates Google Signals settings for a property.
        pub async fn update_google_signals_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleSignalsSettingsRequest>,
        ) -> Result<tonic::Response<super::GoogleSignalsSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateGoogleSignalsSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a conversion event with the specified attributes.
        pub async fn create_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversionEventRequest>,
        ) -> Result<tonic::Response<super::ConversionEvent>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieve a single conversion event.
        pub async fn get_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversionEventRequest>,
        ) -> Result<tonic::Response<super::ConversionEvent>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a conversion event in a property.
        pub async fn delete_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversionEventRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of conversion events in the specified parent property.
        ///
        /// Returns an empty list if no conversion events are found.
        pub async fn list_conversion_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversionEventsRequest>,
        ) -> Result<
            tonic::Response<super::ListConversionEventsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListConversionEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Look up a single DisplayVideo360AdvertiserLink
        pub async fn get_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDisplayVideo360AdvertiserLinkRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLink>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDisplayVideo360AdvertiserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all DisplayVideo360AdvertiserLinks on a property.
        pub async fn list_display_video360_advertiser_links(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDisplayVideo360AdvertiserLinksRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListDisplayVideo360AdvertiserLinksResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListDisplayVideo360AdvertiserLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a DisplayVideo360AdvertiserLink.
        /// This can only be utilized by users who have proper authorization both on
        /// the Google Analytics property and on the Display & Video 360 advertiser.
        /// Users who do not have access to the Display & Video 360 advertiser should
        /// instead seek to create a DisplayVideo360LinkProposal.
        pub async fn create_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateDisplayVideo360AdvertiserLinkRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLink>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateDisplayVideo360AdvertiserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a DisplayVideo360AdvertiserLink on a property.
        pub async fn delete_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteDisplayVideo360AdvertiserLinkRequest,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteDisplayVideo360AdvertiserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a DisplayVideo360AdvertiserLink on a property.
        pub async fn update_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateDisplayVideo360AdvertiserLinkRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLink>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDisplayVideo360AdvertiserLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single DisplayVideo360AdvertiserLinkProposal.
        pub async fn get_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDisplayVideo360AdvertiserLinkProposalRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDisplayVideo360AdvertiserLinkProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DisplayVideo360AdvertiserLinkProposals on a property.
        pub async fn list_display_video360_advertiser_link_proposals(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDisplayVideo360AdvertiserLinkProposalsRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListDisplayVideo360AdvertiserLinkProposalsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListDisplayVideo360AdvertiserLinkProposals",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a DisplayVideo360AdvertiserLinkProposal.
        pub async fn create_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateDisplayVideo360AdvertiserLinkProposalRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateDisplayVideo360AdvertiserLinkProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a DisplayVideo360AdvertiserLinkProposal on a property.
        /// This can only be used on cancelled proposals.
        pub async fn delete_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteDisplayVideo360AdvertiserLinkProposalRequest,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteDisplayVideo360AdvertiserLinkProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Approves a DisplayVideo360AdvertiserLinkProposal.
        /// The DisplayVideo360AdvertiserLinkProposal will be deleted and a new
        /// DisplayVideo360AdvertiserLink will be created.
        pub async fn approve_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ApproveDisplayVideo360AdvertiserLinkProposalRequest,
            >,
        ) -> Result<
            tonic::Response<super::ApproveDisplayVideo360AdvertiserLinkProposalResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ApproveDisplayVideo360AdvertiserLinkProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels a DisplayVideo360AdvertiserLinkProposal.
        /// Cancelling can mean either:
        /// - Declining a proposal initiated from Display & Video 360
        /// - Withdrawing a proposal initiated from Google Analytics
        /// After being cancelled, a proposal will eventually be deleted automatically.
        pub async fn cancel_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CancelDisplayVideo360AdvertiserLinkProposalRequest,
            >,
        ) -> Result<
            tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CancelDisplayVideo360AdvertiserLinkProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a CustomDimension.
        pub async fn create_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a CustomDimension on a property.
        pub async fn update_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CustomDimensions on a property.
        pub async fn list_custom_dimensions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomDimensionsRequest>,
        ) -> Result<
            tonic::Response<super::ListCustomDimensionsResponse>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListCustomDimensions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Archives a CustomDimension on a property.
        pub async fn archive_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomDimensionRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ArchiveCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single CustomDimension.
        pub async fn get_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a CustomMetric.
        pub async fn create_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a CustomMetric on a property.
        pub async fn update_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CustomMetrics on a property.
        pub async fn list_custom_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomMetricsRequest>,
        ) -> Result<tonic::Response<super::ListCustomMetricsResponse>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListCustomMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Archives a CustomMetric on a property.
        pub async fn archive_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomMetricRequest>,
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ArchiveCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup for a single CustomMetric.
        pub async fn get_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the singleton data retention settings for this property.
        pub async fn get_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataRetentionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRetentionSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataRetentionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the singleton data retention settings for this property.
        pub async fn update_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRetentionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRetentionSettings>, tonic::Status> {
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
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDataRetentionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
