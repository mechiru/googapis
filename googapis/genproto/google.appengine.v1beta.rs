/// [Google Cloud Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/)
/// configuration for API handlers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfigHandler {
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "1")]
    pub auth_fail_action: i32,
    /// Level of login required to access this resource. Defaults to
    /// `optional`.
    #[prost(enumeration = "LoginRequirement", tag = "2")]
    pub login: i32,
    /// Path to the script from the application root directory.
    #[prost(string, tag = "3")]
    pub script: std::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "4")]
    pub security_level: i32,
    /// URL to serve the endpoint at.
    #[prost(string, tag = "5")]
    pub url: std::string::String,
}
/// Custom static error page to be served when an error occurs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorHandler {
    /// Error condition this handler applies to.
    #[prost(enumeration = "error_handler::ErrorCode", tag = "1")]
    pub error_code: i32,
    /// Static file content to be served for this error.
    #[prost(string, tag = "2")]
    pub static_file: std::string::String,
    /// MIME type of file. Defaults to `text/html`.
    #[prost(string, tag = "3")]
    pub mime_type: std::string::String,
}
pub mod error_handler {
    /// Error codes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCode {
        /// Not specified. ERROR_CODE_DEFAULT is assumed.
        Unspecified = 0,
        /// Application has exceeded a resource quota.
        OverQuota = 1,
        /// Client blocked by the application's Denial of Service protection
        /// configuration.
        DosApiDenial = 2,
        /// Deadline reached before the application responds.
        Timeout = 3,
    }
}
/// URL pattern and description of how the URL should be handled. App Engine can
/// handle URLs by executing application code or by serving static files
/// uploaded with the version, such as images, CSS, or JavaScript.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlMap {
    /// URL prefix. Uses regular expression syntax, which means regexp
    /// special characters must be escaped, but should not contain groupings.
    /// All URLs that begin with this prefix are handled by this handler, using the
    /// portion of the URL after the prefix as part of the file path.
    #[prost(string, tag = "1")]
    pub url_regex: std::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "5")]
    pub security_level: i32,
    /// Level of login required to access this resource. Not supported for Node.js
    /// in the App Engine standard environment.
    #[prost(enumeration = "LoginRequirement", tag = "6")]
    pub login: i32,
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "7")]
    pub auth_fail_action: i32,
    /// `30x` code to use when performing redirects for the `secure` field.
    /// Defaults to `302`.
    #[prost(enumeration = "url_map::RedirectHttpResponseCode", tag = "8")]
    pub redirect_http_response_code: i32,
    /// Type of handler for this URL pattern.
    #[prost(oneof = "url_map::HandlerType", tags = "2, 3, 4")]
    pub handler_type: ::std::option::Option<url_map::HandlerType>,
}
pub mod url_map {
    /// Redirect codes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RedirectHttpResponseCode {
        /// Not specified. `302` is assumed.
        Unspecified = 0,
        /// `301 Moved Permanently` code.
        RedirectHttpResponseCode301 = 1,
        /// `302 Moved Temporarily` code.
        RedirectHttpResponseCode302 = 2,
        /// `303 See Other` code.
        RedirectHttpResponseCode303 = 3,
        /// `307 Temporary Redirect` code.
        RedirectHttpResponseCode307 = 4,
    }
    /// Type of handler for this URL pattern.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HandlerType {
        /// Returns the contents of a file, such as an image, as the response.
        #[prost(message, tag = "2")]
        StaticFiles(super::StaticFilesHandler),
        /// Executes a script to handle the requests that match this URL
        /// pattern. Only the `auto` value is supported for Node.js in the
        /// App Engine standard environment, for example `"script": "auto"`.
        #[prost(message, tag = "3")]
        Script(super::ScriptHandler),
        /// Uses API Endpoints to handle requests.
        #[prost(message, tag = "4")]
        ApiEndpoint(super::ApiEndpointHandler),
    }
}
/// Files served directly to the user for a given URL, such as images, CSS
/// stylesheets, or JavaScript source files. Static file handlers describe which
/// files in the application directory are static files, and which URLs serve
/// them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticFilesHandler {
    /// Path to the static files matched by the URL pattern, from the
    /// application root directory. The path can refer to text matched in groupings
    /// in the URL pattern.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Regular expression that matches the file paths for all files that should be
    /// referenced by this handler.
    #[prost(string, tag = "2")]
    pub upload_path_regex: std::string::String,
    /// HTTP headers to use for all responses from these URLs.
    #[prost(map = "string, string", tag = "3")]
    pub http_headers: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// MIME type used to serve all files served by this handler.
    ///
    /// Defaults to file-specific MIME types, which are derived from each file's
    /// filename extension.
    #[prost(string, tag = "4")]
    pub mime_type: std::string::String,
    /// Time a static file served by this handler should be cached
    /// by web proxies and browsers.
    #[prost(message, optional, tag = "5")]
    pub expiration: ::std::option::Option<::prost_types::Duration>,
    /// Whether this handler should match the request if the file
    /// referenced by the handler does not exist.
    #[prost(bool, tag = "6")]
    pub require_matching_file: bool,
    /// Whether files should also be uploaded as code data. By default, files
    /// declared in static file handlers are uploaded as static
    /// data and are only served to end users; they cannot be read by the
    /// application. If enabled, uploads are charged against both your code and
    /// static data storage resource quotas.
    #[prost(bool, tag = "7")]
    pub application_readable: bool,
}
/// Executes a script to handle the request that matches the URL pattern.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: std::string::String,
}
/// Uses Google Cloud Endpoints to handle requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEndpointHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: std::string::String,
}
/// Health checking configuration for VM instances. Unhealthy instances
/// are killed and replaced with new instances. Only applicable for
/// instances in App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    /// Whether to explicitly disable health checks for this instance.
    #[prost(bool, tag = "1")]
    pub disable_health_check: bool,
    /// Host header to send when performing an HTTP health check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: std::string::String,
    /// Number of consecutive successful health checks required before receiving
    /// traffic.
    #[prost(uint32, tag = "3")]
    pub healthy_threshold: u32,
    /// Number of consecutive failed health checks required before removing
    /// traffic.
    #[prost(uint32, tag = "4")]
    pub unhealthy_threshold: u32,
    /// Number of consecutive failed health checks required before an instance is
    /// restarted.
    #[prost(uint32, tag = "5")]
    pub restart_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "6")]
    pub check_interval: ::std::option::Option<::prost_types::Duration>,
    /// Time before the health check is considered failed.
    #[prost(message, optional, tag = "7")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
}
/// Readiness checking configuration for VM instances. Unhealthy instances
/// are removed from traffic rotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadinessCheck {
    /// The request path.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Host header to send when performing a HTTP Readiness check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: std::string::String,
    /// Number of consecutive failed checks required before removing
    /// traffic.
    #[prost(uint32, tag = "3")]
    pub failure_threshold: u32,
    /// Number of consecutive successful checks required before receiving
    /// traffic.
    #[prost(uint32, tag = "4")]
    pub success_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "5")]
    pub check_interval: ::std::option::Option<::prost_types::Duration>,
    /// Time before the check is considered failed.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
    /// A maximum time limit on application initialization, measured from moment
    /// the application successfully replies to a healthcheck until it is ready to
    /// serve traffic.
    #[prost(message, optional, tag = "7")]
    pub app_start_timeout: ::std::option::Option<::prost_types::Duration>,
}
/// Health checking configuration for VM instances. Unhealthy instances
/// are killed and replaced with new instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivenessCheck {
    /// The request path.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Host header to send when performing a HTTP Liveness check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: std::string::String,
    /// Number of consecutive failed checks required before considering the
    /// VM unhealthy.
    #[prost(uint32, tag = "3")]
    pub failure_threshold: u32,
    /// Number of consecutive successful checks required before considering
    /// the VM healthy.
    #[prost(uint32, tag = "4")]
    pub success_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "5")]
    pub check_interval: ::std::option::Option<::prost_types::Duration>,
    /// Time before the check is considered failed.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
    /// The initial delay before starting to execute the checks.
    #[prost(message, optional, tag = "7")]
    pub initial_delay: ::std::option::Option<::prost_types::Duration>,
}
/// Third-party Python runtime library that is required by the application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Library {
    /// Name of the library. Example: "django".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Version of the library to select, or "latest".
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
/// Actions to take when the user is not logged in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthFailAction {
    /// Not specified. `AUTH_FAIL_ACTION_REDIRECT` is assumed.
    Unspecified = 0,
    /// Redirects user to "accounts.google.com". The user is redirected back to the
    /// application URL after signing in or creating an account.
    Redirect = 1,
    /// Rejects request with a `401` HTTP status code and an error
    /// message.
    Unauthorized = 2,
}
/// Methods to restrict access to a URL based on login status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginRequirement {
    /// Not specified. `LOGIN_OPTIONAL` is assumed.
    LoginUnspecified = 0,
    /// Does not require that the user is signed in.
    LoginOptional = 1,
    /// If the user is not signed in, the `auth_fail_action` is taken.
    /// In addition, if the user is not an administrator for the
    /// application, they are given an error message regardless of
    /// `auth_fail_action`. If the user is an administrator, the handler
    /// proceeds.
    LoginAdmin = 2,
    /// If the user has signed in, the handler proceeds normally. Otherwise, the
    /// auth_fail_action is taken.
    LoginRequired = 3,
}
/// Methods to enforce security (HTTPS) on a URL.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityLevel {
    /// Not specified.
    SecureUnspecified = 0,
    /// Requests for a URL that match this handler that use HTTPS are automatically
    /// redirected to the HTTP equivalent URL.
    SecureNever = 1,
    /// Both HTTP and HTTPS requests with URLs that match the handler succeed
    /// without redirects. The application can examine the request to determine
    /// which protocol was used and respond accordingly.
    SecureOptional = 2,
    /// Requests for a URL that match this handler that do not use HTTPS are
    /// automatically redirected to the HTTPS URL with the same path. Query
    /// parameters are reserved for the redirect.
    SecureAlways = 3,
}
/// An Application resource contains the top-level configuration of an App
/// Engine application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Full path to the Application resource in the API.
    /// Example: `apps/myapp`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Identifier of the Application resource. This identifier is equivalent
    /// to the project ID of the Google Cloud Platform project where you want to
    /// deploy your application.
    /// Example: `myapp`.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// HTTP path dispatch rules for requests to the application that do not
    /// explicitly target a service or version. Rules are order-dependent.
    /// Up to 20 dispatch rules can be supported.
    #[prost(message, repeated, tag = "3")]
    pub dispatch_rules: ::std::vec::Vec<UrlDispatchRule>,
    /// Google Apps authentication domain that controls which users can access
    /// this application.
    ///
    /// Defaults to open access for any Google Account.
    #[prost(string, tag = "6")]
    pub auth_domain: std::string::String,
    /// Location from which this application runs. Application instances
    /// run out of the data centers in the specified location, which is also where
    /// all of the application's end user content is stored.
    ///
    /// Defaults to `us-central`.
    ///
    /// View the list of
    /// [supported locations](https://cloud.google.com/appengine/docs/locations).
    #[prost(string, tag = "7")]
    pub location_id: std::string::String,
    /// Google Cloud Storage bucket that can be used for storing files
    /// associated with this application. This bucket is associated with the
    /// application and can be used by the gcloud deployment commands.
    ///
    /// @OutputOnly
    #[prost(string, tag = "8")]
    pub code_bucket: std::string::String,
    /// Cookie expiration policy for this application.
    #[prost(message, optional, tag = "9")]
    pub default_cookie_expiration: ::std::option::Option<::prost_types::Duration>,
    /// Serving status of this application.
    #[prost(enumeration = "application::ServingStatus", tag = "10")]
    pub serving_status: i32,
    /// Hostname used to reach this application, as resolved by App Engine.
    ///
    /// @OutputOnly
    #[prost(string, tag = "11")]
    pub default_hostname: std::string::String,
    /// Google Cloud Storage bucket that can be used by this application to store
    /// content.
    ///
    /// @OutputOnly
    #[prost(string, tag = "12")]
    pub default_bucket: std::string::String,
    #[prost(message, optional, tag = "14")]
    pub iap: ::std::option::Option<application::IdentityAwareProxy>,
    /// The Google Container Registry domain used for storing managed build docker
    /// images for this application.
    #[prost(string, tag = "16")]
    pub gcr_domain: std::string::String,
    /// The type of the Cloud Firestore or Cloud Datastore database associated with
    /// this application.
    #[prost(enumeration = "application::DatabaseType", tag = "17")]
    pub database_type: i32,
    /// The feature specific settings to be used in the application.
    #[prost(message, optional, tag = "18")]
    pub feature_settings: ::std::option::Option<application::FeatureSettings>,
}
pub mod application {
    /// Identity-Aware Proxy
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityAwareProxy {
        /// Whether the serving infrastructure will authenticate and
        /// authorize all incoming requests.
        ///
        /// If true, the `oauth2_client_id` and `oauth2_client_secret`
        /// fields must be non-empty.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        /// OAuth2 client ID to use for the authentication flow.
        #[prost(string, tag = "2")]
        pub oauth2_client_id: std::string::String,
        /// OAuth2 client secret to use for the authentication flow.
        ///
        /// For security reasons, this value cannot be retrieved via the API.
        /// Instead, the SHA-256 hash of the value is returned in the
        /// `oauth2_client_secret_sha256` field.
        ///
        /// @InputOnly
        #[prost(string, tag = "3")]
        pub oauth2_client_secret: std::string::String,
        /// Hex-encoded SHA-256 hash of the client secret.
        ///
        /// @OutputOnly
        #[prost(string, tag = "4")]
        pub oauth2_client_secret_sha256: std::string::String,
    }
    /// The feature specific settings to be used in the application. These define
    /// behaviors that are user configurable.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeatureSettings {
        /// Boolean value indicating if split health checks should be used instead
        /// of the legacy health checks. At an app.yaml level, this means defaulting
        /// to 'readiness_check' and 'liveness_check' values instead of
        /// 'health_check' ones. Once the legacy 'health_check' behavior is
        /// deprecated, and this value is always true, this setting can
        /// be removed.
        #[prost(bool, tag = "1")]
        pub split_health_checks: bool,
        /// If true, use [Container-Optimized OS](https://cloud.google.com/container-optimized-os/)
        /// base image for VMs, rather than a base Debian image.
        #[prost(bool, tag = "2")]
        pub use_container_optimized_os: bool,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServingStatus {
        /// Serving status is unspecified.
        Unspecified = 0,
        /// Application is serving.
        Serving = 1,
        /// Application has been disabled by the user.
        UserDisabled = 2,
        /// Application has been disabled by the system.
        SystemDisabled = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DatabaseType {
        /// Database type is unspecified.
        Unspecified = 0,
        /// Cloud Datastore
        CloudDatastore = 1,
        /// Cloud Firestore Native
        CloudFirestore = 2,
        /// Cloud Firestore in Datastore Mode
        CloudDatastoreCompatibility = 3,
    }
}
/// Rules to match an HTTP request and dispatch that request to a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlDispatchRule {
    /// Domain name to match against. The wildcard "`*`" is supported if
    /// specified before a period: "`*.`".
    ///
    /// Defaults to matching all domains: "`*`".
    #[prost(string, tag = "1")]
    pub domain: std::string::String,
    /// Pathname within the host. Must start with a "`/`". A
    /// single "`*`" can be included at the end of the path.
    ///
    /// The sum of the lengths of the domain and path may not
    /// exceed 100 characters.
    #[prost(string, tag = "2")]
    pub path: std::string::String,
    /// Resource ID of a service in this application that should
    /// serve the matched request. The service must already
    /// exist. Example: `default`.
    #[prost(string, tag = "3")]
    pub service: std::string::String,
}
/// An SSL certificate that a user has been authorized to administer. A user
/// is authorized to administer any certificate that applies to one of their
/// authorized domains.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedCertificate {
    /// Full path to the `AuthorizedCertificate` resource in the API. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the certificate. This is a unique value autogenerated
    /// on `AuthorizedCertificate` resource creation. Example: `12345`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// The user-specified display name of the certificate. This is not
    /// guaranteed to be unique. Example: `My Certificate`.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// Topmost applicable domains of this certificate. This certificate
    /// applies to these domains and their subdomains. Example: `example.com`.
    ///
    /// @OutputOnly
    #[prost(string, repeated, tag = "4")]
    pub domain_names: ::std::vec::Vec<std::string::String>,
    /// The time when this certificate expires. To update the renewal time on this
    /// certificate, upload an SSL certificate with a different expiration time
    /// using [`AuthorizedCertificates.UpdateAuthorizedCertificate`]().
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The SSL certificate serving the `AuthorizedCertificate` resource. This
    /// must be obtained independently from a certificate authority.
    #[prost(message, optional, tag = "6")]
    pub certificate_raw_data: ::std::option::Option<CertificateRawData>,
    /// Only applicable if this certificate is managed by App Engine. Managed
    /// certificates are tied to the lifecycle of a `DomainMapping` and cannot be
    /// updated or deleted via the `AuthorizedCertificates` API. If this
    /// certificate is manually administered by the user, this field will be empty.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "7")]
    pub managed_certificate: ::std::option::Option<ManagedCertificate>,
    /// The full paths to user visible Domain Mapping resources that have this
    /// certificate mapped. Example: `apps/myapp/domainMappings/example.com`.
    ///
    /// This may not represent the full list of mapped domain mappings if the user
    /// does not have `VIEWER` permissions on all of the applications that have
    /// this certificate mapped. See `domain_mappings_count` for a complete count.
    ///
    /// Only returned by `GET` or `LIST` requests when specifically requested by
    /// the `view=FULL_CERTIFICATE` option.
    ///
    /// @OutputOnly
    #[prost(string, repeated, tag = "8")]
    pub visible_domain_mappings: ::std::vec::Vec<std::string::String>,
    /// Aggregate count of the domain mappings with this certificate mapped. This
    /// count includes domain mappings on applications for which the user does not
    /// have `VIEWER` permissions.
    ///
    /// Only returned by `GET` or `LIST` requests when specifically requested by
    /// the `view=FULL_CERTIFICATE` option.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "9")]
    pub domain_mappings_count: i32,
}
/// An SSL certificate obtained from a certificate authority.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRawData {
    /// PEM encoded x.509 public key certificate. This field is set once on
    /// certificate creation. Must include the header and footer. Example:
    /// <pre>
    /// -----BEGIN CERTIFICATE-----
    /// <certificate_value>
    /// -----END CERTIFICATE-----
    /// </pre>
    #[prost(string, tag = "1")]
    pub public_certificate: std::string::String,
    /// Unencrypted PEM encoded RSA private key. This field is set once on
    /// certificate creation and then encrypted. The key size must be 2048
    /// bits or fewer. Must include the header and footer. Example:
    /// <pre>
    /// -----BEGIN RSA PRIVATE KEY-----
    /// <unencrypted_key_value>
    /// -----END RSA PRIVATE KEY-----
    /// </pre>
    /// @InputOnly
    #[prost(string, tag = "2")]
    pub private_key: std::string::String,
}
/// A certificate managed by App Engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedCertificate {
    /// Time at which the certificate was last renewed. The renewal process is
    /// fully managed. Certificate renewal will automatically occur before the
    /// certificate expires. Renewal errors can be tracked via `ManagementStatus`.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "1")]
    pub last_renewal_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Status of certificate management. Refers to the most recent certificate
    /// acquisition or renewal attempt.
    ///
    /// @OutputOnly
    #[prost(enumeration = "ManagementStatus", tag = "2")]
    pub status: i32,
}
/// State of certificate management. Refers to the most recent certificate
/// acquisition or renewal attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ManagementStatus {
    Unspecified = 0,
    /// Certificate was successfully obtained and inserted into the serving
    /// system.
    Ok = 1,
    /// Certificate is under active attempts to acquire or renew.
    Pending = 2,
    /// Most recent renewal failed due to an invalid DNS setup and will be
    /// retried. Renewal attempts will continue to fail until the certificate
    /// domain's DNS configuration is fixed. The last successfully provisioned
    /// certificate may still be serving.
    FailedRetryingNotVisible = 4,
    /// All renewal attempts have been exhausted, likely due to an invalid DNS
    /// setup.
    FailedPermanent = 6,
    /// Most recent renewal failed due to an explicit CAA record that does not
    /// include one of the in-use CAs (Google CA and Let's Encrypt). Renewals will
    /// continue to fail until the CAA is reconfigured. The last successfully
    /// provisioned certificate may still be serving.
    FailedRetryingCaaForbidden = 7,
    /// Most recent renewal failed due to a CAA retrieval failure. This means that
    /// the domain's DNS provider does not properly handle CAA records, failing
    /// requests for CAA records when no CAA records are defined. Renewals will
    /// continue to fail until the DNS provider is changed or a CAA record is
    /// added for the given domain. The last successfully provisioned certificate
    /// may still be serving.
    FailedRetryingCaaChecking = 8,
}
/// A domain that a user has been authorized to administer. To authorize use
/// of a domain, verify ownership via
/// [Webmaster Central](https://www.google.com/webmasters/verification/home).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedDomain {
    /// Full path to the `AuthorizedDomain` resource in the API. Example:
    /// `apps/myapp/authorizedDomains/example.com`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Fully qualified domain name of the domain authorized for use. Example:
    /// `example.com`.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
