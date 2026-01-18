//! Pack validation types and helpers.

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use semver::Version;
use serde_json::Value;

use crate::{PackId, PackManifest};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

fn empty_data() -> Value {
    Value::Null
}

fn data_is_empty(value: &Value) -> bool {
    value.is_null()
}

/// Severity level for validation diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Severity {
    /// Informational validation message.
    Info,
    /// Warning-level validation message.
    Warn,
    /// Error-level validation message.
    Error,
}

/// Diagnostic entry produced by pack validators.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Diagnostic {
    /// Severity of the diagnostic.
    pub severity: Severity,
    /// Stable machine-readable identifier (for example `PACK_MISSING_SCHEMA`).
    pub code: String,
    /// Human-readable description.
    pub message: String,
    /// Optional path inside the pack or manifest (for example `extensions.messaging.setup`).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub path: Option<String>,
    /// Optional actionable guidance.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub hint: Option<String>,
    /// Optional structured payload for tooling.
    #[cfg_attr(
        feature = "serde",
        serde(default = "empty_data", skip_serializing_if = "data_is_empty")
    )]
    #[cfg_attr(feature = "schemars", schemars(default = "empty_data"))]
    pub data: Value,
}

/// Aggregated validation report for a pack.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ValidationReport {
    /// Optional pack identifier this report refers to.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub pack_id: Option<PackId>,
    /// Optional pack semantic version.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "String", description = "SemVer version")
    )]
    pub pack_version: Option<Version>,
    /// Collected diagnostics.
    #[cfg_attr(feature = "serde", serde(default))]
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    /// Returns `true` when the report includes error diagnostics.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diag| matches!(diag.severity, Severity::Error))
    }

    /// Appends a diagnostic to the report.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

/// Validator for pack manifests that emits diagnostics.
pub trait PackValidator {
    /// Returns the stable validator identifier.
    fn id(&self) -> &'static str;
    /// Returns `true` when the validator applies to the provided manifest.
    fn applies(&self, manifest: &PackManifest) -> bool;
    /// Validates the manifest and returns diagnostics.
    fn validate(&self, manifest: &PackManifest) -> Vec<Diagnostic>;
}

