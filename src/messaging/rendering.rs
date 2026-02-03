//! Provider-agnostic DTOs for host messaging render plans.
//!
//! These types describe renderer modes, capability profiles, and diagnostics for adapters and
//! host-side logic. They keep `greentic-types` strictly declarative; actual downgrade and rendering
//! decisions live in the caller implementations, and providers may ignore hints they do not support.

extern crate alloc;

use alloc::{string::String, vec::Vec};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

fn is_false(value: &bool) -> bool {
    !*value
}

/// Stable renderer modes hosts can request from providers.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(tag = "mode", rename_all = "snake_case"))]
pub enum RendererMode {
    /// Forward the card/message exactly as produced.
    Passthrough,
    /// Downgrade to plain text (Tier D behavior).
    TextOnly,
    /// Downgrade adaptive cards to a safer version before sending to the provider.
    AdaptiveCardDowngrade {
        /// Target adaptive card version (e.g. "1.4").
        target_version: AdaptiveCardVersion,
        /// Fail on unsupported features when `true`, otherwise best-effort downgrade.
        #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_false"))]
        strict: bool,
    },
}

/// Wrapper for adaptive card schema versions so DTO contracts remain explicit even if no validation is applied.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct AdaptiveCardVersion(String);

impl AdaptiveCardVersion {
    /// Returns the inner version string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for AdaptiveCardVersion {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for AdaptiveCardVersion {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<str> for AdaptiveCardVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::fmt::Display for AdaptiveCardVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Provider-agnostic render tiers used for diagnostics and plans.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Tier {
    /// Tier A renders the full adaptive card experience when the destination fully supports it.
    TierA,
    /// Tier B targets partial rendering when some features may be missing.
    TierB,
    /// Tier C offers simplified layouts or read-only cards where interactions are limited.
    TierC,
    /// Tier D downgrades to plain text or minimal content for the most restrictive destinations.
    TierD,
}

/// Capabilities associated with a destination/provider.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CapabilityProfile {
    /// Maximum adaptive card version the destination declares support for.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub max_adaptive_card_version: Option<AdaptiveCardVersion>,
    /// Whether adaptive cards are supported (unknown when `None`).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supports_adaptive_cards: Option<bool>,
    /// Whether interactive actions (e.g. submit, openUrl) are supported.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supports_actions: Option<bool>,
    /// Whether media playback/gifs are supported by the destination.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supports_media: Option<bool>,
    /// Whether input controls (choice set, date picker) are supported.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supports_input_controls: Option<bool>,
    /// Whether Markdown formatting is supported.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supports_markdown: Option<bool>,
}

/// Diagnostics attached to a render plan for logs and tests.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RenderDiagnostics {
    /// Optional tier-level summary for diagnostics.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub tier: Option<Tier>,
    /// Warning messages describing graceful degradations.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub warnings: Vec<String>,
    /// Error messages describing failures or skipped content.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub errors: Vec<String>,
}

/// Hints that can be attached to render plans without coupling to a specific provider implementation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RenderPlanHints {
    /// Desired renderer mode requested by the host.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub renderer_mode: Option<RendererMode>,
    /// Capability profile for the target destination or channel.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub capability_profile: Option<CapabilityProfile>,
    /// Diagnostics emitted while producing the plan.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub diagnostics: Option<RenderDiagnostics>,
    /// Optional tier summary for the plan body.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub tier: Option<Tier>,
}
