//! Provider installation records shared across provisioning workflows.

use alloc::collections::BTreeMap;
use alloc::string::String;

use semver::Version;
use serde_json::Value;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "time")]
use time::OffsetDateTime;

use crate::{PackId, ProviderInstallId, TenantCtx};

/// Reference map for configuration or secret entries.
pub type ProviderInstallRefs = BTreeMap<String, String>;

/// Provider installation record shared across domains.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderInstallRecord {
    /// Tenant context for the install.
    pub tenant: TenantCtx,
    /// Provider identifier (string-based to avoid enum coupling).
    pub provider_id: String,
    /// Installation identifier.
    pub install_id: ProviderInstallId,
    /// Pack identifier used for provisioning.
    pub pack_id: PackId,
    /// Pack version used for provisioning.
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "String", description = "SemVer version")
    )]
    pub pack_version: Version,
    /// Install creation timestamp.
    #[cfg(feature = "time")]
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "String", description = "RFC3339 timestamp (UTC)")
    )]
    pub created_at: OffsetDateTime,
    /// Install last update timestamp.
    #[cfg(feature = "time")]
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    #[cfg_attr(
        feature = "schemars",
        schemars(with = "String", description = "RFC3339 timestamp (UTC)")
    )]
    pub updated_at: OffsetDateTime,
    /// Configuration references produced by provisioning.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub config_refs: ProviderInstallRefs,
    /// Secret references produced by provisioning.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub secret_refs: ProviderInstallRefs,
    /// Webhook provisioning state (opaque).
    #[cfg_attr(feature = "serde", serde(default))]
    pub webhook_state: Value,
    /// Subscription provisioning state (opaque).
    #[cfg_attr(feature = "serde", serde(default))]
    pub subscriptions_state: Value,
    /// Free-form metadata from provisioning workflows.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}
