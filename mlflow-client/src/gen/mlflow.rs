// @generated
/// Defines the set of options declared for every service RPC which are used to
/// direct RPCs to endpoints, as well as other metadata about the RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabricksRpcOptions {
    #[prost(message, repeated, tag="1")]
    pub endpoints: ::prost::alloc::vec::Vec<HttpEndpoint>,
    /// Indicates which users are allowed to initiate this RPC.
    #[prost(enumeration="Visibility", optional, tag="2")]
    pub visibility: ::core::option::Option<i32>,
    /// Complete definition of all error codes (from a statically defined set) which this method
    /// may return.
    #[prost(enumeration="ErrorCode", repeated, packed="false", tag="3")]
    pub error_codes: ::prost::alloc::vec::Vec<i32>,
    /// If defined, a rate limit will be applied to this RPC for all requests from the API proxy.
    #[prost(message, optional, tag="4")]
    pub rate_limit: ::core::option::Option<RateLimit>,
    /// If defined, overrides the default title used for in the API docs. See ProtobufDocGenerator
    /// for more info.
    #[prost(string, optional, tag="5")]
    pub rpc_doc_title: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpEndpoint {
    /// HTTP method like POST or GET.
    #[prost(string, optional, tag="1", default="POST")]
    pub method: ::core::option::Option<::prost::alloc::string::String>,
    /// Conceptual path of the API, like "/clusters" or "/clusters/create". Should start with a slash.
    #[prost(string, optional, tag="2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// A version like 1.1 which is prepended to the URL (e.g., GET /1.1/clusters).
    /// Breaking changes to an RPC must use a different version number.
    #[prost(message, optional, tag="3")]
    pub since: ::core::option::Option<ApiVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiVersion {
    #[prost(int32, optional, tag="1")]
    pub major: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub minor: ::core::option::Option<i32>,
}
/// API rate limits applied to RPCs coming from the API Proxy. The rate limits are applied on a
/// per organization basis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// The maximum burst of API requests allowed for a single endpoint. In the context of the
    /// token bucket algorithm, this constant represents the total capacity of the token bucket.
    #[prost(int64, optional, tag="1")]
    pub max_burst: ::core::option::Option<i64>,
    /// The maximum sustained request per second limit for a single endpoint. In the context of the,
    /// token bucket algorithm, this constant represents the rate at which the token bucket fills.
    #[prost(int64, optional, tag="2")]
    pub max_sustained_per_second: ::core::option::Option<i64>,
}
/// A block of documentation that is added to the AST after parsing the original protocol buffer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationMetadata {
    /// The string of documentation attached to this particular item.
    #[prost(string, optional, tag="1")]
    pub docstring: ::core::option::Option<::prost::alloc::string::String>,
    /// The string of documentation that is *before* this item. This only makes sense for top-level
    /// items such as (top-level) messages, (top-level) enumerations, or services. In all other
    /// cases, this string is empty.
    #[prost(string, optional, tag="2")]
    pub lead_doc: ::core::option::Option<::prost::alloc::string::String>,
    /// The visibility level when the docstring was generated.
    /// The documentation extractor builds multiple versions of the documentation, one for each
    /// visibility level. The documentation is then generated for each visibility level.
    #[prost(enumeration="Visibility", optional, tag="3")]
    pub visibility: ::core::option::Option<i32>,
    /// The original proto path in the internal representation. This is useful when performing field
    /// flattening to figure out what the original field was.
    /// One example is \["jobs","Run","original_attempt_run_id"\] for jobs.
    /// This path is unique.
    #[prost(string, repeated, tag="4")]
    pub original_proto_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The location (line number) of the start of the documentation. This is required to keep the
    /// pieces of documentation sorted.
    #[prost(int32, optional, tag="5")]
    pub position: ::core::option::Option<i32>,
}
/// Serialization format for DatabricksServiceException.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabricksServiceExceptionProto {
    #[prost(enumeration="ErrorCode", optional, tag="1")]
    pub error_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub stack_trace: ::core::option::Option<::prost::alloc::string::String>,
}
/// Visibility defines who is allowed to use the RPC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    /// Public indicates visible to both external and internal customers.
    Public = 1,
    /// Internal is only available to Databricks-internal clients.
    Internal = 2,
    /// Public-undocumented are accessible via public endpoints, but not documented. This is useful
    /// for internal clients that depend on public endpoints (e.g. workflows running in the driver).
    PublicUndocumented = 3,
}
impl Visibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Visibility::Public => "PUBLIC",
            Visibility::Internal => "INTERNAL",
            Visibility::PublicUndocumented => "PUBLIC_UNDOCUMENTED",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    ///
    /// Internal, system-level error codes, which generally cannot be resolved by the user, but
    /// instead are due to service issues.
    ///
    /// Generic internal error occurred.
    InternalError = 1,
    /// An internal system could not be contacted due to a period of unavailability.
    TemporarilyUnavailable = 2,
    /// Indicates that an IOException has been internally thrown.
    IoError = 3,
    /// The request is invalid.
    BadRequest = 4,
    ///
    /// Common application-level error codes, which were caused by the user input but may be returned
    /// by multiple services.
    ///
    /// Supplied value for a parameter was invalid (e.g., giving a number for a string parameter).
    InvalidParameterValue = 1000,
    /// Indicates that the given API endpoint does not exist.
    EndpointNotFound = 1001,
    /// Indicates that the given API request was malformed.
    MalformedRequest = 1002,
    /// If one or more of the inputs to a given RPC are not in a valid state for the action.
    InvalidState = 1003,
    /// If a given user/entity doesn't have the required permission(s) to perform an action
    PermissionDenied = 1004,
    /// If a given user/entity is trying to use a feature which has been disabled
    FeatureDisabled = 1005,
    /// If customer-provided credentials are not authorized to perform an operation
    CustomerUnauthorized = 1006,
    /// If the API request is rejected due to throttling
    RequestLimitExceeded = 1007,
    // /////////
    // VAULT //
    // /////////

    /// If the user attempts to perform an invalid state transition on a shard.
    InvalidStateTransition = 2001,
    /// Unable to perform the operation because the shard was locked by some other operation.
    CouldNotAcquireLock = 2002,
    // /////////////
    // EXECUTION //
    // /////////////

    /// Operation was performed on a resource that already exists.
    ResourceAlreadyExists = 3001,
    /// Operation was performed on a resource that does not exist.
    ResourceDoesNotExist = 3002,
    // /////////
    // DBFS ///
    // /////////

    QuotaExceeded = 4001,
    MaxBlockSizeExceeded = 4002,
    MaxReadSizeExceeded = 4003,
    // ////////////
    // CLUSTERS //
    // ////////////

    DryRunFailed = 5001,
    /// Cluster request was rejected because it would exceed a resource limit.
    ResourceLimitExceeded = 5002,
    // ////////////
    // WORKSPACE //
    // ////////////

    DirectoryNotEmpty = 6001,
    DirectoryProtected = 6002,
    MaxNotebookSizeExceeded = 6003,
}
impl ErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorCode::InternalError => "INTERNAL_ERROR",
            ErrorCode::TemporarilyUnavailable => "TEMPORARILY_UNAVAILABLE",
            ErrorCode::IoError => "IO_ERROR",
            ErrorCode::BadRequest => "BAD_REQUEST",
            ErrorCode::InvalidParameterValue => "INVALID_PARAMETER_VALUE",
            ErrorCode::EndpointNotFound => "ENDPOINT_NOT_FOUND",
            ErrorCode::MalformedRequest => "MALFORMED_REQUEST",
            ErrorCode::InvalidState => "INVALID_STATE",
            ErrorCode::PermissionDenied => "PERMISSION_DENIED",
            ErrorCode::FeatureDisabled => "FEATURE_DISABLED",
            ErrorCode::CustomerUnauthorized => "CUSTOMER_UNAUTHORIZED",
            ErrorCode::RequestLimitExceeded => "REQUEST_LIMIT_EXCEEDED",
            ErrorCode::InvalidStateTransition => "INVALID_STATE_TRANSITION",
            ErrorCode::CouldNotAcquireLock => "COULD_NOT_ACQUIRE_LOCK",
            ErrorCode::ResourceAlreadyExists => "RESOURCE_ALREADY_EXISTS",
            ErrorCode::ResourceDoesNotExist => "RESOURCE_DOES_NOT_EXIST",
            ErrorCode::QuotaExceeded => "QUOTA_EXCEEDED",
            ErrorCode::MaxBlockSizeExceeded => "MAX_BLOCK_SIZE_EXCEEDED",
            ErrorCode::MaxReadSizeExceeded => "MAX_READ_SIZE_EXCEEDED",
            ErrorCode::DryRunFailed => "DRY_RUN_FAILED",
            ErrorCode::ResourceLimitExceeded => "RESOURCE_LIMIT_EXCEEDED",
            ErrorCode::DirectoryNotEmpty => "DIRECTORY_NOT_EMPTY",
            ErrorCode::DirectoryProtected => "DIRECTORY_PROTECTED",
            ErrorCode::MaxNotebookSizeExceeded => "MAX_NOTEBOOK_SIZE_EXCEEDED",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactCredentialInfo {
    /// The ID of the MLflow Run containing the artifact that can be accessed
    /// with the credential
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The path, relative to the Run's artifact root location, of the artifact
    /// that can be accessed with the credential
    #[prost(string, optional, tag="2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// The signed URI credential that provides access to the artifact
    #[prost(string, optional, tag="3")]
    pub signed_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// A collection of HTTP headers that should be specified when uploading to
    /// or downloading from the specified `signed_uri`
    #[prost(message, repeated, tag="4")]
    pub headers: ::prost::alloc::vec::Vec<artifact_credential_info::HttpHeader>,
    /// The type of the signed credential URI (e.g., an AWS presigned URL
    /// or an Azure Shared Access Signature URI)
    #[prost(enumeration="ArtifactCredentialType", optional, tag="5")]
    pub r#type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ArtifactCredentialInfo`.
pub mod artifact_credential_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpHeader {
        /// The HTTP header name
        #[prost(string, optional, tag="1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        /// The HTTP header value
        #[prost(string, optional, tag="2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialsForRead {
    /// The ID of the MLflow Run for which to fetch artifact read credentials
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The artifact paths, relative to the Run's artifact root location, for which to
    /// fetch artifact read credentials. Must not be empty.
    #[prost(string, repeated, tag="2")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Token specifying the page of credentials to fetch for large requests that require pagination
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetCredentialsForRead`.
pub mod get_credentials_for_read {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Credentials for reading from the specified artifact locations
        #[prost(message, repeated, tag="2")]
        pub credential_infos: ::prost::alloc::vec::Vec<super::ArtifactCredentialInfo>,
        /// Token used to fetch the next page of credentials for large requests that require pagination
        #[prost(string, optional, tag="3")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialsForWrite {
    /// The ID of the MLflow Run for which to fetch artifact write credentials
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The artifact paths, relative to the Run's artifact root location, for which to
    /// fetch artifact write credentials. Must not be empty.
    #[prost(string, repeated, tag="2")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Token specifying the page of credentials to fetch for large requests that require pagination
    #[prost(string, optional, tag="3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetCredentialsForWrite`.
pub mod get_credentials_for_write {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Credentials for writing to the specified artifact locations
        #[prost(message, repeated, tag="2")]
        pub credential_infos: ::prost::alloc::vec::Vec<super::ArtifactCredentialInfo>,
        /// Token used to fetch the next page of credentials for large requests that require pagination
        #[prost(string, optional, tag="3")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// The type of a given artifact access credential
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArtifactCredentialType {
    /// The credential is an Azure Shared Access Signature URI. For more information, see
    /// <https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview>
    AzureSasUri = 1,
    /// The credential is an AWS Presigned URL. For more information, see
    /// <https://docs.aws.amazon.com/AmazonS3/latest/dev/ShareObjectPreSignedURL.html>
    AwsPresignedUrl = 2,
    /// The credential is a GCP Signed URL. For more information, see
    /// <https://cloud.google.com/storage/docs/access-control/signed-urls>
    GcpSignedUrl = 3,
    /// The credential is an Azure Shared Access Signature URI for ADLS.  For more
    /// information see
    /// <https://docs.microsoft.com/en-us/rest/api/storageservices/data-lake-storage-gen2>
    /// and
    /// <https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview>
    AzureAdlsGen2SasUri = 4,
}
impl ArtifactCredentialType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArtifactCredentialType::AzureSasUri => "AZURE_SAS_URI",
            ArtifactCredentialType::AwsPresignedUrl => "AWS_PRESIGNED_URL",
            ArtifactCredentialType::GcpSignedUrl => "GCP_SIGNED_URL",
            ArtifactCredentialType::AzureAdlsGen2SasUri => "AZURE_ADLS_GEN2_SAS_URI",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredModel {
    /// Unique name for the model.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Timestamp recorded when this ``registered_model`` was created.
    #[prost(int64, optional, tag="2")]
    pub creation_timestamp: ::core::option::Option<i64>,
    /// Timestamp recorded when metadata for this ``registered_model`` was last updated.
    #[prost(int64, optional, tag="3")]
    pub last_updated_timestamp: ::core::option::Option<i64>,
    /// User that created this ``registered_model``
    /// NOTE: this field is not currently returned.
    #[prost(string, optional, tag="4")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of this ``registered_model``.
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Collection of latest model versions for each stage.
    /// Only contains models with current ``READY`` status.
    #[prost(message, repeated, tag="6")]
    pub latest_versions: ::prost::alloc::vec::Vec<ModelVersion>,
    /// Tags: Additional metadata key-value pairs for this ``registered_model``.
    #[prost(message, repeated, tag="7")]
    pub tags: ::prost::alloc::vec::Vec<RegisteredModelTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelVersion {
    /// Unique name of the model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model's version number.
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Timestamp recorded when this ``model_version`` was created.
    #[prost(int64, optional, tag="3")]
    pub creation_timestamp: ::core::option::Option<i64>,
    /// Timestamp recorded when metadata for this ``model_version`` was last updated.
    #[prost(int64, optional, tag="4")]
    pub last_updated_timestamp: ::core::option::Option<i64>,
    /// User that created this ``model_version``.
    #[prost(string, optional, tag="5")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Current stage for this ``model_version``.
    #[prost(string, optional, tag="6")]
    pub current_stage: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of this ``model_version``.
    #[prost(string, optional, tag="7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// URI indicating the location of the source model artifacts, used when creating ``model_version``
    #[prost(string, optional, tag="8")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    /// MLflow run ID used when creating ``model_version``, if ``source`` was generated by an
    /// experiment run stored in MLflow tracking server.
    #[prost(string, optional, tag="9")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Current status of ``model_version``
    #[prost(enumeration="ModelVersionStatus", optional, tag="10")]
    pub status: ::core::option::Option<i32>,
    /// Details on current ``status``, if it is pending or failed.
    #[prost(string, optional, tag="11")]
    pub status_message: ::core::option::Option<::prost::alloc::string::String>,
    /// Tags: Additional metadata key-value pairs for this ``model_version``.
    #[prost(message, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<ModelVersionTag>,
    /// Run Link: Direct link to the run that generated this version. This field is set at model version creation time
    /// only for model versions whose source run is from a tracking server that is different from the registry server.
    #[prost(string, optional, tag="13")]
    pub run_link: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegisteredModel {
    /// Register models under this name
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional metadata for registered model.
    #[prost(message, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<RegisteredModelTag>,
    /// Optional description for registered model.
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `CreateRegisteredModel`.
pub mod create_registered_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag="1")]
        pub registered_model: ::core::option::Option<super::RegisteredModel>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRegisteredModel {
    /// Registered model unique name identifier.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// If provided, updates the name for this ``registered_model``.
    #[prost(string, optional, tag="2")]
    pub new_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RenameRegisteredModel`.
pub mod rename_registered_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag="1")]
        pub registered_model: ::core::option::Option<super::RegisteredModel>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegisteredModel {
    /// Registered model unique name identifier.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// If provided, updates the description for this ``registered_model``.
    #[prost(string, optional, tag="2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UpdateRegisteredModel`.
pub mod update_registered_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag="1")]
        pub registered_model: ::core::option::Option<super::RegisteredModel>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegisteredModel {
    /// Registered model unique name identifier.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteRegisteredModel`.
pub mod delete_registered_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegisteredModel {
    /// Registered model unique name identifier.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetRegisteredModel`.
pub mod get_registered_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag="1")]
        pub registered_model: ::core::option::Option<super::RegisteredModel>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRegisteredModels {
    /// String filter condition, like "name LIKE 'my-model-name'".
    /// Interpreted in the backend automatically as "name LIKE '%my-model-name%'".
    /// Single boolean condition, with string values wrapped in single quotes.
    #[prost(string, optional, tag="1")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Maximum number of models desired. Default is 100. Max threshold is 1000.
    #[prost(int64, optional, tag="2", default="100")]
    pub max_results: ::core::option::Option<i64>,
    /// List of columns for ordering search results, which can include model name and last updated
    /// timestamp with an optional "DESC" or "ASC" annotation, where "ASC" is the default.
    /// Tiebreaks are done by model name ASC.
    #[prost(string, repeated, tag="3")]
    pub order_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pagination token to go to the next page based on a previous search query.
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SearchRegisteredModels`.
pub mod search_registered_models {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Registered Models that match the search criteria.
        #[prost(message, repeated, tag="1")]
        pub registered_models: ::prost::alloc::vec::Vec<super::RegisteredModel>,
        /// Pagination token to request the next page of models.
        #[prost(string, optional, tag="2")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestVersions {
    /// Registered model unique name identifier.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// List of stages.
    #[prost(string, repeated, tag="2")]
    pub stages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetLatestVersions`.
pub mod get_latest_versions {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Latest version models for each requests stage. Only return models with current ``READY`` status.
        /// If no ``stages`` provided, returns the latest version for each stage, including ``"None"``.
        #[prost(message, repeated, tag="1")]
        pub model_versions: ::prost::alloc::vec::Vec<super::ModelVersion>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelVersion {
    /// Register model under this name
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// URI indicating the location of the model artifacts.
    #[prost(string, optional, tag="2")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    /// MLflow run ID for correlation, if ``source`` was generated by an experiment run in
    /// MLflow tracking server
    #[prost(string, optional, tag="3")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional metadata for model version.
    #[prost(message, repeated, tag="4")]
    pub tags: ::prost::alloc::vec::Vec<ModelVersionTag>,
    /// MLflow run link - this is the exact link of the run that generated this model version,
    /// potentially hosted at another instance of MLflow.
    #[prost(string, optional, tag="5")]
    pub run_link: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional description for model version.
    #[prost(string, optional, tag="6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `CreateModelVersion`.
pub mod create_model_version {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Return new version number generated for this model in registry.
        #[prost(message, optional, tag="1")]
        pub model_version: ::core::option::Option<super::ModelVersion>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModelVersion {
    /// Name of the registered model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// If provided, updates the description for this ``registered_model``.
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UpdateModelVersion`.
pub mod update_model_version {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Return new version number generated for this model in registry.
        #[prost(message, optional, tag="1")]
        pub model_version: ::core::option::Option<super::ModelVersion>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionModelVersionStage {
    /// Name of the registered model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Transition `model_version` to new stage.
    #[prost(string, optional, tag="3")]
    pub stage: ::core::option::Option<::prost::alloc::string::String>,
    /// When transitioning a model version to a particular stage, this flag dictates whether all
    /// existing model versions in that stage should be atomically moved to the "archived" stage.
    /// This ensures that at-most-one model version exists in the target stage.
    /// This field is *required* when transitioning a model versions's stage
    #[prost(bool, optional, tag="4")]
    pub archive_existing_versions: ::core::option::Option<bool>,
}
/// Nested message and enum types in `TransitionModelVersionStage`.
pub mod transition_model_version_stage {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Updated model version
        #[prost(message, optional, tag="1")]
        pub model_version: ::core::option::Option<super::ModelVersion>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelVersion {
    /// Name of the registered model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteModelVersion`.
pub mod delete_model_version {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelVersion {
    /// Name of the registered model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetModelVersion`.
pub mod get_model_version {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag="1")]
        pub model_version: ::core::option::Option<super::ModelVersion>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchModelVersions {
    /// String filter condition, like "name='my-model-name'". Must be a single boolean condition,
    /// with string values wrapped in single quotes.
    #[prost(string, optional, tag="1")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Maximum number of models desired. Max threshold is 200K.
    #[prost(int64, optional, tag="2", default="200000")]
    pub max_results: ::core::option::Option<i64>,
    /// List of columns to be ordered by including model name, version, stage with an
    /// optional "DESC" or "ASC" annotation, where "ASC" is the default.
    /// Tiebreaks are done by latest stage transition timestamp, followed by name ASC, followed by
    /// version DESC.
    #[prost(string, repeated, tag="3")]
    pub order_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pagination token to go to next page based on previous search query.
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SearchModelVersions`.
pub mod search_model_versions {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Models that match the search criteria
        #[prost(message, repeated, tag="1")]
        pub model_versions: ::prost::alloc::vec::Vec<super::ModelVersion>,
        /// Pagination token to request next page of models for the same search query.
        #[prost(string, optional, tag="2")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelVersionDownloadUri {
    /// Name of the registered model
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetModelVersionDownloadUri`.
pub mod get_model_version_download_uri {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// URI corresponding to where artifacts for this model version are stored.
        #[prost(string, optional, tag="1")]
        pub artifact_uri: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// Tag for a model version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelVersionTag {
    /// The tag key.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The tag value.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Tag for a registered model
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredModelTag {
    /// The tag key.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The tag value.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRegisteredModelTag {
    /// Unique name of the model.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. Maximum size depends on storage backend.
    /// If a tag with this name already exists, its preexisting value will be replaced by the specified `value`.
    /// All storage backends are guaranteed to support key values up to 250 bytes in size.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// String value of the tag being logged. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 5000 bytes in size.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SetRegisteredModelTag`.
pub mod set_registered_model_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModelVersionTag {
    /// Unique name of the model.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number.
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. Maximum size depends on storage backend.
    /// If a tag with this name already exists, its preexisting value will be replaced by the specified `value`.
    /// All storage backends are guaranteed to support key values up to 250 bytes in size.
    #[prost(string, optional, tag="3")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// String value of the tag being logged. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 5000 bytes in size.
    #[prost(string, optional, tag="4")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SetModelVersionTag`.
pub mod set_model_version_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegisteredModelTag {
    /// Name of the registered model that the tag was logged under.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteRegisteredModelTag`.
pub mod delete_registered_model_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelVersionTag {
    /// Name of the registered model that the tag was logged under.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Model version number that the tag was logged under.
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes.
    #[prost(string, optional, tag="3")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteModelVersionTag`.
pub mod delete_model_version_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelVersionStatus {
    /// Request to register a new model version is pending as server performs background tasks.
    PendingRegistration = 1,
    /// Request to register a new model version has failed.
    FailedRegistration = 2,
    /// Model version is ready for use.
    Ready = 3,
}
impl ModelVersionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModelVersionStatus::PendingRegistration => "PENDING_REGISTRATION",
            ModelVersionStatus::FailedRegistration => "FAILED_REGISTRATION",
            ModelVersionStatus::Ready => "READY",
        }
    }
}
/// Metric associated with a run, represented as a key-value pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// Key identifying this metric.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Value associated with this metric.
    #[prost(double, optional, tag="2")]
    pub value: ::core::option::Option<f64>,
    /// The timestamp at which this metric was recorded.
    #[prost(int64, optional, tag="3")]
    pub timestamp: ::core::option::Option<i64>,
    /// Step at which to log the metric.
    #[prost(int64, optional, tag="4", default="0")]
    pub step: ::core::option::Option<i64>,
}
/// Param associated with a run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
    /// Key identifying this param.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Value associated with this param.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// A single run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Run {
    /// Run metadata.
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<RunInfo>,
    /// Run data.
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<RunData>,
}
/// Run data (metrics, params, and tags).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunData {
    /// Run metrics.
    #[prost(message, repeated, tag="1")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// Run parameters.
    #[prost(message, repeated, tag="2")]
    pub params: ::prost::alloc::vec::Vec<Param>,
    /// Additional metadata key-value pairs.
    #[prost(message, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<RunTag>,
}
/// Tag for a run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTag {
    /// The tag key.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The tag value.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Tag for an experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentTag {
    /// The tag key.
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The tag value.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Metadata of a single run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInfo {
    /// Unique identifier for the run.
    #[prost(string, optional, tag="15")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] Unique identifier for the run. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the run.
    #[prost(string, optional, tag="3")]
    pub run_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The experiment ID.
    #[prost(string, optional, tag="2")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    /// User who initiated the run.
    /// This field is deprecated as of MLflow 1.0, and will be removed in a future
    /// MLflow release. Use 'mlflow.user' tag instead.
    #[prost(string, optional, tag="6")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Current status of the run.
    #[prost(enumeration="RunStatus", optional, tag="7")]
    pub status: ::core::option::Option<i32>,
    /// Unix timestamp of when the run started in milliseconds.
    #[prost(int64, optional, tag="8")]
    pub start_time: ::core::option::Option<i64>,
    /// Unix timestamp of when the run ended in milliseconds.
    #[prost(int64, optional, tag="9")]
    pub end_time: ::core::option::Option<i64>,
    /// URI of the directory where artifacts should be uploaded.
    /// This can be a local path (starting with "/"), or a distributed file system (DFS)
    /// path, like ``s3://bucket/directory`` or ``dbfs:/my/directory``.
    /// If not set, the local ``./mlruns`` directory is  chosen.
    #[prost(string, optional, tag="13")]
    pub artifact_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Current life cycle stage of the experiment : OneOf("active", "deleted")
    #[prost(string, optional, tag="14")]
    pub lifecycle_stage: ::core::option::Option<::prost::alloc::string::String>,
}
/// Experiment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Experiment {
    /// Unique identifier for the experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable name that identifies the experiment.
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Location where artifacts for the experiment are stored.
    #[prost(string, optional, tag="3")]
    pub artifact_location: ::core::option::Option<::prost::alloc::string::String>,
    /// Current life cycle stage of the experiment: "active" or "deleted".
    /// Deleted experiments are not returned by APIs.
    #[prost(string, optional, tag="4")]
    pub lifecycle_stage: ::core::option::Option<::prost::alloc::string::String>,
    /// Last update time
    #[prost(int64, optional, tag="5")]
    pub last_update_time: ::core::option::Option<i64>,
    /// Creation time
    #[prost(int64, optional, tag="6")]
    pub creation_time: ::core::option::Option<i64>,
    /// Tags: Additional metadata key-value pairs.
    #[prost(message, repeated, tag="7")]
    pub tags: ::prost::alloc::vec::Vec<ExperimentTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExperiment {
    /// Experiment name.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Location where all artifacts for the experiment are stored.
    /// If not provided, the remote server will select an appropriate default.
    #[prost(string, optional, tag="2")]
    pub artifact_location: ::core::option::Option<::prost::alloc::string::String>,
    /// A collection of tags to set on the experiment. Maximum tag size and number of tags per request
    /// depends on the storage backend. All storage backends are guaranteed to support tag keys up
    /// to 250 bytes in size and tag values up to 5000 bytes in size. All storage backends are also
    /// guaranteed to support up to 20 tags per request.
    #[prost(message, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<ExperimentTag>,
}
/// Nested message and enum types in `CreateExperiment`.
pub mod create_experiment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Unique identifier for the experiment.
        #[prost(string, optional, tag="1")]
        pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchExperiments {
    /// Maximum number of experiments desired.
    /// Servers may select a desired default `max_results` value. All servers are
    /// guaranteed to support a `max_results` threshold of at least 1,000 but may
    /// support more. Callers of this endpoint are encouraged to pass max_results
    /// explicitly and leverage page_token to iterate through experiments.
    #[prost(int64, optional, tag="1")]
    pub max_results: ::core::option::Option<i64>,
    /// Token indicating the page of experiments to fetch
    #[prost(string, optional, tag="2")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// A filter expression over experiment attributes and tags that allows returning a subset of
    /// experiments. The syntax is a subset of SQL that supports ANDing together binary operations
    /// between an attribute or tag, and a constant.
    ///
    /// Example: ``name LIKE 'test-%' AND tags.key = 'value'``
    ///
    /// You can select columns with special characters (hyphen, space, period, etc.) by using
    /// double quotes or backticks.
    ///
    /// Example: ``tags."extra-key" = 'value'`` or ``tags.`extra-key` = 'value'``
    ///
    /// Supported operators are ``=``, ``!=``, ``LIKE``, and ``ILIKE``.
    #[prost(string, optional, tag="3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// List of columns for ordering search results, which can include experiment name and id
    /// with an optional "DESC" or "ASC" annotation, where "ASC" is the default.
    /// Tiebreaks are done by experiment id DESC.
    #[prost(string, repeated, tag="4")]
    pub order_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Qualifier for type of experiments to be returned.
    /// If unspecified, return only active experiments.
    #[prost(enumeration="ViewType", optional, tag="5")]
    pub view_type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `SearchExperiments`.
pub mod search_experiments {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Experiments that match the search criteria
        #[prost(message, repeated, tag="1")]
        pub experiments: ::prost::alloc::vec::Vec<super::Experiment>,
        /// Token that can be used to retrieve the next page of experiments.
        /// An empty token means that no more experiments are available for retrieval.
        #[prost(string, optional, tag="2")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExperiment {
    /// ID of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetExperiment`.
pub mod get_experiment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Experiment details.
        #[prost(message, optional, tag="1")]
        pub experiment: ::core::option::Option<super::Experiment>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExperiment {
    /// ID of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteExperiment`.
pub mod delete_experiment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreExperiment {
    /// ID of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RestoreExperiment`.
pub mod restore_experiment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExperiment {
    /// ID of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    /// If provided, the experiment's name is changed to the new name. The new name must be unique.
    #[prost(string, optional, tag="2")]
    pub new_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UpdateExperiment`.
pub mod update_experiment {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRun {
    /// ID of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    /// ID of the user executing the run.
    /// This field is deprecated as of MLflow 1.0, and will be removed in a future
    /// MLflow release. Use 'mlflow.user' tag instead.
    #[prost(string, optional, tag="2")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the run.
    #[prost(string, optional, tag="3")]
    pub run_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Unix timestamp in milliseconds of when the run started.
    #[prost(int64, optional, tag="7")]
    pub start_time: ::core::option::Option<i64>,
    /// Additional metadata for run.
    #[prost(message, repeated, tag="9")]
    pub tags: ::prost::alloc::vec::Vec<RunTag>,
}
/// Nested message and enum types in `CreateRun`.
pub mod create_run {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// The newly created run.
        #[prost(message, optional, tag="1")]
        pub run: ::core::option::Option<super::Run>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRun {
    /// ID of the run to update. Must be provided.
    #[prost(string, optional, tag="4")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run to update.. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Updated status of the run.
    #[prost(enumeration="RunStatus", optional, tag="2")]
    pub status: ::core::option::Option<i32>,
    /// Unix timestamp in milliseconds of when the run ended.
    #[prost(int64, optional, tag="3")]
    pub end_time: ::core::option::Option<i64>,
    /// Updated name of the run.
    #[prost(string, optional, tag="5")]
    pub run_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UpdateRun`.
pub mod update_run {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Updated metadata of the run.
        #[prost(message, optional, tag="1")]
        pub run_info: ::core::option::Option<super::RunInfo>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRun {
    /// ID of the run to delete.
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteRun`.
pub mod delete_run {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreRun {
    /// ID of the run to restore.
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RestoreRun`.
pub mod restore_run {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMetric {
    /// ID of the run under which to log the metric. Must be provided.
    #[prost(string, optional, tag="6")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run under which to log the metric. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the metric.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Double value of the metric being logged.
    #[prost(double, optional, tag="3")]
    pub value: ::core::option::Option<f64>,
    /// Unix timestamp in milliseconds at the time metric was logged.
    #[prost(int64, optional, tag="4")]
    pub timestamp: ::core::option::Option<i64>,
    /// Step at which to log the metric
    #[prost(int64, optional, tag="5", default="0")]
    pub step: ::core::option::Option<i64>,
}
/// Nested message and enum types in `LogMetric`.
pub mod log_metric {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogParam {
    /// ID of the run under which to log the param. Must be provided.
    #[prost(string, optional, tag="4")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run under which to log the param. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the param. Maximum size is 255 bytes.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// String value of the param being logged. Maximum size is 500 bytes.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `LogParam`.
pub mod log_param {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetExperimentTag {
    /// ID of the experiment under which to log the tag. Must be provided.
    #[prost(string, optional, tag="1")]
    pub experiment_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 250 bytes in size.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// String value of the tag being logged. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 5000 bytes in size.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SetExperimentTag`.
pub mod set_experiment_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTag {
    /// ID of the run under which to log the tag. Must be provided.
    #[prost(string, optional, tag="4")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run under which to log the tag. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 250 bytes in size.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// String value of the tag being logged. Maximum size depends on storage backend.
    /// All storage backends are guaranteed to support key values up to 5000 bytes in size.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SetTag`.
pub mod set_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTag {
    /// ID of the run that the tag was logged under. Must be provided.
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the tag. Maximum size is 255 bytes. Must be provided.
    #[prost(string, optional, tag="2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DeleteTag`.
pub mod delete_tag {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRun {
    /// ID of the run to fetch. Must be provided.
    #[prost(string, optional, tag="2")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run to fetch. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetRun`.
pub mod get_run {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Run metadata (name, start time, etc) and data (metrics, params, and tags).
        #[prost(message, optional, tag="1")]
        pub run: ::core::option::Option<super::Run>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRuns {
    /// List of experiment IDs to search over.
    #[prost(string, repeated, tag="1")]
    pub experiment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A filter expression over params, metrics, and tags, that allows returning a subset of
    /// runs. The syntax is a subset of SQL that supports ANDing together binary operations
    /// between a param, metric, or tag and a constant.
    ///
    /// Example: ``metrics.rmse < 1 and params.model_class = 'LogisticRegression'``
    ///
    /// You can select columns with special characters (hyphen, space, period, etc.) by using double quotes:
    /// ``metrics."model class" = 'LinearRegression' and tags."user-name" = 'Tomas'``
    ///
    /// Supported operators are ``=``, ``!=``, ``>``, ``>=``, ``<``, and ``<=``.
    #[prost(string, optional, tag="4")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether to display only active, only deleted, or all runs.
    /// Defaults to only active runs.
    #[prost(enumeration="ViewType", optional, tag="3", default="ActiveOnly")]
    pub run_view_type: ::core::option::Option<i32>,
    /// Maximum number of runs desired. If unspecified, defaults to 1000.
    /// All servers are guaranteed to support a `max_results` threshold of at least 50,000
    /// but may support more. Callers of this endpoint are encouraged to pass max_results
    /// explicitly and leverage page_token to iterate through experiments.
    #[prost(int32, optional, tag="5", default="1000")]
    pub max_results: ::core::option::Option<i32>,
    /// List of columns to be ordered by, including attributes, params, metrics, and tags with an
    /// optional "DESC" or "ASC" annotation, where "ASC" is the default.
    /// Example: ["params.input DESC", "metrics.alpha ASC", "metrics.rmse"]
    /// Tiebreaks are done by start_time DESC followed by run_id for runs with the same start time
    /// (and this is the default ordering criterion if order_by is not provided).
    #[prost(string, repeated, tag="6")]
    pub order_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SearchRuns`.
pub mod search_runs {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Runs that match the search criteria.
        #[prost(message, repeated, tag="1")]
        pub runs: ::prost::alloc::vec::Vec<super::Run>,
        #[prost(string, optional, tag="2")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListArtifacts {
    /// ID of the run whose artifacts to list. Must be provided.
    #[prost(string, optional, tag="3")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run whose artifacts to list. This field will
    /// be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Filter artifacts matching this path (a relative path from the root artifact directory).
    #[prost(string, optional, tag="2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Token indicating the page of artifact results to fetch
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ListArtifacts`.
pub mod list_artifacts {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Root artifact directory for the run.
        #[prost(string, optional, tag="1")]
        pub root_uri: ::core::option::Option<::prost::alloc::string::String>,
        /// File location and metadata for artifacts.
        #[prost(message, repeated, tag="2")]
        pub files: ::prost::alloc::vec::Vec<super::FileInfo>,
        /// Token that can be used to retrieve the next page of artifact results
        #[prost(string, optional, tag="3")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// Metadata of a single artifact file or directory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    /// Path relative to the root artifact directory run.
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether the path is a directory.
    #[prost(bool, optional, tag="2")]
    pub is_dir: ::core::option::Option<bool>,
    /// Size in bytes. Unset for directories.
    #[prost(int64, optional, tag="3")]
    pub file_size: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricHistory {
    /// ID of the run from which to fetch metric values. Must be provided.
    #[prost(string, optional, tag="3")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// [Deprecated, use run_id instead] ID of the run from which to fetch metric values. This field
    /// will be removed in a future MLflow version.
    #[prost(string, optional, tag="1")]
    pub run_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the metric.
    #[prost(string, optional, tag="2")]
    pub metric_key: ::core::option::Option<::prost::alloc::string::String>,
    /// Token indicating the page of metric history to fetch
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Maximum number of logged instances of a metric for a run to return per call.
    /// Backend servers may restrict the value of `max_results` depending on performance requirements.
    /// Requests that do not specify this value will behave as non-paginated queries where all
    /// metric history values for a given metric within a run are returned in a single response.
    #[prost(int32, optional, tag="5")]
    pub max_results: ::core::option::Option<i32>,
}
/// Nested message and enum types in `GetMetricHistory`.
pub mod get_metric_history {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// All logged values for this metric.
        #[prost(message, repeated, tag="1")]
        pub metrics: ::prost::alloc::vec::Vec<super::Metric>,
        /// Token that can be used to issue a query for the next page of metric history values.
        /// A missing token indicates that no additional metrics are available to fetch.
        #[prost(string, optional, tag="2")]
        pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogBatch {
    /// ID of the run to log under
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Metrics to log. A single request can contain up to 1000 metrics, and up to 1000
    /// metrics, params, and tags in total.
    #[prost(message, repeated, tag="2")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// Params to log. A single request can contain up to 100 params, and up to 1000
    /// metrics, params, and tags in total.
    #[prost(message, repeated, tag="3")]
    pub params: ::prost::alloc::vec::Vec<Param>,
    /// Tags to log. A single request can contain up to 100 tags, and up to 1000
    /// metrics, params, and tags in total.
    #[prost(message, repeated, tag="4")]
    pub tags: ::prost::alloc::vec::Vec<RunTag>,
}
/// Nested message and enum types in `LogBatch`.
pub mod log_batch {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogModel {
    /// ID of the run to log under
    #[prost(string, optional, tag="1")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// MLmodel file in json format.
    #[prost(string, optional, tag="2")]
    pub model_json: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `LogModel`.
pub mod log_model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExperimentByName {
    /// Name of the associated experiment.
    #[prost(string, optional, tag="1")]
    pub experiment_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `GetExperimentByName`.
pub mod get_experiment_by_name {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Experiment details.
        #[prost(message, optional, tag="1")]
        pub experiment: ::core::option::Option<super::Experiment>,
    }
}
/// View type for ListExperiments query.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ViewType {
    /// Default. Return only active experiments.
    ActiveOnly = 1,
    /// Return only deleted experiments.
    DeletedOnly = 2,
    /// Get all experiments.
    All = 3,
}
impl ViewType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ViewType::ActiveOnly => "ACTIVE_ONLY",
            ViewType::DeletedOnly => "DELETED_ONLY",
            ViewType::All => "ALL",
        }
    }
}
/// Source that generated a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SourceType {
    /// Databricks notebook environment.
    Notebook = 1,
    /// Scheduled or Run Now job.
    Job = 2,
    /// As a prepackaged project: either a Docker image or GitHub source, etc.
    Project = 3,
    /// Local run: Using CLI, IDE, or local notebook.
    Local = 4,
    /// Unknown source type.
    Unknown = 1000,
}
impl SourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SourceType::Notebook => "NOTEBOOK",
            SourceType::Job => "JOB",
            SourceType::Project => "PROJECT",
            SourceType::Local => "LOCAL",
            SourceType::Unknown => "UNKNOWN",
        }
    }
}
/// Status of a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunStatus {
    /// Run has been initiated.
    Running = 1,
    /// Run is scheduled to run at a later time.
    Scheduled = 2,
    /// Run has completed.
    Finished = 3,
    /// Run execution failed.
    Failed = 4,
    /// Run killed by user.
    Killed = 5,
}
impl RunStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RunStatus::Running => "RUNNING",
            RunStatus::Scheduled => "SCHEDULED",
            RunStatus::Finished => "FINISHED",
            RunStatus::Failed => "FAILED",
            RunStatus::Killed => "KILLED",
        }
    }
}
include!("mlflow.tonic.rs");
include!("mlflow.serde.rs");
// @@protoc_insertion_point(module)