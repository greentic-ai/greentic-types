#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

//! Shared types and helpers for Greentic multi-tenant flows.

extern crate alloc;

pub mod pack_spec;

pub mod context;
pub mod error;
pub mod outcome;
pub mod pack;
pub mod policy;
pub mod session;
pub mod state;
pub mod telemetry;
pub mod tenant;

pub use context::{Cloud, DeploymentCtx, Platform};
pub use error::{ErrorCode, GResult, GreenticError};
pub use outcome::Outcome;
pub use pack::{PackRef, Signature, SignatureAlgorithm};
pub use pack_spec::{PackSpec, ToolSpec};
pub use policy::{AllowList, NetworkPolicy, PolicyDecision, Protocol};
pub use session::{SessionCursor, SessionKey};
pub use state::{StateKey, StatePath};
pub use telemetry::SpanContext;
#[cfg(feature = "telemetry-autoinit")]
pub use telemetry::TelemetryCtx;
pub use tenant::{Impersonation, TenantIdentity};

use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use core::fmt;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "time")]
use time::OffsetDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use alloc::boxed::Box;

#[cfg(feature = "std")]
use std::error::Error as StdError;

macro_rules! id_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "schemars", derive(JsonSchema))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name(pub String);

        impl $name {
            /// Returns the identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

id_newtype!(EnvId, "Environment identifier for a tenant context.");
id_newtype!(TenantId, "Tenant identifier within an environment.");
id_newtype!(TeamId, "Team identifier belonging to a tenant.");
id_newtype!(UserId, "User identifier within a tenant.");

/// Deadline metadata for an invocation, stored as Unix epoch milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InvocationDeadline {
    unix_millis: i128,
}

impl InvocationDeadline {
    /// Creates a deadline from a Unix timestamp expressed in milliseconds.
    pub const fn from_unix_millis(unix_millis: i128) -> Self {
        Self { unix_millis }
    }

    /// Returns the deadline as Unix epoch milliseconds.
    pub const fn unix_millis(&self) -> i128 {
        self.unix_millis
    }

    /// Converts the deadline into an [`OffsetDateTime`].
    #[cfg(feature = "time")]
    pub fn to_offset_date_time(&self) -> Result<OffsetDateTime, time::error::ComponentRange> {
        OffsetDateTime::from_unix_timestamp_nanos(self.unix_millis * 1_000_000)
    }

    /// Creates a deadline from an [`OffsetDateTime`], truncating to milliseconds.
    #[cfg(feature = "time")]
    pub fn from_offset_date_time(value: OffsetDateTime) -> Self {
        let nanos = value.unix_timestamp_nanos();
        Self {
            unix_millis: nanos / 1_000_000,
        }
    }
}

/// Context that accompanies every invocation across Greentic runtimes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TenantCtx {
    /// Environment scope (for example `dev`, `staging`, or `prod`).
    pub env: EnvId,
    /// Tenant identifier for the current execution.
    pub tenant: TenantId,
    /// Stable tenant identifier reference used across systems.
    pub tenant_id: TenantId,
    /// Optional team identifier scoped to the tenant.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub team: Option<TeamId>,
    /// Optional team identifier accessible via the shared schema.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub team_id: Option<TeamId>,
    /// Optional user identifier scoped to the tenant.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub user: Option<UserId>,
    /// Optional user identifier aligned with the shared schema.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub user_id: Option<UserId>,
    /// Optional session identifier propagated by the runtime.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub session_id: Option<String>,
    /// Optional flow identifier for the current execution.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub flow_id: Option<String>,
    /// Optional node identifier within the flow.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub node_id: Option<String>,
    /// Optional provider identifier describing the runtime surface.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub provider_id: Option<String>,
    /// Distributed tracing identifier when available.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub trace_id: Option<String>,
    /// Correlation identifier for linking related events.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub correlation_id: Option<String>,
    /// Deadline when the invocation should finish.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub deadline: Option<InvocationDeadline>,
    /// Attempt counter for retried invocations (starting at zero).
    pub attempt: u32,
    /// Stable idempotency key propagated across retries.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub idempotency_key: Option<String>,
    /// Optional impersonation context describing the acting identity.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub impersonation: Option<Impersonation>,
}

impl TenantCtx {
    /// Creates a new tenant context with the provided environment and tenant identifiers.
    pub fn new(env: EnvId, tenant: TenantId) -> Self {
        let tenant_id = tenant.clone();
        Self {
            env,
            tenant: tenant.clone(),
            tenant_id,
            team: None,
            team_id: None,
            user: None,
            user_id: None,
            session_id: None,
            flow_id: None,
            node_id: None,
            provider_id: None,
            trace_id: None,
            correlation_id: None,
            deadline: None,
            attempt: 0,
            idempotency_key: None,
            impersonation: None,
        }
    }

