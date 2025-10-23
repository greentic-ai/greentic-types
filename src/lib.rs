#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

//! Shared types and helpers for Greentic multi-tenant flows.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fmt;

macro_rules! id_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

/// Context that accompanies every invocation across Greentic runtimes.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Deadline in Unix epoch milliseconds when the invocation should finish.
    pub deadline_unix_ms: Option<u64>,
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
}

/// Error type emitted by Greentic nodes.
#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    /// Application provided failure.
    #[error("{code}: {message}")]
    Fail {
        /// Machine readable error code.
        code: String,
        /// Human readable message explaining the failure.
        message: String,
        /// Whether the failure is retryable by the runtime.
        retryable: bool,
        /// Optional backoff duration in milliseconds for the next retry.
        backoff_ms: Option<u64>,
        /// Optional source error when available.
        #[source]
        source: Option<Box<dyn Error + Send + Sync>>,
        /// Optional structured error detail payload.
        details: Option<Value>,
    },
}

impl NodeError {
    /// Constructs a non-retryable failure with the supplied code and message.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        NodeError::Fail {
            code: code.into(),
            message: message.into(),
            retryable: false,
            backoff_ms: None,
            source: None,
            details: None,
        }
    }

    /// Marks the error as retryable with an optional backoff value.
    pub fn with_retry(mut self, backoff_ms: Option<u64>) -> Self {
        match &mut self {
            NodeError::Fail {
                retryable,
                backoff_ms: existing_backoff,
                ..
            } => {
                *retryable = true;
                *existing_backoff = backoff_ms;
            }
        }
        self
    }

    /// Attaches structured details to the error.
    pub fn with_details(mut self, details: Value) -> Self {
        match &mut self {
            NodeError::Fail { details: slot, .. } => {
                *slot = Some(details);
            }
        }
        self
    }

    /// Attaches a source error to the failure for debugging purposes.
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        match &mut self {
            NodeError::Fail { source: slot, .. } => {
                *slot = Some(Box::new(source));
            }
        }
        self
    }
}

/// Alias for results returned by node handlers.
pub type NodeResult<T> = Result<T, NodeError>;

/// Normalized ingress payload delivered to nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub payload: Value,
    /// Raw metadata propagated from the ingress surface.
    pub metadata: Value,
}

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
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

/// Serializes a value into JSON while returning a structured error on failure.
pub fn safe_json<T>(value: &T) -> NodeResult<Value>
where
    T: Serialize,
{
    serde_json::to_value(value).map_err(|err| {
        let message = err.to_string();
        NodeError::new("SERDE_JSON", "failed to serialize value to JSON")
            .with_source(err)
            .with_details(json!({ "error": message }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    fn sample_ctx() -> TenantCtx {
        TenantCtx {
            env: EnvId::from("prod"),
            tenant: TenantId::from("tenant-123"),
            team: Some(TeamId::from("team-456")),
            user: Some(UserId::from("user-789")),
            trace_id: Some("trace-abc".to_owned()),
            correlation_id: Some("corr-xyz".to_owned()),
            deadline_unix_ms: Some(1_700_000_000_000),
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
    }

    #[test]
    fn idempotent_key_uses_context_correlation() {
        let ctx = sample_ctx();
        let key = make_idempotency_key(&ctx, "flow-1", None, None);
        let expected = make_idempotency_key(&ctx, "flow-1", None, ctx.correlation_id.as_deref());
        assert_eq!(key, expected);
    }

    #[test]
    fn safe_json_serializes_structures() {
        let payload = json!({ "hello": "world" });
        let value = safe_json(&payload).expect("serialization succeeds");
        assert_eq!(value, payload);
    }

    #[test]
    fn safe_json_propagates_serde_errors() {
        struct NonSerializable;

        impl Serialize for NonSerializable {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom("not serializable"))
            }
        }

        let error = safe_json(&NonSerializable).expect_err("serialization should fail");
        match error {
            NodeError::Fail {
                code,
                details,
                message,
                ..
            } => {
                assert_eq!(code, "SERDE_JSON");
                assert!(message.contains("failed to serialize"));
                assert!(details.is_some());
            }
        }
    }

    #[test]
    fn node_error_builders_set_fields() {
        let err = NodeError::new("TEST", "example")
            .with_retry(Some(500))
            .with_details(json!({ "context": "value" }));
        match err {
            NodeError::Fail {
                retryable,
                backoff_ms,
                details,
                ..
            } => {
                assert!(retryable);
                assert_eq!(backoff_ms, Some(500));
                assert_eq!(details.unwrap()["context"], "value");
            }
        }
    }
}
