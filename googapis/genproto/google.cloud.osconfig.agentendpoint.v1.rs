// OS Config Inventory is a service for collecting and reporting operating
// system and package information on VM instances.

/// The inventory details of a VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    /// Base level operating system information for the VM.
    #[prost(message, optional, tag = "1")]
    pub os_info: ::core::option::Option<inventory::OsInfo>,
    /// A list of installed packages currently on the VM.
    #[prost(message, repeated, tag = "2")]
    pub installed_packages: ::prost::alloc::vec::Vec<inventory::SoftwarePackage>,
    /// A list of software updates available for the VM as reported by the update
    /// managers.
    #[prost(message, repeated, tag = "3")]
    pub available_packages: ::prost::alloc::vec::Vec<inventory::SoftwarePackage>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    /// Operating system information for the VM.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsInfo {
        /// The VM hostname.
        #[prost(string, tag = "1")]
        pub hostname: ::prost::alloc::string::String,
        /// The operating system long name.
        /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
        /// Datacenter'.
        #[prost(string, tag = "2")]
        pub long_name: ::prost::alloc::string::String,
        /// The operating system short name.
        /// For example, 'windows' or 'debian'.
        #[prost(string, tag = "3")]
        pub short_name: ::prost::alloc::string::String,
        /// The version of the operating system.
        #[prost(string, tag = "4")]
        pub version: ::prost::alloc::string::String,
        /// The system architecture of the operating system.
        #[prost(string, tag = "5")]
        pub architecture: ::prost::alloc::string::String,
        /// The kernel version of the operating system.
        #[prost(string, tag = "6")]
        pub kernel_version: ::prost::alloc::string::String,
        /// The kernel release of the operating system.
        #[prost(string, tag = "7")]
        pub kernel_release: ::prost::alloc::string::String,
        /// The current version of the OS Config agent running on the VM.
        #[prost(string, tag = "8")]
        pub osconfig_agent_version: ::prost::alloc::string::String,
    }
    /// Software package information of the operating system.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SoftwarePackage {
        /// Information about the different types of software packages.
        #[prost(
            oneof = "software_package::Details",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
        )]
        pub details: ::core::option::Option<software_package::Details>,
    }
    /// Nested message and enum types in `SoftwarePackage`.
    pub mod software_package {
        /// Information about the different types of software packages.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Yum package info.
            /// For details about the yum package manager, see
            /// https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum.
            #[prost(message, tag = "1")]
            YumPackage(super::VersionedPackage),
            /// Details of an APT package.
            /// For details about the apt package manager, see
            /// https://wiki.debian.org/Apt.
            #[prost(message, tag = "2")]
            AptPackage(super::VersionedPackage),
            /// Details of a Zypper package.
            /// For details about the Zypper package manager, see
            /// https://en.opensuse.org/SDB:Zypper_manual.
            #[prost(message, tag = "3")]
            ZypperPackage(super::VersionedPackage),
            /// Details of a Googet package.
            ///  For details about the googet package manager, see
            ///  https://github.com/google/googet.
            #[prost(message, tag = "4")]
            GoogetPackage(super::VersionedPackage),
            /// Details of a Zypper patch.
            /// For details about the Zypper package manager, see
            /// https://en.opensuse.org/SDB:Zypper_manual.
            #[prost(message, tag = "5")]
            ZypperPatch(super::ZypperPatch),
            /// Details of a Windows Update package.
            /// See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for
            /// information about Windows Update.
            #[prost(message, tag = "6")]
            WuaPackage(super::WindowsUpdatePackage),
            /// Details of a Windows Quick Fix engineering package.
            /// See
            /// https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
            /// for info in Windows Quick Fix Engineering.
            #[prost(message, tag = "7")]
            QfePackage(super::WindowsQuickFixEngineeringPackage),
            /// Details of a COS package.
            #[prost(message, tag = "8")]
            CosPackage(super::VersionedPackage),
            /// Details of Windows Application.
            #[prost(message, tag = "9")]
            WindowsApplication(super::WindowsApplication),
        }
    }
    /// Information related to the a standard versioned package.  This includes
    /// package info for APT, Yum, Zypper, and Googet package managers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionedPackage {
        /// The name of the package.
        #[prost(string, tag = "1")]
        pub package_name: ::prost::alloc::string::String,
        /// The system architecture this package is intended for.
        #[prost(string, tag = "2")]
        pub architecture: ::prost::alloc::string::String,
        /// The version of the package.
        #[prost(string, tag = "3")]
        pub version: ::prost::alloc::string::String,
    }
    /// Information related to a Quick Fix Engineering package.
    /// Fields are taken from Windows QuickFixEngineering Interface and match
    /// the source names:
    /// https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsQuickFixEngineeringPackage {
        /// A short textual description of the QFE update.
        #[prost(string, tag = "1")]
        pub caption: ::prost::alloc::string::String,
        /// A textual description of the QFE update.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Unique identifier associated with a particular QFE update.
        #[prost(string, tag = "3")]
        pub hot_fix_id: ::prost::alloc::string::String,
        /// Date that the QFE update was installed.  Mapped from installed_on field.
        #[prost(message, optional, tag = "4")]
        pub install_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Details related to a Zypper Patch.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ZypperPatch {
        /// The name of the patch.
        #[prost(string, tag = "1")]
        pub patch_name: ::prost::alloc::string::String,
        /// The category of the patch.
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
        /// The severity specified for this patch
        #[prost(string, tag = "3")]
        pub severity: ::prost::alloc::string::String,
        /// Any summary information provided about this patch.
        #[prost(string, tag = "4")]
        pub summary: ::prost::alloc::string::String,
    }
    /// Details related to a Windows Update package.
    /// Field data and names are taken from Windows Update API IUpdate Interface:
    /// https://docs.microsoft.com/en-us/windows/win32/api/_wua/
    /// Descriptive fields like title, and description are localized based on
    /// the locale of the VM being updated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsUpdatePackage {
        /// The localized title of the update package.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The localized description of the update package.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The categories that are associated with this update package.
        #[prost(message, repeated, tag = "3")]
        pub categories: ::prost::alloc::vec::Vec<windows_update_package::WindowsUpdateCategory>,
        /// A collection of Microsoft Knowledge Base article IDs that are associated
        /// with the update package.
        #[prost(string, repeated, tag = "4")]
        pub kb_article_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A hyperlink to the language-specific support information for the update.
        #[prost(string, tag = "5")]
        pub support_url: ::prost::alloc::string::String,
        /// A collection of URLs that provide more information about the update
        /// package.
        #[prost(string, repeated, tag = "6")]
        pub more_info_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Gets the identifier of an update package.  Stays the same across
        /// revisions.
        #[prost(string, tag = "7")]
        pub update_id: ::prost::alloc::string::String,
        /// The revision number of this update package.
        #[prost(int32, tag = "8")]
        pub revision_number: i32,
        /// The last published date of the update, in (UTC) date and time.
        #[prost(message, optional, tag = "9")]
        pub last_deployment_change_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `WindowsUpdatePackage`.
    pub mod windows_update_package {
        /// Categories specified by the Windows Update.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WindowsUpdateCategory {
            /// The identifier of the windows update category.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// The name of the windows update category.
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
        }
    }
    /// Details about Windows Application - based on Windows Registry.
    /// All fields in this message are taken from:
    /// https://docs.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsApplication {
        /// DisplayName field from Windows Registry.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// DisplayVersion field from Windows Registry.
        #[prost(string, tag = "2")]
        pub display_version: ::prost::alloc::string::String,
        /// Publisher field from Windows Registry.
        #[prost(string, tag = "3")]
        pub publisher: ::prost::alloc::string::String,
        /// Installation date field from Windows Registry.
        #[prost(message, optional, tag = "4")]
        pub install_date: ::core::option::Option<super::super::super::super::super::r#type::Date>,
        /// HelpLink field from Windows Registry.
        #[prost(string, tag = "5")]
        pub help_link: ::prost::alloc::string::String,
    }
}
/// Step performed by the OS Config agent for configuring an `OSPolicyResource`
/// to its desired state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyResourceConfigStep {
    /// Configuration step type.
    #[prost(enumeration = "os_policy_resource_config_step::Type", tag = "1")]
    pub r#type: i32,
    /// Outcome of the configuration step.
    #[prost(enumeration = "os_policy_resource_config_step::Outcome", tag = "2")]
    pub outcome: i32,
    /// An error message recorded during the execution of this step.
    /// Only populated when outcome is FAILED.
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OSPolicyResourceConfigStep`.
pub mod os_policy_resource_config_step {
    /// Supported configuration step types
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Validation to detect resource conflicts, schema errors, etc.
        Validation = 1,
        /// Check the current desired state status of the resource.
        DesiredStateCheck = 2,
        /// Enforce the desired state for a resource that is not in desired state.
        DesiredStateEnforcement = 3,
        /// Re-check desired state status for a resource after enforcement of all
        /// resources in the current configuration run.
        ///
        /// This step is used to determine the final desired state status for the
        /// resource. It accounts for any resources that might have drifted from
        /// their desired state due to side effects from configuring other resources
        /// during the current configuration run.
        DesiredStateCheckPostEnforcement = 4,
    }
    /// Supported outcomes for a configuration step.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Outcome {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The step succeeded.
        Succeeded = 1,
        /// The step failed.
        Failed = 2,
    }
}
/// Compliance data for an OS policy resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyResourceCompliance {
    /// The id of the OS policy resource.
    #[prost(string, tag = "1")]
    pub os_policy_resource_id: ::prost::alloc::string::String,
    /// Ordered list of configuration steps taken by the agent for the OS policy
    /// resource.
    #[prost(message, repeated, tag = "2")]
    pub config_steps: ::prost::alloc::vec::Vec<OsPolicyResourceConfigStep>,
    /// Compliance state of the OS policy resource.
    #[prost(enumeration = "OsPolicyComplianceState", tag = "3")]
    pub state: i32,
    /// Resource specific output.
    #[prost(oneof = "os_policy_resource_compliance::Output", tags = "4")]
    pub output: ::core::option::Option<os_policy_resource_compliance::Output>,
}
/// Nested message and enum types in `OSPolicyResourceCompliance`.
pub mod os_policy_resource_compliance {
    /// ExecResource specific output.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecResourceOutput {
        /// Output from Enforcement phase output file (if run).
        /// Output size is limited to 100K bytes.
        #[prost(bytes = "vec", tag = "2")]
        pub enforcement_output: ::prost::alloc::vec::Vec<u8>,
    }
    /// Resource specific output.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// ExecResource specific output.
        #[prost(message, tag = "4")]
        ExecResourceOutput(ExecResourceOutput),
    }
}
/// Supported OSPolicy compliance states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OsPolicyComplianceState {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// Compliant state.
    Compliant = 1,
    /// Non-compliant state
    NonCompliant = 2,
    /// Unknown compliance state.
    Unknown = 3,
    /// No applicable OS policies were found for the instance.
    /// This state is only applicable to the instance.
    NoOsPoliciesApplicable = 4,
}
/// An OS policy defines the desired state configuration for an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicy {}
/// Nested message and enum types in `OSPolicy`.
pub mod os_policy {
    /// An OS policy resource is used to define the desired state configuration
    /// and provides a specific functionality like installing/removing packages,
    /// executing a script etc.
    ///
    /// The system ensures that resources are always in their desired state by
    /// taking necessary actions if they have drifted from their desired state.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// Required. The id of the resource with the following restrictions:
        ///
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the OS policy.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Resource type.
        #[prost(oneof = "resource::ResourceType", tags = "2, 3, 4, 5")]
        pub resource_type: ::core::option::Option<resource::ResourceType>,
    }
    /// Nested message and enum types in `Resource`.
    pub mod resource {
        /// A remote or local file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct File {
            /// Defaults to false. When false, files are subject to validations
            /// based on the file type:
            ///
            /// Remote: A checksum must be specified.
            /// Cloud Storage: An object generation number must be specified.
            #[prost(bool, tag = "4")]
            pub allow_insecure: bool,
            /// A specific type of file.
            #[prost(oneof = "file::Type", tags = "1, 2, 3")]
            pub r#type: ::core::option::Option<file::Type>,
        }
        /// Nested message and enum types in `File`.
        pub mod file {
            /// Specifies a file available via some URI.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Remote {
                /// Required. URI from which to fetch the object. It should contain both
                /// the protocol and path following the format `{protocol}://{location}`.
                #[prost(string, tag = "1")]
                pub uri: ::prost::alloc::string::String,
                /// SHA256 checksum of the remote file.
                #[prost(string, tag = "2")]
                pub sha256_checksum: ::prost::alloc::string::String,
            }
            /// Specifies a file available as a Cloud Storage Object.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Gcs {
                /// Required. Bucket of the Cloud Storage object.
                #[prost(string, tag = "1")]
                pub bucket: ::prost::alloc::string::String,
                /// Required. Name of the Cloud Storage object.
                #[prost(string, tag = "2")]
                pub object: ::prost::alloc::string::String,
                /// Generation number of the Cloud Storage object.
                #[prost(int64, tag = "3")]
                pub generation: i64,
            }
            /// A specific type of file.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// A generic remote file.
                #[prost(message, tag = "1")]
                Remote(Remote),
                /// A Cloud Storage object.
                #[prost(message, tag = "2")]
                Gcs(Gcs),
                /// A local path to use.
                #[prost(string, tag = "3")]
                LocalPath(::prost::alloc::string::String),
            }
        }
        /// A resource that manages a system package.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PackageResource {
            /// Required. The desired state the agent should maintain for this package.
            /// The default is to ensure the package is installed.
            #[prost(enumeration = "package_resource::DesiredState", tag = "1")]
            pub desired_state: i32,
            /// A system package.
            #[prost(
                oneof = "package_resource::SystemPackage",
                tags = "2, 3, 4, 5, 6, 7, 8"
            )]
            pub system_package: ::core::option::Option<package_resource::SystemPackage>,
        }
        /// Nested message and enum types in `PackageResource`.
        pub mod package_resource {
            /// A deb package file. dpkg packages only support INSTALLED state.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Deb {
                /// Required. A deb package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// install when false: `dpkg -i package`
                /// install when true: `apt-get update && apt-get -y install
                /// package.deb`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by APT.
            /// install: `apt-get update && apt-get -y install [name]`
            /// remove: `apt-get -y remove [name]`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Apt {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An RPM package file. RPM packages only support INSTALLED state.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Rpm {
                /// Required. An rpm package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// install when false: `rpm --upgrade --replacepkgs package.rpm`
                /// install when true: `yum -y install package.rpm` or
                /// `zypper -y install package.rpm`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by YUM.
            /// install: `yum -y install package`
            /// remove: `yum -y remove package`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Yum {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by Zypper.
            /// install: `zypper -y install package`
            /// remove: `zypper -y rm package`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Zypper {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by GooGet.
            /// install: `googet -noconfirm install package`
            /// remove: `googet -noconfirm remove package`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooGet {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An MSI package. MSI packages only support INSTALLED state.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Msi {
                /// Required. The MSI package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Additional properties to use during installation.
                /// This should be in the format of Property=Setting.
                /// Appended to the defaults of "ACTION=INSTALL
                /// REBOOT=ReallySuppress".
                #[prost(string, repeated, tag = "2")]
                pub properties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// The desired state that the OS Config agent maintains on the VM.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure that the package is installed.
                Installed = 1,
                /// The agent ensures that the package is not installed and
                /// uninstalls it if detected.
                Removed = 2,
            }
            /// A system package.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum SystemPackage {
                /// A package managed by Apt.
                #[prost(message, tag = "2")]
                Apt(Apt),
                /// A deb package file.
                #[prost(message, tag = "3")]
                Deb(Deb),
                /// A package managed by YUM.
                #[prost(message, tag = "4")]
                Yum(Yum),
                /// A package managed by Zypper.
                #[prost(message, tag = "5")]
                Zypper(Zypper),
                /// An rpm package file.
                #[prost(message, tag = "6")]
                Rpm(Rpm),
                /// A package managed by GooGet.
                #[prost(message, tag = "7")]
                Googet(GooGet),
                /// An MSI package.
                #[prost(message, tag = "8")]
                Msi(Msi),
            }
        }
        /// A resource that manages a package repository.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RepositoryResource {
            /// A specific type of repository.
            #[prost(oneof = "repository_resource::Repository", tags = "1, 2, 3, 4")]
            pub repository: ::core::option::Option<repository_resource::Repository>,
        }
        /// Nested message and enum types in `RepositoryResource`.
        pub mod repository_resource {
            /// Represents a single apt package repository. These will be added to
            /// a repo file that will be managed at
            /// /etc/apt/sources.list.d/google_osconfig.list.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AptRepository {
                /// Required. Type of archive files in this repository. The default
                /// behavior is DEB.
                #[prost(enumeration = "apt_repository::ArchiveType", tag = "1")]
                pub archive_type: i32,
                /// Required. URI for this repository.
                #[prost(string, tag = "2")]
                pub uri: ::prost::alloc::string::String,
                /// Required. Distribution of this repository.
                #[prost(string, tag = "3")]
                pub distribution: ::prost::alloc::string::String,
                /// Required. List of components for this repository. Must contain at
                /// least one item.
                #[prost(string, repeated, tag = "4")]
                pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// URI of the key file for this repository. The agent maintains a
                /// keyring at /etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg.
                #[prost(string, tag = "5")]
                pub gpg_key: ::prost::alloc::string::String,
            }
            /// Nested message and enum types in `AptRepository`.
            pub mod apt_repository {
                /// Type of archive.
                #[derive(
                    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
                )]
                #[repr(i32)]
                pub enum ArchiveType {
                    /// Unspecified is invalid.
                    Unspecified = 0,
                    /// Deb indicates that the archive contains binary files.
                    Deb = 1,
                    /// Deb-src indicates that the archive contains source files.
                    DebSrc = 2,
                }
            }
            /// Represents a single yum package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/yum.repos.d/google_osconfig.repo`.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct YumRepository {
                /// Required. A one word, unique name for this repository. This is  the
                /// `repo id` in the yum config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for resource conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a single zypper package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/zypp/repos.d/google_osconfig.repo`.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ZypperRepository {
                /// Required. A one word, unique name for this repository. This is the
                /// `repo id` in the zypper config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for GuestPolicy conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a Goo package repository. These are added to a repo file
            /// that is managed at
            /// `C:/ProgramData/GooGet/repos/google_osconfig.repo`.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooRepository {
                /// Required. The name of the repository.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                /// Required. The url of the repository.
                #[prost(string, tag = "2")]
                pub url: ::prost::alloc::string::String,
            }
            /// A specific type of repository.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Repository {
                /// An Apt Repository.
                #[prost(message, tag = "1")]
                Apt(AptRepository),
                /// A Yum Repository.
                #[prost(message, tag = "2")]
                Yum(YumRepository),
                /// A Zypper Repository.
                #[prost(message, tag = "3")]
                Zypper(ZypperRepository),
                /// A Goo Repository.
                #[prost(message, tag = "4")]
                Goo(GooRepository),
            }
        }
        /// A resource that contains custom validation and enforcement steps.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecResource {
            /// Required. What to run to validate this resource is in the desired
            /// state. An exit code of 100 indicates "in desired state", and exit code
            /// of 101 indicates "not in desired state". Any other exit code indicates
            /// a failure running validate.
            #[prost(message, optional, tag = "1")]
            pub validate: ::core::option::Option<exec_resource::Exec>,
            /// What to run to bring this resource into the desired state.
            /// A exit code of 100 indicates "success", any other exit code idicates a
            /// failure running enforce.
            #[prost(message, optional, tag = "2")]
            pub enforce: ::core::option::Option<exec_resource::Exec>,
        }
        /// Nested message and enum types in `ExecResource`.
        pub mod exec_resource {
            /// A file or script to execute.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Exec {
                /// Optional arguments to pass to the source during execution.
                #[prost(string, repeated, tag = "3")]
                pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Required. The script interpreter to use.
                #[prost(enumeration = "exec::Interpreter", tag = "4")]
                pub interpreter: i32,
                /// Only recorded for enforce Exec.
                /// Path to an output file (that is created by this Exec) whose
                /// content will be recorded in OSPolicyResourceCompliance after a
                /// successful run. Absence or failure to read this file will result in
                /// this ExecResource being non-compliant. Output file size is limited to
                /// 100K bytes.
                #[prost(string, tag = "5")]
                pub output_file_path: ::prost::alloc::string::String,
                /// What to execute.
                #[prost(oneof = "exec::Source", tags = "1, 2")]
                pub source: ::core::option::Option<exec::Source>,
            }
            /// Nested message and enum types in `Exec`.
            pub mod exec {
                /// The interpreter to use.
                #[derive(
                    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
                )]
                #[repr(i32)]
                pub enum Interpreter {
                    /// Defaults to NONE.
                    Unspecified = 0,
                    /// If no interpreter is specified the
                    /// source will be executed directly, which will likely only
                    /// succeed for executables and scripts with shebang lines.
                    /// [Wikipedia
                    /// shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)).
                    None = 1,
                    /// Indicates that the script will be run with /bin/sh on Linux and
                    /// cmd.exe on windows.
                    Shell = 2,
                    /// Indicates that the script will be run with powershell.
                    Powershell = 3,
                }
                /// What to execute.
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Source {
                    /// A remote or local file.
                    #[prost(message, tag = "1")]
                    File(super::super::File),
                    /// An inline script.
                    #[prost(string, tag = "2")]
                    Script(::prost::alloc::string::String),
                }
            }
        }
        /// A resource that manages the state of a file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FileResource {
            /// Required. The absolute path of the file.
            #[prost(string, tag = "3")]
            pub path: ::prost::alloc::string::String,
            /// Required. Desired state of the file.
            #[prost(enumeration = "file_resource::DesiredState", tag = "4")]
            pub state: i32,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod
            /// utility). Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag = "5")]
            pub permissions: ::prost::alloc::string::String,
            /// The source for the contents of the file.
            #[prost(oneof = "file_resource::Source", tags = "1, 2")]
            pub source: ::core::option::Option<file_resource::Source>,
        }
        /// Nested message and enum types in `FileResource`.
        pub mod file_resource {
            /// Desired state of the file.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure file at path is present.
                Present = 1,
                /// Ensure file at path is absent.
                Absent = 2,
                /// Ensure the contents of the file at path matches. If the file does
                /// not exist it will be created.
                ContentsMatch = 3,
            }
            /// The source for the contents of the file.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Source {
                /// A remote or local source.
                #[prost(message, tag = "1")]
                File(super::File),
                /// A a file with this content.
                #[prost(string, tag = "2")]
                Content(::prost::alloc::string::String),
            }
        }
        /// Resource type.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ResourceType {
            /// Package resource
            #[prost(message, tag = "2")]
            Pkg(PackageResource),
            /// Package repository resource
            #[prost(message, tag = "3")]
            Repository(RepositoryResource),
            /// Exec resource
            #[prost(message, tag = "4")]
            Exec(ExecResource),
            /// File resource
            #[prost(message, tag = "5")]
            File(FileResource),
        }
    }
    /// Policy mode
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Invalid mode
        Unspecified = 0,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state. No actions are performed if they are not in the
        /// desired state. This mode is used for reporting purposes.
        Validation = 1,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state, and if not, enforces the desired state.
        Enforcement = 2,
    }
}
/// Patch configuration specifications. Contains details on how to
/// apply patches to a VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration = "patch_config::RebootConfig", tag = "1")]
    pub reboot_config: i32,
    /// Retry strategy can be defined to have the agent retry patching
    /// during the window if patching fails. If omitted, the agent will use its
    /// default retry strategy.
    #[prost(message, optional, tag = "2")]
    pub retry_strategy: ::core::option::Option<RetryStrategy>,
    /// Apt update settings. Use this override the default apt patch rules.
    #[prost(message, optional, tag = "3")]
    pub apt: ::core::option::Option<AptSettings>,
    /// Yum update settings. Use this override the default yum patch rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::core::option::Option<YumSettings>,
    /// Goo update settings. Use this override the default goo patch rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::core::option::Option<GooSettings>,
    /// Zypper update settings. Use this override the default zypper patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::core::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::core::option::Option<WindowsUpdateSettings>,
    /// The ExecStep to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::core::option::Option<ExecStep>,
    /// The ExecStep to run after the patch update.
    #[prost(message, optional, tag = "9")]
    pub post_step: ::core::option::Option<ExecStep>,
}
/// Nested message and enum types in `PatchConfig`.
pub mod patch_config {
    /// Post-patch reboot settings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RebootConfig {
        /// The default behavior is DEFAULT.
        Unspecified = 0,
        /// The agent decides if a reboot is necessary by checking
        /// signals such as registry keys on Windows or `/var/run/reboot-required` on
        /// APT based systems. On RPM based systems, a set of core system package
        /// install times are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
}
/// Apt patching will be performed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching will be performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration = "apt_settings::Type", tag = "1")]
    pub r#type: i32,
    /// List of packages to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AptSettings`.
