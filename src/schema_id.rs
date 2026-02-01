//! Schema identifiers and reference sources built on canonical CBOR.
use alloc::{format, string::String};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{cbor::canonical, cbor_bytes::CborBytes};

const SCHEMA_ID_PREFIX: &str = "schema:v1:";

/// Stable identifier derived from canonical schema CBOR.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SchemaId(String);

impl SchemaId {
    /// Parse an existing `schema:v1:` identifier.
    pub fn parse(value: &str) -> Result<Self, SchemaIdError> {
        if !value.starts_with(SCHEMA_ID_PREFIX) {
            return Err(SchemaIdError::InvalidPrefix);
        }
        let encoded = &value[SCHEMA_ID_PREFIX.len()..];
        canonical::decode_base32_crockford(encoded)?;
        Ok(Self(value.to_owned()))
    }

    /// Serialize as `schema:v1:<base32>` string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for SchemaId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Compute the canonical schema ID for the provided CBOR bytes.
pub fn schema_id_for_cbor(schema_cbor: &[u8]) -> Result<SchemaId, SchemaIdError> {
    canonical::ensure_canonical(schema_cbor)?;
    let digest = canonical::blake3_128(schema_cbor);
    let encoded = canonical::encode_base32_crockford(&digest);
    Ok(SchemaId(format!("{SCHEMA_ID_PREFIX}{encoded}")))
}

/// Errors emitted while parsing or deriving schema IDs.
#[derive(Debug, Error)]
pub enum SchemaIdError {
    /// Identifier does not have the required `schema:v1:` prefix.
    #[error("schema ID must begin with {SCHEMA_ID_PREFIX}")]
    InvalidPrefix,
    /// Payload part of the ID is not valid Crockford Base32.
    #[error("invalid base32 payload: {0}")]
    Base32(#[from] canonical::Base32Error),
    /// Canonical enforcement failed while deriving the ID.
    #[error(transparent)]
    Canonical(#[from] canonical::CanonicalError),
}

/// Schema references used for both runtime I/O and QA answers.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SchemaSource {
    /// Refer to a schema by its canonical ID.
    CborSchemaId(SchemaId),
    /// Embed canonical CBOR schema bytes inline.
    InlineCbor(CborBytes),
    /// Embed JSON schema bytes for debugging (feature gated).
    #[cfg(feature = "json-compat")]
    InlineJson(String),
    /// Refer to a schema stored in another pack.
    RefPackPath(String),
    /// Refer to a schema hosted at an arbitrary URI.
    RefUri(String),
}

/// QA schema references reuse the same set of sources today.
pub type QaSchemaSource = SchemaSource;
/// Invoke-time schemas are currently the same as QA schemas.
pub type IoSchemaSource = SchemaSource;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_id_roundtrip() {
        let schema = canon_schema();
        let id = match schema_id_for_cbor(&schema) {
            Ok(value) => value,
            Err(err) => panic!("id generation failed: {err:?}"),
        };
        let parsed = match SchemaId::parse(id.as_str()) {
            Ok(value) => value,
            Err(err) => panic!("parse failed: {err:?}"),
        };
        assert_eq!(parsed.as_str(), id.as_str());
    }

    fn canon_schema() -> Vec<u8> {
        match canonical::to_canonical_cbor(&"schema") {
            Ok(bytes) => bytes,
            Err(err) => panic!("canonicalize schema failed: {err:?}"),
        }
    }
}
