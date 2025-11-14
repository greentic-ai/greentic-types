//! Serde-ready representations for Greentic `pack.yaml` manifests.

use alloc::{string::String, vec::Vec};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Canonical on-disk pack specification (`pack.yaml`). Deprecated in favor of [`PackManifest`](crate::pack_manifest::PackManifest).
///
/// Fields default to empty collections to keep additive evolution backwards compatible.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deprecated(
    since = "0.3.2",
    note = "Use PackManifest from `pack_manifest` for new work."
)]
pub struct PackSpec {
    /// Unique identifier for the pack.
    pub id: String,
    /// Semantic pack version.
    pub version: String,
    /// Relative flow file paths bundled with the pack.
    #[cfg_attr(feature = "serde", serde(default))]
    pub flow_files: Vec<String>,
    /// Template directories that should be bundled with the pack.
    #[cfg_attr(feature = "serde", serde(default))]
    pub template_dirs: Vec<String>,
    /// Optional set of required imports enforced by the host.
    #[cfg_attr(feature = "serde", serde(default))]
    pub imports_required: Vec<String>,
    /// Optional legacy tool definitions. Prefer MCP-first designs.
    #[cfg_attr(feature = "serde", serde(default))]
    pub tools: Vec<ToolSpec>,
}

/// Tool metadata referenced by a [`PackSpec`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToolSpec {
    /// Tool identifier referenced by flows.
    pub name: String,
    /// Optional hint for the loader to determine the tool source (for example `mcp`).
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<String>,
    /// Filesystem path hint for embedded tools.
    #[cfg_attr(feature = "serde", serde(default))]
    pub path: Option<String>,
    /// Actions exposed by the tool.
    #[cfg_attr(feature = "serde", serde(default))]
    pub actions: Vec<String>,
}
