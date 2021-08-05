initSidebarItems({"enum":[["IdentityType","Specifies the types of identities that are allowed access in either [IngressFrom] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressFrom] or [EgressFrom] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressFrom] rules."]],"mod":[["ingress_source","Nested message and enum types in `IngressSource`."],["method_selector","Nested message and enum types in `MethodSelector`."]],"struct":[["ApiOperation","Identification for an API Operation."],["EgressFrom","Defines the conditions under which an [EgressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy] matches a request. Conditions based on information about the source of the request. Note that if the destination of the request is also protected by a [ServicePerimeter] [google.identity.accesscontextmanager.v1.ServicePerimeter], then that [ServicePerimeter] [google.identity.accesscontextmanager.v1.ServicePerimeter] must have an [IngressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy] which allows access in order for this request to succeed."],["EgressPolicy","Policy for egress from perimeter."],["EgressTo","Defines the conditions under which an [EgressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.EgressPolicy] matches a request. Conditions are based on information about the [ApiOperation] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation] intended to be performed on the `resources` specified. Note that if the destination of the request is also protected by a [ServicePerimeter] [google.identity.accesscontextmanager.v1.ServicePerimeter], then that [ServicePerimeter] [google.identity.accesscontextmanager.v1.ServicePerimeter] must have an [IngressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy] which allows access in order for this request to succeed. The request must match `operations` AND `resources` fields in order to be allowed egress out of the perimeter."],["IngressFrom","Defines the conditions under which an [IngressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy] matches a request. Conditions are based on information about the source of the request. The request must satisfy what is defined in `sources` AND identity related fields in order to match."],["IngressPolicy","Policy for ingress into [ServicePerimeter] [google.identity.accesscontextmanager.v1.ServicePerimeter]."],["IngressSource","The source that [IngressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy] authorizes access from."],["IngressTo","Defines the conditions under which an [IngressPolicy] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.IngressPolicy] matches a request. Conditions are based on information about the [ApiOperation] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation] intended to be performed on the target resource of the request. The request must satisfy what is defined in `operations` AND `resources` in order to match."],["MethodSelector","An allowed method or permission of a service specified in [ApiOperation] [google.identity.accesscontextmanager.v1.ServicePerimeterConfig.ApiOperation]."],["VpcAccessibleServices","Specifies how APIs are allowed to communicate within the Service Perimeter."]]});