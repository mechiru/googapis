initSidebarItems({"mod":[["assist_config","Nested message and enum types in `AssistConfig`."],["assist_request","Nested message and enum types in `AssistRequest`."],["assist_response","Nested message and enum types in `AssistResponse`."],["audio_in_config","Nested message and enum types in `AudioInConfig`."],["audio_out_config","Nested message and enum types in `AudioOutConfig`."],["device_location","Nested message and enum types in `DeviceLocation`."],["dialog_state_out","Nested message and enum types in `DialogStateOut`."],["embedded_assistant_client","Generated client implementations."],["screen_out","Nested message and enum types in `ScreenOut`."],["screen_out_config","Nested message and enum types in `ScreenOutConfig`."]],"struct":[["AssistConfig","Specifies how to process the `AssistRequest` messages."],["AssistRequest","The top-level message sent by the client. Clients must send at least two, and typically numerous `AssistRequest` messages. The first message must contain a `config` message and must not contain `audio_in` data. All subsequent messages must contain `audio_in` data and must not contain a `config` message."],["AssistResponse","The top-level message received by the client. A series of one or more `AssistResponse` messages are streamed back to the client."],["AudioInConfig","Specifies how to process the `audio_in` data that will be provided in subsequent requests. For recommended settings, see the Google Assistant SDK best practices."],["AudioOut","The audio containing the Assistant’s response to the query. Sequential chunks of audio data are received in sequential `AssistResponse` messages."],["AudioOutConfig","Specifies the desired format for the server to use when it returns `audio_out` messages."],["DebugConfig","Debugging parameters for the current request."],["DebugInfo","Debug info for developer. Only returned if request set `return_debug_info` to true."],["DeviceAction","The response returned to the device if the user has triggered a Device Action. For example, a device which supports the query Turn on the light would receive a `DeviceAction` with a JSON payload containing the semantics of the request."],["DeviceConfig","Required Fields that identify the device to the Assistant."],["DeviceLocation","There are three sources of locations. They are used with this precedence:"],["DialogStateIn","Provides information about the current dialog state."],["DialogStateOut","The dialog state resulting from the user’s query. Multiple of these messages may be received."],["ScreenOut","The Assistant’s visual output response to query. Enabled by `screen_out_config`."],["ScreenOutConfig","Specifies the desired format for the server to use when it returns `screen_out` response."],["SpeechRecognitionResult","The estimated transcription of a phrase the user has spoken. This could be a single segment or the full guess of the user’s spoken query."]]});