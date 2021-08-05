initSidebarItems({"enum":[["ContentOption","Options describing which parts of the provided content should be scanned."],["DlpJobType","An enum to represent the various types of DLP jobs."],["FileType","Definitions of file type groups to scan. New types will be added to this list."],["InfoTypeSupportedBy","Parts of the APIs which use certain infoTypes."],["Likelihood","Categorization of results based on how likely they are to represent a match, based on the number of elements they contain which imply a match."],["MatchingType","Type of the match which can be applied to different ways of matching, like Dictionary, regular expression and intersecting with findings of another info type."],["MetadataType","Type of metadata containing the finding."],["RelationalOperator","Operators available for comparing the value of fields."],["StoredInfoTypeState","State of a StoredInfoType version."]],"mod":[["action","Nested message and enum types in `Action`."],["analyze_data_source_risk_details","Nested message and enum types in `AnalyzeDataSourceRiskDetails`."],["big_query_options","Nested message and enum types in `BigQueryOptions`."],["bucketing_config","Nested message and enum types in `BucketingConfig`."],["byte_content_item","Nested message and enum types in `ByteContentItem`."],["chars_to_ignore","Nested message and enum types in `CharsToIgnore`."],["cloud_storage_options","Nested message and enum types in `CloudStorageOptions`."],["content_item","Nested message and enum types in `ContentItem`."],["content_location","Nested message and enum types in `ContentLocation`."],["create_dlp_job_request","Nested message and enum types in `CreateDlpJobRequest`."],["crypto_key","Nested message and enum types in `CryptoKey`."],["crypto_replace_ffx_fpe_config","Nested message and enum types in `CryptoReplaceFfxFpeConfig`."],["custom_info_type","Nested message and enum types in `CustomInfoType`."],["date_shift_config","Nested message and enum types in `DateShiftConfig`."],["date_time","Nested message and enum types in `DateTime`."],["deidentify_config","Nested message and enum types in `DeidentifyConfig`."],["dlp_job","Nested message and enum types in `DlpJob`."],["dlp_service_client","Generated client implementations."],["exclusion_rule","Nested message and enum types in `ExclusionRule`."],["field_transformation","Nested message and enum types in `FieldTransformation`."],["info_type_transformations","Nested message and enum types in `InfoTypeTransformations`."],["inspect_config","Nested message and enum types in `InspectConfig`."],["inspect_data_source_details","Nested message and enum types in `InspectDataSourceDetails`."],["inspection_rule","Nested message and enum types in `InspectionRule`."],["job_trigger","Nested message and enum types in `JobTrigger`."],["key","Nested message and enum types in `Key`."],["large_custom_dictionary_config","Nested message and enum types in `LargeCustomDictionaryConfig`."],["metadata_location","Nested message and enum types in `MetadataLocation`."],["output_storage_config","Nested message and enum types in `OutputStorageConfig`."],["primitive_transformation","Nested message and enum types in `PrimitiveTransformation`."],["privacy_metric","Nested message and enum types in `PrivacyMetric`."],["quasi_id","Nested message and enum types in `QuasiId`."],["quote_info","Nested message and enum types in `QuoteInfo`."],["record_condition","Nested message and enum types in `RecordCondition`."],["record_key","Nested message and enum types in `RecordKey`."],["redact_image_request","Nested message and enum types in `RedactImageRequest`."],["schedule","Nested message and enum types in `Schedule`."],["statistical_table","Nested message and enum types in `StatisticalTable`."],["storage_config","Nested message and enum types in `StorageConfig`."],["stored_info_type_config","Nested message and enum types in `StoredInfoTypeConfig`."],["stored_info_type_stats","Nested message and enum types in `StoredInfoTypeStats`."],["table","Nested message and enum types in `Table`."],["time_part_config","Nested message and enum types in `TimePartConfig`."],["transformation_error_handling","Nested message and enum types in `TransformationErrorHandling`."],["transformation_summary","Nested message and enum types in `TransformationSummary`."],["value","Nested message and enum types in `Value`."]],"struct":[["Action","A task to execute on the completion of a job. See https://cloud.google.com/dlp/docs/concepts-actions to learn more."],["ActivateJobTriggerRequest","Request message for ActivateJobTrigger."],["AnalyzeDataSourceRiskDetails","Result of a risk analysis operation request."],["BigQueryField","Message defining a field of a BigQuery table."],["BigQueryKey","Row key for identifying a record in BigQuery table."],["BigQueryOptions","Options defining BigQuery table and row identifiers."],["BigQueryTable","Message defining the location of a BigQuery table. A table is uniquely identified  by its project_id, dataset_id, and table_name. Within a query a table is often referenced with a string in the format of: `<project_id>:<dataset_id>.<table_id>` or `<project_id>.<dataset_id>.<table_id>`."],["BoundingBox","Bounding box encompassing detected text within an image."],["BucketingConfig","Generalization function that buckets values based on ranges. The ranges and replacement values are dynamically provided by the user for custom behavior, such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH This can be used on data of type: number, long, string, timestamp. If the bound `Value` type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more."],["ByteContentItem","Container for bytes to inspect or redact."],["CancelDlpJobRequest","The request message for canceling a DLP job."],["CharacterMaskConfig","Partially mask a string by replacing a given number of characters with a fixed character. Masking can start from the beginning or end of the string. This can be used on data of any type (numbers, longs, and so on) and when de-identifying structured data we’ll attempt to preserve the original data’s type. (This allows you to take a long like 123 and modify it to a string like **3."],["CharsToIgnore","Characters to skip when doing deidentification of a value. These will be left alone and skipped."],["CloudStorageFileSet","Message representing a set of files in Cloud Storage."],["CloudStorageOptions","Options defining a file or a set of files within a Google Cloud Storage bucket."],["CloudStoragePath","Message representing a single file or path in Cloud Storage."],["CloudStorageRegexFileSet","Message representing a set of files in a Cloud Storage bucket. Regular expressions are used to allow fine-grained control over which files in the bucket to include."],["Color","Represents a color in the RGB color space."],["Container","Represents a container that may contain DLP findings. Examples of a container include a file, table, or database record."],["ContentItem","Container structure for the content to inspect."],["ContentLocation","Precise location of the finding within a document, record, image, or metadata container."],["CreateDeidentifyTemplateRequest","Request message for CreateDeidentifyTemplate."],["CreateDlpJobRequest","Request message for CreateDlpJobRequest. Used to initiate long running jobs such as calculating risk metrics or inspecting Google Cloud Storage."],["CreateInspectTemplateRequest","Request message for CreateInspectTemplate."],["CreateJobTriggerRequest","Request message for CreateJobTrigger."],["CreateStoredInfoTypeRequest","Request message for CreateStoredInfoType."],["CryptoDeterministicConfig","Pseudonymization method that generates deterministic encryption for the given input. Outputs a base64 encoded representation of the encrypted output. Uses AES-SIV based on the RFC https://tools.ietf.org/html/rfc5297."],["CryptoHashConfig","Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. The key size must be either 32 or 64 bytes. Outputs a base64 encoded representation of the hashed output (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=). Currently, only string and integer values can be hashed. See https://cloud.google.com/dlp/docs/pseudonymization to learn more."],["CryptoKey","This is a data encryption key (DEK) (as opposed to a key encryption key (KEK) stored by KMS). When using KMS to wrap/unwrap DEKs, be sure to set an appropriate IAM policy on the KMS CryptoKey (KEK) to ensure an attacker cannot unwrap the data crypto key."],["CryptoReplaceFfxFpeConfig","Replaces an identifier with a surrogate using Format Preserving Encryption (FPE) with the FFX mode of operation; however when used in the `ReidentifyContent` API method, it serves the opposite function by reversing the surrogate back into the original identifier. The identifier must be encoded as ASCII. For a given crypto key and context, the same identifier will be replaced with the same surrogate. Identifiers must be at least two characters long. In the case that the identifier is the empty string, it will be skipped. See https://cloud.google.com/dlp/docs/pseudonymization to learn more."],["CustomInfoType","Custom information type provided by the user. Used to find domain-specific sensitive information configurable to the data in question."],["DatastoreKey","Record key for a finding in Cloud Datastore."],["DatastoreOptions","Options defining a data set within Google Cloud Datastore."],["DateShiftConfig","Shifts dates by random number of days, with option to be consistent for the same context. See https://cloud.google.com/dlp/docs/concepts-date-shifting to learn more."],["DateTime","Message for a date time object. e.g. 2018-01-01, 5th August."],["DeidentifyConfig","The configuration that controls how the data will change."],["DeidentifyContentRequest","Request to de-identify a list of items."],["DeidentifyContentResponse","Results of de-identifying a ContentItem."],["DeidentifyTemplate","DeidentifyTemplates contains instructions on how to de-identify content. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."],["DeleteDeidentifyTemplateRequest","Request message for DeleteDeidentifyTemplate."],["DeleteDlpJobRequest","The request message for deleting a DLP job."],["DeleteInspectTemplateRequest","Request message for DeleteInspectTemplate."],["DeleteJobTriggerRequest","Request message for DeleteJobTrigger."],["DeleteStoredInfoTypeRequest","Request message for DeleteStoredInfoType."],["DlpJob","Combines all of the information about a DLP job."],["DocumentLocation","Location of a finding within a document."],["EntityId","An entity in a dataset is a field or set of fields that correspond to a single person. For example, in medical records the `EntityId` might be a patient identifier, or for financial records it might be an account identifier. This message is used when generalizations or analysis must take into account that multiple rows correspond to the same entity."],["Error","Details information about an error encountered during job execution or the results of an unsuccessful activation of the JobTrigger."],["ExcludeInfoTypes","List of exclude infoTypes."],["ExclusionRule","The rule that specifies conditions when findings of infoTypes specified in `InspectionRuleSet` are removed from results."],["FieldId","General identifier of a data field in a storage service."],["FieldTransformation","The transformation to apply to the field."],["Finding","Represents a piece of potentially sensitive content."],["FinishDlpJobRequest","The request message for finishing a DLP hybrid job."],["FixedSizeBucketingConfig","Buckets values based on fixed size ranges. The Bucketing transformation can provide all of this functionality, but requires more configuration. This message is provided as a convenience to the user for simple bucketing strategies."],["GetDeidentifyTemplateRequest","Request message for GetDeidentifyTemplate."],["GetDlpJobRequest","The request message for [DlpJobs.GetDlpJob][]."],["GetInspectTemplateRequest","Request message for GetInspectTemplate."],["GetJobTriggerRequest","Request message for GetJobTrigger."],["GetStoredInfoTypeRequest","Request message for GetStoredInfoType."],["HybridContentItem","An individual hybrid item to inspect. Will be stored temporarily during processing."],["HybridFindingDetails","Populate to associate additional data with each finding."],["HybridInspectDlpJobRequest","Request to search for potentially sensitive info in a custom location."],["HybridInspectJobTriggerRequest","Request to search for potentially sensitive info in a custom location."],["HybridInspectResponse","Quota exceeded errors will be thrown once quota has been met."],["HybridInspectStatistics","Statistics related to processing hybrid inspect requests."],["HybridOptions","Configuration to control jobs where the content being inspected is outside of Google Cloud Platform."],["ImageLocation","Location of the finding within an image."],["InfoType","Type of information detected by the API."],["InfoTypeDescription","InfoType description."],["InfoTypeStats","Statistics regarding a specific InfoType."],["InfoTypeTransformations","A type of transformation that will scan unstructured text and apply various `PrimitiveTransformation`s to each finding, where the transformation is applied to only values that were identified as a specific info_type."],["InspectConfig","Configuration description of the scanning process. When used with redactContent only info_types and min_likelihood are currently used."],["InspectContentRequest","Request to search for potentially sensitive info in a ContentItem."],["InspectContentResponse","Results of inspecting an item."],["InspectDataSourceDetails","The results of an inspect DataSource job."],["InspectJobConfig","Controls what and how to inspect for findings."],["InspectResult","All the findings for a single scanned item."],["InspectTemplate","The inspectTemplate contains a configuration (set of types of sensitive data to be detected) to be used anywhere you otherwise would normally specify InspectConfig. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."],["InspectionRule","A single inspection rule to be applied to infoTypes, specified in `InspectionRuleSet`."],["InspectionRuleSet","Rule set for modifying a set of infoTypes to alter behavior under certain circumstances, depending on the specific details of the rules within the set."],["JobTrigger","Contains a configuration to make dlp api calls on a repeating basis. See https://cloud.google.com/dlp/docs/concepts-job-triggers to learn more."],["Key","A unique identifier for a Datastore entity. If a key’s partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts."],["KindExpression","A representation of a Datastore kind."],["KmsWrappedCryptoKey","Include to use an existing data crypto key wrapped by KMS. The wrapped key must be a 128/192/256 bit key. Authorization requires the following IAM permissions when sending a request to perform a crypto transformation using a kms-wrapped crypto key: dlp.kms.encrypt"],["LargeCustomDictionaryConfig","Configuration for a custom dictionary created from a data source of any size up to the maximum size defined in the limits page. The artifacts of dictionary creation are stored in the specified Google Cloud Storage location. Consider using `CustomInfoType.Dictionary` for smaller dictionaries that satisfy the size requirements."],["LargeCustomDictionaryStats","Summary statistics of a custom dictionary."],["ListDeidentifyTemplatesRequest","Request message for ListDeidentifyTemplates."],["ListDeidentifyTemplatesResponse","Response message for ListDeidentifyTemplates."],["ListDlpJobsRequest","The request message for listing DLP jobs."],["ListDlpJobsResponse","The response message for listing DLP jobs."],["ListInfoTypesRequest","Request for the list of infoTypes."],["ListInfoTypesResponse","Response to the ListInfoTypes request."],["ListInspectTemplatesRequest","Request message for ListInspectTemplates."],["ListInspectTemplatesResponse","Response message for ListInspectTemplates."],["ListJobTriggersRequest","Request message for ListJobTriggers."],["ListJobTriggersResponse","Response message for ListJobTriggers."],["ListStoredInfoTypesRequest","Request message for ListStoredInfoTypes."],["ListStoredInfoTypesResponse","Response message for ListStoredInfoTypes."],["Location","Specifies the location of the finding."],["Manual","Job trigger option for hybrid jobs. Jobs must be manually created and finished."],["MetadataLocation","Metadata Location"],["OutputStorageConfig","Cloud repository for storing output."],["PartitionId","Datastore partition ID. A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty."],["PrimitiveTransformation","A rule for transforming a value."],["PrivacyMetric","Privacy metric to compute for reidentification risk analysis."],["QuasiId","A column with a semantic tag attached."],["QuoteInfo","Message for infoType-dependent details parsed from quote."],["Range","Generic half-open interval [start, end)"],["RecordCondition","A condition for determining whether a transformation should be applied to a field."],["RecordKey","Message for a unique key indicating a record that contains a finding."],["RecordLocation","Location of a finding within a row or record."],["RecordSuppression","Configuration to suppress records whose suppression conditions evaluate to true."],["RecordTransformations","A type of transformation that is applied over structured data such as a table."],["RedactConfig","Redact a given value. For example, if used with an `InfoTypeTransformation` transforming PHONE_NUMBER, and input ‘My phone number is 206-555-0123’, the output would be ’My phone number is ’."],["RedactImageRequest","Request to search for potentially sensitive info in an image and redact it by covering it with a colored rectangle."],["RedactImageResponse","Results of redacting an image."],["ReidentifyContentRequest","Request to re-identify an item."],["ReidentifyContentResponse","Results of re-identifying a item."],["ReplaceValueConfig","Replace each input value with a given `Value`."],["ReplaceWithInfoTypeConfig","Replace each matching finding with the name of the info_type."],["RiskAnalysisJobConfig","Configuration for a risk analysis job. See https://cloud.google.com/dlp/docs/concepts-risk-analysis to learn more."],["Schedule","Schedule for triggeredJobs."],["StatisticalTable","An auxiliary table containing statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable)."],["StorageConfig","Shared message indicating Cloud storage type."],["StorageMetadataLabel","Storage metadata label to indicate which metadata entry contains findings."],["StoredInfoType","StoredInfoType resource message that contains information about the current version and any pending updates."],["StoredInfoTypeConfig","Configuration for stored infoTypes. All fields and subfield are provided by the user. For more information, see https://cloud.google.com/dlp/docs/creating-custom-infotypes."],["StoredInfoTypeStats","Statistics for a StoredInfoType."],["StoredInfoTypeVersion","Version of a StoredInfoType, including the configuration used to build it, create timestamp, and current state."],["StoredType","A reference to a StoredInfoType to use with scanning."],["Table","Structured content to inspect. Up to 50,000 `Value`s per request allowed. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more."],["TableLocation","Location of a finding within a table."],["TableOptions","Instructions regarding the table content being inspected."],["TimePartConfig","For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a portion of the value."],["TransformationErrorHandling","How to handle transformation errors during de-identification. A transformation error occurs when the requested transformation is incompatible with the data. For example, trying to de-identify an IP address using a `DateShift` transformation would result in a transformation error, since date info cannot be extracted from an IP address. Information about any incompatible transformations, and how they were handled, is returned in the response as part of the `TransformationOverviews`."],["TransformationOverview","Overview of the modifications that occurred."],["TransformationSummary","Summary of a single transformation. Only one of ‘transformation’, ‘field_transformation’, or ‘record_suppress’ will be set."],["TransientCryptoKey","Use this to have a random data crypto key generated. It will be discarded after the request finishes."],["UnwrappedCryptoKey","Using raw keys is prone to security risks due to accidentally leaking the key. Choose another type of key if possible."],["UpdateDeidentifyTemplateRequest","Request message for UpdateDeidentifyTemplate."],["UpdateInspectTemplateRequest","Request message for UpdateInspectTemplate."],["UpdateJobTriggerRequest","Request message for UpdateJobTrigger."],["UpdateStoredInfoTypeRequest","Request message for UpdateStoredInfoType."],["Value","Set of primitive values supported by the system. Note that for the purposes of inspection or transformation, the number of bytes considered to comprise a ‘Value’ is based on its representation as a UTF-8 encoded string. For example, if ‘integer_value’ is set to 123456789, the number of bytes would be counted as 9, even though an int64 only holds up to 8 bytes of data."],["ValueFrequency","A value of a field, including its frequency."]]});