/// A domain serving an App Engine application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainMapping {
    /// Full path to the `DomainMapping` resource in the API. Example:
    /// `apps/myapp/domainMapping/example.com`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the domain serving the application. Example:
    /// `example.com`.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// SSL configuration for this domain. If unconfigured, this domain will not
    /// serve with SSL.
    #[prost(message, optional, tag = "3")]
    pub ssl_settings: ::std::option::Option<SslSettings>,
    /// The resource records required to configure this domain mapping. These
    /// records must be added to the domain's DNS configuration in order to
    /// serve the application via this domain mapping.
    ///
    /// @OutputOnly
    #[prost(message, repeated, tag = "4")]
    pub resource_records: ::std::vec::Vec<ResourceRecord>,
}
/// SSL configuration for a `DomainMapping` resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslSettings {
    /// ID of the `AuthorizedCertificate` resource configuring SSL for the
    /// application. Clearing this field will remove SSL support.
    ///
    /// By default, a managed certificate is automatically created for every
    /// domain mapping. To omit SSL support or to configure SSL manually, specify
    /// `SslManagementType.MANUAL` on a `CREATE` or `UPDATE` request. You must
    /// be authorized to administer the `AuthorizedCertificate` resource to
    /// manually map it to a `DomainMapping` resource.
    /// Example: `12345`.
    #[prost(string, tag = "1")]
    pub certificate_id: std::string::String,
    /// SSL management type for this domain. If `AUTOMATIC`, a managed certificate
    /// is automatically provisioned. If `MANUAL`, `certificate_id` must be
    /// manually specified in order to configure SSL for this domain.
    #[prost(enumeration = "ssl_settings::SslManagementType", tag = "3")]
    pub ssl_management_type: i32,
    /// ID of the managed `AuthorizedCertificate` resource currently being
    /// provisioned, if applicable. Until the new managed certificate has been
    /// successfully provisioned, the previous SSL state will be preserved. Once
    /// the provisioning process completes, the `certificate_id` field will reflect
    /// the new managed certificate and this field will be left empty. To remove
    /// SSL support while there is still a pending managed certificate, clear the
    /// `certificate_id` field with an `UpdateDomainMappingRequest`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "4")]
    pub pending_managed_certificate_id: std::string::String,
}
pub mod ssl_settings {
    /// The SSL management type for this domain.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SslManagementType {
        /// SSL support for this domain is configured automatically. The mapped SSL
        /// certificate will be automatically renewed.
        Automatic = 0,
        /// SSL support for this domain is configured manually by the user. Either
        /// the domain has no SSL support or a user-obtained SSL certificate has been
        /// explictly mapped to this domain.
        Manual = 1,
    }
}
/// A DNS resource record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRecord {
    /// Relative name of the object affected by this record. Only applicable for
    /// `CNAME` records. Example: 'www'.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Data for this record. Values vary by record type, as defined in RFC 1035
    /// (section 5) and RFC 1034 (section 3.6.1).
    #[prost(string, tag = "2")]
    pub rrdata: std::string::String,
    /// Resource record type. Example: `AAAA`.
    #[prost(enumeration = "resource_record::RecordType", tag = "3")]
    pub r#type: i32,
}
pub mod resource_record {
    /// A resource record type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecordType {
        /// An A resource record. Data is an IPv4 address.
        A = 0,
        /// An AAAA resource record. Data is an IPv6 address.
        Aaaa = 1,
        /// A CNAME resource record. Data is a domain name to be aliased.
        Cname = 2,
    }
}
/// A single firewall rule that is evaluated against incoming traffic
/// and provides an action to take on matched requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRule {
    /// A positive integer between [1, Int32.MaxValue-1] that defines the order of
    /// rule evaluation. Rules with the lowest priority are evaluated first.
    ///
    /// A default rule at priority Int32.MaxValue matches all IPv4 and IPv6 traffic
    /// when no previous rule matches. Only the action of this rule can be modified
    /// by the user.
    #[prost(int32, tag = "1")]
    pub priority: i32,
    /// The action to take on matched requests.
    #[prost(enumeration = "firewall_rule::Action", tag = "2")]
    pub action: i32,
    /// IP address or range, defined using CIDR notation, of requests that this
    /// rule applies to. You can use the wildcard character "*" to match all IPs
    /// equivalent to "0/0" and "::/0" together.
    /// Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32`
    ///           or `2001:0db8:0000:0042:0000:8a2e:0370:7334`.
    ///
    ///
    /// <p>Truncation will be silently performed on addresses which are not
    /// properly truncated. For example, `1.2.3.4/24` is accepted as the same
    /// address as `1.2.3.0/24`. Similarly, for IPv6, `2001:db8::1/32` is accepted
    /// as the same address as `2001:db8::/32`.
    #[prost(string, tag = "3")]
    pub source_range: std::string::String,
    /// An optional string description of this rule.
    /// This field has a maximum length of 100 characters.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
}
pub mod firewall_rule {
    /// Available actions to take on matching requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        UnspecifiedAction = 0,
        /// Matching requests are allowed.
        Allow = 1,
        /// Matching requests are denied.
        Deny = 2,
    }
}
/// An Instance resource is the computing unit that App Engine uses to
/// automatically scale an application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Full path to the Instance resource in the API.
    /// Example: `apps/myapp/services/default/versions/v1/instances/instance-1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the instance within the version.
    /// Example: `instance-1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// App Engine release this instance is running on.
    ///
    /// @OutputOnly
    #[prost(string, tag = "3")]
    pub app_engine_release: std::string::String,
    /// Availability of the instance.
    ///
    /// @OutputOnly
    #[prost(enumeration = "instance::Availability", tag = "4")]
    pub availability: i32,
    /// Name of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "5")]
    pub vm_name: std::string::String,
    /// Zone where the virtual machine is located. Only applicable for instances
    /// in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "6")]
    pub vm_zone_name: std::string::String,
    /// Virtual machine ID of this instance. Only applicable for instances in
    /// App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "7")]
    pub vm_id: std::string::String,
    /// Time that this instance was started.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "8")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Number of requests since this instance was started.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "9")]
    pub requests: i32,
    /// Number of errors since this instance was started.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "10")]
    pub errors: i32,
    /// Average queries per second (QPS) over the last minute.
    ///
    /// @OutputOnly
    #[prost(float, tag = "11")]
    pub qps: f32,
    /// Average latency (ms) over the last minute.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "12")]
    pub average_latency: i32,
    /// Total memory in use (bytes).
    ///
    /// @OutputOnly
    #[prost(int64, tag = "13")]
    pub memory_usage: i64,
    /// Status of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "14")]
    pub vm_status: std::string::String,
    /// Whether this instance is in debug mode. Only applicable for instances in
    /// App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "15")]
    pub vm_debug_enabled: bool,
    /// The IP address of this instance. Only applicable for instances in App
    /// Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "16")]
    pub vm_ip: std::string::String,
}
pub mod instance {
    /// Availability of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Availability {
        Unspecified = 0,
        Resident = 1,
        Dynamic = 2,
    }
}
/// Code and application artifacts used to deploy a version to App Engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Manifest of the files stored in Google Cloud Storage that are included
    /// as part of this version. All files must be readable using the
    /// credentials supplied with this call.
    #[prost(map = "string, message", tag = "1")]
    pub files: ::std::collections::HashMap<std::string::String, FileInfo>,
    /// The Docker image for the container that runs the version.
    /// Only applicable for instances running in the App Engine flexible environment.
    #[prost(message, optional, tag = "2")]
    pub container: ::std::option::Option<ContainerInfo>,
    /// The zip file for this deployment, if this is a zip deployment.
    #[prost(message, optional, tag = "3")]
    pub zip: ::std::option::Option<ZipInfo>,
    /// Google Cloud Build build information. Only applicable for instances running
    /// in the App Engine flexible environment.
    #[prost(message, optional, tag = "5")]
    pub build: ::std::option::Option<BuildInfo>,
    /// Options for any Google Cloud Build builds created as a part of this
    /// deployment.
    ///
    /// These options will only be used if a new build is created, such as when
    /// deploying to the App Engine flexible environment using files or zip.
    #[prost(message, optional, tag = "6")]
    pub cloud_build_options: ::std::option::Option<CloudBuildOptions>,
}
/// Single source file that is part of the version to be deployed. Each source
/// file that is deployed must be specified separately.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    /// URL source to use to fetch this file. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "1")]
    pub source_url: std::string::String,
    /// The SHA1 hash of the file, in hex.
    #[prost(string, tag = "2")]
    pub sha1_sum: std::string::String,
    /// The MIME type of the file.
    ///
    /// Defaults to the value from Google Cloud Storage.
    #[prost(string, tag = "3")]
    pub mime_type: std::string::String,
}
/// Docker image that is used to create a container and start a VM instance for
/// the version that you deploy. Only applicable for instances running in the App
/// Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerInfo {
    /// URI to the hosted container image in Google Container Registry. The URI
    /// must be fully qualified and include a tag or digest.
    /// Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    #[prost(string, tag = "1")]
    pub image: std::string::String,
}
/// Google Cloud Build information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildInfo {
    /// The Google Cloud Build id.
    /// Example: "f966068f-08b2-42c8-bdfe-74137dff2bf9"
    #[prost(string, tag = "1")]
    pub cloud_build_id: std::string::String,
}
/// Options for the build operations performed as a part of the version
/// deployment. Only applicable for App Engine flexible environment when creating
/// a version using source code directly.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudBuildOptions {
    /// Path to the yaml file used in deployment, used to determine runtime
    /// configuration details.
    ///
    /// Required for flexible environment builds.
    ///
    /// See https://cloud.google.com/appengine/docs/standard/python/config/appref
    /// for more details.
    #[prost(string, tag = "1")]
    pub app_yaml_path: std::string::String,
    /// The Cloud Build timeout used as part of any dependent builds performed by
    /// version creation. Defaults to 10 minutes.
    #[prost(message, optional, tag = "2")]
    pub cloud_build_timeout: ::std::option::Option<::prost_types::Duration>,
}
/// The zip file information for a zip deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZipInfo {
    /// URL of the zip file to deploy from. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "3")]
    pub source_url: std::string::String,
    /// An estimate of the number of files in a zip for a zip deployment.
    /// If set, must be greater than or equal to the actual number of files.
    /// Used for optimizing performance; if not provided, deployment may be slow.
    #[prost(int32, tag = "4")]
    pub files_count: i32,
}
/// A Version resource is a specific set of source code and configuration files
/// that are deployed into a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Full path to the Version resource in the API.  Example:
    /// `apps/myapp/services/default/versions/v1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the version within the service.  Example: `v1`.
    /// Version names can contain only lowercase letters, numbers, or hyphens.
    /// Reserved names: "default", "latest", and any name with the prefix "ah-".
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Before an application can receive email or XMPP messages, the application
    /// must be configured to enable the service.
    #[prost(enumeration = "InboundServiceType", repeated, tag = "6")]
    pub inbound_services: ::std::vec::Vec<i32>,
    /// Instance class that is used to run this version. Valid values are:
    ///
    /// * AutomaticScaling: `F1`, `F2`, `F4`, `F4_1G`
    /// * ManualScaling or BasicScaling: `B1`, `B2`, `B4`, `B8`, `B4_1G`
    ///
    /// Defaults to `F1` for AutomaticScaling and `B1` for ManualScaling or
    /// BasicScaling.
    #[prost(string, tag = "7")]
    pub instance_class: std::string::String,
    /// Extra network settings.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "8")]
    pub network: ::std::option::Option<Network>,
    /// The Google Compute Engine zones that are supported by this version in the
    /// App Engine flexible environment. Deprecated.
    #[prost(string, repeated, tag = "118")]
    pub zones: ::std::vec::Vec<std::string::String>,
    /// Machine resources for this version.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "9")]
    pub resources: ::std::option::Option<Resources>,
    /// Desired runtime. Example: `python27`.
    #[prost(string, tag = "10")]
    pub runtime: std::string::String,
    /// The channel of the runtime to use. Only available for some
    /// runtimes. Defaults to the `default` channel.
    #[prost(string, tag = "117")]
    pub runtime_channel: std::string::String,
    /// Whether multiple requests can be dispatched to this version at once.
    #[prost(bool, tag = "11")]
    pub threadsafe: bool,
    /// Whether to deploy this version in a container on a virtual machine.
    #[prost(bool, tag = "12")]
    pub vm: bool,
    /// Metadata settings that are supplied to this version to enable
    /// beta runtime features.
    #[prost(map = "string, string", tag = "13")]
    pub beta_settings: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// App Engine execution environment for this version.
    ///
    /// Defaults to `standard`.
    #[prost(string, tag = "14")]
    pub env: std::string::String,
    /// Current serving status of this version. Only the versions with a
    /// `SERVING` status create instances and can be billed.
    ///
    /// `SERVING_STATUS_UNSPECIFIED` is an invalid value. Defaults to `SERVING`.
    #[prost(enumeration = "ServingStatus", tag = "15")]
    pub serving_status: i32,
    /// Email address of the user who created this version.
    ///
    /// @OutputOnly
    #[prost(string, tag = "16")]
    pub created_by: std::string::String,
    /// Time that this version was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "17")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Total size in bytes of all the files that are included in this version
    /// and currently hosted on the App Engine disk.
    ///
    /// @OutputOnly
    #[prost(int64, tag = "18")]
    pub disk_usage_bytes: i64,
    /// The version of the API in the given runtime environment. Please see the
    /// app.yaml reference for valid values at
    /// https://cloud.google.com/appengine/docs/standard/<language>/config/appref
    #[prost(string, tag = "21")]
    pub runtime_api_version: std::string::String,
    /// The path or name of the app's main executable.
    #[prost(string, tag = "22")]
    pub runtime_main_executable_path: std::string::String,
    /// An ordered list of URL-matching patterns that should be applied to incoming
    /// requests. The first matching URL handles the request and other request
    /// handlers are not attempted.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "100")]
    pub handlers: ::std::vec::Vec<UrlMap>,
    /// Custom static error pages. Limited to 10KB per page.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "101")]
    pub error_handlers: ::std::vec::Vec<ErrorHandler>,
    /// Configuration for third-party Python runtime libraries that are required
    /// by the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "102")]
    pub libraries: ::std::vec::Vec<Library>,
    /// Serving configuration for
    /// [Google Cloud Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/).
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "103")]
    pub api_config: ::std::option::Option<ApiConfigHandler>,
    /// Environment variables available to the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(map = "string, string", tag = "104")]
    pub env_variables: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Duration that static files should be cached by web proxies and browsers.
    /// Only applicable if the corresponding
    /// [StaticFilesHandler](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StaticFilesHandler)
    /// does not specify its own expiration time.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "105")]
    pub default_expiration: ::std::option::Option<::prost_types::Duration>,
    /// Configures health checking for instances. Unhealthy instances are
    /// stopped and replaced with new instances.
    /// Only applicable in the App Engine flexible environment.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "106")]
    pub health_check: ::std::option::Option<HealthCheck>,
    /// Configures readiness health checking for instances.
    /// Unhealthy instances are not put into the backend traffic rotation.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "112")]
    pub readiness_check: ::std::option::Option<ReadinessCheck>,
    /// Configures liveness health checking for instances.
    /// Unhealthy instances are stopped and replaced with new instances
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "113")]
    pub liveness_check: ::std::option::Option<LivenessCheck>,
    /// Files that match this pattern will not be built into this version.
    /// Only applicable for Go runtimes.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(string, tag = "107")]
    pub nobuild_files_regex: std::string::String,
    /// Code and application artifacts that make up this version.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "108")]
    pub deployment: ::std::option::Option<Deployment>,
    /// Serving URL for this version. Example:
    /// "https://myversion-dot-myservice-dot-myapp.appspot.com"
    ///
    /// @OutputOnly
    #[prost(string, tag = "109")]
    pub version_url: std::string::String,
    /// Cloud Endpoints configuration.
    ///
    /// If endpoints_api_service is set, the Cloud Endpoints Extensible Service
    /// Proxy will be provided to serve the API implemented by the app.
    #[prost(message, optional, tag = "110")]
    pub endpoints_api_service: ::std::option::Option<EndpointsApiService>,
    /// The entrypoint for the application.
    #[prost(message, optional, tag = "122")]
    pub entrypoint: ::std::option::Option<Entrypoint>,
    /// Enables VPC connectivity for standard apps.
    #[prost(message, optional, tag = "121")]
    pub vpc_access_connector: ::std::option::Option<VpcAccessConnector>,
    /// Controls how instances are created.
    ///
    /// Defaults to `AutomaticScaling`.
    #[prost(oneof = "version::Scaling", tags = "3, 4, 5")]
    pub scaling: ::std::option::Option<version::Scaling>,
}
pub mod version {
    /// Controls how instances are created.
    ///
    /// Defaults to `AutomaticScaling`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scaling {
        /// Automatic scaling is based on request rate, response latencies, and other
        /// application metrics.
        #[prost(message, tag = "3")]
        AutomaticScaling(super::AutomaticScaling),
        /// A service with basic scaling will create an instance when the application
        /// receives a request. The instance will be turned down when the app becomes
        /// idle. Basic scaling is ideal for work that is intermittent or driven by
        /// user activity.
        #[prost(message, tag = "4")]
        BasicScaling(super::BasicScaling),
        /// A service with manual scaling runs continuously, allowing you to perform
        /// complex initialization and rely on the state of its memory over time.
        #[prost(message, tag = "5")]
        ManualScaling(super::ManualScaling),
    }
}
/// [Cloud Endpoints](https://cloud.google.com/endpoints) configuration.
/// The Endpoints API Service provides tooling for serving Open API and gRPC
/// endpoints via an NGINX proxy. Only valid for App Engine Flexible environment
/// deployments.
///
/// The fields here refer to the name and configuration ID of a "service"
/// resource in the [Service Management API](https://cloud.google.com/service-management/overview).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointsApiService {
    /// Endpoints service name which is the name of the "service" resource in the
    /// Service Management API. For example "myapi.endpoints.myproject.cloud.goog"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Endpoints service configuration ID as specified by the Service Management
    /// API. For example "2016-09-19r1".
    ///
    /// By default, the rollout strategy for Endpoints is `RolloutStrategy.FIXED`.
    /// This means that Endpoints starts up with a particular configuration ID.
    /// When a new configuration is rolled out, Endpoints must be given the new
    /// configuration ID. The `config_id` field is used to give the configuration
    /// ID and is required in this case.
    ///
    /// Endpoints also has a rollout strategy called `RolloutStrategy.MANAGED`.
    /// When using this, Endpoints fetches the latest configuration and does not
    /// need the configuration ID. In this case, `config_id` must be omitted.
    #[prost(string, tag = "2")]
    pub config_id: std::string::String,
    /// Endpoints rollout strategy. If `FIXED`, `config_id` must be specified. If
    /// `MANAGED`, `config_id` must be omitted.
    #[prost(enumeration = "endpoints_api_service::RolloutStrategy", tag = "3")]
    pub rollout_strategy: i32,
    /// Enable or disable trace sampling. By default, this is set to false for
    /// enabled.
    #[prost(bool, tag = "4")]
    pub disable_trace_sampling: bool,
}
pub mod endpoints_api_service {
    /// Available rollout strategies.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RolloutStrategy {
        /// Not specified. Defaults to `FIXED`.
        UnspecifiedRolloutStrategy = 0,
        /// Endpoints service configuration ID will be fixed to the configuration ID
        /// specified by `config_id`.
        Fixed = 1,
        /// Endpoints service configuration ID will be updated with each rollout.
        Managed = 2,
    }
}
/// Automatic scaling is based on request rate, response latencies, and other
/// application metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomaticScaling {
    /// The time period that the
    /// [Autoscaler](https://cloud.google.com/compute/docs/autoscaler/)
    /// should wait before it starts collecting information from a new instance.
    /// This prevents the autoscaler from collecting information when the instance
    /// is initializing, during which the collected usage would not be reliable.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "1")]
    pub cool_down_period: ::std::option::Option<::prost_types::Duration>,
    /// Target scaling by CPU usage.
    #[prost(message, optional, tag = "2")]
    pub cpu_utilization: ::std::option::Option<CpuUtilization>,
    /// Number of concurrent requests an automatic scaling instance can accept
    /// before the scheduler spawns a new instance.
    ///
    /// Defaults to a runtime-specific value.
    #[prost(int32, tag = "3")]
    pub max_concurrent_requests: i32,
    /// Maximum number of idle instances that should be maintained for this
    /// version.
    #[prost(int32, tag = "4")]
    pub max_idle_instances: i32,
    /// Maximum number of instances that should be started to handle requests for
    /// this version.
    #[prost(int32, tag = "5")]
    pub max_total_instances: i32,
    /// Maximum amount of time that a request should wait in the pending queue
    /// before starting a new instance to handle it.
    #[prost(message, optional, tag = "6")]
    pub max_pending_latency: ::std::option::Option<::prost_types::Duration>,
    /// Minimum number of idle instances that should be maintained for
    /// this version. Only applicable for the default version of a service.
    #[prost(int32, tag = "7")]
    pub min_idle_instances: i32,
    /// Minimum number of running instances that should be maintained for this
    /// version.
    #[prost(int32, tag = "8")]
    pub min_total_instances: i32,
    /// Minimum amount of time a request should wait in the pending queue before
    /// starting a new instance to handle it.
    #[prost(message, optional, tag = "9")]
    pub min_pending_latency: ::std::option::Option<::prost_types::Duration>,
    /// Target scaling by request utilization.
    #[prost(message, optional, tag = "10")]
    pub request_utilization: ::std::option::Option<RequestUtilization>,
    /// Target scaling by disk usage.
    #[prost(message, optional, tag = "11")]
    pub disk_utilization: ::std::option::Option<DiskUtilization>,
    /// Target scaling by network usage.
    #[prost(message, optional, tag = "12")]
    pub network_utilization: ::std::option::Option<NetworkUtilization>,
    /// Target scaling by user-provided metrics.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, repeated, tag = "21")]
    pub custom_metrics: ::std::vec::Vec<CustomMetric>,
    /// Scheduler settings for standard environment.
    #[prost(message, optional, tag = "20")]
    pub standard_scheduler_settings: ::std::option::Option<StandardSchedulerSettings>,
}
/// A service with basic scaling will create an instance when the application
/// receives a request. The instance will be turned down when the app becomes
/// idle. Basic scaling is ideal for work that is intermittent or driven by
/// user activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicScaling {
    /// Duration of time after the last request that an instance must wait before
    /// the instance is shut down.
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::std::option::Option<::prost_types::Duration>,
    /// Maximum number of instances to create for this version.
    #[prost(int32, tag = "2")]
    pub max_instances: i32,
}
/// A service with manual scaling runs continuously, allowing you to perform
/// complex initialization and rely on the state of its memory over time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualScaling {
    /// Number of instances to assign to the service at the start. This number
    /// can later be altered by using the
    /// [Modules API](https://cloud.google.com/appengine/docs/python/modules/functions)
    /// `set_num_instances()` function.
    #[prost(int32, tag = "1")]
    pub instances: i32,
}
/// Target scaling by CPU usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuUtilization {
    /// Period of time over which CPU utilization is calculated.
    #[prost(message, optional, tag = "1")]
    pub aggregation_window_length: ::std::option::Option<::prost_types::Duration>,
    /// Target CPU utilization ratio to maintain when scaling. Must be between 0
    /// and 1.
    #[prost(double, tag = "2")]
    pub target_utilization: f64,
}
/// Target scaling by request utilization.
/// Only applicable in the App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUtilization {
    /// Target requests per second.
    #[prost(int32, tag = "1")]
    pub target_request_count_per_second: i32,
    /// Target number of concurrent requests.
    #[prost(int32, tag = "2")]
    pub target_concurrent_requests: i32,
}
/// Target scaling by disk usage.
/// Only applicable in the App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUtilization {
    /// Target bytes written per second.
    #[prost(int32, tag = "14")]
    pub target_write_bytes_per_second: i32,
    /// Target ops written per second.
    #[prost(int32, tag = "15")]
    pub target_write_ops_per_second: i32,
    /// Target bytes read per second.
    #[prost(int32, tag = "16")]
    pub target_read_bytes_per_second: i32,
    /// Target ops read per seconds.
    #[prost(int32, tag = "17")]
    pub target_read_ops_per_second: i32,
}
/// Target scaling by network usage.
/// Only applicable in the App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkUtilization {
    /// Target bytes sent per second.
    #[prost(int32, tag = "1")]
    pub target_sent_bytes_per_second: i32,
    /// Target packets sent per second.
    #[prost(int32, tag = "11")]
    pub target_sent_packets_per_second: i32,
    /// Target bytes received per second.
    #[prost(int32, tag = "12")]
    pub target_received_bytes_per_second: i32,
    /// Target packets received per second.
    #[prost(int32, tag = "13")]
    pub target_received_packets_per_second: i32,
}
/// Allows autoscaling based on Stackdriver metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetric {
    /// The name of the metric.
    #[prost(string, tag = "1")]
    pub metric_name: std::string::String,
    /// The type of the metric. Must be a string representing a Stackdriver
    /// metric type e.g. GAGUE, DELTA_PER_SECOND, etc.
    #[prost(string, tag = "2")]
    pub target_type: std::string::String,
    /// Allows filtering on the metric's fields.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// The target spec.
    #[prost(oneof = "custom_metric::TargetSpec", tags = "3, 4")]
    pub target_spec: ::std::option::Option<custom_metric::TargetSpec>,
}
pub mod custom_metric {
    /// The target spec.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetSpec {
        /// The target value for the metric.
        #[prost(double, tag = "3")]
        TargetUtilization(f64),
        /// May be used instead of `target_utilization` when an instance can handle a
        /// specific amount of work/resources and the metric value is equal to the
        /// current amount of work remaining. The autoscaler will try to keep the
        /// number of instances equal to the metric value divided by
        /// `single_instance_assignment`.
        #[prost(double, tag = "4")]
        SingleInstanceAssignment(f64),
    }
}
/// Scheduler settings for standard environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSchedulerSettings {
    /// Target CPU utilization ratio to maintain when scaling.
    #[prost(double, tag = "1")]
    pub target_cpu_utilization: f64,
    /// Target throughput utilization ratio to maintain when scaling
    #[prost(double, tag = "2")]
    pub target_throughput_utilization: f64,
    /// Minimum number of instances to run for this version. Set to zero to disable
    /// `min_instances` configuration.
    #[prost(int32, tag = "3")]
    pub min_instances: i32,
    /// Maximum number of instances to run for this version. Set to zero to disable
    /// `max_instances` configuration.
    #[prost(int32, tag = "4")]
    pub max_instances: i32,
}
/// Extra network settings.
/// Only applicable in the App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// List of ports, or port pairs, to forward from the virtual machine to the
    /// application container.
    /// Only applicable in the App Engine flexible environment.
    #[prost(string, repeated, tag = "1")]
    pub forwarded_ports: ::std::vec::Vec<std::string::String>,
    /// Tag to apply to the instance during creation.
    /// Only applicable in the App Engine flexible environment.
    #[prost(string, tag = "2")]
    pub instance_tag: std::string::String,
    /// Google Compute Engine network where the virtual machines are created.
    /// Specify the short name, not the resource path.
    ///
    /// Defaults to `default`.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Google Cloud Platform sub-network where the virtual machines are created.
    /// Specify the short name, not the resource path.
    ///
    /// If a subnetwork name is specified, a network name will also be required
    /// unless it is for the default network.
    ///
    /// * If the network that the instance is being created in is a Legacy network,
    /// then the IP address is allocated from the IPv4Range.
    /// * If the network that the instance is being created in is an auto Subnet
    /// Mode Network, then only network name should be specified (not the
    /// subnetwork_name) and the IP address is created from the IPCidrRange of the
    /// subnetwork that exists in that zone for that network.
    /// * If the network that the instance is being created in is a custom Subnet
    /// Mode Network, then the subnetwork_name must be specified and the
    /// IP address is created from the IPCidrRange of the subnetwork.
    ///
    /// If specified, the subnetwork must exist in the same region as the
    /// App Engine flexible environment application.
    #[prost(string, tag = "4")]
    pub subnetwork_name: std::string::String,
    /// Enable session affinity.
    /// Only applicable in the App Engine flexible environment.
    #[prost(bool, tag = "5")]
    pub session_affinity: bool,
}
/// Volumes mounted within the app container.
/// Only applicable in the App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Unique name for the volume.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Underlying volume type, e.g. 'tmpfs'.
    #[prost(string, tag = "2")]
    pub volume_type: std::string::String,
    /// Volume size in gigabytes.
    #[prost(double, tag = "3")]
    pub size_gb: f64,
}
/// Machine resources for a version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    /// Number of CPU cores needed.
    #[prost(double, tag = "1")]
    pub cpu: f64,
    /// Disk size (GB) needed.
    #[prost(double, tag = "2")]
    pub disk_gb: f64,
    /// Memory (GB) needed.
    #[prost(double, tag = "3")]
    pub memory_gb: f64,
    /// User specified volumes.
    #[prost(message, repeated, tag = "4")]
    pub volumes: ::std::vec::Vec<Volume>,
}
/// VPC access connector specification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcAccessConnector {
    /// Full Serverless VPC Access Connector name e.g.
    /// /projects/my-project/locations/us-central1/connectors/c1.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The entrypoint for the application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entrypoint {
    /// The command to run.
    #[prost(oneof = "entrypoint::Command", tags = "1")]
    pub command: ::std::option::Option<entrypoint::Command>,
}
pub mod entrypoint {
    /// The command to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        /// The format should be a shell command that can be fed to `bash -c`.
        #[prost(string, tag = "1")]
        Shell(std::string::String),
    }
}
/// Available inbound services.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InboundServiceType {
    /// Not specified.
    InboundServiceUnspecified = 0,
    /// Allows an application to receive mail.
    InboundServiceMail = 1,
    /// Allows an application to receive email-bound notifications.
    InboundServiceMailBounce = 2,
    /// Allows an application to receive error stanzas.
    InboundServiceXmppError = 3,
    /// Allows an application to receive instant messages.
    InboundServiceXmppMessage = 4,
    /// Allows an application to receive user subscription POSTs.
    InboundServiceXmppSubscribe = 5,
    /// Allows an application to receive a user's chat presence.
    InboundServiceXmppPresence = 6,
    /// Registers an application for notifications when a client connects or
    /// disconnects from a channel.
    InboundServiceChannelPresence = 7,
    /// Enables warmup requests.
    InboundServiceWarmup = 9,
}
/// Run states of a version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingStatus {
    /// Not specified.
    Unspecified = 0,
    /// Currently serving. Instances are created according to the
    /// scaling settings of the version.
    Serving = 1,
    /// Disabled. No instances will be created and the scaling
    /// settings are ignored until the state of the version changes
    /// to `SERVING`.
    Stopped = 2,
}
/// A Service resource is a logical component of an application that can share
/// state and communicate in a secure fashion with other services.
/// For example, an application that handles customer requests might
/// include separate services to handle tasks such as backend data
/// analysis or API requests from mobile devices. Each service has a
/// collection of versions that define a specific set of code used to
/// implement the functionality of that service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Full path to the Service resource in the API.
    /// Example: `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the service within the application.
    /// Example: `default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Mapping that defines fractional HTTP traffic diversion to
    /// different versions within the service.
    #[prost(message, optional, tag = "3")]
    pub split: ::std::option::Option<TrafficSplit>,
}
/// Traffic routing configuration for versions within a single service. Traffic
/// splits define how traffic directed to the service is assigned to versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficSplit {
    /// Mechanism used to determine which version a request is sent to.
    /// The traffic selection algorithm will
    /// be stable for either type until allocations are changed.
    #[prost(enumeration = "traffic_split::ShardBy", tag = "1")]
    pub shard_by: i32,
    /// Mapping from version IDs within the service to fractional
    /// (0.000, 1] allocations of traffic for that version. Each version can
    /// be specified only once, but some versions in the service may not
    /// have any traffic allocation. Services that have traffic allocated
    /// cannot be deleted until either the service is deleted or
    /// their traffic allocation is removed. Allocations must sum to 1.
    /// Up to two decimal place precision is supported for IP-based splits and
    /// up to three decimal places is supported for cookie-based splits.
    #[prost(map = "string, double", tag = "2")]
    pub allocations: ::std::collections::HashMap<std::string::String, f64>,
}
pub mod traffic_split {
    /// Available sharding mechanisms.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ShardBy {
        /// Diversion method unspecified.
        Unspecified = 0,
        /// Diversion based on a specially named cookie, "GOOGAPPUID." The cookie
        /// must be set by the application itself or no diversion will occur.
        Cookie = 1,
        /// Diversion based on applying the modulus operation to a fingerprint
        /// of the IP address.
        Ip = 2,
        /// Diversion based on weighted random assignment. An incoming request is
        /// randomly routed to a version in the traffic split, with probability
        /// proportional to the version's traffic share.
        Random = 3,
    }
}
/// Request message for `Applications.GetApplication`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Name of the Application resource to get. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Applications.CreateApplication`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationRequest {
    /// Application configuration.
    #[prost(message, optional, tag = "2")]
    pub application: ::std::option::Option<Application>,
}
/// Request message for `Applications.UpdateApplication`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationRequest {
    /// Name of the Application resource to update. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// An Application containing the updated resource.
    #[prost(message, optional, tag = "2")]
    pub application: ::std::option::Option<Application>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for 'Applications.RepairApplication'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairApplicationRequest {
    /// Name of the application to repair. Example: `apps/myapp`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Services.ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `Services.ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The services belonging to the requested application.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Services.GetService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Services.UpdateService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Name of the resource to update. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A Service resource containing the updated service. Only fields set in the
    /// field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::std::option::Option<Service>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Set to `true` to gradually shift traffic to one or more versions that you
    /// specify. By default, traffic is shifted immediately.
    /// For gradual traffic migration, the target versions
    /// must be located within instances that are configured for both
    /// [warmup requests](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#InboundServiceType)
    /// and
    /// [automatic scaling](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#AutomaticScaling).
    /// You must specify the
    /// [`shardBy`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services#ShardBy)
    /// field in the Service resource. Gradual traffic migration is not
    /// supported in the App Engine flexible environment. For examples, see
    /// [Migrating and Splitting Traffic](https://cloud.google.com/appengine/docs/admin-api/migrating-splitting-traffic).
    #[prost(bool, tag = "4")]
    pub migrate_traffic: bool,
}
/// Request message for `Services.DeleteService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Versions.ListVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Name of the parent Service resource. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Controls the set of fields returned in the `List` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
    /// Maximum results to return per page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response message for `Versions.ListVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The versions belonging to the requested service.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::std::vec::Vec<Version>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Versions.GetVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Controls the set of fields returned in the `Get` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
}
/// Request message for `Versions.CreateVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Name of the parent resource to create this version under. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Application deployment configuration.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
}
/// Request message for `Versions.UpdateVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVersionRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/services/default/versions/1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A Version containing the updated resource. Only fields set in the field
    /// mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `Versions.DeleteVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.ListInstances`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Name of the parent Version resource. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `Instances.ListInstances`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The instances belonging to the requested version.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Instances.GetInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.DeleteInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.DebugInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Public SSH key to add to the instance. Examples:
    ///
    /// * `[USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME]`
    /// * `[USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}`
    ///
    /// For more information, see
    /// [Adding and Removing SSH Keys](https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys).
    #[prost(string, tag = "2")]
    pub ssh_key: std::string::String,
}
/// Request message for `Firewall.ListIngressRules`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressRulesRequest {
    /// Name of the Firewall collection to retrieve.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// A valid IP Address. If set, only rules matching this address will be
    /// returned. The first returned rule will be the rule that fires on requests
    /// from this IP.
    #[prost(string, tag = "4")]
    pub matching_address: std::string::String,
}
/// Response message for `Firewall.ListIngressRules`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressRulesResponse {
    /// The ingress FirewallRules for this application.
    #[prost(message, repeated, tag = "1")]
    pub ingress_rules: ::std::vec::Vec<FirewallRule>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Firewall.BatchUpdateIngressRules`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIngressRulesRequest {
    /// Name of the Firewall collection to set.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A list of FirewallRules to replace the existing set.
    #[prost(message, repeated, tag = "2")]
    pub ingress_rules: ::std::vec::Vec<FirewallRule>,
}
/// Response message for `Firewall.UpdateAllIngressRules`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIngressRulesResponse {
    /// The full list of ingress FirewallRules for this application.
    #[prost(message, repeated, tag = "1")]
    pub ingress_rules: ::std::vec::Vec<FirewallRule>,
}
/// Request message for `Firewall.CreateIngressRule`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngressRuleRequest {
    /// Name of the parent Firewall collection in which to create a new rule.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A FirewallRule containing the new resource.
    ///
    /// The user may optionally provide a position at which the new rule will be
    /// placed. The positions define a sequential list starting at 1. If a rule
    /// already exists at the given position, rules greater than the provided
    /// position will be moved forward by one.
    ///
    /// If no position is provided, the server will place the rule as the second to
    /// last rule in the sequence before the required default allow-all or deny-all
    /// rule.
    #[prost(message, optional, tag = "2")]
    pub rule: ::std::option::Option<FirewallRule>,
}
/// Request message for `Firewall.GetIngressRule`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIngressRuleRequest {
    /// Name of the Firewall resource to retrieve.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Firewall.UpdateIngressRule`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIngressRuleRequest {
    /// Name of the Firewall resource to update.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A FirewallRule containing the updated resource
    #[prost(message, optional, tag = "2")]
    pub rule: ::std::option::Option<FirewallRule>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `Firewall.DeleteIngressRule`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIngressRuleRequest {
    /// Name of the Firewall resource to delete.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `AuthorizedDomains.ListAuthorizedDomains`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedDomainsRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `AuthorizedDomains.ListAuthorizedDomains`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedDomainsResponse {
    /// The authorized domains belonging to the user.
    #[prost(message, repeated, tag = "1")]
    pub domains: ::std::vec::Vec<AuthorizedDomain>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `AuthorizedCertificates.ListAuthorizedCertificates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedCertificatesRequest {
    /// Name of the parent `Application` resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Controls the set of fields returned in the `LIST` response.
    #[prost(enumeration = "AuthorizedCertificateView", tag = "4")]
    pub view: i32,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `AuthorizedCertificates.ListAuthorizedCertificates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedCertificatesResponse {
    /// The SSL certificates the user is authorized to administer.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::std::vec::Vec<AuthorizedCertificate>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `AuthorizedCertificates.GetAuthorizedCertificate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizedCertificateRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Controls the set of fields returned in the `GET` response.
    #[prost(enumeration = "AuthorizedCertificateView", tag = "2")]
    pub view: i32,
}
/// Request message for `AuthorizedCertificates.CreateAuthorizedCertificate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizedCertificateRequest {
    /// Name of the parent `Application` resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// SSL certificate data.
    #[prost(message, optional, tag = "2")]
    pub certificate: ::std::option::Option<AuthorizedCertificate>,
}
/// Request message for `AuthorizedCertificates.UpdateAuthorizedCertificate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizedCertificateRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// An `AuthorizedCertificate` containing the updated resource. Only fields set
    /// in the field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub certificate: ::std::option::Option<AuthorizedCertificate>,
    /// Standard field mask for the set of fields to be updated. Updates are only
    /// supported on the `certificate_raw_data` and `display_name` fields.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AuthorizedCertificates.DeleteAuthorizedCertificate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizedCertificateRequest {
    /// Name of the resource to delete. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `DomainMappings.ListDomainMappings`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainMappingsRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `DomainMappings.ListDomainMappings`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainMappingsResponse {
    /// The domain mappings for the application.
    #[prost(message, repeated, tag = "1")]
    pub domain_mappings: ::std::vec::Vec<DomainMapping>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `DomainMappings.GetDomainMapping`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainMappingRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `DomainMappings.CreateDomainMapping`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDomainMappingRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Domain mapping configuration.
    #[prost(message, optional, tag = "2")]
    pub domain_mapping: ::std::option::Option<DomainMapping>,
    /// Whether the domain creation should override any existing mappings for this
    /// domain. By default, overrides are rejected.
    #[prost(enumeration = "DomainOverrideStrategy", tag = "4")]
    pub override_strategy: i32,
}
/// Request message for `DomainMappings.UpdateDomainMapping`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDomainMappingRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A domain mapping containing the updated resource. Only fields set
    /// in the field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub domain_mapping: ::std::option::Option<DomainMapping>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `DomainMappings.DeleteDomainMapping`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDomainMappingRequest {
    /// Name of the resource to delete. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Fields that should be returned when [Version][google.appengine.v1beta.Version] resources
