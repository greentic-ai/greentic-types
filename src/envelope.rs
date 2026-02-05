//! Canonical CBOR envelope for typed payloads.
use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::cbor::canonical;
use crate::cbor_bytes::CborBytes;

/// Envelope carrying a canonical CBOR payload.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Envelope {
    /// Domain-specific kind (pack/component/etc.).
    pub kind: String,
    /// Schema identifier for the body.
    pub schema: String,
    /// Schema version for the body.
    pub version: u32,
    /// Canonical CBOR-encoded payload.
    pub body: CborBytes,
}

impl Envelope {
    /// Ensure the body payload is canonical CBOR.
    pub fn ensure_canonical(&self) -> canonical::Result<()> {
        self.body.ensure_canonical()
    }
}

#[cfg(feature = "serde")]
impl Envelope {
    /// Build an envelope from a serializable body, encoding as canonical CBOR.
    pub fn new<T: serde::Serialize>(
        kind: impl Into<String>,
        schema: impl Into<String>,
        version: u32,
        body: &T,
    ) -> canonical::Result<Self> {
        let bytes = canonical::to_canonical_cbor(body)?;
        Ok(Self {
            kind: kind.into(),
            schema: schema.into(),
            version,
            body: CborBytes::new(bytes),
        })
    }

    /// Decode the CBOR body to the requested type.
    pub fn decode_body<T: serde::de::DeserializeOwned>(&self) -> canonical::Result<T> {
        self.body.ensure_canonical()?;
        self.body.decode::<T>()
    }
}
