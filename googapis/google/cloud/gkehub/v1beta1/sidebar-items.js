initSidebarItems({"mod":[["gke_hub_membership_service_client","Generated client implementations."],["membership","Nested message and enum types in `Membership`."],["membership_state","Nested message and enum types in `MembershipState`."]],"struct":[["Authority","Authority encodes how Google will recognize identities from this Membership. See the workload identity documentation for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity"],["ConnectAgent","The information required from end users to use GKE Connect."],["ConnectAgentResource","ConnectAgentResource represents a Kubernetes resource manifest for Connect Agent deployment."],["CreateMembershipRequest","Request message for the `GkeHubMembershipService.CreateMembership` method."],["DeleteMembershipRequest","Request message for `GkeHubMembershipService.DeleteMembership` method."],["GenerateConnectManifestRequest","Request message for `GkeHubMembershipService.GenerateConnectManifest` method. ."],["GenerateConnectManifestResponse","GenerateConnectManifestResponse contains manifest information for installing/upgrading a Connect agent."],["GenerateExclusivityManifestRequest","The request to generate the manifests for exclusivity artifacts."],["GenerateExclusivityManifestResponse","The response of the exclusivity artifacts manifests for the client to apply."],["GetMembershipRequest","Request message for `GkeHubMembershipService.GetMembership` method."],["GkeCluster","GkeCluster contains information specific to GKE clusters."],["KubernetesMetadata","KubernetesMetadata provides informational metadata for Memberships representing Kubernetes clusters."],["KubernetesResource","KubernetesResource contains the YAML manifests and configuration for Membership Kubernetes resources in the cluster. After CreateMembership or UpdateMembership, these resources should be re-applied in the cluster."],["ListMembershipsRequest","Request message for `GkeHubMembershipService.ListMemberships` method."],["ListMembershipsResponse","Response message for the `GkeHubMembershipService.ListMemberships` method."],["Membership","Membership contains information about a member cluster."],["MembershipEndpoint","MembershipEndpoint contains information needed to contact a Kubernetes API, endpoint and any additional Kubernetes metadata."],["MembershipState","State of the Membership resource."],["OperationMetadata","Represents the metadata of the long-running operation."],["ResourceManifest","ResourceManifest represents a single Kubernetes resource to be applied to the cluster."],["ResourceOptions","ResourceOptions represent options for Kubernetes resource generation."],["TypeMeta","TypeMeta is the type information needed for content unmarshalling of Kubernetes resources in the manifest."],["UpdateMembershipRequest","Request message for `GkeHubMembershipService.UpdateMembership` method."],["ValidateExclusivityRequest","The request to validate the existing state of the membership CR in the cluster."],["ValidateExclusivityResponse","The response of exclusivity artifacts validation result status."]]});