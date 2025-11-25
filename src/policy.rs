//! Network policy primitives.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Network protocols supported by allow lists.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Protocol {
    /// Hypertext Transfer Protocol.
    Http,
    /// Hypertext Transfer Protocol Secure.
    Https,
    /// Generic TCP connectivity.
    Tcp,
    /// Generic UDP connectivity.
    Udp,
    /// gRPC.
    Grpc,
    /// Any protocol not covered above.
    Custom(String),
}

/// Allow list describing permitted domains, ports, and protocols.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AllowList {
    /// Allowed domain suffixes or exact hosts.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub domains: Vec<String>,
    /// Allowed port numbers.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub ports: Vec<u16>,
    /// Allowed network protocols.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub protocols: Vec<Protocol>,
}

impl AllowList {
    /// Creates an empty allow list.
    pub fn empty() -> Self {
        Self {
            domains: Vec::new(),
            ports: Vec::new(),
            protocols: Vec::new(),
        }
    }

    /// Returns `true` when the allow list contains no rules.
    pub fn is_empty(&self) -> bool {
        self.domains.is_empty() && self.ports.is_empty() && self.protocols.is_empty()
    }
}

impl Default for AllowList {
    fn default() -> Self {
        Self::empty()
    }
}

/// High-level network policy composed of allow lists.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct NetworkPolicy {
    /// Allow list enforced for egress connections.
    pub egress: AllowList,
    /// Whether destinations not present in the allow list should be denied.
    pub deny_on_miss: bool,
}

impl NetworkPolicy {
    /// Creates a policy denying unknown destinations by default.
    pub fn strict(egress: AllowList) -> Self {
        Self {
            egress,
            deny_on_miss: true,
        }
    }
}

/// Result of evaluating a network policy.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PolicyDecision {
    /// Whether the request is allowed.
    pub allow: bool,
    /// Optional human-readable reason.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reason: Option<String>,
    /// Status enum mirroring the allow flag.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub status: Option<PolicyDecisionStatus>,
    /// Optional list of reasons.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub reasons: Vec<String>,
}

/// Status for a policy decision.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum PolicyDecisionStatus {
    /// Request is allowed.
    Allow,
    /// Request is denied.
    Deny,
}