/// are retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VersionView {
    /// Basic version information including scaling and inbound services,
    /// but not detailed deployment information.
    Basic = 0,
    /// The information from `BASIC`, plus detailed information about the
    /// deployment. This format is required when creating resources, but
    /// is not returned in `Get` or `List` by default.
    Full = 1,
}
/// Fields that should be returned when an AuthorizedCertificate resource is
/// retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorizedCertificateView {
    /// Basic certificate information, including applicable domains and expiration
    /// date.
    BasicCertificate = 0,
    /// The information from `BASIC_CERTIFICATE`, plus detailed information on the
    /// domain mappings that have this certificate mapped.
    FullCertificate = 1,
}
/// Override strategy for mutating an existing mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainOverrideStrategy {
    /// Strategy unspecified. Defaults to `STRICT`.
    UnspecifiedDomainOverrideStrategy = 0,
    /// Overrides not allowed. If a mapping already exists for the
    /// specified domain, the request will return an ALREADY_EXISTS (409).
    Strict = 1,
    /// Overrides allowed. If a mapping already exists for the specified domain,
    /// the request will overwrite it. Note that this might stop another
    /// Google product from serving. For example, if the domain is
    /// mapped to another App Engine application, that app will no
    /// longer serve from that domain.
    Override = 2,
}
#[doc = r" Generated client implementations."]
pub mod applications_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages App Engine applications."]
    pub struct ApplicationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApplicationsClient<T>
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
        #[doc = " Gets information about an application."]
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Applications/GetApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an App Engine application for a Google Cloud Platform project."]
        #[doc = " Required fields:"]
        #[doc = ""]
        #[doc = " * `id` - The ID of the target Cloud Platform project."]
        #[doc = " * *location* - The [region](https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located."]
        #[doc = ""]
        #[doc = " For more information about App Engine applications, see [Managing Projects, Applications, and Billing](https://cloud.google.com/appengine/docs/standard/python/console/)."]
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Applications/CreateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified Application resource."]
        #[doc = " You can update the following fields:"]
        #[doc = ""]
        #[doc = " * `auth_domain` - Google authentication domain for controlling user access to the application."]
        #[doc = " * `default_cookie_expiration` - Cookie expiration policy for the application."]
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Applications/UpdateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Recreates the required App Engine features for the specified App Engine"]
        #[doc = " application, for example a Cloud Storage bucket or App Engine service"]
        #[doc = " account."]
        #[doc = " Use this method if you receive an error message about a missing feature,"]
        #[doc = " for example, *Error retrieving the App Engine service account*."]
        #[doc = " If you have deleted your App Engine service account, this will"]
        #[doc = " not be able to recreate it. Instead, you should attempt to use the"]
        #[doc = " IAM undelete API if possible at https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts/undelete?apix_params=%7B\"name\"%3A\"projects%2F-%2FserviceAccounts%2Funique_id\"%2C\"resource\"%3A%7B%7D%7D ."]
        #[doc = " If the deletion was recent, the numeric ID can be found in the Cloud"]
        #[doc = " Console Activity Log."]
        pub async fn repair_application(
            &mut self,
            request: impl tonic::IntoRequest<super::RepairApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Applications/RepairApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ApplicationsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ApplicationsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ApplicationsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod services_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages services of an application."]
    pub struct ServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServicesClient<T>
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
        #[doc = " Lists all the services in the application."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Services/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the current configuration of the specified service."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Services/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the configuration of the specified service."]
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Services/UpdateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified service and all enclosed versions."]
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Services/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ServicesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ServicesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ServicesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod versions_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages versions of a service."]
    pub struct VersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VersionsClient<T>
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
        #[doc = " Lists the versions of a service."]
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Versions/ListVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified Version resource."]
        #[doc = " By default, only a `BASIC_VIEW` will be returned."]
        #[doc = " Specify the `FULL_VIEW` parameter to get the full resource."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Versions/GetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deploys code and resource files to a new version."]
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Versions/CreateVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified Version resource."]
        #[doc = " You can specify the following fields depending on the App Engine"]
        #[doc = " environment and type of scaling that the version resource uses:"]
        #[doc = ""]
        #[doc = " **Standard environment**"]
        #[doc = ""]
        #[doc = " * [`instance_class`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.instance_class)"]
        #[doc = ""]
        #[doc = " *automatic scaling* in the standard environment:"]
        #[doc = ""]
        #[doc = " * [`automatic_scaling.min_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.max_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.max_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.min_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.target_cpu_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.target_throughput_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = ""]
        #[doc = " *basic scaling* or *manual scaling* in the standard environment:"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)"]
        #[doc = " * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#manualscaling)"]
        #[doc = ""]
        #[doc = " **Flexible environment**"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)"]
        #[doc = ""]
        #[doc = " *automatic scaling* in the flexible environment:"]
        #[doc = ""]
        #[doc = " * [`automatic_scaling.min_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.max_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.cool_down_period_sec`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.cpu_utilization.target_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = ""]
        #[doc = " *manual scaling* in the flexible environment:"]
        #[doc = ""]
        #[doc = " * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#manualscaling)"]
        pub async fn update_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Versions/UpdateVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing Version resource."]
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Versions/DeleteVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VersionsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VersionsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VersionsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod instances_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages instances of a version."]
    pub struct InstancesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InstancesClient<T>
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
        #[doc = " Lists the instances of a version."]
        #[doc = ""]
        #[doc = " Tip: To aggregate details about instances over time, see the"]
        #[doc = " [Stackdriver Monitoring API](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Instances/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets instance information."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Instances/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a running instance."]
        #[doc = ""]
        #[doc = " The instance might be automatically recreated based on the scaling settings"]
        #[doc = " of the version. For more information, see \"How Instances are Managed\""]
        #[doc = " ([standard environment](https://cloud.google.com/appengine/docs/standard/python/how-instances-are-managed) |"]
        #[doc = " [flexible environment](https://cloud.google.com/appengine/docs/flexible/python/how-instances-are-managed))."]
        #[doc = ""]
        #[doc = " To ensure that instances are not re-created and avoid getting billed, you"]
        #[doc = " can stop all instances within the target version by changing the serving"]
        #[doc = " status of the version to `STOPPED` with the"]
        #[doc = " [`apps.services.versions.patch`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions/patch)"]
        #[doc = " method."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Instances/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables debugging on a VM instance. This allows you to use the SSH"]
        #[doc = " command to connect to the virtual machine where the instance lives."]
        #[doc = " While in \"debug mode\", the instance continues to serve live traffic."]
        #[doc = " You should delete the instance when you are done debugging and then"]
        #[doc = " allow the system to take over and determine if another instance"]
        #[doc = " should be started."]
        #[doc = ""]
        #[doc = " Only applicable for instances in App Engine flexible environment."]
        pub async fn debug_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Instances/DebugInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for InstancesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for InstancesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "InstancesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod firewall_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Firewall resources are used to define a collection of access control rules"]
    #[doc = " for an Application. Each rule is defined with a position which specifies"]
    #[doc = " the rule's order in the sequence of rules, an IP range to be matched against"]
    #[doc = " requests, and an action to take upon matching requests."]
    #[doc = ""]
    #[doc = " Every request is evaluated against the Firewall rules in priority order."]
    #[doc = " Processesing stops at the first rule which matches the request's IP address."]
    #[doc = " A final rule always specifies an action that applies to all remaining"]
    #[doc = " IP addresses. The default final rule for a newly-created application will be"]
    #[doc = " set to \"allow\" if not otherwise specified by the user."]
    pub struct FirewallClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FirewallClient<T>
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
        #[doc = " Lists the firewall rules of an application."]
        pub async fn list_ingress_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIngressRulesRequest>,
        ) -> Result<tonic::Response<super::ListIngressRulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/ListIngressRules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Replaces the entire firewall ruleset in one bulk operation. This overrides"]
        #[doc = " and replaces the rules of an existing firewall with the new rules."]
        #[doc = ""]
        #[doc = " If the final rule does not match traffic with the '*' wildcard IP range,"]
        #[doc = " then an \"allow all\" rule is explicitly added to the end of the list."]
        pub async fn batch_update_ingress_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateIngressRulesRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateIngressRulesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/BatchUpdateIngressRules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a firewall rule for the application."]
        pub async fn create_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/CreateIngressRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified firewall rule."]
        pub async fn get_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/GetIngressRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified firewall rule."]
        pub async fn update_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/UpdateIngressRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified firewall rule."]
        pub async fn delete_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIngressRuleRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.Firewall/DeleteIngressRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FirewallClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FirewallClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FirewallClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod authorized_domains_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages domains a user is authorized to administer. To authorize use of a"]
    #[doc = " domain, verify ownership via"]
    #[doc = " [Webmaster Central](https://www.google.com/webmasters/verification/home)."]
    pub struct AuthorizedDomainsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AuthorizedDomainsClient<T>
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
        #[doc = " Lists all domains the user is authorized to administer."]
        pub async fn list_authorized_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizedDomainsRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizedDomainsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedDomains/ListAuthorizedDomains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AuthorizedDomainsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AuthorizedDomainsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AuthorizedDomainsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod authorized_certificates_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages SSL certificates a user is authorized to administer. A user can"]
    #[doc = " administer any SSL certificates applicable to their authorized domains."]
    pub struct AuthorizedCertificatesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AuthorizedCertificatesClient<T>
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
        #[doc = " Lists all SSL certificates the user is authorized to administer."]
        pub async fn list_authorized_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizedCertificatesRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizedCertificatesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedCertificates/ListAuthorizedCertificates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified SSL certificate."]
        pub async fn get_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedCertificates/GetAuthorizedCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Uploads the specified SSL certificate."]
        pub async fn create_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedCertificates/CreateAuthorizedCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified SSL certificate. To renew a certificate and maintain"]
        #[doc = " its existing domain mappings, update `certificate_data` with a new"]
        #[doc = " certificate. The new certificate must be applicable to the same domains as"]
        #[doc = " the original certificate. The certificate `display_name` may also be"]
        #[doc = " updated."]
        pub async fn update_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedCertificates/UpdateAuthorizedCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified SSL certificate."]
        pub async fn delete_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.AuthorizedCertificates/DeleteAuthorizedCertificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AuthorizedCertificatesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AuthorizedCertificatesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AuthorizedCertificatesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod domain_mappings_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages domains serving an application."]
    pub struct DomainMappingsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DomainMappingsClient<T>
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
        #[doc = " Lists the domain mappings on an application."]
        pub async fn list_domain_mappings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDomainMappingsRequest>,
        ) -> Result<tonic::Response<super::ListDomainMappingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.DomainMappings/ListDomainMappings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified domain mapping."]
        pub async fn get_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainMappingRequest>,
        ) -> Result<tonic::Response<super::DomainMapping>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.DomainMappings/GetDomainMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Maps a domain to an application. A user must be authorized to administer a"]
        #[doc = " domain in order to map it to an application. For a list of available"]
        #[doc = " authorized domains, see [`AuthorizedDomains.ListAuthorizedDomains`]()."]
        pub async fn create_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.DomainMappings/CreateDomainMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified domain mapping. To map an SSL certificate to a"]
        #[doc = " domain mapping, update `certificate_id` to point to an `AuthorizedCertificate`"]
        #[doc = " resource. A user must be authorized to administer the associated domain"]
        #[doc = " in order to update a `DomainMapping` resource."]
        pub async fn update_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.DomainMappings/UpdateDomainMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified domain mapping. A user must be authorized to"]
        #[doc = " administer the associated domain in order to delete a `DomainMapping`"]
        #[doc = " resource."]
        pub async fn delete_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1beta.DomainMappings/DeleteDomainMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DomainMappingsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DomainMappingsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DomainMappingsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod applications_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApplicationsServer."]
    #[async_trait]
    pub trait Applications: Send + Sync + 'static {
        #[doc = " Gets information about an application."]
        async fn get_application(
            &self,
            request: tonic::Request<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status>;
        #[doc = " Creates an App Engine application for a Google Cloud Platform project."]
        #[doc = " Required fields:"]
        #[doc = ""]
        #[doc = " * `id` - The ID of the target Cloud Platform project."]
        #[doc = " * *location* - The [region](https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located."]
        #[doc = ""]
        #[doc = " For more information about App Engine applications, see [Managing Projects, Applications, and Billing](https://cloud.google.com/appengine/docs/standard/python/console/)."]
        async fn create_application(
            &self,
            request: tonic::Request<super::CreateApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Updates the specified Application resource."]
        #[doc = " You can update the following fields:"]
        #[doc = ""]
        #[doc = " * `auth_domain` - Google authentication domain for controlling user access to the application."]
        #[doc = " * `default_cookie_expiration` - Cookie expiration policy for the application."]
        async fn update_application(
            &self,
            request: tonic::Request<super::UpdateApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Recreates the required App Engine features for the specified App Engine"]
        #[doc = " application, for example a Cloud Storage bucket or App Engine service"]
        #[doc = " account."]
        #[doc = " Use this method if you receive an error message about a missing feature,"]
        #[doc = " for example, *Error retrieving the App Engine service account*."]
        #[doc = " If you have deleted your App Engine service account, this will"]
        #[doc = " not be able to recreate it. Instead, you should attempt to use the"]
        #[doc = " IAM undelete API if possible at https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts/undelete?apix_params=%7B\"name\"%3A\"projects%2F-%2FserviceAccounts%2Funique_id\"%2C\"resource\"%3A%7B%7D%7D ."]
        #[doc = " If the deletion was recent, the numeric ID can be found in the Cloud"]
        #[doc = " Console Activity Log."]
        async fn repair_application(
            &self,
            request: tonic::Request<super::RepairApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages App Engine applications."]
    #[derive(Debug)]
    pub struct ApplicationsServer<T: Applications> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Applications> ApplicationsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ApplicationsServer<T>
    where
        T: Applications,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.Applications/GetApplication" => {
                    #[allow(non_camel_case_types)]
                    struct GetApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications> tonic::server::UnaryService<super::GetApplicationRequest>
                        for GetApplicationSvc<T>
                    {
                        type Response = super::Application;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Applications/CreateApplication" => {
                    #[allow(non_camel_case_types)]
                    struct CreateApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications>
                        tonic::server::UnaryService<super::CreateApplicationRequest>
                        for CreateApplicationSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Applications/UpdateApplication" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications>
                        tonic::server::UnaryService<super::UpdateApplicationRequest>
                        for UpdateApplicationSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Applications/RepairApplication" => {
                    #[allow(non_camel_case_types)]
                    struct RepairApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications>
                        tonic::server::UnaryService<super::RepairApplicationRequest>
                        for RepairApplicationSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepairApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).repair_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RepairApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Applications> Clone for ApplicationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Applications> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod services_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ServicesServer."]
    #[async_trait]
    pub trait Services: Send + Sync + 'static {
        #[doc = " Lists all the services in the application."]
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        #[doc = " Gets the current configuration of the specified service."]
        async fn get_service(
            &self,
            request: tonic::Request<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        #[doc = " Updates the configuration of the specified service."]
        async fn update_service(
            &self,
            request: tonic::Request<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Deletes the specified service and all enclosed versions."]
        async fn delete_service(
            &self,
            request: tonic::Request<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages services of an application."]
    #[derive(Debug)]
    pub struct ServicesServer<T: Services> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Services> ServicesServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ServicesServer<T>
    where
        T: Services,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.Services/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::ListServicesRequest> for ListServicesSvc<T> {
                        type Response = super::ListServicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_services(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServicesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Services/GetService" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::GetServiceRequest> for GetServiceSvc<T> {
                        type Response = super::Service;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Services/UpdateService" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::UpdateServiceRequest> for UpdateServiceSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Services/DeleteService" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::DeleteServiceRequest> for DeleteServiceSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Services> Clone for ServicesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Services> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod versions_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VersionsServer."]
    #[async_trait]
    pub trait Versions: Send + Sync + 'static {
        #[doc = " Lists the versions of a service."]
        async fn list_versions(
            &self,
            request: tonic::Request<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status>;
        #[doc = " Gets the specified Version resource."]
        #[doc = " By default, only a `BASIC_VIEW` will be returned."]
        #[doc = " Specify the `FULL_VIEW` parameter to get the full resource."]
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status>;
        #[doc = " Deploys code and resource files to a new version."]
        async fn create_version(
            &self,
            request: tonic::Request<super::CreateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Updates the specified Version resource."]
        #[doc = " You can specify the following fields depending on the App Engine"]
        #[doc = " environment and type of scaling that the version resource uses:"]
        #[doc = ""]
        #[doc = " **Standard environment**"]
        #[doc = ""]
        #[doc = " * [`instance_class`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.instance_class)"]
        #[doc = ""]
        #[doc = " *automatic scaling* in the standard environment:"]
        #[doc = ""]
        #[doc = " * [`automatic_scaling.min_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.max_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.max_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.min_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.target_cpu_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = " * [`automaticScaling.standard_scheduler_settings.target_throughput_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StandardSchedulerSettings)"]
        #[doc = ""]
        #[doc = " *basic scaling* or *manual scaling* in the standard environment:"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)"]
        #[doc = " * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#manualscaling)"]
        #[doc = ""]
        #[doc = " **Flexible environment**"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.serving_status)"]
        #[doc = ""]
        #[doc = " *automatic scaling* in the flexible environment:"]
        #[doc = ""]
        #[doc = " * [`automatic_scaling.min_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.max_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.cool_down_period_sec`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = " * [`automatic_scaling.cpu_utilization.target_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#Version.FIELDS.automatic_scaling)"]
        #[doc = ""]
        #[doc = " *manual scaling* in the flexible environment:"]
        #[doc = ""]
        #[doc = " * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#manualscaling)"]
        async fn update_version(
            &self,
            request: tonic::Request<super::UpdateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Deletes an existing Version resource."]
        async fn delete_version(
            &self,
            request: tonic::Request<super::DeleteVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages versions of a service."]
    #[derive(Debug)]
    pub struct VersionsServer<T: Versions> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Versions> VersionsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for VersionsServer<T>
    where
        T: Versions,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.Versions/ListVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListVersionsSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::ListVersionsRequest> for ListVersionsSvc<T> {
                        type Response = super::ListVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Versions/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::GetVersionRequest> for GetVersionSvc<T> {
                        type Response = super::Version;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Versions/CreateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::CreateVersionRequest> for CreateVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Versions/UpdateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::UpdateVersionRequest> for UpdateVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Versions/DeleteVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::DeleteVersionRequest> for DeleteVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Versions> Clone for VersionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Versions> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod instances_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InstancesServer."]
    #[async_trait]
    pub trait Instances: Send + Sync + 'static {
        #[doc = " Lists the instances of a version."]
        #[doc = ""]
        #[doc = " Tip: To aggregate details about instances over time, see the"]
        #[doc = " [Stackdriver Monitoring API](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets instance information."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Stops a running instance."]
        #[doc = ""]
        #[doc = " The instance might be automatically recreated based on the scaling settings"]
        #[doc = " of the version. For more information, see \"How Instances are Managed\""]
        #[doc = " ([standard environment](https://cloud.google.com/appengine/docs/standard/python/how-instances-are-managed) |"]
        #[doc = " [flexible environment](https://cloud.google.com/appengine/docs/flexible/python/how-instances-are-managed))."]
        #[doc = ""]
        #[doc = " To ensure that instances are not re-created and avoid getting billed, you"]
        #[doc = " can stop all instances within the target version by changing the serving"]
        #[doc = " status of the version to `STOPPED` with the"]
        #[doc = " [`apps.services.versions.patch`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions/patch)"]
        #[doc = " method."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Enables debugging on a VM instance. This allows you to use the SSH"]
        #[doc = " command to connect to the virtual machine where the instance lives."]
        #[doc = " While in \"debug mode\", the instance continues to serve live traffic."]
        #[doc = " You should delete the instance when you are done debugging and then"]
        #[doc = " allow the system to take over and determine if another instance"]
        #[doc = " should be started."]
        #[doc = ""]
        #[doc = " Only applicable for instances in App Engine flexible environment."]
        async fn debug_instance(
            &self,
            request: tonic::Request<super::DebugInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages instances of a version."]
    #[derive(Debug)]
    pub struct InstancesServer<T: Instances> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Instances> InstancesServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for InstancesServer<T>
    where
        T: Instances,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.Instances/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_instances(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstancesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Instances/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::GetInstanceRequest> for GetInstanceSvc<T> {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Instances/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Instances/DebugInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DebugInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::DebugInstanceRequest>
                        for DebugInstanceSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).debug_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DebugInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Instances> Clone for InstancesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Instances> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod firewall_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FirewallServer."]
    #[async_trait]
    pub trait Firewall: Send + Sync + 'static {
        #[doc = " Lists the firewall rules of an application."]
        async fn list_ingress_rules(
            &self,
            request: tonic::Request<super::ListIngressRulesRequest>,
        ) -> Result<tonic::Response<super::ListIngressRulesResponse>, tonic::Status>;
        #[doc = " Replaces the entire firewall ruleset in one bulk operation. This overrides"]
        #[doc = " and replaces the rules of an existing firewall with the new rules."]
        #[doc = ""]
        #[doc = " If the final rule does not match traffic with the '*' wildcard IP range,"]
        #[doc = " then an \"allow all\" rule is explicitly added to the end of the list."]
        async fn batch_update_ingress_rules(
            &self,
            request: tonic::Request<super::BatchUpdateIngressRulesRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateIngressRulesResponse>, tonic::Status>;
        #[doc = " Creates a firewall rule for the application."]
        async fn create_ingress_rule(
            &self,
            request: tonic::Request<super::CreateIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status>;
        #[doc = " Gets the specified firewall rule."]
        async fn get_ingress_rule(
            &self,
            request: tonic::Request<super::GetIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status>;
        #[doc = " Updates the specified firewall rule."]
        async fn update_ingress_rule(
            &self,
            request: tonic::Request<super::UpdateIngressRuleRequest>,
        ) -> Result<tonic::Response<super::FirewallRule>, tonic::Status>;
        #[doc = " Deletes the specified firewall rule."]
        async fn delete_ingress_rule(
            &self,
            request: tonic::Request<super::DeleteIngressRuleRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Firewall resources are used to define a collection of access control rules"]
    #[doc = " for an Application. Each rule is defined with a position which specifies"]
    #[doc = " the rule's order in the sequence of rules, an IP range to be matched against"]
    #[doc = " requests, and an action to take upon matching requests."]
    #[doc = ""]
    #[doc = " Every request is evaluated against the Firewall rules in priority order."]
    #[doc = " Processesing stops at the first rule which matches the request's IP address."]
    #[doc = " A final rule always specifies an action that applies to all remaining"]
    #[doc = " IP addresses. The default final rule for a newly-created application will be"]
    #[doc = " set to \"allow\" if not otherwise specified by the user."]
    #[derive(Debug)]
    pub struct FirewallServer<T: Firewall> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Firewall> FirewallServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for FirewallServer<T>
    where
        T: Firewall,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.Firewall/ListIngressRules" => {
                    #[allow(non_camel_case_types)]
                    struct ListIngressRulesSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall> tonic::server::UnaryService<super::ListIngressRulesRequest>
                        for ListIngressRulesSvc<T>
                    {
                        type Response = super::ListIngressRulesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListIngressRulesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_ingress_rules(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListIngressRulesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Firewall/BatchUpdateIngressRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateIngressRulesSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall>
                        tonic::server::UnaryService<super::BatchUpdateIngressRulesRequest>
                        for BatchUpdateIngressRulesSvc<T>
                    {
                        type Response = super::BatchUpdateIngressRulesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateIngressRulesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).batch_update_ingress_rules(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdateIngressRulesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Firewall/CreateIngressRule" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIngressRuleSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall> tonic::server::UnaryService<super::CreateIngressRuleRequest>
                        for CreateIngressRuleSvc<T>
                    {
                        type Response = super::FirewallRule;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateIngressRuleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_ingress_rule(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateIngressRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Firewall/GetIngressRule" => {
                    #[allow(non_camel_case_types)]
                    struct GetIngressRuleSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall> tonic::server::UnaryService<super::GetIngressRuleRequest>
                        for GetIngressRuleSvc<T>
                    {
                        type Response = super::FirewallRule;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIngressRuleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_ingress_rule(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIngressRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Firewall/UpdateIngressRule" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateIngressRuleSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall> tonic::server::UnaryService<super::UpdateIngressRuleRequest>
                        for UpdateIngressRuleSvc<T>
                    {
                        type Response = super::FirewallRule;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateIngressRuleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_ingress_rule(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateIngressRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.Firewall/DeleteIngressRule" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteIngressRuleSvc<T: Firewall>(pub Arc<T>);
                    impl<T: Firewall> tonic::server::UnaryService<super::DeleteIngressRuleRequest>
                        for DeleteIngressRuleSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteIngressRuleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_ingress_rule(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteIngressRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Firewall> Clone for FirewallServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Firewall> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod authorized_domains_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AuthorizedDomainsServer."]
    #[async_trait]
    pub trait AuthorizedDomains: Send + Sync + 'static {
        #[doc = " Lists all domains the user is authorized to administer."]
        async fn list_authorized_domains(
            &self,
            request: tonic::Request<super::ListAuthorizedDomainsRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizedDomainsResponse>, tonic::Status>;
    }
    #[doc = " Manages domains a user is authorized to administer. To authorize use of a"]
    #[doc = " domain, verify ownership via"]
    #[doc = " [Webmaster Central](https://www.google.com/webmasters/verification/home)."]
    #[derive(Debug)]
    pub struct AuthorizedDomainsServer<T: AuthorizedDomains> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AuthorizedDomains> AuthorizedDomainsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for AuthorizedDomainsServer<T>
    where
        T: AuthorizedDomains,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.AuthorizedDomains/ListAuthorizedDomains" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuthorizedDomainsSvc<T: AuthorizedDomains>(pub Arc<T>);
                    impl<T: AuthorizedDomains>
                        tonic::server::UnaryService<super::ListAuthorizedDomainsRequest>
                        for ListAuthorizedDomainsSvc<T>
                    {
                        type Response = super::ListAuthorizedDomainsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuthorizedDomainsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).list_authorized_domains(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListAuthorizedDomainsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AuthorizedDomains> Clone for AuthorizedDomainsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AuthorizedDomains> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod authorized_certificates_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AuthorizedCertificatesServer."]
    #[async_trait]
    pub trait AuthorizedCertificates: Send + Sync + 'static {
        #[doc = " Lists all SSL certificates the user is authorized to administer."]
        async fn list_authorized_certificates(
            &self,
            request: tonic::Request<super::ListAuthorizedCertificatesRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizedCertificatesResponse>, tonic::Status>;
        #[doc = " Gets the specified SSL certificate."]
        async fn get_authorized_certificate(
            &self,
            request: tonic::Request<super::GetAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status>;
        #[doc = " Uploads the specified SSL certificate."]
        async fn create_authorized_certificate(
            &self,
            request: tonic::Request<super::CreateAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status>;
        #[doc = " Updates the specified SSL certificate. To renew a certificate and maintain"]
        #[doc = " its existing domain mappings, update `certificate_data` with a new"]
        #[doc = " certificate. The new certificate must be applicable to the same domains as"]
        #[doc = " the original certificate. The certificate `display_name` may also be"]
        #[doc = " updated."]
        async fn update_authorized_certificate(
            &self,
            request: tonic::Request<super::UpdateAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<super::AuthorizedCertificate>, tonic::Status>;
        #[doc = " Deletes the specified SSL certificate."]
        async fn delete_authorized_certificate(
            &self,
            request: tonic::Request<super::DeleteAuthorizedCertificateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Manages SSL certificates a user is authorized to administer. A user can"]
    #[doc = " administer any SSL certificates applicable to their authorized domains."]
    #[derive(Debug)]
    pub struct AuthorizedCertificatesServer<T: AuthorizedCertificates> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AuthorizedCertificates> AuthorizedCertificatesServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for AuthorizedCertificatesServer<T>
    where
        T: AuthorizedCertificates,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.AuthorizedCertificates/ListAuthorizedCertificates" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuthorizedCertificatesSvc<T: AuthorizedCertificates>(pub Arc<T>);
                    impl<T: AuthorizedCertificates>
                        tonic::server::UnaryService<super::ListAuthorizedCertificatesRequest>
                        for ListAuthorizedCertificatesSvc<T>
                    {
                        type Response = super::ListAuthorizedCertificatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuthorizedCertificatesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).list_authorized_certificates(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListAuthorizedCertificatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.AuthorizedCertificates/GetAuthorizedCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct GetAuthorizedCertificateSvc<T: AuthorizedCertificates>(pub Arc<T>);
                    impl<T: AuthorizedCertificates>
                        tonic::server::UnaryService<super::GetAuthorizedCertificateRequest>
                        for GetAuthorizedCertificateSvc<T>
                    {
                        type Response = super::AuthorizedCertificate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAuthorizedCertificateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_authorized_certificate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAuthorizedCertificateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.AuthorizedCertificates/CreateAuthorizedCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthorizedCertificateSvc<T: AuthorizedCertificates>(pub Arc<T>);
                    impl<T: AuthorizedCertificates>
                        tonic::server::UnaryService<super::CreateAuthorizedCertificateRequest>
                        for CreateAuthorizedCertificateSvc<T>
                    {
                        type Response = super::AuthorizedCertificate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAuthorizedCertificateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_authorized_certificate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateAuthorizedCertificateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.AuthorizedCertificates/UpdateAuthorizedCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAuthorizedCertificateSvc<T: AuthorizedCertificates>(pub Arc<T>);
                    impl<T: AuthorizedCertificates>
                        tonic::server::UnaryService<super::UpdateAuthorizedCertificateRequest>
                        for UpdateAuthorizedCertificateSvc<T>
                    {
                        type Response = super::AuthorizedCertificate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAuthorizedCertificateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_authorized_certificate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateAuthorizedCertificateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.AuthorizedCertificates/DeleteAuthorizedCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAuthorizedCertificateSvc<T: AuthorizedCertificates>(pub Arc<T>);
                    impl<T: AuthorizedCertificates>
                        tonic::server::UnaryService<super::DeleteAuthorizedCertificateRequest>
                        for DeleteAuthorizedCertificateSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAuthorizedCertificateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_authorized_certificate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAuthorizedCertificateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AuthorizedCertificates> Clone for AuthorizedCertificatesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AuthorizedCertificates> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod domain_mappings_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DomainMappingsServer."]
    #[async_trait]
    pub trait DomainMappings: Send + Sync + 'static {
        #[doc = " Lists the domain mappings on an application."]
        async fn list_domain_mappings(
            &self,
            request: tonic::Request<super::ListDomainMappingsRequest>,
        ) -> Result<tonic::Response<super::ListDomainMappingsResponse>, tonic::Status>;
        #[doc = " Gets the specified domain mapping."]
        async fn get_domain_mapping(
            &self,
            request: tonic::Request<super::GetDomainMappingRequest>,
        ) -> Result<tonic::Response<super::DomainMapping>, tonic::Status>;
        #[doc = " Maps a domain to an application. A user must be authorized to administer a"]
        #[doc = " domain in order to map it to an application. For a list of available"]
        #[doc = " authorized domains, see [`AuthorizedDomains.ListAuthorizedDomains`]()."]
        async fn create_domain_mapping(
            &self,
            request: tonic::Request<super::CreateDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Updates the specified domain mapping. To map an SSL certificate to a"]
        #[doc = " domain mapping, update `certificate_id` to point to an `AuthorizedCertificate`"]
        #[doc = " resource. A user must be authorized to administer the associated domain"]
        #[doc = " in order to update a `DomainMapping` resource."]
        async fn update_domain_mapping(
            &self,
            request: tonic::Request<super::UpdateDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Deletes the specified domain mapping. A user must be authorized to"]
        #[doc = " administer the associated domain in order to delete a `DomainMapping`"]
        #[doc = " resource."]
        async fn delete_domain_mapping(
            &self,
            request: tonic::Request<super::DeleteDomainMappingRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages domains serving an application."]
    #[derive(Debug)]
    pub struct DomainMappingsServer<T: DomainMappings> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DomainMappings> DomainMappingsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DomainMappingsServer<T>
    where
        T: DomainMappings,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.appengine.v1beta.DomainMappings/ListDomainMappings" => {
                    #[allow(non_camel_case_types)]
                    struct ListDomainMappingsSvc<T: DomainMappings>(pub Arc<T>);
                    impl<T: DomainMappings>
                        tonic::server::UnaryService<super::ListDomainMappingsRequest>
                        for ListDomainMappingsSvc<T>
                    {
                        type Response = super::ListDomainMappingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDomainMappingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_domain_mappings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDomainMappingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.DomainMappings/GetDomainMapping" => {
                    #[allow(non_camel_case_types)]
                    struct GetDomainMappingSvc<T: DomainMappings>(pub Arc<T>);
                    impl<T: DomainMappings>
                        tonic::server::UnaryService<super::GetDomainMappingRequest>
                        for GetDomainMappingSvc<T>
                    {
                        type Response = super::DomainMapping;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDomainMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_domain_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDomainMappingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.DomainMappings/CreateDomainMapping" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDomainMappingSvc<T: DomainMappings>(pub Arc<T>);
                    impl<T: DomainMappings>
                        tonic::server::UnaryService<super::CreateDomainMappingRequest>
                        for CreateDomainMappingSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDomainMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_domain_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDomainMappingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.DomainMappings/UpdateDomainMapping" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDomainMappingSvc<T: DomainMappings>(pub Arc<T>);
                    impl<T: DomainMappings>
                        tonic::server::UnaryService<super::UpdateDomainMappingRequest>
                        for UpdateDomainMappingSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDomainMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_domain_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDomainMappingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.appengine.v1beta.DomainMappings/DeleteDomainMapping" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDomainMappingSvc<T: DomainMappings>(pub Arc<T>);
                    impl<T: DomainMappings>
                        tonic::server::UnaryService<super::DeleteDomainMappingRequest>
                        for DeleteDomainMappingSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDomainMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_domain_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteDomainMappingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DomainMappings> Clone for DomainMappingsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DomainMappings> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
/// App Engine admin service audit log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[prost(oneof = "audit_data::Method", tags = "1, 2")]
    pub method: ::std::option::Option<audit_data::Method>,
}
pub mod audit_data {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// Detailed information about UpdateService call.
        #[prost(message, tag = "1")]
        UpdateService(super::UpdateServiceMethod),
        /// Detailed information about CreateVersion call.
        #[prost(message, tag = "2")]
        CreateVersion(super::CreateVersionMethod),
    }
}
/// Detailed information about UpdateService call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceMethod {
    /// Update service request.
    #[prost(message, optional, tag = "1")]
    pub request: ::std::option::Option<UpdateServiceRequest>,
}
/// Detailed information about CreateVersion call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionMethod {
    /// Create version request.
    #[prost(message, optional, tag = "1")]
    pub request: ::std::option::Option<CreateVersionRequest>,
}
/// Metadata for the given [google.longrunning.Operation][google.longrunning.Operation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1Beta {
    /// API method that initiated this operation. Example:
    /// `google.appengine.v1beta.Versions.CreateVersion`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    /// Time that this operation was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "2")]
    pub insert_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time that this operation completed.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// User who requested this operation.
    ///
    /// @OutputOnly
    #[prost(string, tag = "4")]
    pub user: std::string::String,
    /// Name of the resource that this operation is acting on. Example:
    /// `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "5")]
    pub target: std::string::String,
    /// Ephemeral message that may change every time the operation is polled.
    /// @OutputOnly
    #[prost(string, tag = "6")]
    pub ephemeral_message: std::string::String,
    /// Durable messages that persist on every operation poll.
    /// @OutputOnly
    #[prost(string, repeated, tag = "7")]
    pub warning: ::std::vec::Vec<std::string::String>,
    /// Metadata specific to the type of operation in progress.
    /// @OutputOnly
    #[prost(oneof = "operation_metadata_v1_beta::MethodMetadata", tags = "8")]
    pub method_metadata: ::std::option::Option<operation_metadata_v1_beta::MethodMetadata>,
}
pub mod operation_metadata_v1_beta {
    /// Metadata specific to the type of operation in progress.
    /// @OutputOnly
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MethodMetadata {
        #[prost(message, tag = "8")]
        CreateVersionMetadata(super::CreateVersionMetadataV1Beta),
    }
}
/// Metadata for the given [google.longrunning.Operation][google.longrunning.Operation] during a
/// [google.appengine.v1beta.CreateVersionRequest][google.appengine.v1beta.CreateVersionRequest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionMetadataV1Beta {
    /// The Cloud Build ID if one was created as part of the version create.
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub cloud_build_id: std::string::String,
}
