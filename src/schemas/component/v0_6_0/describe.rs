//! Component description schema (v0.6.0).
use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ciborium::value::Value;

use crate::i18n_text::I18nText;

/// Component metadata.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentInfo {
    /// Stable component identifier.
    pub id: String,
    /// Semantic version string.
    pub version: String,
    /// Component role (runtime/provider/tool/etc.).
    pub role: String,
    /// Optional display name.
    pub display_name: Option<I18nText>,
}

/// Component description payload.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentDescribe {
    /// Core component metadata.
    pub info: ComponentInfo,
    /// Capabilities provided by the component.
    pub provided_capabilities: Vec<String>,
    /// Capabilities required by the component.
    pub required_capabilities: Vec<String>,
    /// Optional metadata payload.
    pub metadata: BTreeMap<String, Value>,
}

/// Placeholder run input schema.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentRunInput {
    /// Input values keyed by name.
    pub values: BTreeMap<String, Value>,
}

/// Placeholder run output schema.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentRunOutput {
    /// Output values keyed by name.
    pub values: BTreeMap<String, Value>,
}
