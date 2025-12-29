//! Provider manifest and index data structures.
//!
//! These types model the JSON returned by provider-core `describe()` and the provider index
//! entries used by store, deployer, and runner components.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Manifest describing a provider returned by `describe()`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderManifest {
    /// Provider type identifier (string-based to avoid enum coupling).
    pub provider_type: String,
    /// Capabilities advertised by the provider.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub capabilities: Vec<String>,
    /// Operations exposed by the provider.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub ops: Vec<String>,
    /// Optional JSON Schema reference for configuration.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub config_schema_ref: Option<String>,
    /// Optional JSON Schema reference for provider state.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub state_schema_ref: Option<String>,
}

/// Runtime binding for a provider implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderRuntimeRef {
    /// Component identifier for the provider runtime.
    pub component_ref: String,
    /// Exported function implementing the provider runtime.
    pub export: String,
    /// WIT world for the provider runtime.
    pub world: String,
}

/// Provider declaration stored in indexes and extension payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderDecl {
    /// Provider type identifier (string-based to avoid enum coupling).
    pub provider_type: String,
    /// Capabilities advertised by the provider.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub capabilities: Vec<String>,
    /// Operations exposed by the provider.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub ops: Vec<String>,
    /// JSON Schema reference for configuration.
    pub config_schema_ref: String,
    /// Optional JSON Schema reference for provider state.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub state_schema_ref: Option<String>,
    /// Runtime binding information for the provider.
    pub runtime: ProviderRuntimeRef,
    /// Optional documentation reference.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub docs_ref: Option<String>,
}

/// Inline extension payload embedding provider declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderExtensionInline {
    /// Providers included in the extension payload.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub providers: Vec<ProviderDecl>,
}
