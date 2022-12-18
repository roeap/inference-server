// @generated
///
/// ServerLive messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerLiveRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerLiveResponse {
    /// True if the inference server is live, false if not live.
    #[prost(bool, tag="1")]
    pub live: bool,
}
///
/// ServerReady messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReadyRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReadyResponse {
    /// True if the inference server is ready, false if not ready.
    #[prost(bool, tag="1")]
    pub ready: bool,
}
///
/// ModelReady messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReadyRequest {
    /// The name of the model to check for readiness.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The version of the model to check for readiness. If not given the
    /// server will choose a version based on the model and internal policy.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReadyResponse {
    /// True if the model is ready, false if not ready.
    #[prost(bool, tag="1")]
    pub ready: bool,
}
///
/// ServerMetadata messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadataRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadataResponse {
    /// The server name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The server version.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// The extensions supported by the server.
    #[prost(string, repeated, tag="3")]
    pub extensions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
/// ModelMetadata messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelMetadataRequest {
    /// The name of the model.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The version of the model to check for readiness. If not given the
    /// server will choose a version based on the model and internal policy.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelMetadataResponse {
    /// The model name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The versions of the model available on the server.
    #[prost(string, repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The model's platform. See Platforms.
    #[prost(string, tag="3")]
    pub platform: ::prost::alloc::string::String,
    /// The model's inputs.
    #[prost(message, repeated, tag="4")]
    pub inputs: ::prost::alloc::vec::Vec<model_metadata_response::TensorMetadata>,
    /// The model's outputs.
    #[prost(message, repeated, tag="5")]
    pub outputs: ::prost::alloc::vec::Vec<model_metadata_response::TensorMetadata>,
    /// Optional default parameters for the request / response.
    /// NOTE: This is an extension to the standard
    #[prost(map="string, message", tag="6")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, InferParameter>,
}
/// Nested message and enum types in `ModelMetadataResponse`.
pub mod model_metadata_response {
    /// Metadata for a tensor.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TensorMetadata {
        /// The tensor name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// The tensor data type.
        #[prost(string, tag="2")]
        pub datatype: ::prost::alloc::string::String,
        /// The tensor shape. A variable-size dimension is represented
        /// by a -1 value.
        #[prost(int64, repeated, tag="3")]
        pub shape: ::prost::alloc::vec::Vec<i64>,
        /// Optional default parameters for input.
        /// NOTE: This is an extension to the standard
        #[prost(map="string, message", tag="4")]
        pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, super::InferParameter>,
    }
}
///
/// ModelInfer messages.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInferRequest {
    /// The name of the model to use for inferencing.
    #[prost(string, tag="1")]
    pub model_name: ::prost::alloc::string::String,
    /// The version of the model to use for inference. If not given the
    /// server will choose a version based on the model and internal policy.
    #[prost(string, tag="2")]
    pub model_version: ::prost::alloc::string::String,
    /// Optional identifier for the request. If specified will be
    /// returned in the response.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// Optional inference parameters.
    #[prost(map="string, message", tag="4")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, InferParameter>,
    /// The input tensors for the inference.
    #[prost(message, repeated, tag="5")]
    pub inputs: ::prost::alloc::vec::Vec<model_infer_request::InferInputTensor>,
    /// The requested output tensors for the inference. Optional, if not
    /// specified all outputs produced by the model will be returned.
    #[prost(message, repeated, tag="6")]
    pub outputs: ::prost::alloc::vec::Vec<model_infer_request::InferRequestedOutputTensor>,
}
/// Nested message and enum types in `ModelInferRequest`.
pub mod model_infer_request {
    /// An input tensor for an inference request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferInputTensor {
        /// The tensor name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// The tensor data type.
        #[prost(string, tag="2")]
        pub datatype: ::prost::alloc::string::String,
        /// The tensor shape.
        #[prost(int64, repeated, tag="3")]
        pub shape: ::prost::alloc::vec::Vec<i64>,
        /// Optional inference input tensor parameters.
        #[prost(map="string, message", tag="4")]
        pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, super::InferParameter>,
        /// The input tensor data.
        #[prost(message, optional, tag="5")]
        pub contents: ::core::option::Option<super::InferTensorContents>,
    }
    /// An output tensor requested for an inference request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferRequestedOutputTensor {
        /// The tensor name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Optional requested output tensor parameters.
        #[prost(map="string, message", tag="2")]
        pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, super::InferParameter>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelInferResponse {
    /// The name of the model used for inference.
    #[prost(string, tag="1")]
    pub model_name: ::prost::alloc::string::String,
    /// The version of the model used for inference.
    #[prost(string, tag="2")]
    pub model_version: ::prost::alloc::string::String,
    /// The id of the inference request if one was specified.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    /// Optional inference response parameters.
    #[prost(map="string, message", tag="4")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, InferParameter>,
    /// The output tensors holding inference results.
    #[prost(message, repeated, tag="5")]
    pub outputs: ::prost::alloc::vec::Vec<model_infer_response::InferOutputTensor>,
}
/// Nested message and enum types in `ModelInferResponse`.
pub mod model_infer_response {
    /// An output tensor returned for an inference request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferOutputTensor {
        /// The tensor name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// The tensor data type.
        #[prost(string, tag="2")]
        pub datatype: ::prost::alloc::string::String,
        /// The tensor shape.
        #[prost(int64, repeated, tag="3")]
        pub shape: ::prost::alloc::vec::Vec<i64>,
        /// Optional output tensor parameters.
        #[prost(map="string, message", tag="4")]
        pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, super::InferParameter>,
        /// The output tensor data.
        #[prost(message, optional, tag="5")]
        pub contents: ::core::option::Option<super::InferTensorContents>,
    }
}
///
/// An inference parameter value.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferParameter {
    /// The parameter value can be a string, an int64, a boolean
    /// or a message specific to a predefined parameter.
    #[prost(oneof="infer_parameter::ParameterChoice", tags="1, 2, 3")]
    pub parameter_choice: ::core::option::Option<infer_parameter::ParameterChoice>,
}
/// Nested message and enum types in `InferParameter`.
pub mod infer_parameter {
    /// The parameter value can be a string, an int64, a boolean
    /// or a message specific to a predefined parameter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ParameterChoice {
        /// A boolean parameter value.
        #[prost(bool, tag="1")]
        BoolParam(bool),
        /// An int64 parameter value.
        #[prost(int64, tag="2")]
        Int64Param(i64),
        /// A string parameter value.
        #[prost(string, tag="3")]
        StringParam(::prost::alloc::string::String),
    }
}
///
/// The data contained in a tensor. For a given data type the
/// tensor contents can be represented in "raw" bytes form or in
/// the repeated type that matches the tensor's data type. Protobuf
/// oneof is not used because oneofs cannot contain repeated fields.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferTensorContents {
    /// Representation for BOOL data type. The size must match what is
    /// expected by the tensor's shape. The contents must be the flattened,
    /// one-dimensional, row-major order of the tensor elements.
    #[prost(bool, repeated, tag="1")]
    pub bool_contents: ::prost::alloc::vec::Vec<bool>,
    /// Representation for INT8, INT16, and INT32 data types. The size
    /// must match what is expected by the tensor's shape. The contents
    /// must be the flattened, one-dimensional, row-major order of the
    /// tensor elements.
    #[prost(int32, repeated, tag="2")]
    pub int_contents: ::prost::alloc::vec::Vec<i32>,
    /// Representation for INT64 data types. The size must match what
    /// is expected by the tensor's shape. The contents must be the
    /// flattened, one-dimensional, row-major order of the tensor elements.
    #[prost(int64, repeated, tag="3")]
    pub int64_contents: ::prost::alloc::vec::Vec<i64>,
    /// Representation for UINT8, UINT16, and UINT32 data types. The size
    /// must match what is expected by the tensor's shape. The contents
    /// must be the flattened, one-dimensional, row-major order of the
    /// tensor elements.
    #[prost(uint32, repeated, tag="4")]
    pub uint_contents: ::prost::alloc::vec::Vec<u32>,
    /// Representation for UINT64 data types. The size must match what
    /// is expected by the tensor's shape. The contents must be the
    /// flattened, one-dimensional, row-major order of the tensor elements.
    #[prost(uint64, repeated, tag="5")]
    pub uint64_contents: ::prost::alloc::vec::Vec<u64>,
    /// Representation for FP32 data type. The size must match what is
    /// expected by the tensor's shape. The contents must be the flattened,
    /// one-dimensional, row-major order of the tensor elements.
    #[prost(float, repeated, tag="6")]
    pub fp32_contents: ::prost::alloc::vec::Vec<f32>,
    /// Representation for FP64 data type. The size must match what is
    /// expected by the tensor's shape. The contents must be the flattened,
    /// one-dimensional, row-major order of the tensor elements.
    #[prost(double, repeated, tag="7")]
    pub fp64_contents: ::prost::alloc::vec::Vec<f64>,
    /// Representation for BYTES data type. The size must match what is
    /// expected by the tensor's shape. The contents must be the flattened,
    /// one-dimensional, row-major order of the tensor elements.
    #[prost(bytes="vec", repeated, tag="8")]
    pub bytes_contents: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
include!("inference.tonic.rs");
include!("inference.serde.rs");
// @@protoc_insertion_point(module)