    /// Updates the team information ensuring legacy and shared fields stay aligned.
    pub fn with_team(mut self, team: Option<TeamId>) -> Self {
        self.team = team.clone();
        self.team_id = team;
        self
    }

    /// Updates the user information ensuring legacy and shared fields stay aligned.
    pub fn with_user(mut self, user: Option<UserId>) -> Self {
        self.user = user.clone();
        self.user_id = user;
        self
    }

    /// Updates the session identifier.
    pub fn with_session(mut self, session: impl Into<String>) -> Self {
        self.session_id = Some(session.into());
        self
    }

    /// Updates the flow identifier.
    pub fn with_flow(mut self, flow: impl Into<String>) -> Self {
        self.flow_id = Some(flow.into());
        self
    }

    /// Updates the node identifier.
    pub fn with_node(mut self, node: impl Into<String>) -> Self {
        self.node_id = Some(node.into());
        self
    }

    /// Updates the provider identifier.
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider_id = Some(provider.into());
        self
    }

    /// Sets the impersonation context.
    pub fn with_impersonation(mut self, impersonation: Option<Impersonation>) -> Self {
        self.impersonation = impersonation;
        self
    }

    /// Returns a copy of the context with the provided attempt value.
    pub fn with_attempt(mut self, attempt: u32) -> Self {
        self.attempt = attempt;
        self
    }

    /// Updates the deadline metadata for subsequent invocations.
    pub fn with_deadline(mut self, deadline: Option<InvocationDeadline>) -> Self {
        self.deadline = deadline;
        self
    }

    /// Returns the session identifier, when present.
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Returns the flow identifier, when present.
    pub fn flow_id(&self) -> Option<&str> {
        self.flow_id.as_deref()
    }

    /// Returns the node identifier, when present.
    pub fn node_id(&self) -> Option<&str> {
        self.node_id.as_deref()
    }

    /// Returns the provider identifier, when present.
    pub fn provider_id(&self) -> Option<&str> {
        self.provider_id.as_deref()
    }
}

/// Primary payload representation shared across envelopes.
pub type BinaryPayload = Vec<u8>;

/// Normalized ingress payload delivered to nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InvocationEnvelope {
    /// Tenant context for the invocation.
    pub ctx: TenantCtx,
    /// Flow identifier the event belongs to.
    pub flow_id: String,
    /// Optional node identifier within the flow.
    pub node_id: Option<String>,
    /// Operation being invoked (for example `on_message` or `tick`).
    pub op: String,
    /// Normalized payload for the invocation.
    pub payload: BinaryPayload,
    /// Raw metadata propagated from the ingress surface.
    pub metadata: BinaryPayload,
}

/// Structured detail payload attached to a node error.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum ErrorDetail {
    /// UTF-8 encoded detail payload.
    Text(String),
    /// Binary payload detail (for example message pack or CBOR).
    Binary(BinaryPayload),
}

/// Error type emitted by Greentic nodes.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct NodeError {
    /// Machine readable error code.
    pub code: String,
    /// Human readable message explaining the failure.
    pub message: String,
    /// Whether the failure is retryable by the runtime.
    pub retryable: bool,
    /// Optional backoff duration in milliseconds for the next retry.
    pub backoff_ms: Option<u64>,
    /// Optional structured error detail payload.
    pub details: Option<ErrorDetail>,
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "serde", serde(skip, default = "default_source"))]
    #[cfg_attr(feature = "schemars", schemars(skip))]
    source: Option<Box<dyn StdError + Send + Sync>>,
}

#[cfg(all(feature = "std", feature = "serde"))]
fn default_source() -> Option<Box<dyn StdError + Send + Sync>> {
    None
}

impl NodeError {
    /// Constructs a non-retryable failure with the supplied code and message.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable: false,
            backoff_ms: None,
            details: None,
            #[cfg(feature = "std")]
            source: None,
        }
    }

    /// Marks the error as retryable with an optional backoff value.
    pub fn with_retry(mut self, backoff_ms: Option<u64>) -> Self {
        self.retryable = true;
        self.backoff_ms = backoff_ms;
        self
    }

    /// Attaches structured details to the error.
    pub fn with_detail(mut self, detail: ErrorDetail) -> Self {
        self.details = Some(detail);
        self
    }

    /// Attaches a textual detail payload to the error.
    pub fn with_detail_text(mut self, detail: impl Into<String>) -> Self {
        self.details = Some(ErrorDetail::Text(detail.into()));
        self
    }

    /// Attaches a binary detail payload to the error.
    pub fn with_detail_binary(mut self, detail: BinaryPayload) -> Self {
        self.details = Some(ErrorDetail::Binary(detail));
        self
    }

    #[cfg(feature = "std")]
    /// Attaches a source error to the failure for debugging purposes.
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the structured details, when available.
    pub fn detail(&self) -> Option<&ErrorDetail> {
        self.details.as_ref()
    }

    #[cfg(feature = "std")]
    /// Returns the attached source error when one has been provided.
    pub fn source(&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[cfg(feature = "std")]
impl StdError for NodeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|err| err.as_ref() as &(dyn StdError + 'static))
    }
}

