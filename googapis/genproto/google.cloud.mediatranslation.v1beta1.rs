/// Provides information to the speech translation that specifies how to process
/// the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateSpeechConfig {
    /// Required. Encoding of audio data.
    /// Supported formats:
    ///
    /// - `linear16`
    ///
    ///   Uncompressed 16-bit signed little-endian samples (Linear PCM).
    ///
    ///
    #[prost(string, tag = "1")]
    pub audio_encoding: ::prost::alloc::string::String,
    /// Required. Source language code (BCP-47) of the input audio.
    #[prost(string, tag = "2")]
    pub source_language_code: ::prost::alloc::string::String,
    /// Optional. A list of up to 3 additional language codes (BCP-47), listing possible
    /// alternative languages of the supplied audio. If alternative source
    /// languages are listed, speech translation result will translate in the most
    /// likely language detected including the main source_language_code. The
    /// translated result will include the language code of the language detected
    /// in the audio.
    #[prost(string, repeated, tag = "6")]
    pub alternative_source_language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Target language code (BCP-47) of the output.
    #[prost(string, tag = "3")]
    pub target_language_code: ::prost::alloc::string::String,
    /// Optional. Sample rate in Hertz of the audio data. Valid values are:
    /// 8000-48000. 16000 is optimal. For best results, set the sampling rate of
    /// the audio source to 16000 Hz. If that's not possible, use the native sample
    /// rate of the audio source (instead of re-sampling). This field can only be
    /// omitted for `FLAC` and `WAV` audio files.
    #[prost(int32, tag = "4")]
    pub sample_rate_hertz: i32,
    /// Optional.
    #[prost(string, tag = "5")]
    pub model: ::prost::alloc::string::String,
}
/// Config used for streaming translation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechConfig {
    /// Required. The common config for all the following audio contents.
    #[prost(message, optional, tag = "1")]
    pub audio_config: ::core::option::Option<TranslateSpeechConfig>,
    /// Optional. If `false` or omitted, the system performs
    /// continuous translation (continuing to wait for and process audio even if
    /// the user pauses speaking) until the client closes the input stream (gRPC
    /// API) or until the maximum time limit has been reached. May return multiple
    /// `StreamingTranslateSpeechResult`s with the `is_final` flag set to `true`.
    ///
    /// If `true`, the speech translator will detect a single spoken utterance.
    /// When it detects that the user has paused or stopped speaking, it will
    /// return an `END_OF_SINGLE_UTTERANCE` event and cease translation.
    /// When the client receives 'END_OF_SINGLE_UTTERANCE' event, the client should
    /// stop sending the requests. However, clients should keep receiving remaining
    /// responses until the stream is terminated. To construct the complete
    /// sentence in a streaming way, one should override (if 'is_final' of previous
    /// response is false), or append (if 'is_final' of previous response is true).
    #[prost(bool, tag = "2")]
    pub single_utterance: bool,
}
/// The top-level message sent by the client for the `StreamingTranslateSpeech`
/// method. Multiple `StreamingTranslateSpeechRequest` messages are sent. The
/// first message must contain a `streaming_config` message and must not contain
/// `audio_content` data. All subsequent messages must contain `audio_content`
/// data and must not contain a `streaming_config` message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechRequest {
    /// The streaming request, which is either a streaming config or content.
    #[prost(
        oneof = "streaming_translate_speech_request::StreamingRequest",
        tags = "1, 2"
    )]
    pub streaming_request:
        ::core::option::Option<streaming_translate_speech_request::StreamingRequest>,
}
/// Nested message and enum types in `StreamingTranslateSpeechRequest`.
pub mod streaming_translate_speech_request {
    /// The streaming request, which is either a streaming config or content.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamingRequest {
        /// Provides information to the recognizer that specifies how to process the
        /// request. The first `StreamingTranslateSpeechRequest` message must contain
        /// a `streaming_config` message.
        #[prost(message, tag = "1")]
        StreamingConfig(super::StreamingTranslateSpeechConfig),
        /// The audio data to be translated. Sequential chunks of audio data are sent
        /// in sequential `StreamingTranslateSpeechRequest` messages. The first
        /// `StreamingTranslateSpeechRequest` message must not contain
        /// `audio_content` data and all subsequent `StreamingTranslateSpeechRequest`
        /// messages must contain `audio_content` data. The audio bytes must be
        /// encoded as specified in `StreamingTranslateSpeechConfig`. Note: as with
        /// all bytes fields, protobuffers use a pure binary representation (not
        /// base64).
        #[prost(bytes, tag = "2")]
        AudioContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// A streaming speech translation result corresponding to a portion of the audio
/// that is currently being processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechResult {
    /// Output only. The debug only recognition result in original language. This field is debug
    /// only and will be set to empty string if not available.
    /// This is implementation detail and will not be backward compatible.
    ///
    /// Still need to decide whether to expose this field by default.
    #[prost(string, tag = "3")]
    pub recognition_result: ::prost::alloc::string::String,
    /// Translation result.
    ///
    /// Use oneof field to reserve for future tts result.
    #[prost(oneof = "streaming_translate_speech_result::Result", tags = "1")]
    pub result: ::core::option::Option<streaming_translate_speech_result::Result>,
}
/// Nested message and enum types in `StreamingTranslateSpeechResult`.
pub mod streaming_translate_speech_result {
    /// Text translation result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextTranslationResult {
        /// Output only. The translated sentence.
        #[prost(string, tag = "1")]
        pub translation: ::prost::alloc::string::String,
        /// Output only. If `false`, this `StreamingTranslateSpeechResult` represents
        /// an interim result that may change. If `true`, this is the final time the
        /// translation service will return this particular
        /// `StreamingTranslateSpeechResult`, the streaming translator will not
        /// return any further hypotheses for this portion of the transcript and
        /// corresponding audio.
        #[prost(bool, tag = "2")]
        pub is_final: bool,
        /// Output only. The source language code (BCP-47) detected in the audio. Speech
        /// translation result will translate in the most likely language detected
        /// including the alternative source languages and main source_language_code.
        #[prost(string, tag = "3")]
        pub detected_source_language_code: ::prost::alloc::string::String,
    }
    /// Translation result.
    ///
    /// Use oneof field to reserve for future tts result.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Text translation result.
        #[prost(message, tag = "1")]
        TextTranslationResult(TextTranslationResult),
    }
}
/// A streaming speech translation response corresponding to a portion of
/// the audio currently processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechResponse {
    /// Output only. If set, returns a [google.rpc.Status][google.rpc.Status] message that
    /// specifies the error for the operation.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The translation result that is currently being processed (is_final could be
    /// true or false).
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<StreamingTranslateSpeechResult>,
    /// Output only. Indicates the type of speech event.
    #[prost(
        enumeration = "streaming_translate_speech_response::SpeechEventType",
        tag = "3"
    )]
    pub speech_event_type: i32,
}
/// Nested message and enum types in `StreamingTranslateSpeechResponse`.
pub mod streaming_translate_speech_response {
    /// Indicates the type of speech event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpeechEventType {
        /// No speech event specified.
        Unspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio (although it may subsequently return
        /// additional results). When the client receives 'END_OF_SINGLE_UTTERANCE'
        /// event, the client should stop sending the requests. However, clients
        /// should keep receiving remaining responses until the stream is terminated.
        /// To construct the complete sentence in a streaming way, one should
        /// override (if 'is_final' of previous response is false), or append (if
        /// 'is_final' of previous response is true). This event is only sent if
        /// `single_utterance` was set to `true`, and is not used otherwise.
        EndOfSingleUtterance = 1,
    }
}
#[doc = r" Generated client implementations."]
pub mod speech_translation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provides translation from/to media types."]
    pub struct SpeechTranslationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpeechTranslationServiceClient<T>
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
        #[doc = " Performs bidirectional streaming speech translation: receive results while"]
        #[doc = " sending audio. This method is only available via the gRPC API (not REST)."]
        pub async fn streaming_translate_speech(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingTranslateSpeechRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingTranslateSpeechResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.mediatranslation.v1beta1.SpeechTranslationService/StreamingTranslateSpeech") ;
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SpeechTranslationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SpeechTranslationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SpeechTranslationServiceClient {{ ... }}")
        }
    }
}
