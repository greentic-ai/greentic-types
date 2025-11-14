//! Generic flow graph definitions used by packs and runtimes.

use alloc::string::String;
use core::hash::BuildHasherDefault;

use fnv::FnvHasher;
use indexmap::IndexMap;
use serde_json::Value;

use crate::{ComponentId, FlowId, NodeId, component::ComponentManifest};

/// Build hasher used for flow node maps (Fnv for `no_std` friendliness).
type FlowHasher = BuildHasherDefault<FnvHasher>;

/// Ordered node container referenced by [`Flow`].
pub type FlowNodes = IndexMap<NodeId, Node, FlowHasher>;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported flow kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum FlowKind {
    /// Session-centric messaging flows (chat, DM, etc.).
    Messaging,
    /// Fire-and-forget event flows (webhooks, timers, etc.).
    Events,
}

/// Canonical .ygtc flow representation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Flow {
    /// Flow execution kind.
    pub kind: FlowKind,
    /// Flow identifier inside the pack.
    pub id: FlowId,
    /// Optional human-friendly summary.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,
    /// Ordered node map describing the flow graph.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "alloc::collections::BTreeMap<NodeId, Node>")
    )]
    pub nodes: FlowNodes,
}

impl Flow {
    /// Returns `true` when no nodes are defined.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the implicit ingress node (first user-declared entry).
    pub fn ingress(&self) -> Option<(&NodeId, &Node)> {
        self.nodes.iter().next()
    }

    /// Validates the flow structure (at least one node).
    pub fn validate_structure(&self) -> Result<(), FlowValidationError> {
        if self.is_empty() {
            return Err(FlowValidationError::EmptyFlow);
        }
        Ok(())
    }

    /// Ensures all referenced components exist and support this flow kind.
    pub fn validate_components<'a, F>(&self, mut resolver: F) -> Result<(), FlowValidationError>
    where
        F: FnMut(&ComponentId) -> Option<&'a ComponentManifest>,
    {
        self.validate_structure()?;
        for (node_id, node) in &self.nodes {
            if let Some(component_id) = &node.component {
                let manifest = resolver(component_id).ok_or_else(|| {
                    FlowValidationError::MissingComponent {
                        node_id: node_id.clone(),
                        component: component_id.clone(),
                    }
                })?;

                if !manifest.supports_kind(self.kind) {
                    return Err(FlowValidationError::UnsupportedComponent {
                        node_id: node_id.clone(),
                        component: component_id.clone(),
                        flow_kind: self.kind,
                    });
                }
            }
        }
        Ok(())
    }
}

/// Flow node metadata. All semantics are opaque strings or documents.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Node {
    /// Component kind (opaque string interpreted by tooling/runtime).
    pub kind: String,
    /// Optional profile override for this node.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub profile: Option<String>,
    /// Optional component binding identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub component: Option<ComponentId>,
    /// Component-specific configuration blob.
    #[cfg_attr(feature = "serde", serde(default))]
    pub config: Value,
    /// Opaque routing document interpreted by the component.
    #[cfg_attr(feature = "serde", serde(default))]
    pub routing: Value,
}

/// Validation errors produced by [`Flow::validate_components`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowValidationError {
    /// Flow has no nodes.
    EmptyFlow,
    /// Node references a component that is missing from the manifest set.
    MissingComponent {
        /// Offending node identifier.
        node_id: NodeId,
        /// Referenced component identifier.
        component: ComponentId,
    },
    /// Component does not support the flow kind.
    UnsupportedComponent {
        /// Offending node identifier.
        node_id: NodeId,
        /// Referenced component identifier.
        component: ComponentId,
        /// Flow kind the node participates in.
        flow_kind: FlowKind,
    },
}

impl core::fmt::Display for FlowValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FlowValidationError::EmptyFlow => f.write_str("flows must declare at least one node"),
            FlowValidationError::MissingComponent { node_id, component } => write!(
                f,
                "node `{}` references missing component `{}`",
                node_id.as_str(),
                component.as_str()
            ),
            FlowValidationError::UnsupportedComponent {
                node_id,
                component,
                flow_kind,
            } => write!(
                f,
                "component `{}` used by node `{}` does not support `{:?}` flows",
                component.as_str(),
                node_id.as_str(),
                flow_kind
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FlowValidationError {}