/// Alias for results returned by node handlers.
pub type NodeResult<T> = Result<T, NodeError>;

/// Generates a stable idempotency key for a node invocation.
///
/// The key uses tenant, flow, node, and correlation identifiers. Missing
/// correlation values fall back to the value stored on the context.
pub fn make_idempotency_key(
    ctx: &TenantCtx,
    flow_id: &str,
    node_id: Option<&str>,
    correlation: Option<&str>,
) -> String {
    let node_segment = node_id.unwrap_or_default();
    let correlation_segment = correlation
        .or(ctx.correlation_id.as_deref())
        .unwrap_or_default();
    let input = format!(
        "{}|{}|{}|{}",
        ctx.tenant_id.as_str(),
        flow_id,
        node_segment,
        correlation_segment
    );
    fnv1a_128_hex(input.as_bytes())
}

const FNV_OFFSET_BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
const FNV_PRIME: u128 = 0x0000000001000000000000000000013b;

fn fnv1a_128_hex(bytes: &[u8]) -> String {
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash ^= byte as u128;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    let mut output = String::with_capacity(32);
    for shift in (0..32).rev() {
        let nibble = ((hash >> (shift * 4)) & 0x0f) as u8;
        output.push(match nibble {
            0..=9 => (b'0' + nibble) as char,
            _ => (b'a' + (nibble - 10)) as char,
        });
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    fn sample_ctx() -> TenantCtx {
        let mut ctx = TenantCtx::new(EnvId::from("prod"), TenantId::from("tenant-123"))
            .with_team(Some(TeamId::from("team-456")))
            .with_user(Some(UserId::from("user-789")))
            .with_attempt(2)
            .with_deadline(Some(InvocationDeadline::from_unix_millis(
                1_700_000_000_000,
            )));
        ctx.trace_id = Some("trace-abc".to_owned());
        ctx.correlation_id = Some("corr-xyz".to_owned());
        ctx.idempotency_key = Some("key-123".to_owned());
        ctx
    }

    #[test]
    fn idempotent_key_stable() {
        let ctx = sample_ctx();
        let key_a = make_idempotency_key(&ctx, "flow-1", Some("node-1"), Some("corr-override"));
        let key_b = make_idempotency_key(&ctx, "flow-1", Some("node-1"), Some("corr-override"));
        assert_eq!(key_a, key_b);
        assert_eq!(key_a.len(), 32);
    }

    #[test]
    fn idempotent_key_uses_context_correlation() {
        let ctx = sample_ctx();
        let key = make_idempotency_key(&ctx, "flow-1", None, None);
        let expected = make_idempotency_key(&ctx, "flow-1", None, ctx.correlation_id.as_deref());
        assert_eq!(key, expected);
    }

    #[test]
    #[cfg(feature = "time")]
    fn deadline_roundtrips_through_offset_datetime() {
        let dt = OffsetDateTime::from_unix_timestamp(1_700_000_000)
            .unwrap_or_else(|err| panic!("valid timestamp: {err}"));
        let deadline = InvocationDeadline::from_offset_date_time(dt);
        let roundtrip = deadline
            .to_offset_date_time()
            .unwrap_or_else(|err| panic!("round-trip conversion failed: {err}"));
        let millis = dt.unix_timestamp_nanos() / 1_000_000;
        assert_eq!(deadline.unix_millis(), millis);
        assert_eq!(roundtrip.unix_timestamp_nanos() / 1_000_000, millis);
    }

    #[test]
    fn node_error_builder_sets_fields() {
        let err = NodeError::new("TEST", "example")
            .with_retry(Some(500))
            .with_detail_text("context");

        assert!(err.retryable);
        assert_eq!(err.backoff_ms, Some(500));
        match err.detail() {
            Some(ErrorDetail::Text(detail)) => assert_eq!(detail, "context"),
            other => panic!("unexpected detail {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn node_error_source_roundtrips() {
        use std::io::Error;

        let source = Error::other("boom");
        let err = NodeError::new("TEST", "example").with_source(source);
        assert!(err.source().is_some());
    }
}
