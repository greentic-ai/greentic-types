//! Types used by QA setup contracts.
use alloc::{string::String, vec::Vec};

use ciborium::value::Value;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{cbor::canonical, cbor_bytes::CborBytes, schema_id::SchemaSource};

/// Where the QA specification lives.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QaSpecSource {
    /// Inline CBOR specification bytes.
    InlineCbor(CborBytes),
    #[cfg(feature = "json-compat")]
    /// Inline JSON spec for transition/debug tooling.
    InlineJson(String),
    /// Schema hosted at a remote URI.
    RefUri(String),
    /// Schema stored in another pack path.
    RefPackPath(String),
}

/// Example answers submitted by a pack author.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExampleAnswers {
    /// Friendly title for the example answers.
    pub title: String,
    /// Canonical CBOR payload.
    pub answers_cbor: CborBytes,
    /// Optional annotations or hints.
    pub notes: Option<String>,
}

/// Outputs produced by the setup flow.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SetupOutput {
    /// Only configuration data is emitted.
    ConfigOnly,
    /// Template-driven scaffold output.
    TemplateScaffold {
        /// Reference to the scaffold template.
        template_ref: String,
        /// Layout/slot description produced by the scaffold.
        output_layout: String,
    },
}

/// QA setup contract (optional convenience wrapper).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetupContract {
    /// Reference to the QA spec.
    pub qa_spec: QaSpecSource,
    /// Optional schema describing answers.
    pub answers_schema: Option<SchemaSource>,
    /// Example answer blobs for documentation.
    pub examples: Vec<ExampleAnswers>,
    /// Declared outputs for the setup run.
    pub outputs: Vec<SetupOutput>,
}

/// Canonical enforcement policy used by `validate_answers`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalPolicy {
    /// Skip canonical enforcement.
    Off,
    /// Treat non-canonical CBOR as errors.
    RequireCanonical,
    /// Re-encode answers into canonical form.
    Canonicalize,
}

/// Errors produced during QA answer validation.
#[derive(Debug, Error)]
pub enum ValidateAnswersError {
    /// Failed to decode the answers payload.
    #[error("CBOR decode failed: {0}")]
    Decode(String),
    /// Answers must be represented as a CBOR map/object.
    #[error("answers must be a CBOR map/object")]
    NotMap,
    /// Canonicalization check failed.
    #[error(transparent)]
    Canonical(#[from] canonical::CanonicalError),
}

/// MVP validator that ensures answer CBOR is a map and optionally canonical.
pub fn validate_answers(
    schema: &SchemaSource,
    answers_cbor: &CborBytes,
    policy: CanonicalPolicy,
) -> Result<CborBytes, ValidateAnswersError> {
    let _ = schema;
    let value: Value = ciborium::de::from_reader(answers_cbor.as_slice())
        .map_err(|err| ValidateAnswersError::Decode(err.to_string()))?;

    if !matches!(value, Value::Map(_)) {
        return Err(ValidateAnswersError::NotMap);
    }

    match policy {
        CanonicalPolicy::Off => Ok(answers_cbor.clone()),
        CanonicalPolicy::RequireCanonical => {
            answers_cbor.ensure_canonical()?;
            Ok(answers_cbor.clone())
        }
        CanonicalPolicy::Canonicalize => {
            let canonical_bytes = canonical::canonicalize(answers_cbor.as_slice())?;
            Ok(CborBytes(canonical_bytes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;

    fn schema_bytes() -> Vec<u8> {
        let mut map = BTreeMap::new();
        map.insert("key", "value");
        match canonical::to_canonical_cbor(&map) {
            Ok(bytes) => bytes,
            Err(err) => panic!("schema canonicalization failed: {err:?}"),
        }
    }

    fn schema_blob() -> CborBytes {
        CborBytes(schema_bytes())
    }

    #[test]
    fn validate_accepts_map_off_policy() {
        let bytes = schema_blob();
        let source = SchemaSource::InlineCbor(bytes.clone());
        let result = match validate_answers(&source, &bytes, CanonicalPolicy::Off) {
            Ok(value) => value,
            Err(err) => panic!("validation failed: {err:?}"),
        };
        assert_eq!(result.as_slice(), bytes.as_slice());
    }

    #[test]
    fn validate_rejects_non_map() {
        let bytes = match canonical::to_canonical_cbor(&"string") {
            Ok(value) => value,
            Err(err) => panic!("canonicalize string failed: {err:?}"),
        };
        let source = SchemaSource::InlineCbor(CborBytes(bytes.clone()));
        assert!(matches!(
            validate_answers(&source, &CborBytes(bytes), CanonicalPolicy::Off),
            Err(ValidateAnswersError::NotMap)
        ));
    }

    #[test]
    fn canonicalize_policy_rewrites_indefinite_map() {
        let indefinite = vec![0xBF, 0x61, b'a', 0x01, 0xFF];
        let source = SchemaSource::InlineCbor(CborBytes::new(indefinite.clone()));
        let canonical_bytes = match validate_answers(
            &source,
            &CborBytes(indefinite),
            CanonicalPolicy::Canonicalize,
        ) {
            Ok(bytes) => bytes,
            Err(err) => panic!("validation failed: {err:?}"),
        };
        if let Err(err) = canonical::ensure_canonical(canonical_bytes.as_slice()) {
            panic!("ensure canonical failed: {err:?}");
        }
    }

    #[test]
    fn require_canonical_rejects_indefinite() {
        let indefinite = vec![0xBF, 0x61, b'a', 0x01, 0xFF];
        let source = SchemaSource::InlineCbor(CborBytes::new(indefinite.clone()));
        assert!(matches!(
            validate_answers(
                &source,
                &CborBytes(indefinite),
                CanonicalPolicy::RequireCanonical
            ),
            Err(ValidateAnswersError::Canonical(_))
        ));
    }
}
