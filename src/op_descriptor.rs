//! Shared self-describing operation descriptors.
use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::{cbor_bytes::CborBytes, schema_id::SchemaSource};

const DEFAULT_CONTENT_TYPE: &str = "application/cbor";

/// Schema plus content type metadata for op I/O.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IoSchema {
    /// Schema reference for invocation inputs/outputs.
    pub schema: SchemaSource,
    /// Content type partners should use for the referenced payload.
    pub content_type: String,
}

impl IoSchema {
    /// Create a schema wrapper defaulting to `application/cbor`.
    pub fn new(schema: SchemaSource) -> Self {
        Self {
            schema,
            content_type: DEFAULT_CONTENT_TYPE.to_owned(),
        }
    }
}

/// Example input/output pairs for referencing operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpExample {
    /// Human-readable title for the example.
    pub title: String,
    /// Canonical CBOR input payload.
    pub input_cbor: CborBytes,
    /// Optional example output payload.
    pub output_cbor: Option<CborBytes>,
    /// Free-text notes for the example.
    pub notes: Option<String>,
}

/// Descriptor for a single self-describing operation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpDescriptor {
    /// Canonical operation name.
    pub name: String,
    /// Optional summary describing the operation.
    pub summary: Option<String>,
    /// Input schema descriptor.
    pub input: IoSchema,
    /// Output schema descriptor.
    pub output: IoSchema,
    /// Documented examples.
    pub examples: Vec<OpExample>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cbor_bytes::CborBytes;
    use crate::schema_id::SchemaSource;
    use alloc::string::ToString;

    #[test]
    fn descriptor_serde_roundtrip() {
        let schema = SchemaSource::InlineCbor(CborBytes::new(vec![0x01]));
        let io = IoSchema {
            schema,
            content_type: "application/cbor".to_string(),
        };
        let descriptor = OpDescriptor {
            name: "op".to_string(),
            summary: Some("desc".to_string()),
            input: io.clone(),
            output: io,
            examples: vec![],
        };
        let serialized = match serde_json::to_string(&descriptor) {
            Ok(json) => json,
            Err(err) => panic!("serialize failed: {err:?}"),
        };
        let roundtrip: OpDescriptor = match serde_json::from_str(&serialized) {
            Ok(desc) => desc,
            Err(err) => panic!("deserialize failed: {err:?}"),
        };
        assert_eq!(roundtrip.name, "op");
    }
}
