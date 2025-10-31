#![cfg(feature = "serde")]

use greentic_types::{
    AllowList, ErrorCode, GreenticError, Impersonation, InvocationDeadline, NetworkPolicy, Outcome,
    PackRef, PolicyDecision, SessionCursor, SessionKey, Signature, SignatureAlgorithm, SpanContext,
    StateKey, StatePath, TenantCtx, TenantIdentity,
};
use semver::Version;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "time")]
use time::OffsetDateTime;

fn assert_roundtrip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + core::fmt::Debug,
{
    let json = serde_json::to_string_pretty(value).expect("serialize");
    let roundtrip: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(&roundtrip, value, "{json}");
}

#[test]
fn tenant_ctx_roundtrip() {
    let mut ctx = TenantCtx::new("prod".into(), "tenant-1".into())
        .with_team(Some("team-9".into()))
        .with_user(Some("user-42".into()));
    ctx.trace_id = Some("trace-1".into());
    ctx.correlation_id = Some("corr-7".into());
    ctx.idempotency_key = Some("idem-3".into());
    ctx.deadline = Some(InvocationDeadline::from_unix_millis(42));
    ctx.impersonation = Some(Impersonation {
        actor_id: "support-ops".into(),
        reason: Some("break-glass".into()),
    });

    assert_roundtrip(&ctx);

    let identity = TenantIdentity::from(&ctx);
    assert_eq!(identity.tenant_id.as_str(), "tenant-1");
    assert_roundtrip(&identity);
}

#[test]
fn session_types_roundtrip() {
    let key = SessionKey::from("sess-123");
    let cursor = SessionCursor::new("node.entry")
        .with_wait_reason("awaiting-input")
        .with_outbox_marker("outbox-1");

    assert_roundtrip(&key);
    assert_roundtrip(&cursor);
}

#[test]
fn state_types_roundtrip() {
    let key = StateKey::from("state::demo");
    let mut path = StatePath::root();
    path.push("meta");
    path.push("progress");

    assert_roundtrip(&key);
    assert_roundtrip(&path);
    assert_eq!(path.to_pointer(), "/meta/progress");
    let parsed = StatePath::from_pointer("/meta/progress");
    assert_eq!(parsed, path);
}

#[test]
fn outcome_roundtrip() {
    let done: Outcome<String> = Outcome::Done("ok".into());
    let pending: Outcome<String> = Outcome::Pending {
        reason: "waiting".into(),
        expected_input: Some(vec!["user_input".into()]),
    };
    let error = Outcome::<String>::Error {
        code: ErrorCode::InvalidInput,
        message: "bad".into(),
    };

    assert_roundtrip(&done);
    assert_roundtrip(&pending);
    assert_roundtrip(&error);
}

#[test]
fn policy_roundtrip() {
    let list = AllowList {
        domains: vec!["api.greentic.ai".into()],
        ports: vec![443],
        protocols: vec![greentic_types::Protocol::Https],
    };

    let policy = NetworkPolicy {
        egress: list,
        deny_on_miss: true,
    };

    let decision = PolicyDecision {
        allow: true,
        reason: Some("matched allow list".into()),
    };

    assert_roundtrip(&policy);
    assert_roundtrip(&decision);
}

#[test]
fn pack_signature_roundtrip() {
    let reference = PackRef::new(
        "oci://registry.greentic.ai/packs/agent",
        Version::parse("1.2.3").expect("semver"),
        "sha256:deadbeef",
    );

    let signature = Signature::new(
        "key-1",
        SignatureAlgorithm::Ed25519,
        vec![0xde, 0xad, 0xbe, 0xef],
    );

    assert_roundtrip(&reference);
    assert_roundtrip(&signature);
}

#[test]
fn span_context_roundtrip() {
    let mut span = SpanContext::new("tenant-2".into(), "flow-alpha", "runtime-core");
    span = span.with_session("sess-9".into()).with_node("node-7");
    #[cfg(feature = "time")]
    {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("timestamp");
        span = span.started(now).finished(now);
    }

    assert_roundtrip(&span);
}

#[test]
fn greentic_error_roundtrip() {
    let err = GreenticError::new(ErrorCode::Internal, "boom");
    let json = serde_json::to_string(&err).expect("serialize");
    let deser: GreenticError = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deser.code, err.code);
    assert_eq!(deser.message, err.message);
}
