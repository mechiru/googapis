initSidebarItems({"enum":[["MetricAggregation","Represents aggregation of metrics."],["MetricType","A metric’s value type."]],"mod":[["alpha_analytics_data_client","Generated client implementations."],["cohorts_range","Nested message and enum types in `CohortsRange`."],["dimension_expression","Nested message and enum types in `DimensionExpression`."],["dimension_value","Nested message and enum types in `DimensionValue`."],["filter","Nested message and enum types in `Filter`."],["filter_expression","Nested message and enum types in `FilterExpression`."],["metric_value","Nested message and enum types in `MetricValue`."],["numeric_value","Nested message and enum types in `NumericValue`."],["order_by","Nested message and enum types in `OrderBy`."]],"struct":[["BatchRunPivotReportsRequest","The batch request containing multiple pivot report requests."],["BatchRunPivotReportsResponse","The batch response containing multiple pivot reports."],["BatchRunReportsRequest","The batch request containing multiple report requests."],["BatchRunReportsResponse","The batch response containing multiple reports."],["Cohort","Defines a cohort selection criteria. A cohort is a group of users who share a common characteristic. For example, users with the same `firstTouchDate` belong to the same cohort."],["CohortReportSettings","Optional settings of a cohort report."],["CohortSpec","Specification of cohorts for a cohort report. Cohort reports can be used for example to create a time series of user retention for the cohort. For example, you could select the cohort of users that were acquired in the first week of September and follow that cohort for the next six weeks. Selecting the users acquired in the first week of September cohort is specified in the `cohort` object. Following that cohort for the next six weeks is specified in the `cohortsRange` object."],["CohortsRange","Configures the extended reporting date range for a cohort report. Specifies an offset duration to follow the cohorts over."],["DateRange","A contiguous set of days: startDate, startDate + 1, …, endDate. Requests are allowed up to 4 date ranges."],["Dimension","Dimensions are attributes of your data. For example, the dimension city indicates the city from which an event originates. Dimension values in report responses are strings; for example, city could be “Paris” or “New York”. Requests are allowed up to 8 dimensions."],["DimensionExpression","Used to express a dimension which is the result of a formula of multiple dimensions. Example usages:"],["DimensionHeader","Describes a dimension column in the report. Dimensions requested in a report produce column entries within rows and DimensionHeaders. However, dimensions used exclusively within filters or expressions do not produce columns in a report; correspondingly, those dimensions do not produce headers."],["DimensionMetadata","Explains a dimension."],["DimensionValue","The value of a dimension."],["Entity","The unique identifier of the property whose events are tracked."],["Filter","An expression to filter dimension or metric values."],["FilterExpression","To express dimension or metric filters. The fields in the same FilterExpression need to be either all dimensions or all metrics."],["FilterExpressionList","A list of filter expressions."],["GetMetadataRequest","Request for a property’s dimension and metric metadata."],["Metadata","The dimensions and metrics currently accepted in reporting methods."],["Metric","The quantitative measurements of a report. For example, the metric `eventCount` is the total number of events. Requests are allowed up to 10 metrics."],["MetricHeader","Describes a metric column in the report. Visible metrics requested in a report produce column entries within rows and MetricHeaders. However, metrics used exclusively within filters or expressions do not produce columns in a report; correspondingly, those metrics do not produce headers."],["MetricMetadata","Explains a metric."],["MetricValue","The value of a metric."],["NumericValue","To represent a number."],["OrderBy","The sort options."],["Pivot","Describes the visible dimension columns and rows in the report response."],["PivotDimensionHeader","Summarizes dimension values from a row for this pivot."],["PivotHeader","Dimensions’ values in a single pivot."],["PropertyQuota","Current state of all quotas for this Analytics Property. If any quota for a property is exhausted, all requests to that property will return Resource Exhausted errors."],["QuotaStatus","Current state for a particular quota group."],["ResponseMetaData","Response’s metadata carrying additional information about the report content."],["Row","Report data for each row. For example if RunReportRequest contains:"],["RunPivotReportRequest","The request to generate a pivot report."],["RunPivotReportResponse","The response pivot report table corresponding to a pivot request."],["RunRealtimeReportRequest","The request to generate a realtime report."],["RunRealtimeReportResponse","The response realtime report table corresponding to a request."],["RunReportRequest","The request to generate a report."],["RunReportResponse","The response report table corresponding to a request."]]});