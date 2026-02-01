//! Lightweight wrappers around CBOR payloads and optional blobs.
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::cbor::canonical;
#[cfg(feature = "serde")]
use serde_bytes::ByteBuf;

/// A canonical CBOR byte buffer that can be re-validated or decoded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CborBytes(pub Vec<u8>);

impl CborBytes {
    /// Wrap an existing buffer.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Access the raw bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Consume this wrapper and return the owned bytes.
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    /// Ensure the payload follows canonical CBOR rules.
    pub fn ensure_canonical(&self) -> canonical::Result<()> {
        canonical::ensure_canonical(&self.0)
    }

    /// Re-encode the payload in canonical form.
    pub fn canonicalize(self) -> canonical::Result<Self> {
        canonical::canonicalize(&self.0).map(Self)
    }

    /// Deserialize the payload into `T`.
    pub fn decode<T: DeserializeOwned>(&self) -> canonical::Result<T> {
        canonical::from_cbor(&self.0)
    }
}

impl From<Vec<u8>> for CborBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<CborBytes> for Vec<u8> {
    fn from(cbor: CborBytes) -> Self {
        cbor.0
    }
}

#[cfg(feature = "serde")]
impl Serialize for CborBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for CborBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = ByteBuf::deserialize(deserializer)?;
        Ok(CborBytes(bytes.into_vec()))
    }
}

/// Optional tuple for passing content type plus canonical CBOR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blob {
    /// MIME type describing the payload.
    pub content_type: String,
    /// Raw CBOR payload bytes.
    pub bytes: Vec<u8>,
}

impl Blob {
    /// Build a blob with the supplied content type header.
    pub fn new(content_type: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            content_type: content_type.into(),
            bytes,
        }
    }
}