/// Performs domain-agnostic structural validation for a pack manifest.
pub fn validate_pack_manifest_core(manifest: &PackManifest) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if manifest.schema_version.trim().is_empty() {
        diagnostics.push(core_diagnostic(
            Severity::Error,
            "PACK_SCHEMA_VERSION_MISSING",
            "Pack manifest schema_version is required.",
            Some("schema_version".to_owned()),
            Some("Set schema_version to a supported pack manifest version.".to_owned()),
        ));
    }

    if manifest.pack_id.as_str().trim().is_empty() {
        diagnostics.push(core_diagnostic(
            Severity::Error,
            "PACK_ID_MISSING",
            "Pack manifest pack_id is required.",
            Some("pack_id".to_owned()),
            Some("Provide a non-empty pack identifier.".to_owned()),
        ));
    }

    let mut component_ids = BTreeSet::new();
    for component in &manifest.components {
        if !component_ids.insert(component.id.clone()) {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_COMPONENT_ID_DUPLICATE",
                "Duplicate component identifiers are not allowed.",
                Some(format!("components.{}", component.id.as_str())),
                Some("Ensure each component id is unique within the pack.".to_owned()),
            ));
        }
    }

    let mut dependency_aliases = BTreeSet::new();
    for dependency in &manifest.dependencies {
        if dependency.alias.trim().is_empty() {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_DEPENDENCY_ALIAS_MISSING",
                "Pack dependency alias is required.",
                Some("dependencies".to_owned()),
                Some("Provide a non-empty alias for each dependency.".to_owned()),
            ));
        }
        if !dependency_aliases.insert(dependency.alias.clone()) {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_DEPENDENCY_ALIAS_DUPLICATE",
                "Duplicate dependency aliases are not allowed.",
                Some(format!("dependencies.{}", dependency.alias)),
                Some("Ensure each dependency alias is unique within the pack.".to_owned()),
            ));
        }
    }

    let mut flow_ids = BTreeSet::new();
    for entry in &manifest.flows {
        if !flow_ids.insert(entry.id.clone()) {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_FLOW_ID_DUPLICATE",
                "Duplicate flow identifiers are not allowed.",
                Some(format!("flows.{}", entry.id.as_str())),
                Some("Ensure each flow id is unique within the pack.".to_owned()),
            ));
        }

        if entry.id != entry.flow.id {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_FLOW_ID_MISMATCH",
                "Pack flow entry id must match the embedded flow id.",
                Some(format!("flows.{}.id", entry.id.as_str())),
                Some("Align the entry id with the flow.id field.".to_owned()),
            ));
        }

        if entry.kind != entry.flow.kind {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_FLOW_KIND_MISMATCH",
                "Pack flow entry kind must match the embedded flow kind.",
                Some(format!("flows.{}.kind", entry.id.as_str())),
                Some("Align the entry kind with the flow.kind field.".to_owned()),
            ));
        }

        if entry.flow.schema_version.trim().is_empty() {
            diagnostics.push(core_diagnostic(
                Severity::Error,
                "PACK_FLOW_SCHEMA_VERSION_MISSING",
                "Embedded flow schema_version is required.",
                Some(format!("flows.{}.flow.schema_version", entry.id.as_str())),
                Some("Set schema_version to a supported flow version.".to_owned()),
            ));
        }
    }

    for component in &manifest.components {
        if let Some(configurators) = &component.configurators {
            if let Some(flow_id) = &configurators.basic {
                if !flow_ids.contains(flow_id) {
                    diagnostics.push(core_diagnostic(
                        Severity::Error,
                        "PACK_COMPONENT_CONFIG_FLOW_MISSING",
                        "Component configurator flow is not present in the pack manifest.",
                        Some(format!(
                            "components.{}.configurators.basic",
                            component.id.as_str()
                        )),
                        Some("Add the referenced flow to the pack manifest flows.".to_owned()),
                    ));
                }
            }
            if let Some(flow_id) = &configurators.full {
                if !flow_ids.contains(flow_id) {
                    diagnostics.push(core_diagnostic(
                        Severity::Error,
                        "PACK_COMPONENT_CONFIG_FLOW_MISSING",
                        "Component configurator flow is not present in the pack manifest.",
                        Some(format!(
                            "components.{}.configurators.full",
                            component.id.as_str()
                        )),
                        Some("Add the referenced flow to the pack manifest flows.".to_owned()),
                    ));
                }
            }
        }
    }

    for entry in &manifest.flows {
        for (node_id, node) in entry.flow.nodes.iter() {
            match &node.component.pack_alias {
                Some(alias) => {
                    if !dependency_aliases.contains(alias) {
                        diagnostics.push(core_diagnostic(
                            Severity::Error,
                            "PACK_FLOW_DEPENDENCY_ALIAS_MISSING",
                            "Flow node references an unknown dependency alias.",
                            Some(format!(
                                "flows.{}.nodes.{}.component.pack_alias",
                                entry.id.as_str(),
                                node_id.as_str()
                            )),
                            Some("Add the dependency alias to the pack manifest.".to_owned()),
                        ));
                    }
                }
                None => {
                    if !component_ids.contains(&node.component.id) {
                        diagnostics.push(core_diagnostic(
                            Severity::Error,
                            "PACK_FLOW_COMPONENT_MISSING",
                            "Flow node references a component not declared in the pack manifest.",
                            Some(format!(
                                "flows.{}.nodes.{}.component.id",
                                entry.id.as_str(),
                                node_id.as_str()
                            )),
                            Some("Declare the component in the pack manifest.".to_owned()),
                        ));
                    }
                }
            }
        }
    }

    diagnostics
}

fn core_diagnostic(
    severity: Severity,
    code: &str,
    message: &str,
    path: Option<String>,
    hint: Option<String>,
) -> Diagnostic {
    Diagnostic {
        severity,
        code: code.to_owned(),
        message: message.to_owned(),
        path,
        hint,
        data: empty_data(),
    }
}
