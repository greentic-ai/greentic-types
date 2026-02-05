//! Pack description schema (v0.6.0).
use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ciborium::value::Value;

use crate::i18n_text::I18nText;

/// Pack metadata.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct PackInfo {
    /// Stable pack identifier.
    pub id: String,
    /// Semantic version string.
    pub version: String,
    /// Pack role (application/provider/etc.).
    pub role: String,
    /// Optional display name.
    pub display_name: Option<I18nText>,
}

/// Pack capability descriptor.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityDescriptor {
    /// Capability identifier.
    pub capability_id: String,
    /// Version requirement string.
    pub version_req: String,
    /// Optional metadata.
    pub metadata: Option<CapabilityMetadata>,
}

/// Capability metadata payload.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityMetadata {
    /// Tag list.
    pub tags: Vec<String>,
    /// Supported modes/features.
    pub supports: BTreeMap<String, Value>,
    /// Constraint map.
    pub constraints: BTreeMap<String, Value>,
    /// Quality hints.
    pub quality_hints: BTreeMap<String, Value>,
    /// Regions where supported.
    pub regions: Vec<String>,
    /// Compliance metadata.
    pub compliance: BTreeMap<String, Value>,
    /// Miscellaneous hints.
    pub hints: BTreeMap<String, Value>,
}

/// Pack description payload.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct PackDescribe {
    /// Core pack metadata.
    pub info: PackInfo,
    /// Capabilities provided by the pack.
    pub provided_capabilities: Vec<CapabilityDescriptor>,
    /// Capabilities required by the pack.
    pub required_capabilities: Vec<CapabilityDescriptor>,
    /// Summary of units (components, flows, etc.).
    pub units_summary: BTreeMap<String, Value>,
    /// Optional metadata payload.
    pub metadata: BTreeMap<String, Value>,
}
