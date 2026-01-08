//! Extension payload describing component source references and resolved artifacts.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use ciborium::{de::from_reader, ser::into_writer};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ComponentId, ComponentSourceRef};

/// Pack extension identifier for component source metadata (v1).
pub const EXT_COMPONENT_SOURCES_V1: &str = "greentic.pack.component_sources@v1";

/// Component sources extension payload (v1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentSourcesV1 {
    /// Schema version for this payload.
    pub schema_version: u32,
    /// Component source entries.
    pub components: Vec<ComponentSourceEntryV1>,
}

impl ComponentSourcesV1 {
    /// Creates a new component sources payload.
    pub fn new(components: Vec<ComponentSourceEntryV1>) -> Self {
        Self {
            schema_version: 1,
            components,
        }
    }

    /// Validates the schema version for forward-compatible decoders.
    pub fn validate_schema_version(&self) -> Result<(), ComponentSourcesError> {
        if self.schema_version == 1 {
            Ok(())
        } else {
            Err(ComponentSourcesError::UnsupportedSchemaVersion(
                self.schema_version,
            ))
        }
    }

    /// Converts the payload to an extension value suitable for `ExtensionInline::Other`.
    #[cfg(feature = "serde")]
    pub fn to_extension_value(&self) -> Result<serde_json::Value, ComponentSourcesError> {
        serde_json::to_value(self).map_err(|err| ComponentSourcesError::Serialize(err.to_string()))
    }

    /// Parses the payload from an extension value.
    #[cfg(feature = "serde")]
    pub fn from_extension_value(value: &serde_json::Value) -> Result<Self, ComponentSourcesError> {
        let decoded: Self = serde_json::from_value(value.clone())
            .map_err(|err| ComponentSourcesError::Deserialize(err.to_string()))?;
        decoded.validate_schema_version()?;
        Ok(decoded)
    }
}

/// Component entry describing the source and resolved artifacts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentSourceEntryV1 {
    /// Human-friendly component name from pack authoring.
    pub name: String,
    /// Optional canonical component identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub component_id: Option<ComponentId>,
    /// Component source reference.
    pub source: ComponentSourceRef,
    /// Resolved metadata for the component.
    pub resolved: ResolvedComponentV1,
    /// Artifact location details.
    pub artifact: ArtifactLocationV1,
    /// Optional licensing hint for store components.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub licensing_hint: Option<String>,
    /// Optional metering hint for store components.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub metering_hint: Option<String>,
}

/// Resolved metadata for a component source.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolvedComponentV1 {
    /// Content digest (for example `sha256:<hex>`).
    pub digest: String,
    /// Optional signature reference or summary.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub signature: Option<String>,
    /// Optional signer identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub signed_by: Option<String>,
}

/// Artifact location descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ArtifactLocationV1 {
    /// Artifact is embedded inside the pack.
    Inline {
        /// Pack-relative path to the wasm artifact.
        wasm_path: String,
        /// Optional pack-relative path to the component manifest.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        manifest_path: Option<String>,
    },
    /// Artifact must be fetched from the remote source.
    Remote,
}

/// Errors produced while encoding or decoding component sources payloads.
#[derive(Debug, thiserror::Error)]
pub enum ComponentSourcesError {
    /// Serialization failed.
    #[error("component sources serialize failed: {0}")]
    Serialize(String),
    /// Deserialization failed.
    #[error("component sources deserialize failed: {0}")]
    Deserialize(String),
    /// Unsupported schema version.
    #[error("unsupported component sources schema_version {0}")]
    UnsupportedSchemaVersion(u32),
    /// Extension payload missing inline data.
    #[error("component sources extension missing inline payload")]
    MissingInline,
    /// Extension payload used an unexpected inline type.
    #[error("component sources extension inline payload has unexpected type")]
    UnexpectedInline,
}

/// Serializes the component sources payload to CBOR bytes.
#[cfg(feature = "serde")]
pub fn encode_component_sources_v1_to_cbor_bytes(
    payload: &ComponentSourcesV1,
) -> Result<Vec<u8>, ComponentSourcesError> {
    let mut buf = Vec::new();
    into_writer(payload, &mut buf)
        .map_err(|err| ComponentSourcesError::Serialize(err.to_string()))?;
    Ok(buf)
}

/// Deserializes the component sources payload from CBOR bytes.
#[cfg(feature = "serde")]
pub fn decode_component_sources_v1_from_cbor_bytes(
    bytes: &[u8],
) -> Result<ComponentSourcesV1, ComponentSourcesError> {
    let decoded: ComponentSourcesV1 =
        from_reader(bytes).map_err(|err| ComponentSourcesError::Deserialize(err.to_string()))?;
    decoded.validate_schema_version()?;
    Ok(decoded)
}
