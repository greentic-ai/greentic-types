//! Shared secret requirement primitives re-exported from `greentic-secrets-spec`.

use crate::{GResult, validate_identifier};
use alloc::{string::String, vec::Vec};
use core::ops::Deref;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Canonical secret identifier used across manifests and bindings.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SecretKey(String);

impl SecretKey {
    /// Constructs a secret key and validates the identifier format.
    pub fn new(key: impl Into<String>) -> GResult<Self> {
        let key = key.into();
        validate_identifier(&key, "secret key")?;
        Ok(Self(key))
    }

    /// Returns the key as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for SecretKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SecretKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}

impl From<&str> for SecretKey {
    fn from(key: &str) -> Self {
        Self(key.to_owned())
    }
}

impl From<SecretKey> for String {
    fn from(key: SecretKey) -> Self {
        key.0
    }
}

/// Canonical secret scope shared with the secrets spec.
pub type SecretScope = greentic_secrets_spec::Scope;
/// Canonical secret format shared with the secrets spec.
pub type SecretFormat = greentic_secrets_spec::ContentType;

/// Structured secret requirement used in capabilities, bindings, and deployment plans.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SecretRequirement {
    /// Logical key the runtime should resolve.
    pub key: SecretKey,
    /// Whether the secret is mandatory for execution.
    #[cfg_attr(
        feature = "serde",
        serde(default = "SecretRequirement::default_required")
    )]
    pub required: bool,
    /// Optional description for operator-facing tooling.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,
    /// Expected scope for resolution (environment/tenant/team).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub scope: Option<SecretScope>,
    /// Preferred secret format when known.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub format: Option<SecretFormat>,
    /// Optional JSON Schema fragment describing the value shape.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub schema: Option<serde_json::Value>,
    /// Example payloads for documentation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub examples: Vec<String>,
}

impl Default for SecretRequirement {
    fn default() -> Self {
        Self {
            key: SecretKey::default(),
            required: true,
            description: None,
            scope: None,
            format: None,
            schema: None,
            examples: Vec::new(),
        }
    }
}

impl SecretRequirement {
    const fn default_required() -> bool {
        true
    }
}
