initSidebarItems({"enum":[["ClassificationType","Type of the classification problem."]],"mod":[["annotation_payload","Nested message and enum types in `AnnotationPayload`."],["auto_ml_client","Generated client implementations."],["batch_predict_input_config","Nested message and enum types in `BatchPredictInputConfig`."],["batch_predict_operation_metadata","Nested message and enum types in `BatchPredictOperationMetadata`."],["batch_predict_output_config","Nested message and enum types in `BatchPredictOutputConfig`."],["bounding_box_metrics_entry","Nested message and enum types in `BoundingBoxMetricsEntry`."],["classification_evaluation_metrics","Nested message and enum types in `ClassificationEvaluationMetrics`."],["dataset","Nested message and enum types in `Dataset`."],["deploy_model_request","Nested message and enum types in `DeployModelRequest`."],["document","Nested message and enum types in `Document`."],["document_dimensions","Nested message and enum types in `DocumentDimensions`."],["example_payload","Nested message and enum types in `ExamplePayload`."],["export_data_operation_metadata","Nested message and enum types in `ExportDataOperationMetadata`."],["export_model_operation_metadata","Nested message and enum types in `ExportModelOperationMetadata`."],["image","Nested message and enum types in `Image`."],["input_config","Nested message and enum types in `InputConfig`."],["model","Nested message and enum types in `Model`."],["model_evaluation","Nested message and enum types in `ModelEvaluation`."],["model_export_output_config","Nested message and enum types in `ModelExportOutputConfig`."],["operation_metadata","Nested message and enum types in `OperationMetadata`."],["output_config","Nested message and enum types in `OutputConfig`."],["prediction_service_client","Generated client implementations."],["text_extraction_annotation","Nested message and enum types in `TextExtractionAnnotation`."],["text_extraction_evaluation_metrics","Nested message and enum types in `TextExtractionEvaluationMetrics`."]],"struct":[["AnnotationPayload","Contains annotation information that is relevant to AutoML."],["AnnotationSpec","A definition of an annotation spec."],["BatchPredictInputConfig","Input configuration for BatchPredict Action."],["BatchPredictOperationMetadata","Details of BatchPredict operation."],["BatchPredictOutputConfig","Output configuration for BatchPredict Action."],["BatchPredictRequest","Request message for [PredictionService.BatchPredict][google.cloud.automl.v1.PredictionService.BatchPredict]."],["BatchPredictResult","Result of the Batch Predict. This message is returned in [response][google.longrunning.Operation.response] of the operation returned by the [PredictionService.BatchPredict][google.cloud.automl.v1.PredictionService.BatchPredict]."],["BoundingBoxMetricsEntry","Bounding box matching model metrics for a single intersection-over-union threshold and multiple label match confidence thresholds."],["BoundingPoly","A bounding polygon of a detected object on a plane. On output both vertices and normalized_vertices are provided. The polygon is formed by connecting vertices in the order they are listed."],["ClassificationAnnotation","Contains annotation details specific to classification."],["ClassificationEvaluationMetrics","Model evaluation metrics for classification problems. Note: For Video Classification this metrics only describe quality of the Video Classification predictions of “segment_classification” type."],["CreateDatasetOperationMetadata","Details of CreateDataset operation."],["CreateDatasetRequest","Request message for [AutoMl.CreateDataset][google.cloud.automl.v1.AutoMl.CreateDataset]."],["CreateModelOperationMetadata","Details of CreateModel operation."],["CreateModelRequest","Request message for [AutoMl.CreateModel][google.cloud.automl.v1.AutoMl.CreateModel]."],["Dataset","A workspace for solving a single, particular machine learning (ML) problem. A workspace contains examples that may be annotated."],["DeleteDatasetRequest","Request message for [AutoMl.DeleteDataset][google.cloud.automl.v1.AutoMl.DeleteDataset]."],["DeleteModelRequest","Request message for [AutoMl.DeleteModel][google.cloud.automl.v1.AutoMl.DeleteModel]."],["DeleteOperationMetadata","Details of operations that perform deletes of any entities."],["DeployModelOperationMetadata","Details of DeployModel operation."],["DeployModelRequest","Request message for [AutoMl.DeployModel][google.cloud.automl.v1.AutoMl.DeployModel]."],["Document","A structured text document e.g. a PDF."],["DocumentDimensions","Message that describes dimension of a document."],["DocumentInputConfig","Input configuration of a [Document][google.cloud.automl.v1.Document]."],["ExamplePayload","Example data used for training or prediction."],["ExportDataOperationMetadata","Details of ExportData operation."],["ExportDataRequest","Request message for [AutoMl.ExportData][google.cloud.automl.v1.AutoMl.ExportData]."],["ExportModelOperationMetadata","Details of ExportModel operation."],["ExportModelRequest","Request message for [AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel]. Models need to be enabled for exporting, otherwise an error code will be returned."],["GcsDestination","The Google Cloud Storage location where the output is to be written to."],["GcsSource","The Google Cloud Storage location for the input content."],["GetAnnotationSpecRequest","Request message for [AutoMl.GetAnnotationSpec][google.cloud.automl.v1.AutoMl.GetAnnotationSpec]."],["GetDatasetRequest","Request message for [AutoMl.GetDataset][google.cloud.automl.v1.AutoMl.GetDataset]."],["GetModelEvaluationRequest","Request message for [AutoMl.GetModelEvaluation][google.cloud.automl.v1.AutoMl.GetModelEvaluation]."],["GetModelRequest","Request message for [AutoMl.GetModel][google.cloud.automl.v1.AutoMl.GetModel]."],["Image","A representation of an image. Only images up to 30MB in size are supported."],["ImageClassificationDatasetMetadata","Dataset metadata that is specific to image classification."],["ImageClassificationModelDeploymentMetadata","Model deployment metadata specific to Image Classification."],["ImageClassificationModelMetadata","Model metadata for image classification."],["ImageObjectDetectionAnnotation","Annotation details for image object detection."],["ImageObjectDetectionDatasetMetadata","Dataset metadata specific to image object detection."],["ImageObjectDetectionEvaluationMetrics","Model evaluation metrics for image object detection problems. Evaluates prediction quality of labeled bounding boxes."],["ImageObjectDetectionModelDeploymentMetadata","Model deployment metadata specific to Image Object Detection."],["ImageObjectDetectionModelMetadata","Model metadata specific to image object detection."],["ImportDataOperationMetadata","Details of ImportData operation."],["ImportDataRequest","Request message for [AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData]."],["InputConfig","Input configuration for [AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData] action."],["ListDatasetsRequest","Request message for [AutoMl.ListDatasets][google.cloud.automl.v1.AutoMl.ListDatasets]."],["ListDatasetsResponse","Response message for [AutoMl.ListDatasets][google.cloud.automl.v1.AutoMl.ListDatasets]."],["ListModelEvaluationsRequest","Request message for [AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations]."],["ListModelEvaluationsResponse","Response message for [AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations]."],["ListModelsRequest","Request message for [AutoMl.ListModels][google.cloud.automl.v1.AutoMl.ListModels]."],["ListModelsResponse","Response message for [AutoMl.ListModels][google.cloud.automl.v1.AutoMl.ListModels]."],["Model","API proto representing a trained machine learning model."],["ModelEvaluation","Evaluation results of a model."],["ModelExportOutputConfig","Output configuration for ModelExport Action."],["NormalizedVertex","A vertex represents a 2D point in the image. The normalized vertex coordinates are between 0 to 1 fractions relative to the original plane (image, video). E.g. if the plane (e.g. whole image) would have size 10 x 20 then a point with normalized coordinates (0.1, 0.3) would be at the position (1, 6) on that plane."],["OperationMetadata","Metadata used across all long running operations returned by AutoML API."],["OutputConfig","For Translation: CSV file `translation.csv`, with each line in format: ML_USE,GCS_FILE_PATH GCS_FILE_PATH leads to a .TSV file which describes examples that have given ML_USE, using the following row format per line: TEXT_SNIPPET (in source language) \\t TEXT_SNIPPET (in target language)"],["PredictRequest","Request message for [PredictionService.Predict][google.cloud.automl.v1.PredictionService.Predict]."],["PredictResponse","Response message for [PredictionService.Predict][google.cloud.automl.v1.PredictionService.Predict]."],["TextClassificationDatasetMetadata","Dataset metadata for classification."],["TextClassificationModelMetadata","Model metadata that is specific to text classification."],["TextExtractionAnnotation","Annotation for identifying spans of text."],["TextExtractionDatasetMetadata","Dataset metadata that is specific to text extraction"],["TextExtractionEvaluationMetrics","Model evaluation metrics for text extraction problems."],["TextExtractionModelMetadata","Model metadata that is specific to text extraction."],["TextSegment","A contiguous part of a text (string), assuming it has an UTF-8 NFC encoding."],["TextSentimentAnnotation","Contains annotation details specific to text sentiment."],["TextSentimentDatasetMetadata","Dataset metadata for text sentiment."],["TextSentimentEvaluationMetrics","Model evaluation metrics for text sentiment problems."],["TextSentimentModelMetadata","Model metadata that is specific to text sentiment."],["TextSnippet","A representation of a text snippet."],["TranslationAnnotation","Annotation details specific to translation."],["TranslationDatasetMetadata","Dataset metadata that is specific to translation."],["TranslationEvaluationMetrics","Evaluation metrics for the dataset."],["TranslationModelMetadata","Model metadata that is specific to translation."],["UndeployModelOperationMetadata","Details of UndeployModel operation."],["UndeployModelRequest","Request message for [AutoMl.UndeployModel][google.cloud.automl.v1.AutoMl.UndeployModel]."],["UpdateDatasetRequest","Request message for [AutoMl.UpdateDataset][google.cloud.automl.v1.AutoMl.UpdateDataset]"],["UpdateModelRequest","Request message for [AutoMl.UpdateModel][google.cloud.automl.v1.AutoMl.UpdateModel]"]]});