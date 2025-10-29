//! Serde-ready representations for Greentic `pack.yaml` manifests.

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// Deserialize the contents of `pack.yaml` to configure packs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackSpec {
    /// Unique identifier for the pack.
    pub id: String,
    /// Semantically versioned identifier for the pack schema.
    pub version: String,
    /// Relative file paths containing flow definitions bundled with the pack.
    #[serde(default)]
    pub flow_files: Vec<String>,
    /// Directories containing flow templates.
    #[serde(default)]
    pub template_dirs: Vec<String>,
    /// Optional tool definitions that ship with the pack.
    #[serde(default)]
    pub tools: Vec<ToolSpec>,
    /// External packs that must be present when executing this pack.
    #[serde(default)]
    pub imports_required: Vec<String>,
}

/// Tool metadata referenced by a [`PackSpec`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    /// Tool identifier referenced by flows.
    pub name: String,
    /// Optional hint for the loader to determine the tool source (for example `mcp`).
    #[serde(default)]
    pub source: Option<String>,
    /// Filesystem path hint for embedded tools.
    #[serde(default)]
    pub path: Option<String>,
    /// Actions exposed by the tool.
    #[serde(default)]
    pub actions: Vec<String>,
}
