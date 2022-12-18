// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryIndexRequest {
    /// The name of the repository. If empty the index is returned
    /// for all repositories.
    #[prost(string, tag="1")]
    pub repository_name: ::prost::alloc::string::String,
    /// If true return only models currently ready for inferencing.
    #[prost(bool, tag="2")]
    pub ready: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryIndexResponse {
    /// An index entry for each model.
    #[prost(message, repeated, tag="1")]
    pub models: ::prost::alloc::vec::Vec<repository_index_response::ModelIndex>,
}
/// Nested message and enum types in `RepositoryIndexResponse`.
pub mod repository_index_response {
    /// Index entry for a model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelIndex {
        /// The name of the model.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// The version of the model.
        #[prost(string, tag="2")]
        pub version: ::prost::alloc::string::String,
        /// The state of the model.
        #[prost(string, tag="3")]
        pub state: ::prost::alloc::string::String,
        /// The reason, if any, that the model is in the given state.
        #[prost(string, tag="4")]
        pub reason: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelLoadRequest {
    /// The name of the repository to load from. If empty the model
    /// is loaded from any repository.
    #[prost(string, tag="1")]
    pub repository_name: ::prost::alloc::string::String,
    /// The name of the model to load, or reload.
    #[prost(string, tag="2")]
    pub model_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelLoadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelUnloadRequest {
    /// The name of the repository from which the model was originally
    /// loaded. If empty the repository is not considered.
    #[prost(string, tag="1")]
    pub repository_name: ::prost::alloc::string::String,
    /// The name of the model to unload.
    #[prost(string, tag="2")]
    pub model_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryModelUnloadResponse {
}
include!("inference.model_repository.tonic.rs");
include!("inference.model_repository.serde.rs");
// @@protoc_insertion_point(module)