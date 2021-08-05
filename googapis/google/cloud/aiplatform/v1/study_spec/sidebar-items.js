initSidebarItems({"enum":[["Algorithm","The available search algorithms for the Study."],["MeasurementSelectionType","This indicates which measurement to use if/when the service automatically selects the final measurement from previously reported intermediate measurements. Choose this based on two considerations: A) Do you expect your measurements to monotonically improve? If so, choose LAST_MEASUREMENT. On the other hand, if you’re in a situation where your system can “over-train” and you expect the performance to get better for a while but then start declining, choose BEST_MEASUREMENT. B) Are your measurements significantly noisy and/or irreproducible? If so, BEST_MEASUREMENT will tend to be over-optimistic, and it may be better to choose LAST_MEASUREMENT. If both or neither of (A) and (B) apply, it doesn’t matter which selection type is chosen."],["ObservationNoise","Describes the noise level of the repeated observations."]],"mod":[["metric_spec","Nested message and enum types in `MetricSpec`."],["parameter_spec","Nested message and enum types in `ParameterSpec`."]],"struct":[["MetricSpec","Represents a metric to optimize."],["ParameterSpec","Represents a single parameter to optimize."]]});