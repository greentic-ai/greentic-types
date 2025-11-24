//! Supply-chain oriented shared types (builds, scans, signing, metadata).

use alloc::{string::String, vec::Vec};
use core::hash::BuildHasherDefault;
use fnv::FnvHasher;
use indexmap::IndexMap;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "time")]
use time::OffsetDateTime;

use crate::{
    ArtifactRef, AttestationRef, BuildRef, ComponentRef, RegistryRef, RepoRef, SbomRef, ScanRef,
    SignatureRef, SigningKeyRef, StatementRef, StoreRef, TenantCtx,
};

/// Hasher used for IndexMap fields to stay `no_std` friendly.
pub type SupplyHasher = BuildHasherDefault<FnvHasher>;

/// Plan describing how to execute a build.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BuildPlan {
    /// Identifier for the build.
    pub build_id: BuildRef,
    /// Component being built.
    pub component: ComponentRef,
    /// Source repository reference.
    pub source_repo: RepoRef,
    /// Commit identifier from the source repository.
    pub commit: String,
    /// Language or ecosystem descriptor (for example `rust`, `nodejs`).
    pub language: String,
    /// Entrypoint or build target.
    pub entrypoint: String,
    /// Environment variables passed to the build.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "IndexMap::is_empty")
    )]
    #[cfg_attr(
        feature = "schemars",
        schemars(
            with = "alloc::collections::BTreeMap<String, String>",
            description = "Environment variables"
        )
    )]
    pub env: IndexMap<String, String, SupplyHasher>,
    /// Expected outputs (artifact references).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub outputs: Vec<ArtifactRef>,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Lifecycle status for a build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum BuildStatusKind {
    /// Build has been accepted but not started.
    Pending,
    /// Build is currently running.
    Running,
    /// Build finished successfully.
    Succeeded,
    /// Build failed.
    Failed,
    /// Build was cancelled before completion.
    Cancelled,
}

/// Summary status for a build execution.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct BuildStatus {
    /// Identifier for the build.
    pub build_id: BuildRef,
    /// Current status.
    pub status: BuildStatusKind,
    /// Build start time (UTC).
    #[cfg_attr(
        all(feature = "schemars", feature = "time"),
        schemars(with = "Option<String>", description = "RFC3339 timestamp in UTC")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg(feature = "time")]
    pub started_at_utc: Option<OffsetDateTime>,
    /// Build finish time (UTC).
    #[cfg_attr(
        all(feature = "schemars", feature = "time"),
        schemars(with = "Option<String>", description = "RFC3339 timestamp in UTC")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg(feature = "time")]
    pub finished_at_utc: Option<OffsetDateTime>,
    /// Produced artifacts.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub artifacts: Vec<ArtifactRef>,
    /// Optional build logs reference.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub logs_ref: Option<String>,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Supported scan kinds.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum ScanKind {
    /// Source code analysis (SAST).
    Source,
    /// Dependency or composition analysis.
    Dependencies,
    /// Binary or container image analysis.
    Artifact,
    /// Custom or provider-specific scan.
    Custom(String),
}

/// Request to execute a scan.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ScanRequest {
    /// Identifier for the scan.
    pub scan_id: ScanRef,
    /// Component being scanned.
    pub component: ComponentRef,
    /// Scan kind.
    pub kind: ScanKind,
    /// Target artifact (when applicable).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub artifact: Option<ArtifactRef>,
    /// Provider-specific inputs.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Lifecycle status for a scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum ScanStatusKind {
    /// Scan has been accepted but not started.
    Pending,
    /// Scan is currently running.
    Running,
    /// Scan finished successfully.
    Succeeded,
    /// Scan failed.
    Failed,
}

/// Result summary for a scan.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ScanResult {
    /// Identifier for the scan.
    pub scan_id: ScanRef,
    /// Component scanned.
    pub component: ComponentRef,
    /// Scan kind.
    pub kind: ScanKind,
    /// Final scan status.
    pub status: ScanStatusKind,
    /// Optional SBOM reference emitted by the scan.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sbom: Option<SbomRef>,
    /// Scanner-specific findings.
    #[cfg_attr(feature = "serde", serde(default))]
    pub findings: Value,
    /// Scan start time (UTC).
    #[cfg_attr(
        all(feature = "schemars", feature = "time"),
        schemars(with = "Option<String>", description = "RFC3339 timestamp in UTC")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg(feature = "time")]
    pub started_at_utc: Option<OffsetDateTime>,
    /// Scan finish time (UTC).
    #[cfg_attr(
        all(feature = "schemars", feature = "time"),
        schemars(with = "Option<String>", description = "RFC3339 timestamp in UTC")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg(feature = "time")]
    pub finished_at_utc: Option<OffsetDateTime>,
}

/// Signing request for an artifact.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SignRequest {
    /// Signing key reference.
    pub signing_key: SigningKeyRef,
    /// Artifact to sign.
    pub artifact: ArtifactRef,
    /// Payload provided to the signer (hashes, claims, etc.).
    #[cfg_attr(feature = "serde", serde(default))]
    pub payload: Value,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Verification request for a signature.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VerifyRequest {
    /// Signature reference to verify.
    pub signature: SignatureRef,
    /// Subject artifact associated with the signature.
    pub artifact: ArtifactRef,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Verification result.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct VerifyResult {
    /// Signature reference.
    pub signature: SignatureRef,
    /// Whether the signature is valid.
    pub valid: bool,
    /// Optional diagnostic message.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub message: Option<String>,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Predicate type for attestations.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum PredicateType {
    /// SLSA provenance predicate.
    Slsa,
    /// Vulnerability assessment predicate.
    Vulnerability,
    /// Custom predicate identified by name.
    Custom(String),
}

/// Attestation statement descriptor.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AttestationStatement {
    /// Attestation identifier.
    pub attestation: AttestationRef,
    /// Predicate type describing the attestation.
    pub predicate_type: PredicateType,
    /// Statement reference (for example DSSE envelope).
    pub statement: StatementRef,
    /// Optional registry where the attestation is stored.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub registry: Option<RegistryRef>,
    /// Optional content store reference.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub store: Option<StoreRef>,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Value,
}

/// Generic metadata record attached to supply-chain entities.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct MetadataRecord {
    /// Optional namespace grouping related keys.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub namespace: Option<String>,
    /// Metadata key (lower_snake_case or dotted).
    pub key: String,
    /// Metadata value as arbitrary JSON.
    pub value: Value,
}

/// Repository-scoped context for convenience.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RepoContext {
    /// Tenant context.
    pub tenant: TenantCtx,
    /// Repository reference.
    pub repo: RepoRef,
}

/// Store-scoped context for convenience.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct StoreContext {
    /// Tenant context.
    pub tenant: TenantCtx,
    /// Store reference.
    pub store: StoreRef,
}
