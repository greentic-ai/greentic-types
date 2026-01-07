//! Extension payload for per-component manifest indexes.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use ciborium::{de::from_reader, ser::into_writer};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Pack extension identifier for the component manifest index (v1).
pub const EXT_COMPONENT_MANIFEST_INDEX_V1: &str = "greentic.pack.component_manifests@v1";

/// Entry list describing component manifest files inside a pack.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentManifestIndexV1 {
    /// Schema version for the index payload.
    pub schema_version: u32,
    /// Indexed component manifest entries.
    pub entries: Vec<ComponentManifestIndexEntryV1>,
}

impl ComponentManifestIndexV1 {
    /// Creates a new component manifest index payload.
    pub fn new(entries: Vec<ComponentManifestIndexEntryV1>) -> Self {
        Self {
            schema_version: 1,
            entries,
        }
    }

    /// Validates the schema version for forward-compatible decoders.
    pub fn validate_schema_version(&self) -> Result<(), ComponentManifestIndexError> {
        if self.schema_version == 1 {
            Ok(())
        } else {
            Err(ComponentManifestIndexError::UnsupportedSchemaVersion(
                self.schema_version,
            ))
        }
    }

    /// Converts the payload to an extension value suitable for `ExtensionInline::Other`.
    #[cfg(feature = "serde")]
    pub fn to_extension_value(&self) -> Result<serde_json::Value, ComponentManifestIndexError> {
        serde_json::to_value(self)
            .map_err(|err| ComponentManifestIndexError::Serialize(err.to_string()))
    }

    /// Parses the payload from an extension value.
    #[cfg(feature = "serde")]
    pub fn from_extension_value(
        value: &serde_json::Value,
    ) -> Result<Self, ComponentManifestIndexError> {
        let decoded: Self = serde_json::from_value(value.clone())
            .map_err(|err| ComponentManifestIndexError::Deserialize(err.to_string()))?;
        decoded.validate_schema_version()?;
        Ok(decoded)
    }
}

/// Component manifest index entry describing a manifest file and encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentManifestIndexEntryV1 {
    /// Canonical component identifier.
    pub component_id: String,
    /// Pack-relative manifest file path, e.g. `<component_id>.manifest.cbor`.
    pub manifest_file: String,
    /// Encoding for the referenced manifest file.
    pub encoding: ManifestEncoding,
    /// Optional content hash (for example `sha256:<hex>`).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub content_hash: Option<String>,
}

/// Supported encodings for per-component manifests.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ManifestEncoding {
    /// CBOR encoding.
    Cbor,
}

/// Errors produced while encoding or decoding the component manifest index.
#[derive(Debug, thiserror::Error)]
pub enum ComponentManifestIndexError {
    /// Serialization failed.
    #[error("component manifest index serialize failed: {0}")]
    Serialize(String),
    /// Deserialization failed.
    #[error("component manifest index deserialize failed: {0}")]
    Deserialize(String),
    /// Unsupported schema version.
    #[error("unsupported component manifest index schema_version {0}")]
    UnsupportedSchemaVersion(u32),
}

/// Serializes the component manifest index payload to CBOR bytes.
#[cfg(feature = "serde")]
pub fn encode_component_manifest_index_v1_to_cbor_bytes(
    payload: &ComponentManifestIndexV1,
) -> Result<Vec<u8>, ComponentManifestIndexError> {
    let mut buf = Vec::new();
    into_writer(payload, &mut buf)
        .map_err(|err| ComponentManifestIndexError::Serialize(err.to_string()))?;
    Ok(buf)
}

/// Deserializes the component manifest index payload from CBOR bytes.
#[cfg(feature = "serde")]
pub fn decode_component_manifest_index_v1_from_cbor_bytes(
    bytes: &[u8],
) -> Result<ComponentManifestIndexV1, ComponentManifestIndexError> {
    let decoded: ComponentManifestIndexV1 = from_reader(bytes)
        .map_err(|err| ComponentManifestIndexError::Deserialize(err.to_string()))?;
    decoded.validate_schema_version()?;
    Ok(decoded)
}
