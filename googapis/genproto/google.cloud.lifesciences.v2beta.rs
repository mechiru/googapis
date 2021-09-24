/// The arguments to the `RunPipeline` method. The requesting user must have
/// the `iam.serviceAccounts.actAs` permission for the Cloud Life Sciences
/// service account or the request will fail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineRequest {
    /// The project and location that this request should be executed against.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The description of the pipeline to run.
    #[prost(message, optional, tag = "1")]
    pub pipeline: ::core::option::Option<Pipeline>,
    /// User-defined labels to associate with the returned operation. These
    /// labels are not propagated to any Google Cloud Platform resources used by
    /// the operation, and can be modified at any time.
    ///
    /// To associate labels with resources created while executing the operation,
    /// see the appropriate resource message (for example, `VirtualMachine`).
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The name of an existing Pub/Sub topic.  The server will publish
    /// messages to this topic whenever the status of the operation changes.
    /// The Life Sciences Service Agent account must have publisher permissions to
    /// the specified topic or notifications will not be sent.
    #[prost(string, tag = "3")]
    pub pub_sub_topic: ::prost::alloc::string::String,
}
/// The response to the RunPipeline method, returned in the operation's result
/// field on success.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineResponse {}
/// Specifies a series of actions to execute, expressed as Docker containers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipeline {
    /// The list of actions to execute, in the order they are specified.
    #[prost(message, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    /// The resources required for execution.
    #[prost(message, optional, tag = "2")]
    pub resources: ::core::option::Option<Resources>,
    /// The environment to pass into every action. Each action can also specify
    /// additional environment variables but cannot delete an entry from this map
    /// (though they can overwrite it with a different value).
    #[prost(map = "string, string", tag = "3")]
    pub environment:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The maximum amount of time to give the pipeline to complete.  This includes
    /// the time spent waiting for a worker to be allocated.  If the pipeline fails
    /// to complete before the timeout, it will be cancelled and the error code
    /// will be set to DEADLINE_EXCEEDED.
    ///
    /// If unspecified, it will default to 7 days.
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Specifies a single action that runs a Docker container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// An optional name for the container. The container hostname will be set to
    /// this name, making it useful for inter-container communication. The name
    /// must contain only upper and lowercase alphanumeric characters and hyphens
    /// and cannot start with a hyphen.
    #[prost(string, tag = "1")]
    pub container_name: ::prost::alloc::string::String,
    /// Required. The URI to pull the container image from. Note that all images referenced
    /// by actions in the pipeline are pulled before the first action runs. If
    /// multiple actions reference the same image, it is only pulled once,
    /// ensuring that the same image is used for all actions in a single pipeline.
    ///
    /// The image URI can be either a complete host and image specification (e.g.,
    /// quay.io/biocontainers/samtools), a library and image name (e.g.,
    /// google/cloud-sdk) or a bare image name ('bash') to pull from the default
    /// library.  No schema is required in any of these cases.
    ///
    /// If the specified image is not public, the service account specified for
    /// the Virtual Machine must have access to pull the images from GCR, or
    /// appropriate credentials must be specified in the
    /// [google.cloud.lifesciences.v2beta.Action.credentials][google.cloud.lifesciences.v2beta.Action.credentials] field.
    #[prost(string, tag = "2")]
    pub image_uri: ::prost::alloc::string::String,
    /// If specified, overrides the `CMD` specified in the container. If the
    /// container also has an `ENTRYPOINT` the values are used as entrypoint
    /// arguments. Otherwise, they are used as a command and arguments to run
    /// inside the container.
    #[prost(string, repeated, tag = "3")]
    pub commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, overrides the `ENTRYPOINT` specified in the container.
    #[prost(string, tag = "4")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The environment to pass into the container. This environment is merged
    /// with values specified in the [google.cloud.lifesciences.v2beta.Pipeline][google.cloud.lifesciences.v2beta.Pipeline]
    /// message, overwriting any duplicate values.
    ///
    /// In addition to the values passed here, a few other values are
    /// automatically injected into the environment. These cannot be hidden or
    /// overwritten.
    ///
    /// `GOOGLE_PIPELINE_FAILED` will be set to "1" if the pipeline failed
    /// because an action has exited with a non-zero status (and did not have the
    /// `IGNORE_EXIT_STATUS` flag set). This can be used to determine if additional
    /// debug or logging actions should execute.
    ///
    /// `GOOGLE_LAST_EXIT_STATUS` will be set to the exit status of the last
    /// non-background action that executed. This can be used by workflow engine
    /// authors to determine whether an individual action has succeeded or failed.
    #[prost(map = "string, string", tag = "5")]
    pub environment:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// An optional identifier for a PID namespace to run the action inside.
    /// Multiple actions should use the same string to share a namespace.  If
    /// unspecified, a separate isolated namespace is used.
    #[prost(string, tag = "6")]
    pub pid_namespace: ::prost::alloc::string::String,
    /// A map of containers to host port mappings for this container. If the
    /// container already specifies exposed ports, use the
    /// `PUBLISH_EXPOSED_PORTS` flag instead.
    ///
    /// The host port number must be less than 65536. If it is zero, an unused
    /// random port is assigned. To determine the resulting port number, consult
    /// the `ContainerStartedEvent` in the operation metadata.
    #[prost(map = "int32, int32", tag = "8")]
    pub port_mappings: ::std::collections::HashMap<i32, i32>,
    /// A list of mounts to make available to the action.
    ///
    /// In addition to the values specified here, every action has a special
    /// virtual disk mounted under `/google` that contains log files and other
    /// operational components.
    ///
    /// <ul>
    ///   <li><code>/google/logs</code> All logs written during the pipeline
    ///   execution.</li>
    ///   <li><code>/google/logs/output</code> The combined standard output and
    ///   standard error of all actions run as part of the pipeline
    ///   execution.</li>
    ///   <li><code>/google/logs/action/*/stdout</code> The complete contents of
    ///   each individual action's standard output.</li>
    ///   <li><code>/google/logs/action/*/stderr</code> The complete contents of
    ///   each individual action's standard error output.</li>
    /// </ul>
    #[prost(message, repeated, tag = "9")]
    pub mounts: ::prost::alloc::vec::Vec<Mount>,
    /// Labels to associate with the action. This field is provided to assist
    /// workflow engine authors in identifying actions (for example, to indicate
    /// what sort of action they perform, such as localization or debugging).
    /// They are returned in the operation metadata, but are otherwise ignored.
    #[prost(map = "string, string", tag = "10")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// If the specified image is hosted on a private registry other than Google
    /// Container Registry, the credentials required to pull the image must be
    /// specified here as an encrypted secret.
    ///
    /// The secret must decrypt to a JSON-encoded dictionary containing both
    /// `username` and `password` keys.
    #[prost(message, optional, tag = "11")]
    pub credentials: ::core::option::Option<Secret>,
    /// The maximum amount of time to give the action to complete. If the action
    /// fails to complete before the timeout, it will be terminated and the exit
    /// status will be non-zero. The pipeline will continue or terminate based
    /// on the rules defined by the `ALWAYS_RUN` and `IGNORE_EXIT_STATUS` flags.
    #[prost(message, optional, tag = "12")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Normally, a non-zero exit status causes the pipeline to fail. This flag
    /// allows execution of other actions to continue instead.
    #[prost(bool, tag = "13")]
    pub ignore_exit_status: bool,
    /// This flag allows an action to continue running in the background while
    /// executing subsequent actions. This is useful to provide services to
    /// other actions (or to provide debugging support tools like SSH servers).
    #[prost(bool, tag = "14")]
    pub run_in_background: bool,
    /// By default, after an action fails, no further actions are run. This flag
    /// indicates that this action must be run even if the pipeline has already
    /// failed. This is useful for actions that copy output files off of the VM
    /// or for debugging. Note that no actions will be run if image prefetching
    /// fails.
    #[prost(bool, tag = "15")]
    pub always_run: bool,
    /// Enable access to the FUSE device for this action. Filesystems can then
    /// be mounted into disks shared with other actions. The other actions do
    /// not need the `enable_fuse` flag to access the mounted filesystem.
    ///
    /// This has the effect of causing the container to be executed with
    /// `CAP_SYS_ADMIN` and exposes `/dev/fuse` to the container, so use it only
    /// for containers you trust.
    #[prost(bool, tag = "16")]
    pub enable_fuse: bool,
    /// Exposes all ports specified by `EXPOSE` statements in the container. To
    /// discover the host side port numbers, consult the `ACTION_STARTED` event
    /// in the operation metadata.
    #[prost(bool, tag = "17")]
    pub publish_exposed_ports: bool,
    /// All container images are typically downloaded before any actions are
    /// executed. This helps prevent typos in URIs or issues like lack of disk
    /// space from wasting large amounts of compute resources.
    ///
    /// If set, this flag prevents the worker from downloading the image until
    /// just before the action is executed.
    #[prost(bool, tag = "18")]
    pub disable_image_prefetch: bool,
    /// A small portion of the container's standard error stream is typically
    /// captured and returned inside the `ContainerStoppedEvent`. Setting this
    /// flag disables this functionality.
    #[prost(bool, tag = "19")]
    pub disable_standard_error_capture: bool,
    /// Prevents the container from accessing the external network.
    #[prost(bool, tag = "20")]
    pub block_external_network: bool,
}
/// Holds encrypted information that is only decrypted and stored in RAM
/// by the worker VM when running the pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// The name of the Cloud KMS key that will be used to decrypt the secret
    /// value. The VM service account must have the required permissions and
    /// authentication scopes to invoke the `decrypt` method on the specified key.
    #[prost(string, tag = "1")]
    pub key_name: ::prost::alloc::string::String,
    /// The value of the cipherText response from the `encrypt` method. This field
    /// is intentionally unaudited.
    #[prost(string, tag = "2")]
    pub cipher_text: ::prost::alloc::string::String,
}
/// Carries information about a particular disk mount inside a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mount {
    /// The name of the disk to mount, as specified in the resources section.
    #[prost(string, tag = "1")]
    pub disk: ::prost::alloc::string::String,
    /// The path to mount the disk inside the container.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// If true, the disk is mounted read-only inside the container.
    #[prost(bool, tag = "3")]
    pub read_only: bool,
}
/// The system resources for the pipeline run.
///
/// At least one zone or region must be specified or the pipeline run will fail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    /// The list of regions allowed for VM allocation. If set, the `zones` field
    /// must not be set.
    #[prost(string, repeated, tag = "2")]
    pub regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of zones allowed for VM allocation. If set, the `regions` field
    /// must not be set.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The virtual machine specification.
    #[prost(message, optional, tag = "4")]
    pub virtual_machine: ::core::option::Option<VirtualMachine>,
}
/// Carries information about a Compute Engine VM resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachine {
    /// Required. The machine type of the virtual machine to create. Must be the short name
    /// of a standard machine type (such as "n1-standard-1") or a custom machine
    /// type (such as "custom-1-4096", where "1" indicates the number of vCPUs and
    /// "4096" indicates the memory in MB). See
    /// [Creating an instance with a custom machine
    /// type](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create)
    /// for more specifications on creating a custom machine type.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// If true, allocate a preemptible VM.
    #[prost(bool, tag = "2")]
    pub preemptible: bool,
    /// Optional set of labels to apply to the VM and any attached disk resources.
    /// These labels must adhere to the [name and value
    /// restrictions](https://cloud.google.com/compute/docs/labeling-resources) on
    /// VM labels imposed by Compute Engine.
    ///
    /// Labels keys with the prefix 'google-' are reserved for use by Google.
    ///
    /// Labels applied at creation time to the VM. Applied on a best-effort basis
    /// to attached disk resources shortly after VM creation.
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The list of disks to create and attach to the VM.
    ///
    /// Specify either the `volumes[]` field or the `disks[]` field, but not both.
    #[prost(message, repeated, tag = "4")]
    pub disks: ::prost::alloc::vec::Vec<Disk>,
    /// The VM network configuration.
    #[prost(message, optional, tag = "5")]
    pub network: ::core::option::Option<Network>,
    /// The list of accelerators to attach to the VM.
    #[prost(message, repeated, tag = "6")]
    pub accelerators: ::prost::alloc::vec::Vec<Accelerator>,
    /// The service account to install on the VM. This account does not need
    /// any permissions other than those required by the pipeline.
    #[prost(message, optional, tag = "7")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    /// The size of the boot disk, in GB. The boot disk must be large
    /// enough to accommodate all of the Docker images from each action in the
    /// pipeline at the same time. If not specified, a small but reasonable
    /// default value is used.
    #[prost(int32, tag = "8")]
    pub boot_disk_size_gb: i32,
    /// The CPU platform to request. An instance based on a newer platform can be
    /// allocated, but never one with fewer capabilities. The value of this
    /// parameter must be a valid Compute Engine CPU platform name (such as "Intel
    /// Skylake"). This parameter is only useful for carefully optimized work
    /// loads where the CPU platform has a significant impact.
    ///
    /// For more information about the effect of this parameter, see
    /// https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform.
    #[prost(string, tag = "9")]
    pub cpu_platform: ::prost::alloc::string::String,
    /// The host operating system image to use.
    ///
    /// Currently, only Container-Optimized OS images can be used.
    ///
    /// The default value is `projects/cos-cloud/global/images/family/cos-stable`,
    /// which selects the latest stable release of Container-Optimized OS.
    ///
    /// This option is provided to allow testing against the beta release of the
    /// operating system to ensure that the new version does not interact
    /// negatively with production pipelines.
    ///
    /// To test a pipeline against the beta release of Container-Optimized OS,
    /// use the value `projects/cos-cloud/global/images/family/cos-beta`.
    #[prost(string, tag = "10")]
    pub boot_image: ::prost::alloc::string::String,
    /// The NVIDIA driver version to use when attaching an NVIDIA GPU accelerator.
    /// The version specified here must be compatible with the GPU libraries
    /// contained in the container being executed, and must be one of the drivers
    /// hosted in the `nvidia-drivers-us-public` bucket on Google Cloud Storage.
    #[deprecated]
    #[prost(string, tag = "11")]
    pub nvidia_driver_version: ::prost::alloc::string::String,
    /// Whether Stackdriver monitoring should be enabled on the VM.
    #[prost(bool, tag = "12")]
    pub enable_stackdriver_monitoring: bool,
    /// The Compute Engine Disk Images to use as a Docker cache. The disks will be
    /// mounted into the Docker folder in a way that the images present in the
    /// cache will not need to be pulled. The digests of the cached images must
    /// match those of the tags used or the latest version will still be pulled.
    /// The root directory of the ext4 image must contain `image` and `overlay2`
    /// directories copied from the Docker directory of a VM where the desired
    /// Docker images have already been pulled. Any images pulled that are not
    /// cached will be stored on the first cache disk instead of the boot disk.
    /// Only a single image is supported.
    #[prost(string, repeated, tag = "13")]
    pub docker_cache_images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of disks and other storage to create or attach to the VM.
    ///
    /// Specify either the `volumes[]` field or the `disks[]` field, but not both.
    #[prost(message, repeated, tag = "14")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
}
/// Carries information about a Google Cloud service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. If not specified, the default
    /// Compute Engine service account for the project will be used.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// List of scopes to be enabled for this service account on the VM, in
    /// addition to the cloud-platform API scope that will be added by default.
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Carries information about an accelerator that can be attached to a VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accelerator {
    /// The accelerator type string (for example, "nvidia-tesla-k80").
    ///
    /// Only NVIDIA GPU accelerators are currently supported. If an NVIDIA GPU is
    /// attached, the required runtime libraries will be made available to all
    /// containers under `/usr/local/nvidia`. The driver version to install must
    /// be specified using the NVIDIA driver version parameter on the virtual
    /// machine specification. Note that attaching a GPU increases the worker VM
    /// startup time by a few minutes.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// How many accelerators of this type to attach.
    #[prost(int64, tag = "2")]
    pub count: i64,
}
/// VM networking options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// The network name to attach the VM's network interface to. The value will
    /// be prefixed with `global/networks/` unless it contains a `/`, in which
    /// case it is assumed to be a fully specified network resource URL.
    ///
    /// If unspecified, the global default network is used.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// If set to true, do not attach a public IP address to the VM. Note that
    /// without a public IP address, additional configuration is required to
    /// allow the VM to access Google services.
    ///
    /// See https://cloud.google.com/vpc/docs/configure-private-google-access
    /// for more information.
    #[prost(bool, tag = "2")]
    pub use_private_address: bool,
    /// If the specified network is configured for custom subnet creation, the
    /// name of the subnetwork to attach the instance to must be specified here.
    ///
    /// The value is prefixed with `regions/*/subnetworks/` unless it contains a
    /// `/`, in which case it is assumed to be a fully specified subnetwork
    /// resource URL.
    ///
    /// If the `*` character appears in the value, it is replaced with the region
    /// that the virtual machine has been allocated in.
    #[prost(string, tag = "3")]
    pub subnetwork: ::prost::alloc::string::String,
}
/// Carries information about a disk that can be attached to a VM.
///
/// See https://cloud.google.com/compute/docs/disks/performance for more
/// information about disk type, size, and performance considerations.
///
/// Specify either [`Volume`][google.cloud.lifesciences.v2beta.Volume] or
/// [`Disk`][google.cloud.lifesciences.v2beta.Disk], but not both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disk {
    /// A user-supplied name for the disk. Used when mounting the disk into
    /// actions. The name must contain only upper and lowercase alphanumeric
    /// characters and hyphens and cannot start with a hyphen.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The size, in GB, of the disk to attach. If the size is not
    /// specified, a default is chosen to ensure reasonable I/O performance.
    ///
    /// If the disk type is specified as `local-ssd`, multiple local drives are
    /// automatically combined to provide the requested size. Note, however, that
    /// each physical SSD is 375GB in size, and no more than 8 drives can be
    /// attached to a single instance.
    #[prost(int32, tag = "2")]
    pub size_gb: i32,
    /// The Compute Engine disk type. If unspecified, `pd-standard` is used.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// An optional image to put on the disk before attaching it to the VM.
    #[prost(string, tag = "4")]
    pub source_image: ::prost::alloc::string::String,
}
/// Carries information about storage that can be attached to a VM.
///
/// Specify either [`Volume`][google.cloud.lifesciences.v2beta.Volume] or
/// [`Disk`][google.cloud.lifesciences.v2beta.Disk], but not both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// A user-supplied name for the volume. Used when mounting the volume into
    /// [`Actions`][google.cloud.lifesciences.v2beta.Action]. The name must contain
    /// only upper and lowercase alphanumeric characters and hyphens and cannot
    /// start with a hyphen.
    #[prost(string, tag = "1")]
    pub volume: ::prost::alloc::string::String,
    #[prost(oneof = "volume::Storage", tags = "2, 3, 4")]
    pub storage: ::core::option::Option<volume::Storage>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Storage {
        /// Configuration for a persistent disk.
        #[prost(message, tag = "2")]
        PersistentDisk(super::PersistentDisk),
        /// Configuration for a existing disk.
        #[prost(message, tag = "3")]
        ExistingDisk(super::ExistingDisk),
        /// Configuration for an NFS mount.
        #[prost(message, tag = "4")]
        NfsMount(super::NfsMount),
    }
}
/// Configuration for a persistent disk to be attached to the VM.
///
/// See https://cloud.google.com/compute/docs/disks/performance for more
/// information about disk type, size, and performance considerations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentDisk {
    /// The size, in GB, of the disk to attach. If the size is not
    /// specified, a default is chosen to ensure reasonable I/O performance.
    ///
    /// If the disk type is specified as `local-ssd`, multiple local drives are
    /// automatically combined to provide the requested size. Note, however, that
    /// each physical SSD is 375GB in size, and no more than 8 drives can be
    /// attached to a single instance.
    #[prost(int32, tag = "1")]
    pub size_gb: i32,
    /// The Compute Engine disk type. If unspecified, `pd-standard` is used.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// An image to put on the disk before attaching it to the VM.
    #[prost(string, tag = "3")]
    pub source_image: ::prost::alloc::string::String,
}
/// Configuration for an existing disk to be attached to the VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistingDisk {
    /// If `disk` contains slashes, the Cloud Life Sciences API assumes that it is
    /// a complete URL for the disk.  If `disk` does not contain slashes, the Cloud
    /// Life Sciences API assumes that the disk is a zonal disk and a URL will be
    /// generated of the form `zones/<zone>/disks/<disk>`, where `<zone>` is the
    /// zone in which the instance is allocated. The disk must be ext4 formatted.
    ///
    /// If all `Mount` references to this disk have the `read_only` flag set to
    /// true, the disk will be attached in `read-only` mode and can be shared with
    /// other instances. Otherwise, the disk will be available for writing but
    /// cannot be shared.
    #[prost(string, tag = "1")]
    pub disk: ::prost::alloc::string::String,
}
/// Configuration for an `NFSMount` to be attached to the VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsMount {
    /// A target NFS mount. The target must be specified as `address:/mount".
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
}
/// Carries information about the pipeline execution that is returned
/// in the long running operation's metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The pipeline this operation represents.
    #[prost(message, optional, tag = "1")]
    pub pipeline: ::core::option::Option<Pipeline>,
    /// The user-defined labels associated with this operation.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The list of events that have happened so far during the execution of this
    /// operation.
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// The time at which the operation was created by the API.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The first time at which resources were allocated to execute the pipeline.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which execution was completed and resources were cleaned up.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The name of the Cloud Pub/Sub topic where notifications of operation status
    /// changes are sent.
    #[prost(string, tag = "7")]
    pub pub_sub_topic: ::prost::alloc::string::String,
}
/// Carries information about events that occur during pipeline execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The time at which the event occurred.
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// A human-readable description of the event. Note that these strings can
    /// change at any time without notice. Any application logic must use the
    /// information in the `details` field.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Machine-readable details about the event.
    #[prost(
        oneof = "event::Details",
        tags = "17, 18, 19, 20, 21, 22, 23, 24, 25, 26"
    )]
    pub details: ::core::option::Option<event::Details>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Machine-readable details about the event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// See [google.cloud.lifesciences.v2beta.DelayedEvent][google.cloud.lifesciences.v2beta.DelayedEvent].
        #[prost(message, tag = "17")]
        Delayed(super::DelayedEvent),
        /// See [google.cloud.lifesciences.v2beta.WorkerAssignedEvent][google.cloud.lifesciences.v2beta.WorkerAssignedEvent].
        #[prost(message, tag = "18")]
        WorkerAssigned(super::WorkerAssignedEvent),
        /// See [google.cloud.lifesciences.v2beta.WorkerReleasedEvent][google.cloud.lifesciences.v2beta.WorkerReleasedEvent].
        #[prost(message, tag = "19")]
        WorkerReleased(super::WorkerReleasedEvent),
        /// See [google.cloud.lifesciences.v2beta.PullStartedEvent][google.cloud.lifesciences.v2beta.PullStartedEvent].
        #[prost(message, tag = "20")]
        PullStarted(super::PullStartedEvent),
        /// See [google.cloud.lifesciences.v2beta.PullStoppedEvent][google.cloud.lifesciences.v2beta.PullStoppedEvent].
        #[prost(message, tag = "21")]
        PullStopped(super::PullStoppedEvent),
        /// See [google.cloud.lifesciences.v2beta.ContainerStartedEvent][google.cloud.lifesciences.v2beta.ContainerStartedEvent].
        #[prost(message, tag = "22")]
        ContainerStarted(super::ContainerStartedEvent),
        /// See [google.cloud.lifesciences.v2beta.ContainerStoppedEvent][google.cloud.lifesciences.v2beta.ContainerStoppedEvent].
        #[prost(message, tag = "23")]
        ContainerStopped(super::ContainerStoppedEvent),
        /// See [google.cloud.lifesciences.v2beta.ContainerKilledEvent][google.cloud.lifesciences.v2beta.ContainerKilledEvent].
        #[prost(message, tag = "24")]
        ContainerKilled(super::ContainerKilledEvent),
        /// See [google.cloud.lifesciences.v2beta.UnexpectedExitStatusEvent][google.cloud.lifesciences.v2beta.UnexpectedExitStatusEvent].
        #[prost(message, tag = "25")]
        UnexpectedExitStatus(super::UnexpectedExitStatusEvent),
        /// See [google.cloud.lifesciences.v2beta.FailedEvent][google.cloud.lifesciences.v2beta.FailedEvent].
        #[prost(message, tag = "26")]
        Failed(super::FailedEvent),
    }
}
/// An event generated whenever a resource limitation or transient error
/// delays execution of a pipeline that was otherwise ready to run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayedEvent {
    /// A textual description of the cause of the delay. The string can change
    /// without notice because it is often generated by another service (such as
    /// Compute Engine).
    #[prost(string, tag = "1")]
    pub cause: ::prost::alloc::string::String,
    /// If the delay was caused by a resource shortage, this field lists the
    /// Compute Engine metrics that are preventing this operation from running
    /// (for example, `CPUS` or `INSTANCES`). If the particular metric is not
    /// known, a single `UNKNOWN` metric will be present.
    #[prost(string, repeated, tag = "2")]
    pub metrics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An event generated after a worker VM has been assigned to run the
/// pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerAssignedEvent {
    /// The zone the worker is running in.
    #[prost(string, tag = "1")]
    pub zone: ::prost::alloc::string::String,
    /// The worker's instance name.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// The machine type that was assigned for the worker.
    #[prost(string, tag = "3")]
    pub machine_type: ::prost::alloc::string::String,
}
/// An event generated when the worker VM that was assigned to the pipeline
/// has been released (deleted).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerReleasedEvent {
    /// The zone the worker was running in.
    #[prost(string, tag = "1")]
    pub zone: ::prost::alloc::string::String,
    /// The worker's instance name.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
}
/// An event generated when the worker starts pulling an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullStartedEvent {
    /// The URI of the image that was pulled.
    #[prost(string, tag = "1")]
    pub image_uri: ::prost::alloc::string::String,
}
/// An event generated when the worker stops pulling an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullStoppedEvent {
    /// The URI of the image that was pulled.
    #[prost(string, tag = "1")]
    pub image_uri: ::prost::alloc::string::String,
}
/// An event generated when a container starts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStartedEvent {
    /// The numeric ID of the action that started this container.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
    /// The container-to-host port mappings installed for this container. This
    /// set will contain any ports exposed using the `PUBLISH_EXPOSED_PORTS` flag
    /// as well as any specified in the `Action` definition.
    #[prost(map = "int32, int32", tag = "2")]
    pub port_mappings: ::std::collections::HashMap<i32, i32>,
    /// The public IP address that can be used to connect to the container. This
    /// field is only populated when at least one port mapping is present. If the
    /// instance was created with a private address, this field will be empty even
    /// if port mappings exist.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// An event generated when a container exits.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStoppedEvent {
    /// The numeric ID of the action that started this container.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
    /// The exit status of the container.
    #[prost(int32, tag = "2")]
    pub exit_status: i32,
    /// The tail end of any content written to standard error by the container.
    /// If the content emits large amounts of debugging noise or contains
    /// sensitive information, you can prevent the content from being printed by
    /// setting the `DISABLE_STANDARD_ERROR_CAPTURE` flag.
    ///
    /// Note that only a small amount of the end of the stream is captured here.
    /// The entire stream is stored in the `/google/logs` directory mounted into
    /// each action, and can be copied off the machine as described elsewhere.
    #[prost(string, tag = "3")]
    pub stderr: ::prost::alloc::string::String,
}
/// An event generated when the execution of a container results in a
/// non-zero exit status that was not otherwise ignored. Execution will
/// continue, but only actions that are flagged as `ALWAYS_RUN` will be
/// executed. Other actions will be skipped.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnexpectedExitStatusEvent {
    /// The numeric ID of the action that started the container.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
    /// The exit status of the container.
    #[prost(int32, tag = "2")]
    pub exit_status: i32,
}
/// An event generated when a container is forcibly terminated by the
/// worker. Currently, this only occurs when the container outlives the
/// timeout specified by the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerKilledEvent {
    /// The numeric ID of the action that started the container.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
}
/// An event generated when the execution of a pipeline has failed. Note
/// that other events can continue to occur after this event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedEvent {
    /// The Google standard error code that best describes this failure.
    #[prost(enumeration = "super::super::super::rpc::Code", tag = "1")]
    pub code: i32,
    /// The human-readable description of the cause of the failure.
    #[prost(string, tag = "2")]
    pub cause: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod workflows_service_v2_beta_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service for running workflows, such as pipelines consisting of Docker"]
    #[doc = " containers."]
    #[derive(Debug, Clone)]
    pub struct WorkflowsServiceV2BetaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WorkflowsServiceV2BetaClient<T>
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
        ) -> WorkflowsServiceV2BetaClient<InterceptedService<T, F>>
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
            WorkflowsServiceV2BetaClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Runs a pipeline.  The returned Operation's [metadata]"]
        #[doc = " [google.longrunning.Operation.metadata] field will contain a"]
        #[doc = " [google.cloud.lifesciences.v2beta.Metadata][google.cloud.lifesciences.v2beta.Metadata] object describing the status"]
        #[doc = " of the pipeline execution. The"]
        #[doc = " [response][google.longrunning.Operation.response] field will contain a"]
        #[doc = " [google.cloud.lifesciences.v2beta.RunPipelineResponse][google.cloud.lifesciences.v2beta.RunPipelineResponse] object if the"]
        #[doc = " pipeline completes successfully."]
        #[doc = ""]
        #[doc = " **Note:** Before you can use this method, the *Life Sciences Service Agent*"]
        #[doc = " must have access to your project. This is done automatically when the"]
        #[doc = " Cloud Life Sciences API is first enabled, but if you delete this permission"]
        #[doc = " you must disable and re-enable the API to grant the Life Sciences"]
        #[doc = " Service Agent the required permissions."]
        #[doc = " Authorization requires the following [Google"]
        #[doc = " IAM](https://cloud.google.com/iam/) permission:"]
        #[doc = ""]
        #[doc = " * `lifesciences.workflows.run`"]
        pub async fn run_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPipelineRequest>,
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
                "/google.cloud.lifesciences.v2beta.WorkflowsServiceV2Beta/RunPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
