//! Canonical pack manifest (.gtpack) representation.

use alloc::string::String;
use alloc::vec::Vec;

use semver::Version;

use crate::{ComponentId, FlowId, PackId, SemverReq};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Hint describing the primary purpose of a pack.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum PackKind {
    /// Normal digital worker packs.
    Application,
    /// Packs whose flows primarily operate on deployment plans.
    Deployment,
    /// Packs that mix both application and deployment flows.
    Mixed,
}

/// Pack manifest describing bundled flows and referenced components.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PackManifest {
    /// Logical pack identifier.
    pub id: PackId,
    /// Pack semantic version.
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "String", description = "SemVer version")
    )]
    pub version: Version,
    /// Optional human-friendly name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<String>,
    /// Flow references bundled within the pack.
    #[cfg_attr(feature = "serde", serde(default))]
    pub flows: Vec<PackFlowRef>,
    /// Component references required by the pack.
    #[cfg_attr(feature = "serde", serde(default))]
    pub components: Vec<PackComponentRef>,
    /// Optional pack-level profile defaults.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub profiles: Option<Value>,
    /// Optional component source metadata (registries, mirrors, etc.).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub component_sources: Option<Value>,
    /// Optional connector metadata describing ingress wiring.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub connectors: Option<Value>,
    /// Optional hint about the primary intent of this pack.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub kind: Option<PackKind>,
}

/// Flow reference within a pack manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PackFlowRef {
    /// Flow identifier as referenced by connectors/runners.
    pub id: FlowId,
    /// Relative file path (inside the pack) to the .ygtc document.
    pub file: String,
}

/// Component reference within a pack manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PackComponentRef {
    /// Component identifier.
    pub id: ComponentId,
    /// Supported version requirement.
    pub version_req: SemverReq,
    /// Optional source hint (registry, OCI ref, etc.).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub source: Option<String>,
}
