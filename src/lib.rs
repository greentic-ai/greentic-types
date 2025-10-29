#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

//! Shared types and helpers for Greentic multi-tenant flows.

extern crate alloc;

#[cfg(feature = "serde")]
pub mod pack_spec;

#[cfg(feature = "serde")]
pub use pack_spec::{PackSpec, ToolSpec};

use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use core::fmt;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub fn to_offset_date_time(&self) -> Result<OffsetDateTime, time::error::ComponentRange> {
        OffsetDateTime::from_unix_timestamp_nanos(self.unix_millis * 1_000_000)
    }

    /// Creates a deadline from an [`OffsetDateTime`], truncating to milliseconds.
    pub fn from_offset_date_time(value: OffsetDateTime) -> Self {
        let nanos = value.unix_timestamp_nanos();
        Self {
            unix_millis: nanos / 1_000_000,
        }
    }
}

/// Context that accompanies every invocation across Greentic runtimes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TenantCtx {
    /// Environment scope (for example `dev`, `staging`, or `prod`).
    pub env: EnvId,
    /// Tenant identifier for the current execution.
    pub tenant: TenantId,
    /// Optional team identifier scoped to the tenant.
    pub team: Option<TeamId>,
    /// Optional user identifier scoped to the tenant.
    pub user: Option<UserId>,
    /// Distributed tracing identifier when available.
    pub trace_id: Option<String>,
    /// Correlation identifier for linking related events.
    pub correlation_id: Option<String>,
    /// Deadline when the invocation should finish.
    pub deadline: Option<InvocationDeadline>,
    /// Attempt counter for retried invocations (starting at zero).
    pub attempt: u32,
    /// Stable idempotency key propagated across retries.
    pub idempotency_key: Option<String>,
}

impl TenantCtx {
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
}

/// Primary payload representation shared across envelopes.
pub type BinaryPayload = Vec<u8>;

/// Normalized ingress payload delivered to nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub enum ErrorDetail {
    /// UTF-8 encoded detail payload.
    Text(String),
    /// Binary payload detail (for example message pack or CBOR).
    Binary(BinaryPayload),
}

/// Error type emitted by Greentic nodes.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        .or_else(|| ctx.correlation_id.as_deref())
        .unwrap_or_default();
    let input = format!(
        "{}|{}|{}|{}",
        ctx.tenant.as_str(),
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
        TenantCtx {
            env: EnvId::from("prod"),
            tenant: TenantId::from("tenant-123"),
            team: Some(TeamId::from("team-456")),
            user: Some(UserId::from("user-789")),
            trace_id: Some("trace-abc".to_owned()),
            correlation_id: Some("corr-xyz".to_owned()),
            deadline: Some(InvocationDeadline::from_unix_millis(1_700_000_000_000)),
            attempt: 2,
            idempotency_key: Some("key-123".to_owned()),
        }
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
    fn deadline_roundtrips_through_offset_datetime() {
        let dt = OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("valid timestamp");
        let deadline = InvocationDeadline::from_offset_date_time(dt);
        let roundtrip = deadline
            .to_offset_date_time()
            .expect("round-trip conversion");
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
        use std::io::{Error, ErrorKind};

        let source = Error::new(ErrorKind::Other, "boom");
        let err = NodeError::new("TEST", "example").with_source(source);
        assert!(err.source().is_some());
    }
}