pub mod apt_settings {
    /// Apt patch type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// By default, upgrade will be performed.
        Unspecified = 0,
        /// Runs `apt-get dist-upgrade`.
        Dist = 1,
        /// Runs `apt-get upgrade`.
        Upgrade = 2,
    }
}
/// Yum patching will be performed by executing `yum update`. Additional options
/// can be set to control how this is executed.
///
/// Note that not all settings are supported on all platforms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumSettings {
    /// Adds the `--security` flag to `yum update`. Not supported on
    /// all platforms.
    #[prost(bool, tag = "1")]
    pub security: bool,
    /// Will cause patch to run `yum update-minimal` instead.
    #[prost(bool, tag = "2")]
    pub minimal: bool,
    /// List of packages to exclude from update. These packages will be excluded by
    /// using the yum `--exclude` flag.
    #[prost(string, repeated, tag = "3")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag = "4")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Googet patching is performed by running `googet update`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooSettings {}
/// Zypper patching is performed by running `zypper patch`.
/// See also https://en.opensuse.org/SDB:Zypper_manual.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperSettings {
    /// Adds the `--with-optional` flag to `zypper patch`.
    #[prost(bool, tag = "1")]
    pub with_optional: bool,
    /// Adds the `--with-update` flag, to `zypper patch`.
    #[prost(bool, tag = "2")]
    pub with_update: bool,
    /// Install only patches with these categories.
    /// Common categories include security, recommended, and feature.
    #[prost(string, repeated, tag = "3")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag = "4")]
    pub severities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag = "5")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag = "6")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Windows patching is performed using the Windows Update Agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdateSettings {
    /// Only apply updates of these windows update classifications. If empty, all
    /// updates will be applied.
    #[prost(
        enumeration = "windows_update_settings::Classification",
        repeated,
        tag = "1"
    )]
    pub classifications: ::prost::alloc::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WindowsUpdateSettings`.
pub mod windows_update_settings {
    /// Microsoft Windows update classifications as defined in
    /// [1]
    /// https://support.microsoft.com/en-us/help/824684/description-of-the-standard-terminology-that-is-used-to-describe-micro
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Classification {
        /// Invalid. If classifications are included, they must be specified.
        Unspecified = 0,
        /// "A widely released fix for a specific problem that addresses a critical,
        /// non-security-related bug." [1]
        Critical = 1,
        /// "A widely released fix for a product-specific, security-related
        /// vulnerability. Security vulnerabilities are rated by their severity. The
        /// severity rating is indicated in the Microsoft security bulletin as
        /// critical, important, moderate, or low." [1]
        Security = 2,
        /// "A widely released and frequent software update that contains additions
        /// to a product’s definition database. Definition databases are often used
        /// to detect objects that have specific attributes, such as malicious code,
        /// phishing websites, or junk mail." [1]
        Definition = 3,
        /// "Software that controls the input and output of a device." [1]
        Driver = 4,
        /// "New product functionality that is first distributed outside the context
        /// of a product release and that is typically included in the next full
        /// product release." [1]
        FeaturePack = 5,
        /// "A tested, cumulative set of all hotfixes, security updates, critical
        /// updates, and updates. Additionally, service packs may contain additional
        /// fixes for problems that are found internally since the release of the
        /// product. Service packs my also contain a limited number of
        /// customer-requested design changes or features." [1]
        ServicePack = 6,
        /// "A utility or feature that helps complete a task or set of tasks." [1]
        Tool = 7,
        /// "A tested, cumulative set of hotfixes, security updates, critical
        /// updates, and updates that are packaged together for easy deployment. A
        /// rollup generally targets a specific area, such as security, or a
        /// component of a product, such as Internet Information Services (IIS)." [1]
        UpdateRollup = 8,
        /// "A widely released fix for a specific problem. An update addresses a
        /// noncritical, non-security-related bug." [1]
        Update = 9,
    }
}
/// The strategy for retrying failed patches during the patch window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryStrategy {
    /// If true, the agent will continue to try and patch until the window has
    /// ended.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// A step that runs an executable for a PatchJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "1")]
    pub linux_exec_step_config: ::core::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "2")]
    pub windows_exec_step_config: ::core::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to [0]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag = "3")]
    pub allowed_success_codes: ::prost::alloc::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with shebang lines.
    /// [Wikipedia shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)).
    #[prost(enumeration = "exec_step_config::Interpreter", tag = "4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof = "exec_step_config::Executable", tags = "1, 2")]
    pub executable: ::core::option::Option<exec_step_config::Executable>,
}
/// Nested message and enum types in `ExecStepConfig`.
pub mod exec_step_config {
    /// The interpreter used to execute the a file.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Interpreter {
        /// Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the
        /// interpreter will be parsed from the shebang line of the script if
        /// unspecified.
        Unspecified = 0,
        /// Indicates that the script will be run with /bin/sh on Linux and cmd
        /// on windows.
        Shell = 1,
        /// Indicates that the file will be run with PowerShell.
        Powershell = 2,
    }
    /// Location of the executable.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag = "1")]
        LocalPath(::prost::alloc::string::String),
        /// A GCS object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// GCS object representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Bucket of the GCS object.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Name of the GCS object.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
    /// Generation number of the GCS object. This is used to ensure that the
    /// ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag = "3")]
    pub generation_number: i64,
}
/// A unit of work to be performed by the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Unique task id.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// The type of task to perform.
    ///
    /// Task details must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTask
    /// EXEC_STEP = ExecStepTask
    /// APPLY_CONFIG_TASK = ApplyConfigTask
    #[prost(enumeration = "TaskType", tag = "2")]
    pub task_type: i32,
    /// Current directive to the agent.
    #[prost(enumeration = "TaskDirective", tag = "3")]
    pub task_directive: i32,
    /// Labels describing the task.  Used for logging by the agent.
    #[prost(map = "string, string", tag = "6")]
    pub service_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific details about the current task to perform.
    #[prost(oneof = "task::TaskDetails", tags = "4, 5, 7")]
    pub task_details: ::core::option::Option<task::TaskDetails>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// Specific details about the current task to perform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskDetails {
        /// Details about the apply patches task to perform.
        #[prost(message, tag = "4")]
        ApplyPatchesTask(super::ApplyPatchesTask),
        /// Details about the exec step task to perform.
        #[prost(message, tag = "5")]
        ExecStepTask(super::ExecStepTask),
        /// Details about the apply config step task to perform.
        #[prost(message, tag = "7")]
        ApplyConfigTask(super::ApplyConfigTask),
    }
}
/// Message which instructs agent to apply patches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTask {
    /// Specific information about how patches should be applied.
    #[prost(message, optional, tag = "1")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// If true, the agent will report its status as it goes through the motions
    /// but won't actually run any updates or perform any reboots.
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskProgress {
    /// Required. The current state of this patch execution.
    #[prost(enumeration = "apply_patches_task_progress::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `ApplyPatchesTaskProgress`.
pub mod apply_patches_task_progress {
    /// The intermediate states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the patch task.
        Started = 4,
        /// The agent is currently downloading patches.
        DownloadingPatches = 1,
        /// The agent is currently applying patches.
        ApplyingPatches = 2,
        /// The agent is currently rebooting the instance.
        Rebooting = 3,
    }
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskOutput {
    /// Required. The final state of this task.
    #[prost(enumeration = "apply_patches_task_output::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `ApplyPatchesTaskOutput`.
pub mod apply_patches_task_output {
    /// The final states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// Applying patches completed successfully.
        Succeeded = 1,
        /// Applying patches completed successfully, but a reboot is required.
        SucceededRebootRequired = 2,
        /// Applying patches failed.
        Failed = 3,
    }
}
/// Message which instructs agent to execute the following command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTask {
    /// Details of the exec step to run.
    #[prost(message, optional, tag = "1")]
    pub exec_step: ::core::option::Option<ExecStep>,
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskProgress {
    /// Required. The current state of this exec step.
    #[prost(enumeration = "exec_step_task_progress::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `ExecStepTaskProgress`.
pub mod exec_step_task_progress {
    /// The intermediate states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the exec step task.
        Started = 1,
    }
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskOutput {
    /// Required. The final state of the exec step.
    #[prost(enumeration = "exec_step_task_output::State", tag = "1")]
    pub state: i32,
    /// Required. The exit code received from the script which ran as part of the
    /// exec step.
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
}
/// Nested message and enum types in `ExecStepTaskOutput`.
pub mod exec_step_task_output {
    /// The final states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The exec step completed normally.
        Completed = 1,
        /// The exec step was terminated because it took too long.
        TimedOut = 2,
        /// The exec step task was cancelled before it started.
        Cancelled = 3,
    }
}
/// Message which instructs OS Config agent to apply the desired state
/// configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyConfigTask {
    /// List of os policies to be applied for the instance.
    #[prost(message, repeated, tag = "1")]
    pub os_policies: ::prost::alloc::vec::Vec<apply_config_task::OsPolicy>,
}
/// Nested message and enum types in `ApplyConfigTask`.
pub mod apply_config_task {
    /// Message representing an OS policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsPolicy {
        /// User provided policy id.
        /// Used for reporting and logging by the agent.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The policy mode
        #[prost(enumeration = "super::os_policy::Mode", tag = "2")]
        pub mode: i32,
        /// Reference to the `OSPolicyAssignment` API resource that this `OSPolicy`
        /// belongs to.
        /// Format:
        /// projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}
        /// Used for reporting and logging by the agent.
        #[prost(string, tag = "3")]
        pub os_policy_assignment: ::prost::alloc::string::String,
        /// List of resources associated with the policy to be set to their
        /// desired state.
        #[prost(message, repeated, tag = "4")]
        pub resources: ::prost::alloc::vec::Vec<super::os_policy::Resource>,
    }
}
/// Information reported from the agent regarding the progress of the task of
/// applying desired state configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyConfigTaskProgress {
    /// The current state of this task.
    #[prost(enumeration = "apply_config_task_progress::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `ApplyConfigTaskProgress`.
pub mod apply_config_task_progress {
    /// The intermediate states of apply config task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state
        Unspecified = 0,
        /// The agent has started the task.
        Started = 1,
        /// The agent is in the process of applying the configuration.
        ApplyingConfig = 2,
    }
}
/// Information reported from the agent regarding the output of the task of
/// applying desired state configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyConfigTaskOutput {
    /// Required. The final state of this task.
    #[prost(enumeration = "apply_config_task_output::State", tag = "1")]
    pub state: i32,
    /// Results of applying desired state config for the OS policies.
    #[prost(message, repeated, tag = "2")]
    pub os_policy_results: ::prost::alloc::vec::Vec<apply_config_task_output::OsPolicyResult>,
}
/// Nested message and enum types in `ApplyConfigTaskOutput`.
pub mod apply_config_task_output {
    /// Result of applying desired state config for an OS policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsPolicyResult {
        /// The OS policy id
        #[prost(string, tag = "1")]
        pub os_policy_id: ::prost::alloc::string::String,
        /// Reference to the `OSPolicyAssignment` API resource that this `OSPolicy`
        /// belongs to.
        /// Format:
        /// projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}
        /// Used for reporting and logging by the agent.
        #[prost(string, tag = "2")]
        pub os_policy_assignment: ::prost::alloc::string::String,
        /// Results of applying desired state config for the OS policy resources.
        #[prost(message, repeated, tag = "3")]
        pub os_policy_resource_compliances:
            ::prost::alloc::vec::Vec<super::OsPolicyResourceCompliance>,
    }
    /// The final state of this task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The apply config task completed successfully.
        Succeeded = 1,
        /// The apply config task failed.
        Failed = 2,
        /// The apply config task was cancelled.
        Cancelled = 3,
    }
}
/// Specifies the current agent behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskDirective {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The task should continue to progress.
    Continue = 1,
    /// Task should not be started, or if already in progress, should stop
    /// at first safe stopping point.  Task should be considered done and will
    /// never repeat.
    Stop = 2,
}
/// Specifies the type of task to perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The apply patches task.
    ApplyPatches = 1,
    /// The exec step task.
    ExecStepTask = 2,
    /// The apply config task
    ApplyConfigTask = 3,
}
/// A request message to receive task notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. The version of the agent making the request.
    #[prost(string, tag = "2")]
    pub agent_version: ::prost::alloc::string::String,
}
/// The streaming rpc message that will notify the agent when it has a task
/// it needs to perform on the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationResponse {}
/// A request message for signaling the start of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
}
/// A response message that contains the details of the task to work on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskResponse {
    /// The details of the task that should be worked on.  Can be empty if there
    /// is no new task to work on.
    #[prost(message, optional, tag = "1")]
    pub task: ::core::option::Option<Task>,
}
/// A request message for reporting the progress of current task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. The type of task to report progress on.
    ///
    /// Progress must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTaskProgress
    /// EXEC_STEP = Progress not supported for this type.
    /// APPLY_CONFIG_TASK = ApplyConfigTaskProgress
    #[prost(enumeration = "TaskType", tag = "3")]
    pub task_type: i32,
    /// Intermediate progress of the current task.
    #[prost(oneof = "report_task_progress_request::Progress", tags = "4, 5, 6")]
    pub progress: ::core::option::Option<report_task_progress_request::Progress>,
}
/// Nested message and enum types in `ReportTaskProgressRequest`.
pub mod report_task_progress_request {
    /// Intermediate progress of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Progress {
        /// Details about the progress of the apply patches task.
        #[prost(message, tag = "4")]
        ApplyPatchesTaskProgress(super::ApplyPatchesTaskProgress),
        /// Details about the progress of the exec step task.
        #[prost(message, tag = "5")]
        ExecStepTaskProgress(super::ExecStepTaskProgress),
        /// Details about the progress of the apply config task.
        #[prost(message, tag = "6")]
        ApplyConfigTaskProgress(super::ApplyConfigTaskProgress),
    }
}
/// The response message after the agent reported the current task progress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressResponse {
    /// Instructs agent to continue or not.
    #[prost(enumeration = "TaskDirective", tag = "1")]
    pub task_directive: i32,
}
/// A request message for signaling the completion of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. The type of task to report completed.
    ///
    /// Output must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTaskOutput
    /// EXEC_STEP = ExecStepTaskOutput
    /// APPLY_CONFIG_TASK = ApplyConfigTaskOutput
    #[prost(enumeration = "TaskType", tag = "3")]
    pub task_type: i32,
    /// Descriptive error message if the task execution ended in error.
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
    /// Final output details of the current task.
    #[prost(oneof = "report_task_complete_request::Output", tags = "5, 6, 7")]
    pub output: ::core::option::Option<report_task_complete_request::Output>,
}
/// Nested message and enum types in `ReportTaskCompleteRequest`.
pub mod report_task_complete_request {
    /// Final output details of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// Final output details of the apply patches task;
        #[prost(message, tag = "5")]
        ApplyPatchesTaskOutput(super::ApplyPatchesTaskOutput),
        /// Final output details of the exec step task;
        #[prost(message, tag = "6")]
        ExecStepTaskOutput(super::ExecStepTaskOutput),
        /// Final output details of the apply config task;
        #[prost(message, tag = "7")]
        ApplyConfigTaskOutput(super::ApplyConfigTaskOutput),
    }
}
/// The response message after the agent signaled the current task complete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteResponse {}
/// The request message for registering the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. The version of the agent.
    #[prost(string, tag = "2")]
    pub agent_version: ::prost::alloc::string::String,
    /// Required. The capabilities supported by the agent. Supported values are:
    /// PATCH_GA
    /// GUEST_POLICY_BETA
    /// CONFIG_V1
    #[prost(string, repeated, tag = "3")]
    pub supported_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The operating system long name.
    /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
    /// Datacenter'.
    #[prost(string, tag = "4")]
    pub os_long_name: ::prost::alloc::string::String,
    /// The operating system short name.
    /// For example, 'windows' or 'debian'.
    #[prost(string, tag = "5")]
    pub os_short_name: ::prost::alloc::string::String,
    /// The version of the operating system.
    #[prost(string, tag = "6")]
    pub os_version: ::prost::alloc::string::String,
    /// The system architecture of the operating system.
    #[prost(string, tag = "7")]
    pub os_architecture: ::prost::alloc::string::String,
}
/// The response message after the agent registered.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentResponse {}
/// The request message for having the agent report inventory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInventoryRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. This is a client created checksum that should be generated based
    /// on the contents of the reported inventory.  This will be used by the
    /// service to determine if it has the latest version of inventory.
    #[prost(string, tag = "2")]
    pub inventory_checksum: ::prost::alloc::string::String,
    /// Optional. This is the details of the inventory.  Should only be provided if
    /// the inventory has changed since the last report, or if instructed by the
    /// service to provide full inventory.
    #[prost(message, optional, tag = "3")]
    pub inventory: ::core::option::Option<Inventory>,
}
/// The response message after the agent has reported inventory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInventoryResponse {
    /// If true, the full inventory should be reported back to the server.
    #[prost(bool, tag = "1")]
    pub report_full_inventory: bool,
}
#[doc = r" Generated client implementations."]
pub mod agent_endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " OS Config agent endpoint API."]
    #[derive(Debug, Clone)]
    pub struct AgentEndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgentEndpointServiceClient<T>
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
        ) -> AgentEndpointServiceClient<InterceptedService<T, F>>
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
            AgentEndpointServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Stream established by client to receive Task notifications."]
        pub async fn receive_task_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::ReceiveTaskNotificationRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReceiveTaskNotificationResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReceiveTaskNotification") ;
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Signals the start of a task execution and returns the task info."]
        pub async fn start_next_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNextTaskRequest>,
        ) -> Result<tonic::Response<super::StartNextTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/StartNextTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signals an intermediary progress checkpoint in task execution."]
        pub async fn report_task_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskProgressRequest>,
        ) -> Result<tonic::Response<super::ReportTaskProgressResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskProgress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signals that the task execution is complete and optionally returns the next"]
        #[doc = " task."]
        pub async fn report_task_complete(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskCompleteRequest>,
        ) -> Result<tonic::Response<super::ReportTaskCompleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskComplete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers the agent running on the VM."]
        pub async fn register_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterAgentRequest>,
        ) -> Result<tonic::Response<super::RegisterAgentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/RegisterAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reports the VMs current inventory."]
        pub async fn report_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportInventoryRequest>,
        ) -> Result<tonic::Response<super::ReportInventoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportInventory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
